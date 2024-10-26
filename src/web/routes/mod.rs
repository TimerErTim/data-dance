pub mod fileserv;

use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use axum::Router;
use tokio::net::TcpListener;
use crate::context::DataDanceContext;

pub async fn run_server(context: DataDanceContext) -> i32 {
    let context = Arc::new(context);

    let ip_addr = context.config.web.host.parse().unwrap();
    let socket = SocketAddr::new(ip_addr, context.config.web.port);
    let Ok(listener) = TcpListener::bind(socket).await else {
        eprintln!("unable to bind to address {}", socket);
        return 2;
    };

    let Ok(routes) = try_build_routes(&context) else {
        return 4;
    };

    let server_result = start_server(
        listener,
        routes.with_state(context)
    ).await;

    match server_result {
        Ok(_) => 0,
        Err(_) => 16
    }
}

pub async fn start_server(
    tcp_listener: TcpListener,
    routes: Router
) -> Result<(), ()> {
    let server_future = axum::serve(tcp_listener, routes)
        .with_graceful_shutdown(async move { tokio::signal:: });
    server_future.await.map_err(|err| ())?;
    Ok(())
}

pub async fn web_server_service(
    context: Arc<MycologContext>,
    shutdown_token: CancellationToken,
) -> anyhow::Result<()> {
    info!("web server service starting...");
    let config = &context.config;
    let socket = SocketAddr::new(config.web_bind_ip.clone(), config.web_bind_port);
    let listener = TcpListener::bind(socket)
        .await
        .inspect_err(|err| error!(?err, "unable to bind web server address"))?;
    let routes = try_build_routes(&context)?;

    run_web_server(listener, shutdown_token, routes.with_state(context)).await
}

async fn run_web_server(
    listener: TcpListener,
    shutdown_token: CancellationToken,
    routes: Router,
) -> anyhow::Result<()> {
    let server_future = axum::serve(listener, routes)
        .with_graceful_shutdown(async move { shutdown_token.cancelled().await });
    info!("web server service started");
    Ok(server_future.await?)
}
