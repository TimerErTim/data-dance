use serde::{Deserialize, Serialize};

mod incremental_backup;
mod restore;

pub use incremental_backup::*;
pub use restore::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JobStates {
    pub restore: Option<RestoreJobState>,
    pub backup: Option<BackupJobState>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BackupJobState {
    Incremental(IncrementalBackupState),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RestoreJobState {
    FullRestoration(RestoreBackupState),
}
