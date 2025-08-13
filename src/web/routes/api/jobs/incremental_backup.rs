use crate::config::load::{read_config_from_env, ConfigLoadError};
use crate::config::DataDanceConfiguration;
use crate::context::DataDanceContext;
use crate::jobs::incremental_backup::IncrementalBackupJob;
use crate::jobs::{BackupJobVariant, ExecutorError, Job, JobVariant};
use crate::objects::BackupHistory;
use crate::services::data_dest::fake::FakeDestService;
use crate::services::data_source::fake::FakeSourceService;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::Router;
use http::StatusCode;
use std::sync::Arc;

pub fn incremental_backup_router(context: &Arc<DataDanceContext>) -> Router<Arc<DataDanceContext>> {
    Router::new().route("/", post(handle_incremental_backup))
}

pub async fn handle_incremental_backup(
    State(context): State<Arc<DataDanceContext>>,
) -> Result<Response, Response> {
    let config = match read_config_from_env() {
        Ok(config) => config,
        Err(err) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response());
        }
    };

    let job = IncrementalBackupJob::from_config(config, ());
    
    match context
        .executor
        .submit_job(JobVariant::Backup(BackupJobVariant::IncrementalDataBackup(
            job,
        ))) {
        Ok(_) => Ok(StatusCode::ACCEPTED.into_response()),
        Err(err) => Err((StatusCode::CONFLICT, err.to_string()).into_response()),
    }
}
