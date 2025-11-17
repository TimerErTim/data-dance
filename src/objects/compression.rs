use poem_openapi::Enum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, Enum)]
pub enum CompressionLevel {
    None,
    Fast,
    Balanced,
    Best,
}
