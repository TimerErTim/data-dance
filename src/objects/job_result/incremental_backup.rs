use crate::objects::CompressionLevel;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IncrementalBackupJobResult {
    started_at: chrono::DateTime<chrono::Utc>,
    finished_at: chrono::DateTime<chrono::Utc>,
    state: IncrementalBackupResultState,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum IncrementalBackupResultState {
    Error(String),
    Success(IncrementalBackupUploadResult),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IncrementalBackupUploadResult {
    parent: Option<i32>,
    remote_filename: String,
    local_snapshot: String,
    bytes_read: usize,
    bytes_written: usize,
    compression_level: CompressionLevel,
    encrypted: bool,
}
