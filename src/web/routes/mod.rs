mod api;
pub mod fileserv;
mod ui;

use crate::context::DataDanceContext;
use crate::web::routes::api::api_router;
use crate::web::routes::fileserv::file_and_error_handler;
use crate::web::routes::ui::ui_router;
use axum::Router;
use std::str::FromStr;
use std::sync::Arc;
use tokio::net::TcpListener;

pub async fn run_server(context: DataDanceContext) -> i32 {
    let context = Arc::new(context);
    let socket = context.bound_socket_addr();
    let Ok(listener) = TcpListener::bind(socket).await else {
        eprintln!("unable to bind to address {}", socket);
        return 2;
    };

    let Ok(routes) = try_build_routes(context).await else {
        return 4;
    };

    let server_result = start_server(listener, routes).await;

    match server_result {
        Ok(_) => 0,
        Err(_) => 16,
    }
}

pub async fn start_server(tcp_listener: TcpListener, routes: Router) -> Result<(), ()> {
    println!(
        "starting server on {}",
        tcp_listener.local_addr().map_err(|_| ())?
    );
    let server_future = axum::serve(tcp_listener, routes)
        .with_graceful_shutdown(async move { tokio::signal::ctrl_c().await.unwrap_or_default() });
    server_future.await.map_err(|err| ())?;
    Ok(())
}

pub async fn try_build_routes(context: Arc<DataDanceContext>) -> Result<Router, ()> {
    let mut leptos_options = leptos::get_configuration(None)
        .await
        .inspect_err(|err| eprintln!("Error building routes: {err}"))
        .map_err(|_| ())?
        .leptos_options;

    leptos_options.site_addr = context.bound_socket_addr();

    let ui_router = ui_router(leptos_options);
    let api_router = api_router(&context).with_state(context);

    let routes = Router::new().merge(ui_router).nest("/api", api_router);
    Ok(routes)
}
