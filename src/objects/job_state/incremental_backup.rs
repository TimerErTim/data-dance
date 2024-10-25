use crate::objects::CompressionLevel;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IncrementalBackupState {
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub stage: IncrementalBackupStage,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum IncrementalBackupStage {
    FetchingMetadata,
    Uploading(IncrementalBackupUploadState),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IncrementalBackupUploadState {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub parent: Option<u32>,
    pub remote_filename: String,
    pub local_snapshot: String,
    pub bytes_read: usize,
    pub bytes_written: usize,
    pub compression_level: CompressionLevel,
    pub encrypted: bool,
    pub finishing: bool,
}
