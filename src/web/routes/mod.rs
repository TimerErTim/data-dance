pub mod api;
mod ui;

use crate::context::DataDanceContext;
use crate::web::routes::api::api_service;
use crate::web::routes::ui::ui_router;
use poem::listener::{Listener, TcpListener};
use poem::middleware::Cors;
use poem::{Endpoint, EndpointExt, Route, Server};
use tokio::net::ToSocketAddrs;
use std::str::FromStr;
use std::sync::Arc;

pub async fn run_server(context: DataDanceContext) -> i32 {
    let context = Arc::new(context);
    let socket = context.bound_socket_addr();
    let listener = TcpListener::bind(socket);

    let Ok(routes) = try_build_routes(context).await else {
        return 4;
    };

    println!(
        "starting server on {}",
        socket
    );
    let server_result = start_server(listener, routes).await;

    match server_result {
        Ok(_) => 0,
        Err(_) => 16,
    }
}

pub async fn start_server<L: Listener>(listener: L, routes: impl Endpoint + 'static) -> Result<(), ()> 
where L::Acceptor: 'static {
    let server_future = Server::new(listener)
        .run_with_graceful_shutdown(
            routes,
            async move { 
                tokio::signal::ctrl_c().await.unwrap_or_default();
                println!("Shutting down server...");
            },
            None,
        );
    server_future.await.map_err(|_| ())?;
    Ok(())
}

pub async fn try_build_routes(context: Arc<DataDanceContext>) -> Result<impl Endpoint, ()> {
    let ui_router = ui_router(&context);
    let api_service = api_service().server("/api");
    let api_swagger = api_service.swagger_ui();
    let api_spec = api_service.spec_endpoint();

    let api_router = Route::new()
        .nest("/", api_service.data(context))
        .nest("/swagger", api_swagger)
        .at("/spec", api_spec);

    let routes = Route::new()
        .nest("/", ui_router)
        .nest("/api", api_router)
        .with(Cors::default());
    Ok(routes)
}
