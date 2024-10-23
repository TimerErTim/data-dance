use crate::objects::CompressionLevel;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackupHistory {
    entries: Vec<BackupEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackupEntry {
    id: u32,
    timestamp: u64,
    remote_filename: String,
    local_snapshot: String,
    parent: Option<u32>,
    backup_type: BackupType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BackupType {
    Full,
    Incremental,
}
