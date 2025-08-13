use crate::config::load::read_config_from_env;
use crate::context::DataDanceContext;
use crate::jobs::restore::RestoreBackupJob;
use crate::jobs::{ExecutorError, Job, JobVariant};
use crate::objects::job_params::RestoreParams;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use http::StatusCode;
use std::sync::Arc;

pub fn restoration_router(context: &Arc<DataDanceContext>) -> Router<Arc<DataDanceContext>> {
    Router::new().route("/", post(handle_restore))
}

pub async fn handle_restore(
    State(context): State<Arc<DataDanceContext>>,
    Json(params): Json<RestoreParams>,
) -> Result<Response, Response> {
    let config = match read_config_from_env() {
        Ok(config) => config,
        Err(err) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response());
        }
    };

    let job = RestoreBackupJob::from_config(config, params);

    match context.executor.submit_job(JobVariant::from(job)) {
        Ok(_) => Ok(StatusCode::ACCEPTED.into_response()),
        Err(err) => Err((StatusCode::CONFLICT, err.to_string()).into_response()),
    }
}
