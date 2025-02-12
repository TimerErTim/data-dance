use serde::{Deserialize, Serialize};

mod incremental_backup;
mod restore;

pub use incremental_backup::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum JobResult {
    IncrementalBackup(IncrementalBackupResult),
    Restore(RestoreResult),
}
