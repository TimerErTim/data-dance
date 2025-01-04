use crate::objects::BackupHistory;
use crate::services::data_dest::DestService;
use std::cell::RefCell;
use std::io::{Sink, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct FakeDestService {
    backup_history: Arc<Mutex<BackupHistory>>,
}

impl FakeDestService {
    pub fn empty() -> Self {
        Self::new(BackupHistory::default())
    }

    pub fn new(backup_history: BackupHistory) -> Self {
        Self {
            backup_history: Arc::new(Mutex::new(backup_history)),
        }
    }

    pub fn live_debug_data(&self) -> FakeDestServiceDebugData {
        FakeDestServiceDebugData {
            backup_history: self.backup_history.clone(),
        }
    }
}

impl DestService for FakeDestService {
    fn backup_history(&self) -> std::io::Result<BackupHistory> {
        thread::sleep(std::time::Duration::from_secs(1));
        let history = self.backup_history.lock().unwrap().clone();
        Ok(history)
    }

    fn get_backup_writer(&self, relative_file_path: PathBuf) -> std::io::Result<Box<dyn Write>> {
        Ok(Box::new(Sink::default()) as Box<dyn Write>)
    }

    fn set_backup_history(&self, history: BackupHistory) -> std::io::Result<()> {
        thread::sleep(std::time::Duration::from_secs(1));
        let mut history_lock = self.backup_history.lock().unwrap();
        *history_lock = history;
        Ok(())
    }

    fn clear_orphaned_backups(&self, history: &BackupHistory) -> std::io::Result<usize> {
        Ok(0)
    }
}

pub struct FakeDestServiceDebugData {
    backup_history: Arc<Mutex<BackupHistory>>,
}

impl FakeDestServiceDebugData {
    pub fn history(&self) -> BackupHistory {
        self.backup_history.lock().unwrap().clone()
    }
}
