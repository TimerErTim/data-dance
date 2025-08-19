use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RestoreBackupState {
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub stage: RestoreJobStage,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RestoreJobStage {
    FetchingMetadata,
}
