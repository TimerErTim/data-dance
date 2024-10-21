use crate::objects::CompressionLevel;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IncrementalBackupJobState {
    started_at: chrono::DateTime<chrono::Utc>,
    stage: IncrementalBackupStage,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum IncrementalBackupStage {
    FetchingMetadata,
    Uploading(IncrementalBackupUploadState),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IncrementalBackupUploadState {
    timestamp: chrono::DateTime<chrono::Utc>,
    parent: Option<i32>,
    remote_filename: String,
    local_snapshot: String,
    bytes_read: usize,
    bytes_written: usize,
    compression_level: CompressionLevel,
    encrypted: bool,
}
