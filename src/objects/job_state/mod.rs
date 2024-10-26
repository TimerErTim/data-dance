use serde::{Deserialize, Serialize};

mod incremental_backup;

pub use incremental_backup::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JobStates {
    pub restore: Option<()>,
    pub backup: Option<BackupJobState>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BackupJobState {
    Incremental(IncrementalBackupState),
}
