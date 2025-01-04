mod error_template;
mod overview;
mod server_fns;

use crate::objects::CompressionLevel;
use crate::web::app::overview::OverviewPage;
use crate::web::app::server_fns::{job_status_query, start_incremental_backup};
use chrono::LocalResult;
use error_template::{AppError, ErrorTemplate};
use leptos::logging::*;
use leptos::prelude::*;
use leptos::*;
use leptos_meta::{provide_meta_context, Link, Stylesheet, Title};
use leptos_query::{provide_query_client, QueryResult};
use leptos_router::{Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    // Provides context for managing queries
    provide_query_client();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/data_dance.css"/>
        <Link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png" />
        <Link rel="icon" type_="image/png" sizes="32x32" href="/favicon-32x32.png" />
        <Link rel="icon" type_="image/png" sizes="16x16" href="/favicon-16x16.png" />
        <Link rel="manifest" href="/site.webmanifest" />
        <Link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/remixicon@4.2.0/fonts/remixicon.css" />

        // sets the document title
        <Title text="data-dance"/>


        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
        }>
            <main class="h-dvh w-full bg-white p-2">
                <Routes>
                    <Route path="/" view=OverviewPage/>
                </Routes>
            </main>
        </Router>
    }
}
