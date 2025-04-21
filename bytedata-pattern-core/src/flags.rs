use core::marker::PhantomData;

#[derive(PartialEq, Eq)]
pub(crate) struct U8Flags<T> {
    pub(crate) flags: u8,
    _phantom: PhantomData<T>,
}
impl<T> Clone for U8Flags<T> {
    #[must_use]
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for U8Flags<T> {}

impl<T> U8Flags<T> {
    #[must_use]
    #[inline]
    pub(crate) const fn new() -> Self {
        Self {
            flags: 0,
            _phantom: PhantomData,
        }
    }

    pub(crate) const fn clear(&mut self) {
        self.flags = 0;
    }

    #[must_use]
    #[inline]
    pub(crate) const fn set_u8(mut self, flag: u8) -> Self {
        self.flags |= flag;
        self
    }

    #[must_use]
    #[inline]
    pub(crate) const fn unset_u8(mut self, flag: u8) -> Self {
        self.flags &= !flag;
        self
    }

    #[must_use]
    #[inline]
    pub(crate) const fn is_set_u8(self, flag: u8) -> bool {
        (self.flags & flag) != 0
    }
}

impl<T> Default for U8Flags<T> {
    #[must_use]
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: TryFrom<u8> + core::fmt::Debug> core::fmt::Debug for U8Flags<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("U8Flags(")?;
        let mut first = true;
        for i in 0..8 {
            if self.is_set_u8(1 << i) {
                if first {
                    first = false;
                } else {
                    f.write_str(" | ")?;
                }
                match T::try_from(1 << i) {
                    Ok(t) => T::fmt(&t, f)?,
                    Err(_) => f.write_fmt(format_args!("0x{:02x}", 1 << i))?,
                }
            }
        }
        if first {
            f.write_str("0)")
        } else {
            f.write_str(")")
        }
    }
}
