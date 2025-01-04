use crate::services::data_tunnel::DataTunnel;
use crate::services::tracking::{BytesCounter, BytesCountingReader, BytesCountingWriter};
use std::cell::{Cell, RefCell};
use std::io::{Read, Write};
use std::ops::DerefMut;

pub struct TrackedTransfer<DT: DataTunnel, R: Read + 'static, W: Write + 'static> {
    inner_tunnel: DT,
    reader: RefCell<Option<BytesCountingReader<R>>>,
    writer: RefCell<Option<BytesCountingWriter<W>>>,
    reader_bytes_count: BytesCounter,
    writer_bytes_count: BytesCounter,
}

impl<DT: DataTunnel, R: Read + 'static, W: Write + 'static> TrackedTransfer<DT, R, W> {
    pub fn new(tunnel: DT, reader: R, writer: W) -> Self {
        let counting_reader = BytesCountingReader::new(reader);
        let counting_writer = BytesCountingWriter::new(writer);

        let reader_bytes_count = counting_reader.counter();
        let writer_bytes_count = counting_writer.counter();

        Self {
            inner_tunnel: tunnel,
            reader: RefCell::new(Some(counting_reader)),
            writer: RefCell::new(Some(counting_writer)),
            reader_bytes_count,
            writer_bytes_count,
        }
    }

    pub fn reader_bytes_count(&self) -> u64 {
        self.reader_bytes_count.value()
    }

    pub fn writer_bytes_count(&self) -> u64 {
        self.writer_bytes_count.value()
    }

    pub fn reader_bytes_counter(&self) -> BytesCounter {
        self.reader_bytes_count.clone()
    }

    pub fn writer_bytes_counter(&self) -> BytesCounter {
        self.writer_bytes_count.clone()
    }

    pub fn run(&self) -> std::io::Result<()> {
        let mut reader = self.reader.borrow_mut();
        let reader = reader.deref_mut();
        let Some(reader) = reader.take() else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Reader already taken",
            ));
        };
        let mut writer = self.writer.borrow_mut();
        let writer = writer.deref_mut();
        let Some(writer) = writer.take() else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Writer already taken",
            ));
        };
        self.inner_tunnel.transfer(reader, writer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::{CompressionLevel, EncryptionLevel};
    use crate::services::channels::ChannelWriter;
    use crate::services::data_tunnel::encoding::EncodingDataTunnel;
    use std::io::Cursor;
    use std::sync::mpsc;

    #[test]
    fn test_tracking_data_tunnel() {
        let tunnel = EncodingDataTunnel {
            compression_level: CompressionLevel::Best,
            encryption_level: EncryptionLevel::Symmetrical {
                password: "pwd123".into(),
            },
        };

        let input = b"Hello, world!";
        let (tx, rx) = mpsc::channel();
        let transfer = TrackedTransfer::new(tunnel, Cursor::new(input), ChannelWriter::new(tx));
        transfer.run();
        let output: Vec<u8> = rx.iter().collect();

        assert_eq!(transfer.reader_bytes_count(), input.len() as u64);
        assert_eq!(transfer.writer_bytes_count(), output.len() as u64);
    }
}
