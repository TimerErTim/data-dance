use crate::jobs::restore::RestoreBackupJob;
use std::path::PathBuf;

impl RestoreBackupJob {
    pub fn run_impl(&self) -> Result<(), RestoreBackupRunError> {
        // Read history to determine sequence
        let history = {
            let remote = self.remote_service.lock().unwrap();
            remote.backup_history().map_err(|err| RestoreBackupRunError::IoError {
                stage: RestoreBackupRunStage::FetchingMetadata,
                source: err,
            })?
        };
        let mut entries = history.entries.clone();
        entries.sort_by_key(|e| e.timestamp);

        use crate::objects::RestoreMetadata;
        let job_id = chrono::Utc::now().timestamp_millis().to_string();
        let metadata_path = self.jobs_folder.join(format!("restore_{}.json", job_id));
        let write_metadata = |meta: &RestoreMetadata| -> std::io::Result<()> {
            let handle = std::fs::File::create(&metadata_path)?;
            serde_json::to_writer(std::io::BufWriter::new(handle), meta)?;
            Ok(())
        };
        write_metadata(&RestoreMetadata {
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
            write_metadata(&RestoreMetadata {
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

#[derive(Debug, Error)]
pub enum RestoreBackupRunError {
    #[error("IO error during restore stage {stage:?}")]
    IoError {
        stage: RestoreBackupRunStage,
        #[source]
        source: std::io::Error,
    },
    #[error("Inner job state was manipulated. Concurrent runs are not allowed: {message}")]
    ConcurrentStateManipulation { message: String },
}

pub enum RestoreBackupRunStage {
    FetchingMetadata,
    RestoringData {
        backup_id: u32,
    },
    ClearingSnapshots,
}
