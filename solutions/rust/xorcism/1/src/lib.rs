use std::{
    borrow::Borrow,
    io::{Read, Write},
};

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    // This field is just to suppress compiler complaints;
    // feel free to delete it at any point.
    key: &'a [u8],
    idx: usize,
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key: AsRef<[u8]> + ?Sized>(key: &'a Key) -> Xorcism<'a> {
        Xorcism {
            key: key.as_ref(),
            idx: 0,
        }
    }

    fn next_munger(&mut self) -> u8 {
        let i = self.idx % self.key.len();
        self.idx += 1;
        self.key[i]
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        for d in data {
            *d ^= self.next_munger()
        }
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<B, Data>(&mut self, data: Data) -> impl Iterator<Item = u8>
    where
        B: Borrow<u8>,
        Data: IntoIterator<Item = B>,
    {
        data.into_iter()
            .map(move |d| d.borrow() ^ self.next_munger())
    }
    pub fn reader(self, reader: impl Read) -> impl Read {
        XorcismReader {
            xorcism: self,
            inner: reader,
        }
    }
    pub fn writer(self, writer: impl Write) -> impl Write {
        XorcismWriter {
            xorcism: self,
            inner: writer,
        }
    }
}

struct XorcismReader<'a, R> {
    xorcism: Xorcism<'a>,
    inner: R,
}
impl<'a, R> Read for XorcismReader<'a, R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.xorcism.munge_in_place(&mut buf[..n]);
        Ok(n)
    }
}

struct XorcismWriter<'a, W> {
    xorcism: Xorcism<'a>,
    inner: W,
}
impl<W: Write> Write for XorcismWriter<'_, W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut chunk = [0; 256];
        for block in buf.chunks(chunk.len()) {
            let munged = &mut chunk[..block.len()];
            munged.copy_from_slice(block);
            self.xorcism.munge_in_place(munged);
            self.inner.write_all(munged)?;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
