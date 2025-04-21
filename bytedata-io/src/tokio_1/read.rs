use core::task::Poll;

use bytedata::ByteQueue;
use tokio_1::io::AsyncRead;

pub struct TokioRead<T> {
    inner: T,
    buffer: ByteQueue<'static>,
    tmpbuf: bytedata::SharedBytesBuilder,
    ended: bool,
}

impl<T: AsyncRead> TokioRead<T> {
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

impl<T: AsyncRead> From<T> for TokioRead<T> {
    #[inline]
    fn from(inner: T) -> Self {
        Self::new(inner)
    }
}

impl<T: AsyncRead> crate::ByteDataSource<'static> for TokioRead<T> {
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


impl<T: AsyncRead> crate::ByteDataSourceAsync<'static> for TokioRead<T> {
    fn poll_fill(self: core::pin::Pin<&mut Self>, ctx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), crate::Error>> {
        let this = unsafe { core::pin::Pin::into_inner_unchecked(self) };
        if this.ended {
            return Poll::Ready(Ok(()));
        }

        let cap = this.tmpbuf.capacity();
        let cap = if cap < 8 * 1024 {
            this.tmpbuf.reserve(8 * 1024 - cap);
            this.tmpbuf.capacity()
        } else {
            cap
        };

        let res = this.tmpbuf.apply_unfilled(|buf| {
            let mut buf = tokio_1::io::ReadBuf::uninit(buf);
            let inner = unsafe { core::pin::Pin::new_unchecked(&mut this.inner) };
            match inner.poll_read(ctx, &mut buf) {
                Poll::Pending => (Poll::Pending, 0),
                Poll::Ready(Ok(_)) => (Poll::Ready(Ok(())), buf.filled().len()),
                Poll::Ready(Err(e)) => (Poll::Ready(Err(e)), 0),
            }
        });
        match res {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(Err(e)) => return Poll::Ready(Err(crate::Error::Io { io_error: e })),
            Poll::Ready(Ok(_)) => (),
        }
        
        if this.tmpbuf.is_empty() {
            this.ended = true;
            return Poll::Ready(Ok(()));
        }
        let buf = if this.tmpbuf.len() > cap - (cap >> 2) {
            // take without copying if 75% of the buffer is filled
            core::mem::take(&mut this.tmpbuf).build()
        } else {
            let a = bytedata::SharedBytes::from(this.tmpbuf.as_slice());
            this.tmpbuf.clear();
            a
        };
        this.buffer.push_back(buf);
        Poll::Ready(Ok(()))
    }
}
