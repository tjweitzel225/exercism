use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    reader: R,
    bytes: usize,
    n_reads: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(reader: R) -> ReadStats<R> {
        ReadStats {
            reader,
            bytes: 0,
            n_reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.n_reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.n_reads += 1;
        let n = self.reader.read(buf)?;
        self.bytes += n;
        Ok(n)
    }
}

pub struct WriteStats<W> {
    writer: W,
    bytes: usize,
    n_writes: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(writer: W) -> WriteStats<W> {
        WriteStats {
            writer,
            bytes: 0,
            n_writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.n_writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.n_writes += 1;
        let n = self.writer.write(buf)?;
        self.bytes += n;
        Ok(n)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
