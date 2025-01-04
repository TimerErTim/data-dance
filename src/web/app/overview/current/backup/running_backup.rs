use crate::objects::job_state::{BackupJobState, IncrementalBackupState};
use crate::web::app::overview::current::backup::incremental::RunningIncrementalBackup;
use leptos::{component, create_owning_memo, view, IntoView, Show, Signal};

#[component]
pub fn RunningBackup(#[prop(into)] job_state: Signal<BackupJobState>) -> impl IntoView {
    let incremental_backup = create_owning_memo(move |old| {
        let data = job_state();
        match (data,old)  {
            (BackupJobState::Incremental(state),_) => (state,true),
            (_,Some(old)) => (old,false),
            _ => unreachable!("This is not possible because `Memo` is lazy and only shown when children renders in `Show`")
        }
    });

    view! {
        <Show when=move || matches!(job_state(), BackupJobState::Incremental(_))>
            <RunningIncrementalBackup backup_state=incremental_backup />
        </Show>
    }
}
