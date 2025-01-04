use std::io::{BufRead, Read, Write};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Clone)]
pub struct BytesCounter {
    bytes_amount: Arc<AtomicU64>,
}

impl BytesCounter {
    pub fn new(bytes: &Arc<AtomicU64>) -> BytesCounter {
        BytesCounter {
            bytes_amount: Arc::clone(bytes),
        }
    }

    pub fn value(&self) -> u64 {
        self.bytes_amount.load(Ordering::Relaxed)
    }
}

pub struct BytesCountingReader<R: Read> {
    inner_reader: R,
    byte_count: Arc<AtomicU64>,
}

impl<R: Read> BytesCountingReader<R> {
    /// Creates a new `ByteCountingReader`.
    pub fn new(inner: R) -> Self {
        BytesCountingReader {
            inner_reader: inner,
            byte_count: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Gets the current byte count.
    pub fn counter(&self) -> BytesCounter {
        BytesCounter::new(&self.byte_count)
    }

    pub fn into_inner(self) -> R {
        self.inner_reader
    }
}

impl<R: Read> Read for BytesCountingReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes_read = self.inner_reader.read(buf)?;
        self.byte_count
            .fetch_add(bytes_read as u64, Ordering::Relaxed);
        Ok(bytes_read)
    }
}

impl<R: Read + BufRead> BufRead for BytesCountingReader<R> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.inner_reader.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.byte_count.fetch_add(amt as u64, Ordering::Relaxed);
        self.inner_reader.consume(amt);
    }
}

/// A wrapper around a `Write` type that tracks the number of bytes written.
pub struct BytesCountingWriter<W: Write> {
    inner: W,
    byte_count: Arc<AtomicU64>,
}

impl<W: Write> BytesCountingWriter<W> {
    /// Creates a new `ByteCountingWriter`.
    pub fn new(inner: W) -> Self {
        BytesCountingWriter {
            inner,
            byte_count: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Gets the current byte count.
    pub fn counter(&self) -> BytesCounter {
        BytesCounter::new(&self.byte_count)
    }

    pub fn into_inner(self) -> W {
        self.inner
    }
}

impl<W: Write> Write for BytesCountingWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let bytes_written = self.inner.write(buf)?;
        self.byte_count
            .fetch_add(bytes_written as u64, Ordering::Relaxed);
        Ok(bytes_written)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
