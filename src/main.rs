#![feature(path_add_extension)]
#![feature(result_flattening)]
#![feature(try_blocks)]
#![feature(if_let_guard)]
#![allow(warnings)]

use std::process::exit;

mod config;
mod context;
mod jobs;
mod objects;
mod scripts;
mod services;
mod web;

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
