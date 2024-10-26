use crate::web::app::App;
use crate::web::routes::fileserv::file_and_error_handler;
use axum::Router;
use leptos::LeptosOptions;
use leptos_axum::{generate_route_list, LeptosRoutes};

pub fn ui_router(leptos_options: LeptosOptions) -> Router {
    let routes = generate_route_list(App);

    let router = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);
    router
}
