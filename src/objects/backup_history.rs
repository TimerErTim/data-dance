use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct BackupHistory {
    pub entries: Vec<BackupEntry>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BackupEntry {
    pub id: u32,
    pub parent: Option<u32>,
    pub timestamp: u64,
    pub remote_filename: PathBuf,
    pub local_snapshot: PathBuf,
    pub backup_type: BackupType,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum BackupType {
    Full,
    Incremental,
}
