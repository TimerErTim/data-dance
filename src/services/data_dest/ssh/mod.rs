use crate::objects::{BackupHistory, SensitiveString};
use crate::services::data_dest::DestService;
use crate::services::data_source::btrfs::BtrfsSourceService;
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
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
}

impl DestService for SshDestService {
    fn backup_history(&self) -> std::io::Result<BackupHistory> {
        let reader = self.open_reader("backup_history.json".into())?;
        let history = serde_json::from_reader(reader)?;
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
