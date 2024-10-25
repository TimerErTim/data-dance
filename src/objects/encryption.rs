use crate::objects::sensitive::SensitiveString;
use std::fmt::Debug;

#[derive(Clone)]
pub enum EncryptionLevel {
    None,
    Symmetrical { password: SensitiveString },
}

impl Debug for EncryptionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncryptionLevel::None => write!(f, "None"),
            EncryptionLevel::Symmetrical { password } => write!(f, "Symmetrical(?)"),
        }
    }
}

impl From<SensitiveString> for EncryptionLevel {
    fn from(value: SensitiveString) -> Self {
        EncryptionLevel::Symmetrical { password: value }
    }
}

impl<T: Into<SensitiveString>> From<Option<T>> for EncryptionLevel {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => EncryptionLevel::Symmetrical {
                password: value.into(),
            },
            None => EncryptionLevel::None,
        }
    }
}
