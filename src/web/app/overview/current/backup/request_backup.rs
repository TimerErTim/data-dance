use crate::web::app::server_fns::{job_status_query, start_incremental_backup};
use leptos::{component, create_signal, spawn_local, view, IntoView};
use leptos_query::QueryResult;
use leptos_remix_icon::Icon;

#[component]
pub fn StartBackupButtons() -> impl IntoView {
    // Creates a reactive value to update the button
    let on_click = move |_| spawn_local(async { start_incremental_backup().await.unwrap() });

    view! {
        <div class="flex flex-col gap-4 items-center w-full h-full">
            <p class="text-lg text-gray-600 flex flex-col gap-2 items-center">
                <span class="relative w-fit scale-[1.75] p-0.5">
                    <Icon icon="upload-cloud-2-line"/>
                    <span class="absolute left-0 bottom-1/2 bg-gray-600 w-full h-0.5 rotate-45 rounded-full"/>
                </span>
                Currently no backup is running
            </p>
            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded text-sm" on:click=on_click>"Start Backup"</button>
        </div>
    }
}
