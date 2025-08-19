use crate::jobs::incremental_backup::state::{
    IncrementalBackupJobState, IncrementalBackupJobUploadState,
};
use crate::jobs::incremental_backup::IncrementalBackupJob;
use crate::objects::job_result::IncrementalBackupUploadResult;
use crate::objects::job_state::IncrementalBackupUploadState;
use crate::objects::{BackupEntry, BackupType, EncryptionLevel};
use crate::services::data_tunnel::{DataTunnel, TrackedTransfer};
use rand::{random, thread_rng, Rng};
use std::ops::{Deref, DerefMut};
use thiserror::Error;

impl IncrementalBackupJob {
    pub fn run_impl(&self) -> Result<IncrementalBackupUploadResult, IncrementalBackupRunError> {
        let mut history = {
            let remote_service_lock = self.remote_service.lock().unwrap();
            remote_service_lock.backup_history().map_err(|err| {
                IncrementalBackupRunError::IoError {
                    stage: IncrementalBackupRunStage::FetchingMetadata,
                    source: err,
                }
            })?
        };

        let backup_src = {
            let local_service_lock = self.local_service.lock().unwrap();
            local_service_lock
                .get_backup_source(&history)
                .map_err(|err| IncrementalBackupRunError::IoError {
                    stage: IncrementalBackupRunStage::CreatingSnapshot,
                    source: err,
                })?
        };

        let dest_filename = if backup_src.parent_backup_id.is_none() {
            backup_src
                .local_snapshot_relative
                .with_added_extension("bin")
        } else {
            backup_src
                .local_snapshot_relative
                .with_added_extension("dbin")
        };

        let dest_writer = {
            let remote_service_lock = self.remote_service.lock().unwrap();
            remote_service_lock
                .get_backup_writer(dest_filename.clone())
                .map_err(|err| IncrementalBackupRunError::IoError {
                    stage: IncrementalBackupRunStage::Uploading,
                    source: err,
                })?
        };

        let transfer = self
            .encoding_data_tunnel
            .clone()
            .tracked_transfer(backup_src.data_stream, dest_writer);

        self.update_internal_state(|old_state| {
            let started_at = match old_state {
                IncrementalBackupJobState::Started { started_at } => started_at,
                IncrementalBackupJobState::Uploading { started_at, .. } => started_at,
                _ => Err(IncrementalBackupRunError::ConcurrentStateManipulation {
                    message: "Cannot be initial state when upload starts".to_string(),
                })?,
            };

            Ok(IncrementalBackupJobState::Uploading {
                started_at: *started_at,
                uploading_state: IncrementalBackupJobUploadState {
                    parent_backup_id: backup_src.parent_backup_id,
                    local_folder_relative: backup_src.local_snapshot_relative.clone(),
                    remote_path_relative: dest_filename.clone(),
                    read_bytes: transfer.reader_bytes_counter(),
                    written_bytes: transfer.writer_bytes_counter(),
                    finishing: false,
                },
            })
        })?;

        transfer
            .run()
            .map_err(|err| IncrementalBackupRunError::IoError {
                stage: IncrementalBackupRunStage::Uploading,
                source: err,
            })?;

        self.update_internal_state(|old_state| match old_state {
            IncrementalBackupJobState::Uploading {
                started_at,
                uploading_state: old_upload_state,
            } => Ok(IncrementalBackupJobState::Uploading {
                started_at: *started_at,
                uploading_state: IncrementalBackupJobUploadState {
                    finishing: true,
                    ..old_upload_state.clone()
                },
            }),
            _ => Err(IncrementalBackupRunError::ConcurrentStateManipulation {
                message: "Cannot switch states when upload finished".to_string(),
            }),
        })?;

        let now = chrono::Utc::now();
        let new_backup_id = now.timestamp() as u32;
        let new_backup_entry = BackupEntry {
            id: new_backup_id,
            parent: backup_src.parent_backup_id,
            timestamp: now.timestamp_millis() as u64,
            remote_filename: dest_filename,
            local_snapshot: backup_src.local_snapshot_relative,
            backup_type: match backup_src.parent_backup_id {
                None => BackupType::Full,
                Some(_) => BackupType::Incremental,
            },
        };

        history.entries.push(new_backup_entry);
        {
            let remote_service_lock = self.remote_service.lock().unwrap();
            remote_service_lock
                .set_backup_history(history.clone())
                .map_err(|err| IncrementalBackupRunError::IoError {
                    stage: IncrementalBackupRunStage::StoringMetadata,
                    source: err,
                })?
        }

        {
            let local_service_lock = self.local_service.lock().unwrap();
            local_service_lock
                .clear_local_snapshots(&history)
                .map_err(|err| IncrementalBackupRunError::IoError {
                    stage: IncrementalBackupRunStage::ClearingSnapshots,
                    source: err,
                })?
        };

        {
            let remote_service_lock = self.remote_service.lock().unwrap();
            remote_service_lock
                .clear_orphaned_backups(&history)
                .map_err(|err| IncrementalBackupRunError::IoError {
                    stage: IncrementalBackupRunStage::ClearingOrphanedBackups,
                    source: err,
                })?
        };

        fn convert_id_to_incremental_backup_job_id(id: u32) -> u32 {
            id
        }

        let state_lock = self.state.lock().unwrap();
        let state = state_lock.deref();
        match state {
            IncrementalBackupJobState::Uploading {
                uploading_state, ..
            } => Ok(IncrementalBackupUploadResult {
                id: convert_id_to_incremental_backup_job_id(new_backup_id),
                parent: uploading_state
                    .parent_backup_id
                    .map(convert_id_to_incremental_backup_job_id),
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
                encrypted: match self.encoding_data_tunnel.encryption_level {
                    EncryptionLevel::None => false,
                    EncryptionLevel::Symmetrical { .. } => true,
                },
            }),
            _ => Err(IncrementalBackupRunError::ConcurrentStateManipulation {
                message: "Initial state".to_string(),
            }),
        }
    }
}

#[derive(Debug, Error)]
pub enum IncrementalBackupRunError {
    #[error("IO error during incremental backup stage {stage:?}")]
    IoError {
        stage: IncrementalBackupRunStage,
        #[source]
        source: std::io::Error,
    },
    #[error("Inner job state was manipulated. Concurrent runs are not allowed: {message}")]
    ConcurrentStateManipulation { message: String },
}

#[derive(Debug)]
pub enum IncrementalBackupRunStage {
    FetchingMetadata,
    CreatingSnapshot,
    Uploading,
    StoringMetadata,
    ClearingSnapshots,
    ClearingOrphanedBackups,
}
