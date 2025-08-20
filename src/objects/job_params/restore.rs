use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RestoreParams {
    pub backup_id: u32,
    pub target_folder: PathBuf,
}
