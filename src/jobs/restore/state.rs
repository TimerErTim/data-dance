use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub enum RestoreBackupState {
    Initial,
    Started {
        started_at: chrono::DateTime<chrono::Utc>,
    },
    FetchingMetadata {
        started_at: chrono::DateTime<chrono::Utc>,
        target_backup_id: u32,
    },
}

impl Default for RestoreBackupState {
    fn default() -> Self {
        Self::Initial
    }
}
