use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RestoreBackupState {
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub current_step: Option<String>,
    pub target_backup_id: Option<u32>,
    pub last_applied_snapshot: Option<String>,
}
