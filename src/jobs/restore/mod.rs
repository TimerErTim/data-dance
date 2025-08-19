use crate::config::DataDanceConfiguration;
use crate::jobs::restore::state::RestoreBackupState;
use crate::jobs::Job;
use crate::objects::job_params::RestoreParams;
use crate::objects::job_result::{
    IncrementalBackupUploadResult, RestoreResult, RestoreResultState,
};
use crate::objects::EncryptionLevel;
use crate::services::data_dest::bare_fs::BareFsDestService;
use crate::services::data_dest::fake::FakeDestService;
use crate::services::data_dest::ssh::SshDestService;
use crate::services::data_dest::DestService;
use crate::services::data_source::btrfs::BtrfsSourceService;
use crate::services::data_source::fake::FakeSourceService;
use crate::services::data_source::SourceService;
use crate::services::data_tunnel::{DataTunnel, DecodingDataTunnel};
use crate::{config, objects};
use std::io::Read;
use std::path::PathBuf;
use std::sync::Mutex;

mod state;
#[cfg(test)]
mod tests;

pub struct RestoreBackupJob {
    decoding_data_tunnel: DecodingDataTunnel,

    remote_service: Mutex<Box<dyn DestService + Send>>,
    local_service: Mutex<Box<dyn SourceService + Send>>,

    state: Mutex<RestoreBackupState>,

    jobs_folder: PathBuf,

    target_backup_id: u32,
}

impl Job for RestoreBackupJob {
    type CompletionStats = RestoreResult;
    type RunningStats = RestoreBackupState;
    type Params = RestoreParams;

    fn from_config(config: DataDanceConfiguration, params: Self::Params) -> Self {
        let src_service = match config.local_storage.source.clone() {
            config::LocalSource::Btrfs {
                snapshots_folder,
                source_folder,
                send_compressed_data,
            } => Box::new(BtrfsSourceService::new(
                snapshots_folder,
                source_folder,
                send_compressed_data,
            )) as Box<dyn SourceService + Send>,
            config::LocalSource::Fake { backup_byte_size } => Box::new(FakeSourceService::new(
                "fake_snapshot".into(),
                backup_byte_size,
            ))
                as Box<dyn SourceService + Send>,
        };
        let dest_service: Box<dyn DestService + Send> = match config.remote_storage.dest.clone() {
            config::RemoteDestination::Local { folder } => Box::new(BareFsDestService::new(folder)),
            config::RemoteDestination::Ssh {
                hostname,
                port,
                username,
                folder,
            } => Box::new(SshDestService::new(port, hostname, username, folder)),
            config::RemoteDestination::Fake => Box::new(FakeDestService::empty()),
        };

        let decoding_tunnel = DecodingDataTunnel {
            compression_level: config.remote_storage.compression,
            encryption_level: config.remote_storage.encryption.clone().into(),
        };

        RestoreBackupJob {
            decoding_data_tunnel: decoding_tunnel,
            remote_service: Mutex::new(dest_service),
            local_service: Mutex::new(src_service),
            state: Mutex::default(),
            jobs_folder: config.local_storage.jobs_folder.clone(),
            target_backup_id: params.backup_id,
        }
    }

    fn run(&self) -> Self::CompletionStats {
        let started_at = chrono::Utc::now();
        let result = self.run_impl();
        let finished_at = chrono::Utc::now();
        RestoreResult {
            started_at,
            finished_at,
            state: match result {
                Ok(()) => RestoreResultState::Success(IncrementalBackupUploadResult {
                    id: 0,
                    parent: None,
                    remote_filename: "".into(),
                    local_snapshot: "".into(),
                    bytes_read: 0,
                    bytes_written: 0,
                    compression_level: self.decoding_data_tunnel.compression_level,
                    encrypted: matches!(
                        self.decoding_data_tunnel.encryption_level,
                        EncryptionLevel::Symmetrical { .. }
                    ),
                }),
                Err(err) => RestoreResultState::Error(err.to_string()),
            },
        }
    }

    fn stats(&self) -> Self::RunningStats {
        self.state.lock().unwrap().clone()
    }
}

impl RestoreBackupJob {
    pub fn new(
        config: DataDanceConfiguration,
        target_backup_id: u32,
        local_service: Box<dyn SourceService + Send>,
        remote_service: Box<dyn DestService + Send>,
    ) -> Self {
        let tunnel = DecodingDataTunnel {
            compression_level: config.remote_storage.compression,
            encryption_level: config.remote_storage.encryption.clone().into(),
        };
        Self {
            decoding_data_tunnel: tunnel,
            remote_service: Mutex::new(remote_service),
            local_service: Mutex::new(local_service),
            state: Mutex::default(),
            jobs_folder: config.local_storage.jobs_folder.clone(),
            target_backup_id,
        }
    }

    pub fn run_impl(&self) -> Result<(), std::io::Error> {
        // Read history to determine sequence
        let history = {
            let remote = self.remote_service.lock().unwrap();
            remote.backup_history()?
        };
        let mut entries = history.entries.clone();
        entries.sort_by_key(|e| e.timestamp);

        use crate::objects::RestoreJobMetadata;
        let job_id = chrono::Utc::now().timestamp_millis().to_string();
        let metadata_path = self.jobs_folder.join(format!("restore_{}.json", job_id));
        let write_metadata = |meta: &RestoreJobMetadata| -> std::io::Result<()> {
            let handle = std::fs::File::create(&metadata_path)?;
            serde_json::to_writer(std::io::BufWriter::new(handle), meta)?;
            Ok(())
        };
        write_metadata(&RestoreJobMetadata {
            job_id: job_id.clone(),
            target_backup_id: self.target_backup_id,
            current_backup_id: None,
            current_snapshot: None,
        })?;

        let mut previous_snapshot: Option<PathBuf> = None;
        for entry in entries {
            let reader = {
                let remote = self.remote_service.lock().unwrap();
                remote.get_backup_reader(entry.remote_filename.clone())?
            };
            let writer = {
                let local = self.local_service.lock().unwrap();
                local.get_restore_writer(entry.local_snapshot.clone())?
            };

            let transfer = self
                .decoding_data_tunnel
                .clone()
                .tracked_transfer(reader, writer);
            transfer.run()?;

            {
                let local = self.local_service.lock().unwrap();
                local.apply_restored_snapshot(
                    previous_snapshot.clone(),
                    entry.local_snapshot.clone(),
                )?;
            }

            // update metadata
            write_metadata(&RestoreJobMetadata {
                job_id: job_id.clone(),
                target_backup_id: self.target_backup_id,
                current_backup_id: Some(entry.id),
                current_snapshot: Some(entry.local_snapshot.clone()),
            })?;

            previous_snapshot = Some(entry.local_snapshot);

            if entry.id == self.target_backup_id {
                break;
            }
        }

        Ok(())
    }
}
