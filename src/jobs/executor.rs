use crate::jobs::variants::JobVariant;
use std::sync::mpsc::{Receiver, Sender};
use thiserror::Error;

pub struct JobExecutor {
    job_sender: Sender<JobVariant>,
}

impl JobExecutor {
    pub fn query_job(&self, job: JobVariant) -> Result<(), ExecutorError> {
        self.job_sender.send(job).unwrap();

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Error)]
pub enum ExecutorError {
    #[error("Job already running")]
    JobAlreadyRunning,
}
