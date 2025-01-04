use crate::objects::CompressionLevel;
use leptos::{expect_context, server, ServerFnError};

#[server(IncrementalBackupServerFn, "/api", "Url")]
pub async fn start_incremental_backup() -> Result<(), ServerFnError> {
    use crate::context::DataDanceContext;
    use crate::web::routes::api::jobs::incremental_backup::handle_incremental_backup;
    use axum::extract::State;
    use std::sync::Arc;

    let state = expect_context::<Arc<DataDanceContext>>();
    handle_incremental_backup(State(state))
        .await
        .map_err(|_| std::io::Error::other("unable to start incremental backup"))?;
    Ok(())
}
