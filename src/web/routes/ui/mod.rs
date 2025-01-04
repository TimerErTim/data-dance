use crate::web::app::App;
use crate::web::routes::fileserv::file_and_error_handler;
use axum::Router;
use leptos::{provide_context, LeptosOptions};
use leptos_axum::{generate_route_list, LeptosRoutes};
use std::sync::Arc;

use crate::context::DataDanceContext;

pub fn ui_router(leptos_options: LeptosOptions, context: &Arc<DataDanceContext>) -> Router {
    leptos_query::suppress_query_load(true);
    let routes = generate_route_list(App);
    leptos_query::suppress_query_load(false);

    let router = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                let context = Arc::clone(&context);
                move || provide_context(Arc::clone(&context))
            },
            App,
        )
        .fallback(file_and_error_handler)
        .with_state(leptos_options);
    router
}
