use crate::objects::CompressionLevel;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Read, Write};

impl CompressionLevel {
    fn to_zstd_level(&self) -> Option<i32> {
        match self.clone() {
            CompressionLevel::None => None,
            CompressionLevel::Fast => Some(3),
            CompressionLevel::Balanced => Some(9),
            CompressionLevel::Best => Some(15),
        }
    }

    pub fn to_zstd_encoder<W: Write + 'static>(&self, w: W) -> Box<dyn Write> {
        match self.to_zstd_level() {
            None => Box::new(w),
            Some(level) => Box::new(
                zstd::stream::write::Encoder::new(w, level)
                    .unwrap()
                    .auto_finish(),
            ),
        }
    }

    pub fn to_zstd_decoder<R: Read + 'static>(&self, r: R) -> Box<dyn Read> {
        match self.to_zstd_level() {
            None => Box::new(r),
            Some(_) => Box::new(zstd::stream::read::Decoder::new(r).unwrap()),
        }
    }
}
