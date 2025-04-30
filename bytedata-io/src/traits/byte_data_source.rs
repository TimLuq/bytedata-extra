use bytedata::ByteQueue;

use crate::Error;

pub trait ByteDataSource<'a> {
    fn has_ended(&self) -> bool;
    fn buffered(&self) -> &ByteQueue<'a>;
    fn buffered_mut(&mut self) -> &mut ByteQueue<'a>;

    /// Attempt to fill the buffer with more data without blocking or awaiting.
    /// Returns the number of bytes in the buffer after completing the attempt.
    ///
    /// Returning `Ok` without having added any bytes to the buffer might indicate end-of-file or that no additional bytes were presently available.
    fn try_fill(&mut self) -> Result<(), Error>;
}

pub trait ByteDataSourceSync<'a>: ByteDataSource<'a> {
    /// Fill the buffer with more data, blocking until the buffer is filled or an error occurs.
    /// Returns the number of bytes in the buffer after completing the attempt.
    /// The result value is the same as the value returned by `buffered().len()`, and if it has increased, it is by the number of bytes added to the buffer.
    ///
    /// Returning `Ok` without having added any bytes to the buffer is signalling end-of-file.
    fn fill_blocking(&mut self) -> Result<(), Error>;
}

pub trait ByteDataSourceAsync<'a>: ByteDataSource<'a> {
    /// Fill the buffer with more data, pending until the buffer is filled or an error occurs.
    /// Returns the number of bytes in the buffer after completing the attempt.
    ///
    /// Returning `Ok` without having added any bytes to the buffer is signalling end-of-file.
    fn poll_fill(
        self: core::pin::Pin<&mut Self>,
        ctx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Result<(), Error>>;
}

pub trait ByteDataSourceAsyncExt<'a>: ByteDataSourceAsync<'a> {
    /// Fill the buffer with more data, pending until the buffer is filled or an error occurs.
    /// Returns the number of bytes in the buffer after completing the attempt.
    /// The result value is the same as the value returned by `buffered().len()`, and is increased by the number of bytes added to the buffer.
    #[must_use]
    fn fill_async(&'_ mut self) -> ByteDataSourceFill<'a, '_, Self>;
}

impl<'a, T: ByteDataSourceAsync<'a> + Unpin + ?Sized> ByteDataSourceAsyncExt<'a> for T {
    fn fill_async(&'_ mut self) -> ByteDataSourceFill<'a, '_, T> {
        ByteDataSourceFill::new(core::pin::Pin::new(self))
    }
}

/// Future returned by [`ByteDataSourceAsyncExt::fill_async`].
/// It is expected to be used with `source.fill_async().await`, as explicit polling can be done with [`ByteDataSourceAsync::poll_fill`].
#[repr(transparent)]
pub struct ByteDataSourceFill<'a: 'b, 'b, T: ByteDataSourceAsync<'a> + ?Sized> {
    source: core::pin::Pin<&'b mut T>,
    _ph: core::marker::PhantomData<&'a ()>,
}

impl<'a: 'b, 'b, T: ByteDataSourceAsync<'a> + ?Sized> ByteDataSourceFill<'a, 'b, T> {
    #[inline]
    fn new(source: core::pin::Pin<&'b mut T>) -> Self {
        Self {
            source,
            _ph: core::marker::PhantomData,
        }
    }
}

impl<'a: 'b, 'b, T: ByteDataSourceAsync<'a> + ?Sized> std::future::Future
    for ByteDataSourceFill<'a, 'b, T>
{
    type Output = Result<(), Error>;

    fn poll(
        mut self: core::pin::Pin<&mut Self>,
        ctx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        self.source.as_mut().poll_fill(ctx)
    }
}
