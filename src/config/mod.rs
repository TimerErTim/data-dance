use crate::objects::{CompressionLevel, SensitiveString};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod load;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataDanceConfiguration {
    pub web: WebConfig,

    pub local_storage: LocalStorageConfig,
    pub remote_storage: RemoteStorageConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebConfig {
    pub port: u16,
    pub host: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocalStorageConfig {
    pub source: LocalSource,
    pub jobs_folder: PathBuf,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LocalSource {
    Btrfs {
        snapshots_folder: PathBuf,
        source_folder: PathBuf,
        send_compressed_data: bool,
    },
    Fake {
        backup_byte_size: usize,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RemoteStorageConfig {
    pub dest: RemoteDestination,

    pub encryption: Option<SensitiveString>,
    pub compression: CompressionLevel,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RemoteDestination {
    Ssh {
        username: String,
        hostname: String,
        port: Option<u16>,
        folder: PathBuf,
    },
    Local {
        folder: PathBuf,
    },
    Fake,
}
