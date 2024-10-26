use serde::{Deserialize, Serialize};

mod incremental_backup;

pub use incremental_backup::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum JobResult {
    IncrementalBackup(IncrementalBackupResult),
}
