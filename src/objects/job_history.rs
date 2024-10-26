use crate::objects::job_result::JobResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JobHistory {
    pub entries: Vec<JobResult>,
}
