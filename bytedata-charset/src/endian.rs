/// The endianness of the charset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
#[expect(clippy::exhaustive_enums)]
pub enum CharsetEndian {
    /// Big endian.
    Big,
    /// Little endian.
    Little,
}

impl CharsetEndian {
    /// The native endianness of the system.
    #[cfg(target_endian = "big")]
    pub const NATIVE: Self = Self::Big;
    /// The native endianness of the system.
    #[cfg(not(target_endian = "big"))]
    pub const NATIVE: Self = Self::Little;

    /// Returns `true` if the endianness is big endian.
    #[inline]
    #[must_use]
    pub const fn is_big(self) -> bool {
        matches!(self, Self::Big)
    }
    /// Returns `true` if the endianness is little endian.
    #[inline]
    #[must_use]
    pub const fn is_little(self) -> bool {
        matches!(self, Self::Little)
    }
    /// Returns `true` if the endianness is the same endianness as the system.
    #[inline]
    #[must_use]
    pub const fn is_native(self) -> bool {
        matches!(self, Self::NATIVE)
    }
}

impl Default for CharsetEndian {
    #[inline]
    fn default() -> Self {
        Self::Big
    }
}
