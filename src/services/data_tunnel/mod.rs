mod decoding;
pub mod encoding;
mod mapped;
mod pass_through;
mod tracker;

pub use decoding::*;
pub use encoding::*;
pub use mapped::*;
pub use pass_through::*;
pub use tracker::*;

use std::io::{Read, Write};

pub trait DataTunnel {
    fn transfer<R: Read + 'static, W: Write + 'static>(
        &self,
        reader: R,
        writer: W,
    ) -> Result<(), std::io::Error>;

    fn tracked_transfer<R: Read + 'static, W: Write + 'static>(
        self,
        reader: R,
        writer: W,
    ) -> TrackedTransfer<Self, R, W>
    where
        Self: Sized,
    {
        TrackedTransfer::new(self, reader, writer)
    }
}
