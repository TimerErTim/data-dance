use crate::objects::CompressionLevel;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IncrementalBackupResult {
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub finished_at: chrono::DateTime<chrono::Utc>,
    pub state: IncrementalBackupResultState,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum IncrementalBackupResultState {
    Error(String),
    Success(IncrementalBackupUploadResult),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IncrementalBackupUploadResult {
    pub id: u32,
    pub parent: Option<u32>,
    pub remote_filename: String,
    pub local_snapshot: String,
    pub bytes_read: usize,
    pub bytes_written: usize,
    pub compression_level: CompressionLevel,
    pub encrypted: bool,
}
