use crate::objects::CompressionLevel;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Read, Write};

impl CompressionLevel {
    fn to_zstd_level(&self) -> i32 {
        match self.clone() {
            CompressionLevel::None => -5,
            CompressionLevel::Fast => 3,
            CompressionLevel::Balanced => 9,
            CompressionLevel::Best => 15,
        }
    }

    pub fn to_zstd_encoder<W: Write + 'static>(&self, w: W) -> impl Write {
        zstd::stream::write::Encoder::new(w, self.to_zstd_level())
            .unwrap()
            .auto_finish()
    }

    pub fn to_zstd_decoder<R: Read + 'static>(&self, r: R) -> impl Read {
        zstd::stream::read::Decoder::new(r).unwrap()
    }
}
