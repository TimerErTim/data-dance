use crate::config::DataDanceConfiguration;
use crate::jobs::JobExecutor;

pub struct DataDanceContext {
    pub config: DataDanceConfiguration,
    pub executor: JobExecutor,
}
