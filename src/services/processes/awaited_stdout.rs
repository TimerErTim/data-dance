use std::io::{Read, Result};
use std::ops::{Deref, DerefMut};

use crate::services::processes::AwaitedChild;

pub struct AwaitedStdout {
    inner: std::process::ChildStdout,
    process: AwaitedChild,
}

impl AwaitedStdout {
    pub fn new(inner: std::process::ChildStdout, process: AwaitedChild) -> Self {
        Self { inner, process }
    }
}

impl From<(std::process::ChildStdout, std::process::Child)> for AwaitedStdout {
    fn from((inner, child): (std::process::ChildStdout, std::process::Child)) -> Self {
        Self::new(inner, child.into())
    }
}

impl From<(std::process::ChildStdout, AwaitedChild)> for AwaitedStdout {
    fn from((inner, process): (std::process::ChildStdout, AwaitedChild)) -> Self {
        Self { inner, process }
    }
}

impl Deref for AwaitedStdout {
    type Target = std::process::ChildStdout;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for AwaitedStdout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Read for AwaitedStdout {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.inner.read(buf)
    }
}
