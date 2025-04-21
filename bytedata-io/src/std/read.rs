use std::mem::MaybeUninit;

use bytedata::ByteQueue;

pub struct StdRead<T> {
    inner: T,
    buffer: ByteQueue<'static>,
    tmpbuf: bytedata::SharedBytesBuilder,
    ended: bool,
}

impl<T: std::io::Read> StdRead<T> {
    #[inline]
    pub const fn new(inner: T) -> Self {
        Self {
            inner,
            buffer: ByteQueue::new(),
            tmpbuf: bytedata::SharedBytesBuilder::new(),
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

impl<T: std::io::Read> From<T> for StdRead<T> {
    #[inline]
    fn from(inner: T) -> Self {
        Self::new(inner)
    }
}

impl<T: std::io::Read> crate::ByteDataSource<'static> for StdRead<T> {
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

impl<T: std::io::Read> crate::ByteDataSourceSync<'static> for StdRead<T> {
    fn fill_blocking(&mut self) -> Result<(), crate::Error> {
        if self.ended {
            return Ok(());
        }
        let cap = self.tmpbuf.capacity();
        let cap = if cap < 8 * 1024 {
            self.tmpbuf.reserve(8 * 1024 - cap);
            self.tmpbuf.capacity()
        } else {
            cap
        };
        let res = self.tmpbuf.apply_unfilled(|buf| {
            let buf = unsafe { core::mem::transmute::<&mut [MaybeUninit<u8>], &mut [u8]>(buf) };
            match self.inner.read(buf) {
                Ok(n) => (Ok(()), n),
                Err(e) => (Err(e), 0),
            }
        });
        if let Err(e) = res {
            return Err(crate::Error::Io { io_error: e });
        }
        if self.tmpbuf.is_empty() {
            self.ended = true;
            return Ok(());
        }
        let buf = if self.tmpbuf.len() > cap - (cap >> 2) {
            // take without copying if 75% of the buffer is filled
            core::mem::take(&mut self.tmpbuf).build()
        } else {
            let a = bytedata::SharedBytes::from(self.tmpbuf.as_slice());
            self.tmpbuf.clear();
            a
        };
        self.buffer.push_back(buf);
        Ok(())
    }
}
