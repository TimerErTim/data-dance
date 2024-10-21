use std::fmt::Debug;

#[derive(Clone)]
pub enum EncryptionLevel {
    None,
    Symmetrical { password: String },
}

impl Debug for EncryptionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncryptionLevel::None => write!(f, "None"),
            EncryptionLevel::Symmetrical { password } => write!(f, "Symmetrical(?)"),
        }
    }
}
