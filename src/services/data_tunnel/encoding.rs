use crate::objects::CompressionLevel;
use crate::services::data_tunnel::DataTunnel;
use std::io;
use std::io::{Read, Write};

#[derive(Clone)]
pub struct EncodingDataTunnel {
    pub compression_level: CompressionLevel,
    pub encryption_level: crate::objects::encryption::EncryptionLevel,
}

impl DataTunnel for EncodingDataTunnel {
    fn transfer<R: Read, W: Write + 'static>(
        &self,
        mut reader: R,
        writer: W,
    ) -> Result<(), io::Error> {
        let encryptor = self.encryption_level.to_encoder(writer);
        let mut compressor = self.compression_level.to_zstd_encoder(encryptor);
        io::copy(&mut reader, &mut compressor)?;
        compressor.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::channels::ChannelWriter;
    use std::io::Cursor;
    use std::sync::mpsc;

    #[test]
    fn test_encoding_data_tunnel_tunnel() {
        let mut tunnel = EncodingDataTunnel {
            compression_level: CompressionLevel::None,
            encryption_level: crate::objects::encryption::EncryptionLevel::None,
        };

        let input = b"Hello, world!";
        let (tx, rx) = mpsc::channel();

        tunnel
            .transfer(Cursor::new(input), ChannelWriter::new(tx))
            .unwrap();

        let output: Vec<u8> = rx.iter().collect();

        assert_eq!(output.as_slice(), input);
    }

    #[test]
    fn test_encoding_data_tunnel_tunnel_with_compression() {
        let mut tunnel = EncodingDataTunnel {
            compression_level: CompressionLevel::Best,
            encryption_level: crate::objects::encryption::EncryptionLevel::None,
        };

        let input = b"Hello, world!";
        let (tx, rx) = mpsc::channel();

        tunnel
            .transfer(Cursor::new(input), ChannelWriter::new(tx))
            .unwrap();

        let output: Vec<u8> = rx.iter().collect();
        assert_ne!(output.as_slice(), input);
    }

    #[test]
    fn test_encoding_data_tunnel_tunnel_with_encryption() {
        let mut tunnel = EncodingDataTunnel {
            compression_level: CompressionLevel::None,
            encryption_level: crate::objects::encryption::EncryptionLevel::Symmetrical {
                password: "pwd123".to_string(),
            },
        };

        let input = b"Hello, world!";
        let (tx, rx) = mpsc::channel();

        tunnel
            .transfer(Cursor::new(input), ChannelWriter::new(tx))
            .unwrap();

        let output: Vec<u8> = rx.iter().collect();
        assert_ne!(output.as_slice(), input);
    }
}
