use std::ops::{Deref, DerefMut};

pub struct AwaitedChild {
    inner: std::process::Child,
}

impl AwaitedChild {
    pub fn new(inner: std::process::Child) -> Self {
        Self { inner }
    }
}

impl From<std::process::Child> for AwaitedChild {
    fn from(inner: std::process::Child) -> Self {
        Self { inner }
    }
}

impl Deref for AwaitedChild {
    type Target = std::process::Child;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for AwaitedChild {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Drop for AwaitedChild {
    fn drop(&mut self) {
        let _ = self.inner.wait();
    }
}
