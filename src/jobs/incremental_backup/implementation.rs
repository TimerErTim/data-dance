use crate::config::DataDanceConfiguration;
use crate::jobs::incremental_backup::state::IncrementalBackupJobState;
use crate::jobs::incremental_backup::IncrementalBackupJob;
use crate::jobs::Job;
use crate::objects;
use crate::objects::job_result::IncrementalBackupUploadResult;
use crate::objects::job_state::IncrementalBackupStage;
use crate::objects::{CompressionLevel, EncryptionLevel};
use crate::services::data_dest::bare_fs::BareFsDestService;
use crate::services::data_source::btrfs::BtrfsSourceService;
use objects::job_state::IncrementalBackupUploadState;
use std::ops::{Deref, DerefMut};

impl Job for IncrementalBackupJob {
    type CompletionStats = objects::job_result::IncrementalBackupResult;
    type RunningStats = objects::job_state::IncrementalBackupState;

    fn from_config(config: DataDanceConfiguration) -> Self {
        let src_service = BtrfsSourceService::new(
            config.local_storage.snapshots_folder.clone(),
            config.local_storage.source_folder.clone(),
            true,
        );
        let dest_service = BareFsDestService::new(config.remote_storage.dest_folder.clone());

        IncrementalBackupJob::new(config, Box::new(src_service), Box::new(dest_service))
    }

    fn run(&self) -> Self::CompletionStats {
        let started_at = chrono::Utc::now();
        self.set_internal_state(IncrementalBackupJobState::Started { started_at });

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
            IncrementalBackupJobState::Initial => objects::job_state::IncrementalBackupState {
                started_at: chrono::Utc::now(),
                stage: IncrementalBackupStage::FetchingMetadata,
            },
            IncrementalBackupJobState::Started { started_at } => {
                objects::job_state::IncrementalBackupState {
                    started_at: *started_at,
                    stage: IncrementalBackupStage::FetchingMetadata,
                }
            }
            IncrementalBackupJobState::Uploading {
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
