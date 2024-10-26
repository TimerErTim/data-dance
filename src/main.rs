#![feature(path_add_extension)]

use std::process::exit;

mod config;
mod context;
mod jobs;
mod objects;
mod scripts;
mod services;
mod web;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use crate::jobs::JobExecutor;
    use crate::web::routes::run_server;

    let config = config::load::read_config_from_env().unwrap();

    let context = context::DataDanceContext {
        executor: JobExecutor::new(config.clone()),
        config,
    };

    let exit_code = run_server(context).await;
    exit(exit_code);
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
