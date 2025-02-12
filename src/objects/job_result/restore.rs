use crate::objects::CompressionLevel;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RestoreResult {
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub finished_at: chrono::DateTime<chrono::Utc>,
    pub state: RestoreResultState,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RestoreResultState {
    Error(String),
    Success(crate::objects::job_result::IncrementalBackupUploadResult),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RestoreSuccess {
    pub id: u32,
    pub parent: Option<u32>,
    pub remote_filename: String,
    pub local_snapshot: String,
    pub bytes_read: u64,
    pub bytes_written: u64,
    pub compression_level: CompressionLevel,
    pub encrypted: bool,
}
