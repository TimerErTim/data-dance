use std::sync::Mutex;
use crate::config::DataDanceConfiguration;
use crate::jobs::incremental_backup::IncrementalBackupJob;
use crate::jobs::Job;
use crate::{config, objects};
use crate::objects::EncryptionLevel;
use crate::objects::job_state::{IncrementalBackupStage, IncrementalBackupUploadState};
use crate::services::data_dest::bare_fs::BareFsDestService;
use crate::services::data_dest::DestService;
use crate::services::data_dest::fake::FakeDestService;
use crate::services::data_dest::ssh::SshDestService;
use crate::services::data_source::btrfs::BtrfsSourceService;
use crate::services::data_source::fake::FakeSourceService;
use crate::services::data_source::SourceService;
use crate::services::data_tunnel::EncodingDataTunnel;

mod state;

pub struct RestoreBackupJob {
    encoding_data_tunnel: EncodingDataTunnel,

    remote_service: Mutex<Box<dyn DestService + Send>>,
    local_service: Mutex<Box<dyn SourceService + Send>>,

    state: Mutex<crate::jobs::incremental_backup::state::IncrementalBackupJobState>,
}

impl Job for IncrementalBackupJob {
    type CompletionStats = objects::job_result::;
    type RunningStats = objects::job_state::IncrementalBackupState;

    fn from_config(config: DataDanceConfiguration) -> Self {
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

        IncrementalBackupJob::new(config, src_service, dest_service)
    }

    fn run(&self) -> Self::CompletionStats {
        let started_at = chrono::Utc::now();
        self.set_internal_state(crate::jobs::incremental_backup::state::IncrementalBackupJobState::Started { started_at });

        let result = self.run_impl();

        let finished_at = chrono::Utc::now();

        objects::job_result::IncrementalBackupResult {
            started_at,
            finished_at,
            state: match result {
                Ok(result) => objects::job_result::IncrementalBackupResultState::Success(result),
                Err(err) => {
                    objects::job_result::IncrementalBackupResultState::Error(err.to_string())
                }
            },
        }
    }

    fn stats(&self) -> Self::RunningStats {
        let state_lock = self.state.lock().unwrap();
        let state = state_lock.deref();
        match state {
            crate::jobs::incremental_backup::state::IncrementalBackupJobState::Initial => objects::job_state::IncrementalBackupState {
                started_at: chrono::Utc::now(),
                stage: IncrementalBackupStage::FetchingMetadata,
            },
            crate::jobs::incremental_backup::state::IncrementalBackupJobState::Started { started_at } => {
                objects::job_state::IncrementalBackupState {
                    started_at: *started_at,
                    stage: IncrementalBackupStage::FetchingMetadata,
                }
            }
            crate::jobs::incremental_backup::state::IncrementalBackupJobState::Uploading {
                started_at,
                uploading_state,
            } => objects::job_state::IncrementalBackupState {
                started_at: *started_at,
                stage: IncrementalBackupStage::Uploading(IncrementalBackupUploadState {
                    timestamp: chrono::Utc::now(),
                    parent: uploading_state.parent_backup_id,
                    remote_filename: uploading_state
                        .remote_path_relative
                        .to_string_lossy()
                        .to_string(),
                    local_snapshot: uploading_state
                        .local_folder_relative
                        .to_string_lossy()
                        .to_string(),
                    bytes_read: uploading_state.read_bytes.value(),
                    bytes_written: uploading_state.written_bytes.value(),
                    compression_level: self.encoding_data_tunnel.compression_level,
                    encrypted: match &self.encoding_data_tunnel.encryption_level {
                        EncryptionLevel::None => false,
                        EncryptionLevel::Symmetrical { .. } => true,
                    },
                    finishing: uploading_state.finishing,
                }),
            },
        }
    }
}