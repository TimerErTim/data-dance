use crate::objects::BackupHistory;
use crate::services::data_dest::DestService;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

pub struct BareFsDestService {
    pub dest_folder: PathBuf,
}

impl BareFsDestService {
    pub fn new(dest_folder: PathBuf) -> Self {
        Self { dest_folder }
    }
}

impl DestService for BareFsDestService {
    fn backup_history(&self) -> std::io::Result<BackupHistory> {
        let file = self.dest_folder.join("backup_history.json");
        let handle = match File::open(file) {
            Ok(file) => file,
            Err(err) => {
                return if err.kind() == std::io::ErrorKind::NotFound {
                    Ok(BackupHistory { entries: vec![] })
                } else {
                    Err(err)
                }
            }
        };

        let history = serde_json::from_reader(BufReader::new(handle))?;
        Ok(history)
    }

    fn get_backup_writer(&self, relative_file_path: PathBuf) -> std::io::Result<Box<dyn Write>> {
        let file = self.dest_folder.join(relative_file_path);
        if file.is_file() {
            return Err(std::io::Error::from(std::io::ErrorKind::AlreadyExists));
        }
        let handle = File::create(file)?;
        Ok(Box::new(BufWriter::new(handle)))
    }

    fn set_backup_history(&self, history: BackupHistory) -> std::io::Result<()> {
        let file = self.dest_folder.join("backup_history.json");
        let handle = File::create(file)?;
        serde_json::to_writer(BufWriter::new(handle), &history)?;
        Ok(())
    }
}
