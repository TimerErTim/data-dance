use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RestoreMetadata {
    pub target_folder: PathBuf,
    pub backup_application_queue: Vec<u32>,
}

impl RestoreMetadata {
    pub fn new(target_folder: PathBuf, backup_application_queue: Vec<u32>) -> Result<Self, String> {
        if backup_application_queue.is_empty() {
            return Err("Backup application queue cannot be empty".to_string());
        }

        Self {
            target_folder,
            backup_application_queue,
        }
    }

    pub fn target_backup_id(&self) -> u32 {
        self.backup_application_queue.last().unwrap()
    }
}
