use std::sync::Mutex;

use crate::config::{self, DataDanceConfiguration, LocalSource, RemoteDestination};
use crate::jobs::restore::{RestoreBackupJob, RestoreBackupState};
use crate::jobs::Job;
use crate::objects::job_params::RestoreParams;
use crate::objects::job_result::RestoreResult;
use crate::objects::EncryptionLevel;
use crate::services::data_dest::{BareFsDestService, DestService, FakeDestService, SshDestService};
use crate::services::data_source::{BtrfsSourceService, FakeSourceService, SourceService};
use crate::services::data_tunnel::DecodingDataTunnel;

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

        RestoreBackupJob::new(config, params.backup_id, params.target_folder, src_service, dest_service)
    }

    fn run(&self) -> Self::CompletionStats {
        let started_at = chrono::Utc::now();
        let result = self.run_impl();
        let finished_at = chrono::Utc::now();
        RestoreResult {
            started_at,
            finished_at,
            state: match result {
                Ok(()) => RestoreResultState::Success(RestoreResult {
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
