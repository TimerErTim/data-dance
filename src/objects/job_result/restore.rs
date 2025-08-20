use crate::objects::CompressionLevel;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RestoreResult {
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub finished_at: chrono::DateTime<chrono::Utc>,
    pub state: RestoreResultState,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RestoreResultState {
    Error(String),
    Success(RestoreSuccess),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RestoreSuccess {
    pub target_backup_id: u32,
}
