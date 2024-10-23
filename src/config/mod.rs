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
    pub snapshots_folder: PathBuf,
    pub source_folder: PathBuf,
    pub jobs_folder: PathBuf,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RemoteStorageConfig {
    pub dest_folder: PathBuf,

    pub password: Option<SensitiveString>,
    pub compression: CompressionLevel,
}
