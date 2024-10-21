use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CompressionLevel {
    None,
    Fast,
    Balanced,
    Best,
}
