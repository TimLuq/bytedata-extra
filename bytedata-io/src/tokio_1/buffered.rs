use core::task::Poll;

use bytedata::ByteQueue;
use tokio_1::io::AsyncBufRead;

pub struct TokioBuffered<T> {
    inner: T,
    buffer: ByteQueue<'static>,
    ended: bool,
}


impl<T: AsyncBufRead> TokioBuffered<T> {
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

impl<T: AsyncBufRead> From<T> for TokioBuffered<T> {
    #[inline]
    fn from(inner: T) -> Self {
        Self::new(inner)
    }
}

impl<T: AsyncBufRead> crate::ByteDataSource<'static> for TokioBuffered<T> {
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

impl<T: AsyncBufRead> crate::ByteDataSourceAsync<'static> for TokioBuffered<T> {
    fn poll_fill(self: core::pin::Pin<&mut Self>, ctx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), crate::Error>> {
        let this = unsafe { core::pin::Pin::into_inner_unchecked(self) };
        if this.ended {
            return Poll::Ready(Ok(()));
        }

        let inner = unsafe { core::pin::Pin::new_unchecked(&mut this.inner) };
        let d = match T::poll_fill_buf(inner, ctx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(Ok(d)) => d,
            Poll::Ready(Err(e)) => return Poll::Ready(Err(crate::Error::Io { io_error: e })),
        };

        if d.is_empty() {
            this.ended = true;
            return Poll::Ready(Ok(()));
        }
        let len = d.len().min(0xFFFF_FF00);
        let mut buf = bytedata::SharedBytesBuilder::with_capacity(len);
        buf.extend_from_slice(d);
        
        let inner = unsafe { core::pin::Pin::new_unchecked(&mut this.inner) };
        T::consume(inner, len);

        this.buffer.push_back(buf.build());
        Poll::Ready(Ok(()))
    }
}
