use crate::objects::job_state::{BackupJobState, IncrementalBackupStage, IncrementalBackupState};
use crate::web::app::overview::current::backup::incremental::running_stats::RunningBackupStats;
use leptos::{component, create_owning_memo, view, IntoView, Show, Signal};

pub mod running_stats;

#[component]
pub fn RunningIncrementalBackup(
    #[prop(into)] backup_state: Signal<IncrementalBackupState>,
) -> impl IntoView {
    let upload_stats = create_owning_memo(move |old| {
        let data = backup_state().stage;
        match (data,old)  {
            (IncrementalBackupStage::Uploading(stats),_) => (stats,true),
            (_,Some(old)) => (old,false),
            _ => unreachable!("This is not possible because `Memo` is lazy and only shown when children renders in `Show`")
        }
    });

    view! {
        <p>"Running Incremental Backup"</p>
        <p>"Started at: "{move || backup_state().started_at.to_rfc3339()}</p>
        <Show when=move || matches!(backup_state().stage, IncrementalBackupStage::FetchingMetadata)>
            <p>"Fetching Metadata..."</p>
        </Show>
        <Show when=move || matches!(backup_state().stage, IncrementalBackupStage::Uploading(_))>
            <RunningBackupStats upload_stats=upload_stats />
        </Show>
    }
}
