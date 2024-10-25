use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CompressionLevel {
    None,
    Fast,
    Balanced,
    Best,
}
