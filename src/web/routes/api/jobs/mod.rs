pub mod history;
pub mod incremental_backup;
pub mod status;

use crate::context::DataDanceContext;
use crate::web::routes::api::jobs::history::job_history_router;
use crate::web::routes::api::jobs::incremental_backup::incremental_backup_router;
use crate::web::routes::api::jobs::status::job_status_router;
use axum::Router;
use std::sync::Arc;

pub fn jobs_router(context: &Arc<DataDanceContext>) -> Router<Arc<DataDanceContext>> {
    Router::new()
        .nest("/incremental_backup", incremental_backup_router(context))
        .nest("/status", job_status_router(context))
        .nest("/history", job_history_router(context))
}
