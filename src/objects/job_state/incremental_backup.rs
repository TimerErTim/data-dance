use crate::objects::CompressionLevel;
use poem_openapi::{Object, Union};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Object)]
pub struct IncrementalBackupState {
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub stage: IncrementalBackupStage,
}

#[derive(Clone, Debug, Serialize, Deserialize, Union)]
#[oai(discriminator_name = "stage")]
pub enum IncrementalBackupStage {
    FetchingMetadata(FetchingMetadataState),
    Uploading(IncrementalBackupUploadState),
}

#[derive(Clone, Debug, Serialize, Deserialize, Object)]
pub struct FetchingMetadataState;

impl From<FetchingMetadataState> for IncrementalBackupStage {
    fn from(state: FetchingMetadataState) -> Self {
        IncrementalBackupStage::FetchingMetadata(state)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Object)]
pub struct IncrementalBackupUploadState {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub parent: Option<u32>,
    pub remote_filename: String,
    pub local_snapshot: String,
    pub bytes_read: u64,
    pub bytes_written: u64,
    pub compression_level: CompressionLevel,
    pub encrypted: bool,
    pub finishing: bool,
}
