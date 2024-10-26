mod jobs;

use crate::context::DataDanceContext;
use crate::web::routes::api::jobs::jobs_router;
use axum::Router;
use std::sync::Arc;

pub fn api_router(context: &Arc<DataDanceContext>) -> Router<Arc<DataDanceContext>> {
    Router::new().nest("/jobs", jobs_router(context))
}
