pub mod jobs;

use crate::context::DataDanceContext;
use crate::web::routes::api::jobs::jobs_router;
use axum::Router;
use http::StatusCode;
use std::sync::Arc;

pub fn api_router(context: &Arc<DataDanceContext>) -> Router {
    Router::new()
        .nest("/jobs", jobs_router(context))
        .with_state(Arc::clone(context))
        .fallback(async || StatusCode::NOT_FOUND)
}
