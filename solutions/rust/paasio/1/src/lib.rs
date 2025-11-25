use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    reader: R,
    total_bytes: usize,
    total_reads: usize
}

impl<R: Read> ReadStats<R> {
    pub fn new(reader: R) -> ReadStats<R> {
        ReadStats { reader, total_bytes: 0, total_reads: 0 }
    }

    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    pub fn bytes_through(&self) -> usize {
        self.total_bytes
    }

    pub fn reads(&self) -> usize {
        self.total_reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let result = self.reader.read(buf);
        
        if let Ok(bytes_read) = result {
            self.total_reads += 1;
            self.total_bytes += bytes_read;
        }
        
        result
    }
}

pub struct WriteStats<W> {
    writer: W,
    total_bytes: usize,
    total_writes: usize
}

impl<W: Write> WriteStats<W> {
    pub fn new(writer: W) -> WriteStats<W> {
        WriteStats { writer, total_bytes: 0, total_writes: 0 }
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    pub fn bytes_through(&self) -> usize {
        self.total_bytes
    }

    pub fn writes(&self) -> usize {
        self.total_writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let result = self.writer.write(buf);
        
        if let Ok(bytes_written) = result {
            self.total_writes += 1;
            self.total_bytes += bytes_written;
        }
        
        result
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
