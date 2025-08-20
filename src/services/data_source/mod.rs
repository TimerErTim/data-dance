pub mod btrfs;
pub mod fake;

pub use btrfs::BtrfsSourceService;
pub use fake::FakeSourceService;

use crate::objects::{BackupHistory, RestoreMetadata};
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;

pub trait SourceService {
    fn get_backup_source(&self, backup_history: &BackupHistory) -> io::Result<SourceBackup>;

    fn clear_local_snapshots(&self, backup_history: &BackupHistory) -> io::Result<()>;

    fn get_restore_writer(&self, backup_history: &BackupHistory, restore_metadata: RestoreMetadata) -> io::Result<SourceRestore>;
}

pub struct SourceBackup {
    pub parent_backup_id: Option<u32>,
    pub local_snapshot_relative: PathBuf,
    pub data_stream: Box<dyn Read>,
}

pub struct SourceRestore {
    pub requested_backup_id: u32,
    pub data_stream: Box<dyn Write> 
}
