use serde::{Deserialize, Serialize, Serializer};
use std::fmt::{Debug, Formatter};

#[derive(Clone, Deserialize)]
pub struct SensitiveString(String);

impl Serialize for SensitiveString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("?")
    }
}

impl Debug for SensitiveString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "?")
    }
}

impl SensitiveString {
    pub fn insecure(&self) -> &str {
        &self.0
    }
}

impl<T: Into<String>> From<T> for SensitiveString {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
