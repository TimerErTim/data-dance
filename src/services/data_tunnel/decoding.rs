use crate::objects::{CompressionLevel, EncryptionLevel};
use crate::services::data_tunnel::DataTunnel;
use std::io;
use std::io::{BufRead, Read, Write};

#[derive(Clone)]
pub struct DecodingDataTunnel {
    pub compression_level: CompressionLevel,
    pub encryption_level: EncryptionLevel,
}

impl DataTunnel for DecodingDataTunnel {
    fn transfer<R: Read + 'static, W: Write + 'static>(
        &self,
        reader: R,
        mut writer: W,
    ) -> Result<(), io::Error> {
        let decryptor = self.encryption_level.to_decoder(reader);
        let mut decompressor = self.compression_level.to_zstd_decoder(decryptor);
        io::copy(&mut decompressor, &mut writer)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::CompressionLevel;
    use crate::services::channels::ChannelWriter;
    use crate::services::data_tunnel::encoding::EncodingDataTunnel;
    use std::io::Cursor;
    use std::sync::mpsc;

    #[derive(Clone)]
    struct TunnelOptions {
        compression_level: CompressionLevel,
        encryption_level: EncryptionLevel,
    }

    fn test_preserve(input: &str, options: TunnelOptions) {
        let (input, output) = encode_decode(input, options.clone(), options.clone()).unwrap();
        assert_eq!(input, output);
    }

    fn encode_decode(
        input: &str,
        encoding_options: TunnelOptions,
        decoding_options: TunnelOptions,
    ) -> io::Result<(Vec<u8>, Vec<u8>)> {
        let mut encoder = EncodingDataTunnel {
            compression_level: encoding_options.compression_level,
            encryption_level: encoding_options.encryption_level,
        };

        let mut decoder = DecodingDataTunnel {
            compression_level: decoding_options.compression_level,
            encryption_level: decoding_options.encryption_level,
        };

        let input_bytes: Vec<u8> = input.bytes().collect();

        let (tx, rx) = mpsc::channel();
        encoder.transfer(Cursor::new(input_bytes.clone()), ChannelWriter::new(tx))?;
        let encoded: Vec<u8> = rx.iter().collect();

        let (tx, rx) = mpsc::channel();
        decoder.transfer(Cursor::new(encoded), ChannelWriter::new(tx))?;
        let output: Vec<u8> = rx.iter().collect();

        Ok((input_bytes, output))
    }

    #[test]
    fn test_encoding_decoding_preserves_information() {
        test_preserve(
            "Hello, world!",
            TunnelOptions {
                compression_level: CompressionLevel::None,
                encryption_level: EncryptionLevel::None,
            },
        );

        test_preserve(
            "Hello, world!",
            TunnelOptions {
                compression_level: CompressionLevel::Best,
                encryption_level: EncryptionLevel::None,
            },
        );

        test_preserve(
            "Hello, world!",
            TunnelOptions {
                compression_level: CompressionLevel::Best,
                encryption_level: EncryptionLevel::Symmetrical {
                    password: "pwd123".into(),
                },
            },
        );
    }

    #[test]
    fn test_encryption_required_round_trip() {
        let result = encode_decode(
            "Hello, world!",
            TunnelOptions {
                compression_level: CompressionLevel::Best,
                encryption_level: EncryptionLevel::Symmetrical {
                    password: "pwd123".into(),
                },
            },
            TunnelOptions {
                compression_level: CompressionLevel::Best,
                encryption_level: EncryptionLevel::None,
            },
        );
        assert!(result.is_err());

        let result = encode_decode(
            "Hello, world!",
            TunnelOptions {
                compression_level: CompressionLevel::Best,
                encryption_level: EncryptionLevel::None,
            },
            TunnelOptions {
                compression_level: CompressionLevel::Best,
                encryption_level: EncryptionLevel::Symmetrical {
                    password: "pwd123".into(),
                },
            },
        );
        assert!(result.is_err());

        let result = encode_decode(
            "Hello, world!",
            TunnelOptions {
                compression_level: CompressionLevel::Best,
                encryption_level: EncryptionLevel::Symmetrical {
                    password: "pwd123".into(),
                },
            },
            TunnelOptions {
                compression_level: CompressionLevel::Best,
                encryption_level: EncryptionLevel::Symmetrical {
                    password: "123456".into(),
                },
            },
        );
        assert!(result.is_err());
    }

    #[test]
    fn compression_level_round_trip() {
        let (input, output) = encode_decode(
            "Hello, world!",
            TunnelOptions {
                compression_level: CompressionLevel::Best,
                encryption_level: EncryptionLevel::None,
            },
            TunnelOptions {
                compression_level: CompressionLevel::Balanced,
                encryption_level: EncryptionLevel::None,
            },
        )
        .unwrap();
        assert_eq!(input, output);

        let (input, output) = encode_decode(
            "Hello, world!",
            TunnelOptions {
                compression_level: CompressionLevel::Best,
                encryption_level: EncryptionLevel::None,
            },
            TunnelOptions {
                compression_level: CompressionLevel::None,
                encryption_level: EncryptionLevel::None,
            },
        )
        .unwrap();
        assert_eq!(input, output);

        let result = encode_decode(
            "Hello, world!",
            TunnelOptions {
                compression_level: CompressionLevel::None,
                encryption_level: EncryptionLevel::None,
            },
            TunnelOptions {
                compression_level: CompressionLevel::Best,
                encryption_level: EncryptionLevel::None,
            },
        );
        assert_eq!(input, output);
    }
}
