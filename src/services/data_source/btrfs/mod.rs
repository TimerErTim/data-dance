use crate::objects::BackupHistory;
use crate::services::data_source::{SourceBackup, SourceService};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
use std::io::{Stdout, Write};
use std::path::PathBuf;
use std::process::{Child, Stdio};
use thiserror::__private::AsDisplay;

pub struct BtrfsSourceService {
    pub snapshot_folder: PathBuf,
    pub source_folder: PathBuf,
    pub compressed_send: bool,
    send_process: RefCell<Option<Child>>,
}

impl BtrfsSourceService {
    pub fn new(snapshot_folder: PathBuf, source_folder: PathBuf, compressed_send: bool) -> Self {
        Self {
            snapshot_folder,
            source_folder,
            compressed_send,
            send_process: RefCell::new(None),
        }
    }
}

impl SourceService for BtrfsSourceService {
    fn get_backup_source(&self, backup_history: &BackupHistory) -> io::Result<SourceBackup> {
        let mut entries = backup_history.entries.clone();
        entries.sort_by_key(|entry| entry.timestamp);

        let now = chrono::Utc::now();
        let new_snapshot_relative_folder = format!("snapshot_{}/", now.format("%Y-%m-%d-%H-%M-%S"));

        let mut snapshot_creation_command = std::process::Command::new("btrfs");
        snapshot_creation_command
            .args(["subvolume", "snapshot", "-r"])
            .arg(&self.source_folder)
            .arg(self.snapshot_folder.join(&new_snapshot_relative_folder));
        let snapshot_creation_status = snapshot_creation_command.status()?;
        if !snapshot_creation_status.success() {
            return Err(io::Error::from(io::ErrorKind::Other));
        }

        let mut parent_entry = None;
        for entry in entries.into_iter().rev() {
            let snapshot_path = self.snapshot_folder.join(&entry.local_snapshot);

            if snapshot_path.is_dir() {
                parent_entry = Some(entry);
                break;
            }
        }

        let mut send_command = std::process::Command::new("btrfs");
        send_command.arg("send").stdout(Stdio::piped());
        if self.compressed_send {
            send_command.arg("--compressed-data");
        }
        if let Some(parent_entry) = &parent_entry {
            send_command
                .arg("-p")
                .arg(self.snapshot_folder.join(&parent_entry.local_snapshot));
        }
        send_command.arg(self.snapshot_folder.join(&new_snapshot_relative_folder));
        let mut send_process = send_command.spawn()?;
        let output = send_process.stdout.take().unwrap();
        let mut old_process = self.send_process.replace(Some(send_process));
        if let Some(mut old_process) = old_process {
            let _ = old_process.kill();
        }

        Ok(SourceBackup {
            parent_backup_id: parent_entry.map(|e| e.id),
            local_snapshot_relative: new_snapshot_relative_folder.into(),
            data_stream: Box::new(output),
        })
    }

    fn clear_local_snapshots(&self, backup_history: &BackupHistory) -> io::Result<()> {
        let all_snapshots = self
            .snapshot_folder
            .read_dir()?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_ok_and(|ft| ft.is_dir()))
            .map(|entry| entry.path())
            .collect::<Vec<_>>();
        let mut expired_snapshots = Vec::new();

        let mut backup_entries = backup_history.entries.clone();
        backup_entries.sort_by_key(|entry| entry.timestamp);
        backup_entries.reverse();
        let latest_backup_entries: Vec<_> = backup_entries.into_iter().rev().take(2).collect();

        for snapshot in all_snapshots {
            let mut retained = false;

            for entry in latest_backup_entries.iter() {
                if snapshot.ends_with(&entry.local_snapshot) {
                    retained = true;

                    break;
                }
            }

            if retained {
                continue;
            }
            expired_snapshots.push(snapshot);
        }

        for expired_snapshot in expired_snapshots {
            let mut remove_subv_command = std::process::Command::new("btrfs");
            remove_subv_command
                .args(["subvolume", "delete", "-c"])
                .arg(&expired_snapshot);
            let remove_subv_status = remove_subv_command.status()?;
            if !remove_subv_status.success() {
                eprintln!(
                    "Failed to remove subvolume '{}' with btrfs subvolume delete status: {}",
                    expired_snapshot.display(),
                    remove_subv_status
                );
            }
        }

        Ok(())
    }

    fn get_restore_writer(&self, restored_folder: PathBuf) -> io::Result<Box<dyn Write>> {
        todo!()
    }
}

impl Drop for BtrfsSourceService {
    fn drop(&mut self) {
        if let Some(mut child) = self.send_process.take() {
            let _ = child.kill();
        }
    }
}
