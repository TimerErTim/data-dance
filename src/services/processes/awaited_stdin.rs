use crate::services::processes::AwaitedChild;
use std::fmt::Arguments;
use std::io::{IoSlice, Write};
use std::ops::{Deref, DerefMut};

pub struct AwaitedStdin {
    inner: std::process::ChildStdin,
    process: AwaitedChild,
}

impl AwaitedStdin {
    pub fn new(inner: std::process::ChildStdin, process: AwaitedChild) -> Self {
        Self { inner, process }
    }
}

impl From<(std::process::ChildStdin, std::process::Child)> for AwaitedStdin {
    fn from((inner, child): (std::process::ChildStdin, std::process::Child)) -> Self {
        Self::new(inner, child.into())
    }
}

impl From<(std::process::ChildStdin, AwaitedChild)> for AwaitedStdin {
    fn from((inner, process): (std::process::ChildStdin, AwaitedChild)) -> Self {
        Self { inner, process }
    }
}

impl Deref for AwaitedStdin {
    type Target = std::process::ChildStdin;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for AwaitedStdin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Write for AwaitedStdin {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }

    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> std::io::Result<usize> {
        self.inner.write_vectored(bufs)
    }

    fn is_write_vectored(&self) -> bool {
        self.inner.is_write_vectored()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.inner.write_all(buf)
    }

    fn write_fmt(&mut self, fmt: Arguments<'_>) -> std::io::Result<()> {
        self.inner.write_fmt(fmt)
    }
}
