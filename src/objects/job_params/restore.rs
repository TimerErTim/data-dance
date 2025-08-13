use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RestoreParams {
    pub backup_id: u32,
}
