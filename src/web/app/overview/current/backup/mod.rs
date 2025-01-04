mod incremental;
mod request_backup;
mod running_backup;

use crate::web::app::overview::current::backup::request_backup::StartBackupButtons;
use crate::web::app::overview::current::backup::running_backup::RunningBackup;
use crate::web::app::server_fns::{job_status_query, start_incremental_backup};
use incremental::running_stats::RunningBackupStats;
use leptos::{
    component, create_owning_memo, create_signal, spawn_local, view, IntoSignal, IntoView, Show,
    Transition,
};
use leptos_query::QueryResult;

#[component]
pub fn OverviewBackupSection() -> impl IntoView {
    let job_status_scope = job_status_query();
    let QueryResult {
        data: job_status, ..
    } = job_status_scope.use_query(|| ());

    view! {
        <div class="flex flex-col items-center p-4">
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                {
                    let optional_job_status = move || {
                        job_status()
                            .map(|job_status| {
                                job_status.ok().map(|job_status| job_status.backup)
                            }).flatten()
                            .flatten()
                        };
                    let job_status = create_owning_memo(move |old| {
                        let data = optional_job_status();
                        match (data,old)  {
                            (Some(data),_) => (data,true),
                            (None,Some(old)) => (old,false),
                            (None,None) => unreachable!("This is not possible because `Memo` is lazy and only shown when children renders in `Show`")
                        }
                    });
                    view! {
                        <Show when=move || optional_job_status().is_some() fallback=move || view! { <StartBackupButtons /> }>
                            <RunningBackup job_state=job_status />
                        </Show>
                    }
                }
            </Transition>
        </div>
    }
}
