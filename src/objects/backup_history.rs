use crate::objects::CompressionLevel;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct BackupHistory {
    id: u32,
    timestamp: u64,
    compression_level: CompressionLevel,
    remote_filename: String,
    local_snapshot: String,
    parent: Option<u32>,
    backup_type: BackupType,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum BackupType {
    Full,
    Incremental,
}
