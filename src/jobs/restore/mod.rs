use crate::config::DataDanceConfiguration;
use crate::jobs::restore::state::RestoreBackupState;
use crate::jobs::Job;
use crate::objects::job_params::RestoreParams;
use crate::objects::job_result::{
    IncrementalBackupUploadResult, RestoreResult, RestoreResultState,
};
use crate::objects::EncryptionLevel;
use crate::services::data_dest::bare_fs::BareFsDestService;
use crate::services::data_dest::fake::FakeDestService;
use crate::services::data_dest::ssh::SshDestService;
use crate::services::data_dest::DestService;
use crate::services::data_source::btrfs::BtrfsSourceService;
use crate::services::data_source::fake::FakeSourceService;
use crate::services::data_source::SourceService;
use crate::services::data_tunnel::{DataTunnel, DecodingDataTunnel};
use crate::{config, objects};
use std::io::Read;
use std::path::PathBuf;
use std::sync::Mutex;

mod state;
mod definition;
mod run;

#[cfg(test)]
mod tests;

pub struct RestoreBackupJob {
    decoding_data_tunnel: DecodingDataTunnel,

    remote_service: Mutex<Box<dyn DestService + Send>>,
    local_service: Mutex<Box<dyn SourceService + Send>>,

    state: Mutex<RestoreBackupState>,

    jobs_folder: PathBuf,

    target_backup_id: u32,
    target_folder: PathBuf,
}

impl RestoreBackupJob {
    pub fn new(
        config: DataDanceConfiguration,
        target_backup_id: u32,
        target_folder: PathBuf,
        local_service: Box<dyn SourceService + Send>,
        remote_service: Box<dyn DestService + Send>,
    ) -> Self {
        let tunnel = DecodingDataTunnel {
            compression_level: config.remote_storage.compression,
            encryption_level: config.remote_storage.encryption.clone().into(),
        };
        Self {
            decoding_data_tunnel: tunnel,
            remote_service: Mutex::new(remote_service),
            local_service: Mutex::new(local_service),
            state: Mutex::default(),
            jobs_folder: config.local_storage.jobs_folder.clone(),
            target_backup_id,
            target_folder,
        }
    }

    pub fn set_internal_state(&self, new_state: RestoreBackupState) {
        {
            let mut state_lock = self.state.lock().unwrap();
            let state = state_lock.deref_mut();
            *state = new_state
        }
    }

    pub fn update_internal_state(
        &self,
        map_state: impl Fn(
            &RestoreBackupState,
        ) -> Result<RestoreBackupState, RestoreBackupRunError>,
    ) -> Result<(), RestoreBackupRunError> {
        let mut state_lock = self.state.lock().unwrap();
        let state = state_lock.deref();
        let new_state = map_state(state)?;
        drop(state_lock);
        self.set_internal_state(new_state);
        Ok(())
    }
}
