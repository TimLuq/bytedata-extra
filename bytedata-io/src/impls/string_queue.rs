use bytedata::StringQueue;

use crate::{StringDataSource, StringDataSourceAsync, StringDataSourceSync};


impl<'a> StringDataSource<'a> for StringQueue<'a> {
    #[inline]
    fn has_ended(&self) -> bool {
        true
    }

    #[inline]
    fn buffered(&self) -> &StringQueue<'a> {
        self
    }

    #[inline]
    fn buffered_mut(&mut self) -> &mut StringQueue<'a> {
        self
    }

    #[inline]
    fn try_fill(&mut self) -> Result<(), crate::Error> {
        Ok(())
    }
}

impl<'a> StringDataSourceAsync<'a> for StringQueue<'a> {
    #[inline]
    fn poll_fill(self: core::pin::Pin<&mut Self>, _ctx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), crate::Error>> {
        core::task::Poll::Ready(Ok(()))
    }
}

impl<'a> StringDataSourceSync<'a> for StringQueue<'a> {
    #[inline]
    fn fill_blocking(&mut self) -> Result<(), crate::Error> {
        Ok(())
    }
}
