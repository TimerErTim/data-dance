use serde::{Deserialize, Serialize};

mod load;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataDanceConfiguration {
    pub port: u16,
    pub host: String,

    pub encryption_password: Option<String>,
}
