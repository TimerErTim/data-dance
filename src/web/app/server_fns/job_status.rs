use crate::objects::job_state::JobStates;
use leptos::{expect_context, server, ServerFnError};
use leptos_query::{create_query, QueryOptions, QueryScope};
use std::time::Duration;

#[server(JobStatusServerFn, "/api", "Url")]
pub async fn job_status() -> Result<JobStates, ServerFnError> {
    use crate::context::DataDanceContext;
    use crate::web::routes::api::jobs::status::handle_job_status;
    use axum::extract::State;
    use axum::Json;
    use std::sync::Arc;

    let state = expect_context::<Arc<DataDanceContext>>();
    let Json(states) = handle_job_status(State(state))
        .await
        .map_err(|_| std::io::Error::other("unable to get current job status"))?;
    Ok(states)
}

pub fn job_status_query() -> QueryScope<(), Result<JobStates, ServerFnError>> {
    let query = |()| job_status();

    create_query(
        query,
        QueryOptions::default().set_refetch_interval(Some(Duration::from_secs(1))),
    )
}
