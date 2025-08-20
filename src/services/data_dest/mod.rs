pub mod bare_fs;
pub mod fake;
pub mod ssh;

pub use bare_fs::*;
pub use fake::*;
pub use ssh::*;

use crate::objects;
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;

pub trait DestService {
    fn backup_history(&self) -> io::Result<objects::BackupHistory>;
    fn set_backup_history(&self, history: objects::BackupHistory) -> io::Result<()>;

    fn get_backup_writer(&self, relative_file_path: PathBuf) -> io::Result<Box<dyn Write>>;
    fn get_backup_reader(&self, relative_file_path: PathBuf) -> io::Result<Box<dyn Read>>;

    fn clear_orphaned_backups(&self, history: &objects::BackupHistory) -> io::Result<usize>;
}
