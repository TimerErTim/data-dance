use crate::config::DataDanceConfiguration;
use crate::jobs::JobExecutor;
use std::net::SocketAddr;

pub struct DataDanceContext {
    pub config: DataDanceConfiguration,
    pub executor: JobExecutor,
}

impl DataDanceContext {
    pub fn bound_socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.config.web.host.parse().unwrap(), self.config.web.port)
    }
}
