mod error_template;
mod server_fns;

use crate::objects::CompressionLevel;
use crate::web::app::server_fns::start_incremental_backup;
use error_template::{AppError, ErrorTemplate};
use leptos::{component, create_signal, spawn_local, view, Errors, IntoView, SignalUpdate};
use leptos_meta::{provide_meta_context, Link, Title};
use leptos_router::{Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        //<Stylesheet id="leptos" href="/pkg/data_dance.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>
        <Link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png" />
        <Link rel="icon" type_="image/png" sizes="32x32" href="/favicon-32x32.png" />
        <Link rel="icon" type_="image/png" sizes="16x16" href="/favicon-16x16.png" />
        <Link rel="manifest" href="/site.webmanifest" />



        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| {
        spawn_local(async {
            start_incremental_backup(CompressionLevel::Best)
                .await
                .unwrap()
        })
    };

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
