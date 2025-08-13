use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RestoreJobMetadata {
    pub job_id: String,
    pub target_backup_id: u32,
    pub current_backup_id: Option<u32>,
    pub current_snapshot: Option<PathBuf>,
}
