use bytedata::ByteQueue;

use crate::{ByteDataSource, ByteDataSourceAsync, ByteDataSourceSync};


impl<'a> ByteDataSource<'a> for ByteQueue<'a> {
    #[inline]
    fn has_ended(&self) -> bool {
        true
    }

    #[inline]
    fn buffered(&self) -> &ByteQueue<'a> {
        self
    }

    #[inline]
    fn buffered_mut(&mut self) -> &mut ByteQueue<'a> {
        self
    }

    #[inline]
    fn try_fill(&mut self) -> Result<(), crate::Error> {
        Ok(())
    }
}

impl<'a> ByteDataSourceAsync<'a> for ByteQueue<'a> {
    #[inline]
    fn poll_fill(self: core::pin::Pin<&mut Self>, _ctx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), crate::Error>> {
        core::task::Poll::Ready(Ok(()))
    }
}

impl<'a> ByteDataSourceSync<'a> for ByteQueue<'a> {
    #[inline]
    fn fill_blocking(&mut self) -> Result<(), crate::Error> {
        Ok(())
    }
}
