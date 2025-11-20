use poem_openapi::{Enum, NewType, Object, types::Example};
use serde::{Deserialize, Serialize};
use std::{ops::Deref, path::PathBuf};

#[derive(Clone, Debug, Default, Eq, PartialEq, Object)]
pub struct BackupHistory {
    pub entries: Vec<BackupEntry>,
}

#[derive(Clone, Debug, Eq, PartialEq, Object)]
pub struct BackupEntry {
    pub id: u32,
    pub parent: Option<u32>,
    pub timestamp: u64,
    pub remote_filename: Path,
    pub local_snapshot: Path,
    pub backup_type: BackupType,
}

#[derive(Clone, Debug, Eq, PartialEq, NewType)]
#[oai(example)]
pub struct Path(String);

impl Example for Path {
    fn example() -> Self {
        Self("some/filename.txt".to_string())
    }
}

impl<T: Into<PathBuf>> From<T> for Path {
    fn from(value: T) -> Self {
        Self(value.into().to_string_lossy().into_owned())
    }
}

impl Deref for Path {
    type Target = std::path::Path;

    fn deref(&self) -> &Self::Target {
        std::path::Path::new(&self.0)
    }
}

impl AsRef<std::path::Path> for Path {
    fn as_ref(&self) -> &std::path::Path {
        std::path::Path::new(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Enum)]
pub enum BackupType {
    Full,
    Incremental,
}
