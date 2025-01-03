use crate::config::DataDanceConfiguration;
use crate::jobs::variants::{BackupJobVariant, JobVariant, RestorationJobVariant};
use crate::jobs::Job;
use crate::objects::job_result::{IncrementalBackupResultState, JobResult};
use crate::objects::job_state::{BackupJobState, JobStates};
use crate::objects::JobHistory;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufReader, BufWriter};
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use thiserror::Error;

pub struct JobExecutor {
    config: DataDanceConfiguration,

    current_restoration: Arc<Mutex<Option<Arc<RestorationJobVariant>>>>,
    current_backup: Arc<Mutex<Option<Arc<BackupJobVariant>>>>,
}

impl JobExecutor {
    pub fn new(config: DataDanceConfiguration) -> Self {
        let history_file = config
            .local_storage
            .jobs_folder
            .clone()
            .join("history.json");
        if !history_file.is_file() {
            let handle = File::create(history_file).unwrap();
            serde_json::to_writer(
                BufWriter::new(handle),
                &JobHistory {
                    entries: Vec::new(),
                },
            )
            .unwrap();
        }

        JobExecutor {
            config,
            current_restoration: Arc::new(Mutex::new(None)),
            current_backup: Arc::new(Mutex::new(None)),
        }
    }

    pub fn submit_job(&self, job: JobVariant) -> Result<(), ExecutorError> {
        let job = match job {
            JobVariant::Backup(backup_job) => {
                let mut current_backup_guard = self.current_backup.lock().unwrap();
                if current_backup_guard.is_some() {
                    return Err(ExecutorError::JobAlreadyRunning);
                }
                let job = Arc::new(backup_job);
                current_backup_guard.deref_mut().replace(Arc::clone(&job));
                JobVariantReference::Backup(job)
            }
            JobVariant::Restoration(restoration_job) => {
                let mut current_restoration_guard = self.current_restoration.lock().unwrap();
                if current_restoration_guard.is_some() {
                    return Err(ExecutorError::JobAlreadyRunning);
                }
                let job = Arc::new(restoration_job);
                current_restoration_guard
                    .deref_mut()
                    .replace(Arc::clone(&job));
                JobVariantReference::Restoration(job)
            }
        };

        self.start_job(job);

        Ok(())
    }

    fn start_job(&self, job: JobVariantReference) {
        let restoration = Arc::clone(&self.current_restoration);
        let backup = Arc::clone(&self.current_backup);

        let history_path = self.history_path();

        std::thread::spawn(move || {
            let result = job.run();

            // Push the result to history
            let persist_history_result: io::Result<()> = try {
                let mut history: JobHistory = {
                    let handle = File::open(history_path.clone())?;
                    serde_json::from_reader(BufReader::new(handle))?
                };
                history.entries.push(result);
                {
                    let handle = File::create(history_path.clone())?;
                    serde_json::to_writer(BufWriter::new(handle), &history)?;
                }
            };
            if let Err(err) = persist_history_result {
                eprintln!("Failed to persist job history: {:#?}", err);
            }

            // Clear current job
            match job {
                JobVariantReference::Backup(_) => {
                    let mut backup_guard = backup.lock().unwrap();
                    backup_guard.deref_mut().take();
                }
                JobVariantReference::Restoration(_) => {
                    let mut restoration_guard = restoration.lock().unwrap();
                    restoration_guard.deref_mut().take();
                }
            };
        });
    }

    pub fn active_jobs(&self) -> JobStates {
        let current_backup = self.current_backup.lock().unwrap();
        let current_restoration = self.current_restoration.lock().unwrap();

        JobStates {
            restore: None,
            backup: current_backup
                .as_deref()
                .map(|job| match job {
                    BackupJobVariant::FullDataBackup() => None,
                    BackupJobVariant::IncrementalDataBackup(incremental_job) => {
                        Some(BackupJobState::Incremental(incremental_job.stats()))
                    }
                })
                .flatten(),
        }
    }

    pub fn history(&self) -> io::Result<JobHistory> {
        let file = self.history_path();
        let handle = File::open(file)?;
        Ok(serde_json::from_reader(BufReader::new(handle))?)
    }

    fn history_path(&self) -> PathBuf {
        self.config
            .local_storage
            .jobs_folder
            .clone()
            .join("history.json")
    }
}

enum JobVariantReference {
    Backup(Arc<BackupJobVariant>),
    Restoration(Arc<RestorationJobVariant>),
}

enum JobVariantResult {}

impl JobVariantReference {
    pub fn run(&self) -> JobResult {
        match self {
            JobVariantReference::Backup(job) => match job.deref() {
                BackupJobVariant::FullDataBackup() => {
                    todo!()
                }
                BackupJobVariant::IncrementalDataBackup(incremental_job) => {
                    JobResult::IncrementalBackup(incremental_job.run())
                }
            },
            JobVariantReference::Restoration(job) => match job {
                &_ => {
                    todo!()
                }
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Error)]
pub enum ExecutorError {
    #[error("Job already running")]
    JobAlreadyRunning,
}
