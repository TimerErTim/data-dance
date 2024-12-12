use crate::objects::{BackupHistory, SensitiveString};
use crate::services::data_dest::DestService;
use crate::services::data_source::btrfs::BtrfsSourceService;
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, ChildStdout, Stdio};

pub struct SshDestService {
    port: Option<u16>,
    host: String,
    username: String,
    folder: PathBuf,

    write_processes: RefCell<Vec<Child>>,
    read_processes: RefCell<Vec<Child>>,
}

impl SshDestService {
    pub fn new(port: Option<u16>, host: String, username: String, folder: PathBuf) -> Self {
        SshDestService {
            port,
            host,
            username,
            folder,

            write_processes: RefCell::new(vec![]),
            read_processes: RefCell::new(vec![]),
        }
    }

    pub fn open_writer(&self, relative_file_path: PathBuf) -> std::io::Result<ChildStdin> {
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
        self.write_processes.borrow_mut().push(process);
        Ok(input)
    }

    pub fn open_reader(&self, relative_file_path: PathBuf) -> std::io::Result<ChildStdout> {
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
        self.read_processes.borrow_mut().push(process);
        Ok(output)
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
        let output = process.stdout.take().unwrap();
        let reader = BufReader::new(output);
        let mut files = vec![];
        for line in reader.lines() {
            files.push(line?.into());
        }
        Ok(files)
    }
}

impl DestService for SshDestService {
    fn backup_history(&self) -> std::io::Result<BackupHistory> {
        let reader = self.open_reader("backup_history.json".into())?;

        let history = match serde_json::from_reader(reader) {
            Err(err) => {
                return if err.is_eof() {
                    Ok(BackupHistory { entries: vec![] })
                } else {
                    Err(err)?
                }
            }
            Ok(reader) => reader,
        };
        Ok(history)
    }

    fn get_backup_writer(&self, relative_file_path: PathBuf) -> std::io::Result<Box<dyn Write>> {
        let writer = self.open_writer(relative_file_path.clone())?;
        Ok(Box::new(writer))
    }

    fn set_backup_history(&self, history: BackupHistory) -> std::io::Result<()> {
        let writer = self.open_writer("backup_history.json".into())?;
        serde_json::to_writer(writer, &history)?;
        Ok(())
    }

    fn clear_orphaned_backups(&self) -> std::io::Result<usize> {
        let mut deleted_counter = 0;
        let history = self.backup_history()?;
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
                .find(|entry| entry.remote_filename == file_name)
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

impl Drop for SshDestService {
    fn drop(&mut self) {
        for mut child in self.read_processes.take() {
            let _ = child.kill();
        }
        for mut child in self.write_processes.take() {
            let _ = child.kill();
        }
    }
}
