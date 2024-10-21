use crate::services::data_tunnel::DataTunnel;
use std::io::{Read, Write};

pub struct MappedDataTunnel<RO: Read + 'static, WO: Write + 'static> {
    pub map_reader: Box<dyn Fn(Box<dyn Read>) -> RO>,
    pub map_writer: Box<dyn Fn(Box<dyn Write>) -> WO>,
}

impl<RO: Read + 'static, WO: Write + 'static> MappedDataTunnel<RO, WO> {
    pub fn new<RF: Fn(Box<dyn Read>) -> RO + 'static, WF: Fn(Box<dyn Write>) -> WO + 'static>(
        map_reader: RF,
        map_writer: WF,
    ) -> Self {
        MappedDataTunnel {
            map_reader: Box::new(map_reader),
            map_writer: Box::new(map_writer),
        }
    }
}

impl<RO: Read, WO: Write> DataTunnel for MappedDataTunnel<RO, WO> {
    fn transfer<RI: Read + 'static, WI: Write + 'static>(
        &self,
        reader: RI,
        writer: WI,
    ) -> Result<(), std::io::Error> {
        let mut mapped_reader = (self.map_reader)(Box::new(reader));
        let mut mapped_writer = (self.map_writer)(Box::new(writer));
        std::io::copy(&mut mapped_reader, &mut mapped_writer)?;
        Ok(())
    }
}
