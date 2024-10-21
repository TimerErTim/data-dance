use crate::objects::CompressionLevel;
use data_dance::objects::encryption::EncryptionLevel;
use std::path::PathBuf;

pub struct FullDataBackupJobOptions {
    pub filesystem_root: PathBuf,
    pub remote_root: PathBuf,
    pub compression: CompressionLevel,
    pub encryption: EncryptionLevel,
}

pub struct FullDataBackupJob {
    options: FullDataBackupJobOptions,
}
