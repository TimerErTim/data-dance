use crate::services::data_tunnel::DataTunnel;
use std::io::{BufRead, Error, Read, Write};

#[derive(Clone, Copy)]
pub struct PassThroughDataTunnel;

impl DataTunnel for PassThroughDataTunnel {
    fn transfer<R: Read, W: Write>(&self, mut reader: R, mut writer: W) -> Result<(), Error> {
        std::io::copy(&mut reader, &mut writer)?;
        Ok(())
    }
}
