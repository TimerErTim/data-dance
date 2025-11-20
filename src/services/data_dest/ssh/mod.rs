use poem_openapi::types::{ParseFromJSON, ToJSON};
use serde_json::Value;

use crate::objects::{BackupHistory, SensitiveString, Path};
use crate::services::data_dest::DestService;
use crate::services::data_source::btrfs::BtrfsSourceService;
use crate::services::processes::{AwaitedChild, AwaitedStdin};
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, ChildStdout, Stdio};
use std::thread;
use std::time::Duration;

pub struct SshDestService {
    port: Option<u16>,
    host: String,
    username: String,
    folder: PathBuf,
}

impl SshDestService {
    pub fn new(port: Option<u16>, host: String, username: String, folder: PathBuf) -> Self {
        SshDestService {
            port,
            host,
            username,
            folder,
        }
    }

    pub fn open_writer(
        &self,
        relative_file_path: PathBuf,
    ) -> std::io::Result<(ChildStdin, AwaitedChild)> {
        let mut command = std::process::Command::new("ssh");
        if let Some(port) = self.port {
            command.args(["-p", &port.to_string()]);
        }
        command
            .args(["-o", "Compression no"])
            .arg(format!("{}@{}", self.username, self.host))
            .arg("dd")
            .arg(format!(
                "of={}",
                self.folder.join(&relative_file_path).display()
            ))
            .arg("conv=fsync")
            .arg("bs=4M")
            .stdin(Stdio::piped())
            .stdout(Stdio::null());

        let mut process = command.spawn()?;
        let input = process.stdin.take().unwrap();
        Ok((input, process.into()))
    }

    pub fn open_reader(
        &self,
        relative_file_path: PathBuf,
    ) -> std::io::Result<(ChildStdout, AwaitedChild)> {
        let mut command = std::process::Command::new("ssh");
        if let Some(port) = self.port {
            command.args(["-p", &port.to_string()]);
        }
        command
            .args(["-o", "Compression no"])
            .arg(format!("{}@{}", self.username, self.host))
            .arg("dd")
            .arg(format!(
                "if={}",
                self.folder.join(&relative_file_path).display()
            ))
            .arg("bs=4M")
            .stdout(Stdio::piped())
            .stdin(Stdio::null());

        let mut process = command.spawn()?;
        let output = process.stdout.take().unwrap();
        Ok((output, process.into()))
    }

    pub fn remove_file(&self, relative_file_path: PathBuf) -> std::io::Result<()> {
        let mut command = std::process::Command::new("ssh");
        if let Some(port) = self.port {
            command.args(["-p", &port.to_string()]);
        }
        command
            .args(["-o", "Compression no"])
            .arg(format!("{}@{}", self.username, self.host))
            .arg("rm")
            .arg(format!(
                "{}",
                self.folder.join(&relative_file_path).display()
            ))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .stdin(Stdio::null());
        command
            .status()
            .map(|status| {
                if status.success() {
                    Ok(())
                } else {
                    Err(std::io::Error::from(std::io::ErrorKind::Other))
                }
            })
            .flatten()
    }

    pub fn move_file(&self, relative_from: PathBuf, relative_to: PathBuf) -> std::io::Result<()> {
        let mut command = std::process::Command::new("ssh");
        if let Some(port) = self.port {
            command.args(["-p", &port.to_string()]);
        }
        command
            .args(["-o", "Compression no"])
            .arg(format!("{}@{}", self.username, self.host))
            .arg("mv -f")
            .arg(format!("{}", self.folder.join(&relative_from).display()))
            .arg(format!("{}", self.folder.join(&relative_to).display()))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .stdin(Stdio::null());
        command
            .status()
            .map(|status| {
                if status.success() {
                    Ok(())
                } else {
                    Err(std::io::Error::from(std::io::ErrorKind::Other))
                }
            })
            .flatten()
    }

    pub fn list_files(&self) -> std::io::Result<Vec<PathBuf>> {
        let mut command = std::process::Command::new("ssh");
        if let Some(port) = self.port {
            command.args(["-p", &port.to_string()]);
        }
        command
            .args(["-o", "Compression no"])
            .arg(format!("{}@{}", self.username, self.host))
            .arg("ls")
            .arg(format!("{}", self.folder.display()))
            .stdout(Stdio::piped())
            .stdin(Stdio::null());
        let mut process = command.spawn()?;
        let output = process.wait_with_output()?;
        let reader = BufReader::new(output.stdout.as_slice());
        let mut files = vec![];
        for line in reader.lines() {
            files.push(line?.into());
        }
        Ok(files)
    }
}

impl SshDestService {
    fn read_history_at(
        &self,
        relative_file_path: impl Into<PathBuf>,
    ) -> std::io::Result<BackupHistory> {
        let (reader, _) = self.open_reader(relative_file_path.into())?;

        let reader_content: Option<Value> = serde_json::from_reader(reader)?;
        match reader_content {
            Some(value) => Ok(BackupHistory::parse_from_json(Some(value)).map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.message()))?),
            None => Ok(BackupHistory { entries: vec![] }),
        }
    }
}

impl DestService for SshDestService {
    fn backup_history(&self) -> std::io::Result<BackupHistory> {
        self.read_history_at("backup_history.json")
    }

    fn get_backup_writer(&self, relative_file_path: PathBuf) -> std::io::Result<Box<dyn Write>> {
        let (writer, process) = self.open_writer(relative_file_path.clone())?;
        Ok(Box::new(AwaitedStdin::new(writer, process)))
    }

    fn set_backup_history(&self, history: BackupHistory) -> std::io::Result<()> {
        let try_setting_history = || -> std::io::Result<()> {
            let (writer, write_process) = self.open_writer("bh_new.json".into())?;
            serde_json::to_writer(writer, &history.to_json())?;
            drop(write_process);
            thread::sleep(Duration::from_secs(1));
            let written_history = self.read_history_at("bh_new.json")?;
            if written_history != history {
                return Err(std::io::Error::other(
                    "backup history not written correctly",
                ));
            }
            self.move_file("bh_new.json".into(), "backup_history.json".into())?;
            Ok(())
        };

        let mut last_error = std::io::Error::from(std::io::ErrorKind::Other);
        for i in 0..5 {
            if let Err(err) = try_setting_history() {
                eprintln!("Error setting backup history: {err}");
                last_error = err;
                thread::sleep(Duration::from_secs(2u64.pow(i)));
            } else {
                return Ok(());
            }
        }
        Err(last_error)
    }

    fn clear_orphaned_backups(&self, history: &BackupHistory) -> std::io::Result<usize> {
        let mut deleted_counter = 0;
        let mut all_backup_file_names = vec![];

        for entry in self.list_files()? {
            let Some(file_extension) = entry.extension() else {
                continue;
            };
            if file_extension == "bin" || file_extension == "dbin" {
                all_backup_file_names.push(entry);
            }
        }

        for file_name in all_backup_file_names {
            if history
                .entries
                .iter()
                .find(|entry| entry.remote_filename == Path::from(file_name.clone()))
                .is_none()
            {
                if self.remove_file(file_name).is_ok() {
                    deleted_counter += 1;
                }
            }
        }

        Ok(deleted_counter)
    }
}
