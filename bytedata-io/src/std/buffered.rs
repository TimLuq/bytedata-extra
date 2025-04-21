use bytedata::ByteQueue;

pub struct StdBuffered<T> {
    inner: T,
    buffer: ByteQueue<'static>,
    ended: bool,
}


impl<T: std::io::BufRead> StdBuffered<T> {
    #[inline]
    pub const fn new(inner: T) -> Self {
        Self {
            inner,
            buffer: ByteQueue::new(),
            ended: false,
        }
    }

    #[inline]
    pub const fn has_ended(&self) -> bool {
        self.ended
    }

    #[inline]
    pub const fn buffered(&self) -> &ByteQueue<'static> {
        &self.buffer
    }

    #[inline]
    pub const fn buffered_mut(&mut self) -> &mut ByteQueue<'static> {
        &mut self.buffer
    }
}

impl<T: std::io::BufRead> From<T> for StdBuffered<T> {
    #[inline]
    fn from(inner: T) -> Self {
        Self::new(inner)
    }
}

impl<T: std::io::BufRead> crate::ByteDataSource<'static> for StdBuffered<T> {
    #[inline]
    fn has_ended(&self) -> bool {
        self.ended
    }

    #[inline]
    fn buffered(&self) -> &ByteQueue<'static> {
        &self.buffer
    }

    #[inline]
    fn buffered_mut(&mut self) -> &mut ByteQueue<'static> {
        &mut self.buffer
    }

    #[inline]
    fn try_fill(&mut self) -> Result<(), crate::Error> {
        Ok(())
    }
}

impl<T: std::io::BufRead> crate::ByteDataSourceSync<'static> for StdBuffered<T> {
    fn fill_blocking(&mut self) -> Result<(), crate::Error> {
        let d = match self.inner.fill_buf() {
            Ok(d) => d,
            Err(e) => return Err(crate::Error::Io { io_error: e }),
        };
        if d.is_empty() {
            self.ended = true;
            return Ok(());
        }
        let len = d.len().min(0xFFFF_FF00);
        let mut buf = bytedata::SharedBytesBuilder::with_capacity(len);
        buf.extend_from_slice(d);
        self.inner.consume(len);
        self.buffer.push_back(buf.build());
        Ok(())
    }
}
