use crate::objects::job_state::JobStates;
use crate::objects::JobHistory;
use leptos::{expect_context, server, ServerFnError};
use leptos_query::{create_query, QueryOptions, QueryScope};
use std::time::Duration;

#[server(JobHistoryServerFn, "/api", "Url")]
pub async fn job_history() -> Result<JobHistory, ServerFnError> {
    use crate::context::DataDanceContext;
    use crate::web::routes::api::jobs::history::handle_job_history;
    use axum::extract::State;
    use axum::Json;
    use std::sync::Arc;

    let state = expect_context::<Arc<DataDanceContext>>();
    let Json(history) = handle_job_history(State(state))
        .await
        .map_err(|_| std::io::Error::other("unable to get current job history"))?;
    Ok(history)
}

pub fn job_history_query() -> QueryScope<(), Result<JobHistory, ServerFnError>> {
    let query = |()| job_history();

    create_query(
        query,
        QueryOptions::default().set_refetch_interval(Some(Duration::from_secs(5))),
    )
}
