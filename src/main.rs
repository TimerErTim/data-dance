#![feature(path_add_extension)]
#![feature(result_flattening)]
#![feature(try_blocks)]
#![feature(if_let_guard)]
#![allow(warnings)]

use std::process::exit;

#[tokio::main]
async fn main() {
    use data_dance::jobs::JobExecutor;
    use data_dance::web::routes::run_server;

    let config = data_dance::config::load::read_config_from_env().unwrap();

    let context = data_dance::context::DataDanceContext {
        executor: JobExecutor::new(config.clone()),
        config,
    };

    let exit_code = run_server(context).await;
    exit(exit_code);
}
