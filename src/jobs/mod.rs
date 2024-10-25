use crate::config::DataDanceConfiguration;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

mod executor;
mod full_backup;
mod incremental_backup;
mod restore;
mod variants;

pub trait Job {
    type CompletionStats: Serialize + DeserializeOwned;
    type RunningStats: Serialize + DeserializeOwned;

    fn from_config(config: DataDanceConfiguration) -> Self;

    fn run(&self) -> Self::CompletionStats;

    fn stats(&self) -> Self::RunningStats;
}
