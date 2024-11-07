mod implementation;
mod run;
mod state;
#[cfg(test)]
mod tests;

use crate::config::DataDanceConfiguration;
use crate::jobs::incremental_backup::run::IncrementalBackupRunError;
use crate::jobs::incremental_backup::state::IncrementalBackupJobState;
use crate::services::data_dest::DestService;
use crate::services::data_source::SourceService;
use crate::services::data_tunnel::{DataTunnel, EncodingDataTunnel, TrackedTransfer};
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::sync::Mutex;

pub struct IncrementalBackupJob {
    encoding_data_tunnel: EncodingDataTunnel,

    remote_service: Mutex<Box<dyn DestService + Send>>,
    local_service: Mutex<Box<dyn SourceService + Send>>,

    state: Mutex<IncrementalBackupJobState>,
}

impl IncrementalBackupJob {
    pub fn new(
        config: DataDanceConfiguration,
        local_service: Box<dyn SourceService + Send>,
        remote_service: Box<dyn DestService + Send>,
    ) -> Self {
        let data_tunnel = EncodingDataTunnel {
            compression_level: config.remote_storage.compression,
            encryption_level: config.remote_storage.encryption.clone().into(),
        };

        Self {
            encoding_data_tunnel: data_tunnel,

            remote_service: Mutex::new(remote_service),
            local_service: Mutex::new(local_service),

            state: Mutex::default(),
        }
    }

    pub fn set_internal_state(&self, new_state: IncrementalBackupJobState) {
        {
            let mut state_lock = self.state.lock().unwrap();
            let state = state_lock.deref_mut();
            *state = new_state
        }
    }

    pub fn update_internal_state(
        &self,
        map_state: impl Fn(
            &IncrementalBackupJobState,
        ) -> Result<IncrementalBackupJobState, IncrementalBackupRunError>,
    ) -> Result<(), IncrementalBackupRunError> {
        let mut state_lock = self.state.lock().unwrap();
        let state = state_lock.deref();
        let new_state = map_state(state)?;
        drop(state_lock);
        self.set_internal_state(new_state);
        Ok(())
    }
}
