pub mod btrfs;
pub mod fake;

use crate::objects::BackupHistory;
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;

pub trait SourceService {
    fn get_backup_source(&self, backup_history: &BackupHistory) -> io::Result<SourceBackup>;

    fn clear_local_snapshots(&self, backup_history: &BackupHistory) -> io::Result<()>;

    fn get_restore_writer(&self, restored_folder: PathBuf) -> io::Result<Box<dyn Write>>;
}

pub struct SourceBackup {
    pub parent_backup_id: Option<u32>,
    pub local_snapshot_relative: PathBuf,
    pub data_stream: Box<dyn Read>,
}
