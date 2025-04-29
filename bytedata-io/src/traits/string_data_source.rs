use bytedata::StringQueue;

use crate::Error;

pub trait StringDataSource<'a> {
    /// Returns `true` if the data source has ended and no more data can be read.
    fn has_ended(&self) -> bool;
    fn buffered(&self) -> &StringQueue<'a>;
    fn buffered_mut(&mut self) -> &mut StringQueue<'a>;
    
    /// Attempt to fill the buffer with more data without blocking or awaiting.
    /// Returns the number of bytes in the buffer after completing the attempt.
    /// 
    /// Returning `Ok` without having added any bytes to the buffer might indicate end-of-file or that no additional bytes were presently available.
    fn try_fill(&mut self) -> Result<(), Error>;
}

pub trait StringDataSourceSync<'a>: StringDataSource<'a> {
    /// Fill the buffer with more data, blocking until the buffer is filled or an error occurs.
    /// Returns the number of bytes in the buffer after completing the attempt.
    /// The result value is the same as the value returned by `buffered().len()`, and if it has increased, it is by the number of bytes added to the buffer.
    /// 
    /// Returning `Ok` without having added any bytes to the buffer is signalling end-of-file.
    fn fill_blocking(&mut self) -> Result<(), Error>;
}

pub trait StringDataSourceAsync<'a>: StringDataSource<'a> {
    /// Fill the buffer with more data, pending until the buffer is filled or an error occurs.
    /// Returns the number of bytes in the buffer after completing the attempt.
    ///
    /// Returning `Ok` without having added any bytes to the buffer is signalling end-of-file.
    fn poll_fill(self: core::pin::Pin<&mut Self>, ctx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), Error>>;
}

pub trait StringDataSourceAsyncExt<'a>: StringDataSourceAsync<'a> {
    /// Fill the buffer with more data, pending until the buffer is filled or an error occurs.
    /// Returns the number of bytes in the buffer after completing the attempt.
    /// The result value is the same as the value returned by `buffered().len()`, and is increased by the number of bytes added to the buffer.
    #[must_use]
    fn fill_async(&'_ mut self) -> StringDataSourceFill<'a, '_, Self>;
}

impl<'a, T: StringDataSourceAsync<'a> + Unpin + ?Sized> StringDataSourceAsyncExt<'a> for T {
    #[inline]
    fn fill_async(&'_ mut self) -> StringDataSourceFill<'a, '_, T> {
        StringDataSourceFill::new(core::pin::Pin::new(self))
    }
}

/// Future returned by [`StringDataSourceAsyncExt::fill_async`].
/// It is expected to be used as `source.fill_async().await`, as explicit polling can be done with [`StringDataSourceAsync::poll_fill`].
#[repr(transparent)]
pub struct StringDataSourceFill<'a: 'b, 'b, T: StringDataSourceAsync<'a> +  ?Sized> {
    source: core::pin::Pin<&'b mut T>,
    _ph: core::marker::PhantomData<&'a ()>,
}

impl<'a: 'b, 'b, T: StringDataSourceAsync<'a> + ?Sized> StringDataSourceFill<'a, 'b, T> {
    #[inline]
    const fn new(source: core::pin::Pin<&'b mut T>) -> Self {
        Self { source, _ph: core::marker::PhantomData }
    }
}

impl<'a: 'b, 'b, T: StringDataSourceAsync<'a> + ?Sized> std::future::Future for StringDataSourceFill<'a, 'b, T> {
    type Output = Result<(), Error>;

    #[inline]
    fn poll(mut self: core::pin::Pin<&mut Self>, ctx: &mut core::task::Context<'_>) -> core::task::Poll<Self::Output> {
        self.source.as_mut().poll_fill(ctx)
    }
}
