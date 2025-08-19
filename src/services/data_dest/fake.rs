use crate::objects::BackupHistory;
use crate::services::data_dest::DestService;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{Empty, Read, Sink, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone)]
pub struct FakeDestService {
    backup_history: Arc<Mutex<BackupHistory>>,
    files: Arc<Mutex<HashMap<PathBuf, Vec<u8>>>>,
}

impl FakeDestService {
    pub fn empty() -> Self {
        Self::new(BackupHistory::default())
    }

    pub fn new(backup_history: BackupHistory) -> Self {
        Self {
            backup_history: Arc::new(Mutex::new(backup_history)),
            files: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn live_debug_data(&self) -> FakeDestServiceDebugData {
        FakeDestServiceDebugData {
            backup_history: self.backup_history.clone(),
            files: self.files.clone(),
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
        let mut files = self.files.lock().unwrap();
        let entry = files
            .entry(relative_file_path.clone())
            .or_insert_with(Vec::new);
        let buffer_ref = Arc::new(Mutex::new(Vec::<u8>::new()));
        // ensure clean slate
        *entry = Vec::new();
        Ok(Box::new(VecWriter {
            path: relative_file_path,
            files: self.files.clone(),
        }) as Box<dyn Write>)
    }

    fn get_backup_reader(&self, relative_file_path: PathBuf) -> std::io::Result<Box<dyn Read>> {
        let files = self.files.lock().unwrap();
        let bytes = files.get(&relative_file_path).cloned().unwrap_or_default();
        Ok(Box::new(std::io::Cursor::new(bytes)) as Box<dyn Read>)
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
    files: Arc<Mutex<HashMap<PathBuf, Vec<u8>>>>,
}

impl FakeDestServiceDebugData {
    pub fn history(&self) -> BackupHistory {
        self.backup_history.lock().unwrap().clone()
    }

    pub fn file_bytes(&self, path: &PathBuf) -> Option<Vec<u8>> {
        self.files.lock().unwrap().get(path).cloned()
    }
}

struct VecWriter {
    path: PathBuf,
    files: Arc<Mutex<HashMap<PathBuf, Vec<u8>>>>,
}

impl Write for VecWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut files = self.files.lock().unwrap();
        let entry = files.entry(self.path.clone()).or_insert_with(Vec::new);
        entry.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
