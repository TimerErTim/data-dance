pub mod bare_fs;
#[cfg(test)]
pub mod fake;

use crate::objects;
use std::io;
use std::io::Write;
use std::path::PathBuf;

pub trait DestService {
    fn backup_history(&self) -> io::Result<objects::BackupHistory>;

    fn get_backup_writer(&self, relative_file_path: PathBuf) -> io::Result<Box<dyn Write>>;
    fn set_backup_history(&self, history: objects::BackupHistory) -> io::Result<()>;
}
