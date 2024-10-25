use crate::objects::{BackupEntry, BackupHistory};
use crate::services::data_source::{SourceBackup, SourceService};
use rand::{thread_rng, Rng, RngCore};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{Read, Repeat, Write};
use std::ops::DerefMut;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;
use std::{io, thread};

pub struct FakeSourceService {
    pub local_snapshot: PathBuf,
    pub backup_byte_size: usize,
    local_snapshots_cleared: Arc<Mutex<bool>>,
}

impl FakeSourceService {
    pub fn new(local_snapshot: PathBuf, backup_byte_size: usize) -> Self {
        Self {
            local_snapshot,
            backup_byte_size,
            local_snapshots_cleared: Arc::new(Mutex::new(false)),
        }
    }

    pub fn live_debug_data(&self) -> FakeSourceServiceDebugData {
        FakeSourceServiceDebugData {
            local_snapshots_cleared: Arc::clone(&self.local_snapshots_cleared),
        }
    }
}

impl SourceService for FakeSourceService {
    fn get_backup_source(&self, backup_history: &BackupHistory) -> io::Result<SourceBackup> {
        thread::sleep(Duration::from_secs(2));

        let mut latest_backup: Option<BackupEntry> = None;
        for backup in backup_history.entries.clone() {
            if let Some(latest) = &latest_backup {
                if backup.timestamp >= latest.timestamp {
                    latest_backup = Some(backup);
                }
            } else {
                latest_backup = Some(backup);
            }
        }

        Ok(SourceBackup {
            parent_backup_id: latest_backup.map(|b| b.id),
            local_snapshot_relative: self.local_snapshot.clone(),
            data_stream: Box::new(RandomByteReader::new(thread_rng(), self.backup_byte_size)),
        })
    }

    fn clear_local_snapshots(&self, backup_history: &BackupHistory) -> io::Result<()> {
        let mut lock = self.local_snapshots_cleared.lock().unwrap();
        *lock.deref_mut() = true;

        thread::sleep(Duration::from_secs(1));

        Ok(())
    }

    fn get_restore_writer(&self, restored_folder: PathBuf) -> std::io::Result<Box<dyn Write>> {
        todo!()
    }
}

pub struct FakeSourceServiceDebugData {
    local_snapshots_cleared: Arc<Mutex<bool>>,
}

impl FakeSourceServiceDebugData {
    pub fn local_snapshots_cleared(&self) -> bool {
        *self.local_snapshots_cleared.lock().unwrap()
    }
}

pub struct RandomByteReader<R: RngCore> {
    pub rng: R,
    total_bytes: usize, // Total number of bytes to generate
    bytes_read: usize,  // Bytes read so far
}

impl<R: RngCore> RandomByteReader<R> {
    /// Create a new RandomByteReader that generates `total_bytes` random bytes
    pub fn new(rng: R, total_bytes: usize) -> Self {
        Self {
            rng,
            total_bytes,
            bytes_read: 0,
        }
    }
}

impl<R: RngCore> Read for RandomByteReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Calculate how many bytes can still be generated
        let remaining_bytes = self.total_bytes.saturating_sub(self.bytes_read);

        // If no bytes left to generate, return Ok(0) to indicate EOF
        if remaining_bytes == 0 {
            return Ok(0);
        }

        // Determine how many bytes we can read in this call
        let bytes_to_generate = remaining_bytes.min(buf.len());

        if bytes_to_generate == buf.len() {
            self.rng.fill_bytes(buf);
        } else {
            for byte in &mut buf[..bytes_to_generate] {
                *byte = self.rng.gen();
            }
        }

        // Update the bytes_read counter
        self.bytes_read += bytes_to_generate;

        Ok(bytes_to_generate)
    }
}
