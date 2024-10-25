use crate::services::data_tunnel::{EncodingDataTunnel, TrackedTransfer};
use crate::services::tracking::BytesCounter;
use std::io::{Read, Write};
use std::path::PathBuf;

pub(crate) enum IncrementalBackupJobState {
    Initial,
    Started {
        started_at: chrono::DateTime<chrono::Utc>,
    },
    Uploading {
        started_at: chrono::DateTime<chrono::Utc>,
        uploading_state: IncrementalBackupJobUploadState,
    },
}

#[derive(Clone)]
pub struct IncrementalBackupJobUploadState {
    pub parent_backup_id: Option<u32>,
    pub remote_path_relative: PathBuf,
    pub local_folder_relative: PathBuf,
    pub read_bytes: BytesCounter,
    pub written_bytes: BytesCounter,
    pub finishing: bool,
}

impl Default for IncrementalBackupJobState {
    fn default() -> Self {
        Self::Initial
    }
}
