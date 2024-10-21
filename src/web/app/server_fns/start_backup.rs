use crate::objects::CompressionLevel;
use leptos::{server, ServerFnError};

#[server(IncrementalBackupServerFn, "/api", "Url", "backup/incremental")]
pub async fn start_incremental_backup(
    compression_level: CompressionLevel,
) -> Result<(), ServerFnError> {
    Ok(())
}
