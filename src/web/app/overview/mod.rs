use crate::web::app::overview::current::backup::OverviewBackupSection;
use crate::web::app::server_fns::{job_status_query, start_incremental_backup};
use leptos::{component, create_signal, spawn_local, view, IntoView};
use leptos_meta::Title;
use leptos_query::QueryResult;

mod current;

#[component]
pub fn OverviewPage() -> impl IntoView {
    view! {
        <Title text="data-dance - Overview"/>

        <p class="text-3xl text-gray-800 font-medium tracking-wider mb-4">"Overview Page"</p>
        <div class="flex flex-row w-full">
            <div class="w-1/2 mx-1 border border-gray-100">
                <OverviewBackupSection />
            </div>

            <div class="w-0.5 rounded-full bg-gray-400">
            </div>

            <div class="w-1/2 bg-blue-200 mx-1 border border-gray-100">
            </div>
        </div>
    }
}
