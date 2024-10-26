use crate::context::DataDanceContext;
use crate::objects::JobHistory;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use http::StatusCode;
use std::sync::Arc;

pub fn job_history_router(context: &Arc<DataDanceContext>) -> Router<Arc<DataDanceContext>> {
    Router::new().route("/", get(handle_job_history))
}

async fn handle_job_history(
    State(context): State<Arc<DataDanceContext>>,
) -> Result<Json<JobHistory>, Response> {
    Ok(Json(context.executor.history().map_err(|err| {
        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
    })?))
}
