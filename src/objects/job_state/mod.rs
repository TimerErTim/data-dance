use poem_openapi::{Object, Union};
use serde::{Deserialize, Serialize};

mod incremental_backup;

pub use incremental_backup::*;

#[derive(Clone, Debug, Serialize, Deserialize, Object)]
pub struct JobStates {
    pub restore: Option<RestoreJobState>,
    pub backup: Option<BackupJobState>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Union)]
#[oai(discriminator_name = "type")]
pub enum BackupJobState {
    Incremental(IncrementalBackupState),
}

#[derive(Clone, Debug, Serialize, Deserialize, Object)]
pub struct RestoreJobState;