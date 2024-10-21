use std::io::Write;

pub struct ChannelWriter {
    inner_producer: std::sync::mpsc::Sender<u8>,
}

impl ChannelWriter {
    pub fn new(sender: std::sync::mpsc::Sender<u8>) -> ChannelWriter {
        ChannelWriter {
            inner_producer: sender,
        }
    }
}

impl Write for ChannelWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        for &byte in buf {
            self.inner_producer.send(byte).unwrap();
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
