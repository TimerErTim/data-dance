use crate::context::DataDanceContext;
use crate::objects::job_state::JobStates;
use crate::objects::JobHistory;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use http::StatusCode;
use std::sync::Arc;

pub fn job_status_router(context: &Arc<DataDanceContext>) -> Router<Arc<DataDanceContext>> {
    Router::new().route("/", get(handle_job_status))
}

pub async fn handle_job_status(
    State(context): State<Arc<DataDanceContext>>,
) -> Result<Json<JobStates>, Response> {
    Ok(Json(context.executor.active_jobs()))
}
