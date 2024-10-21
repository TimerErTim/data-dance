use serde::{Deserialize, Serialize};

mod incremental_backup;

pub use incremental_backup::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JobStates {
    restore: Option<usize>,
    backup: Option<BackupJobState>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BackupJobState {
    Incremental(IncrementalBackupJobState),
}
