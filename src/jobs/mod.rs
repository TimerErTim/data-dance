use crate::config::DataDanceConfiguration;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

mod executor;
mod full_backup;
pub mod restore;
pub mod incremental_backup;
mod variants;

pub use executor::*;
pub use variants::*;

pub trait Job {
    type CompletionStats: Serialize + DeserializeOwned;
    type RunningStats: Serialize + DeserializeOwned;
    type Params: Serialize + DeserializeOwned;

    fn from_config(config: DataDanceConfiguration, params: Self::Params) -> Self;

    fn run(&self) -> Self::CompletionStats;

    fn stats(&self) -> Self::RunningStats;
}
