use std::io::{BufRead, Read, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Clone)]
pub struct BytesCounter {
    bytes_amount: Arc<AtomicUsize>,
}

impl BytesCounter {
    pub fn new(bytes: &Arc<AtomicUsize>) -> BytesCounter {
        BytesCounter {
            bytes_amount: Arc::clone(bytes),
        }
    }

    pub fn value(&self) -> usize {
        self.bytes_amount.load(Ordering::Relaxed)
    }
}

pub struct BytesCountingReader<R: Read> {
    inner_reader: R,
    byte_count: Arc<AtomicUsize>,
}

impl<R: Read> BytesCountingReader<R> {
    /// Creates a new `ByteCountingReader`.
    pub fn new(inner: R) -> Self {
        BytesCountingReader {
            inner_reader: inner,
            byte_count: Arc::new(AtomicUsize::new(0)),
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
        self.byte_count.fetch_add(bytes_read, Ordering::Relaxed);
        Ok(bytes_read)
    }
}

impl<R: Read + BufRead> BufRead for BytesCountingReader<R> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.inner_reader.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.byte_count.fetch_add(amt, Ordering::Relaxed);
        self.inner_reader.consume(amt);
    }
}

/// A wrapper around a `Write` type that tracks the number of bytes written.
pub struct BytesCountingWriter<W: Write> {
    inner: W,
    byte_count: Arc<AtomicUsize>,
}

impl<W: Write> BytesCountingWriter<W> {
    /// Creates a new `ByteCountingWriter`.
    pub fn new(inner: W) -> Self {
        BytesCountingWriter {
            inner,
            byte_count: Arc::new(AtomicUsize::new(0)),
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
        self.byte_count.fetch_add(bytes_written, Ordering::Relaxed);
        Ok(bytes_written)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
