use poem_openapi::{Object, Union};
use serde::{Deserialize, Serialize};

mod incremental_backup;

pub use incremental_backup::*;

#[derive(Clone, Debug, Serialize, Deserialize, Object)]
pub struct JobStates {
    /// Contains the state of the restore job if it is running.
    pub restore: Option<RestoreJobState>,
    /// Contains the state of the backup job if it is running.
    pub backup: Option<BackupJobState>,
}

/// This is a BackupJobState union type. It is used to represent the state of a backup job.
#[derive(Clone, Debug, Serialize, Deserialize, Union)]
#[oai(discriminator_name = "type")]
pub enum BackupJobState {
    Incremental(IncrementalBackupState),
}

#[derive(Clone, Debug, Serialize, Deserialize, Object)]
pub struct RestoreJobState;