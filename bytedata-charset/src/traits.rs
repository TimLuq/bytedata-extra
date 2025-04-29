

/// A reference to a charset.
pub trait CharsetRef {
    /// The name of the charset.
    /// 
    /// This method can be overridden to provide a different name than the internal name dependant on the specifics of `self`.
    fn charset_name(&self) -> &'static str;

    /// The minimum and maximum number of bytes that represent a character in the charset.
    fn size_hint(&self) -> (u16, u16);
}

impl<'a> CharsetRef for &'a (dyn CharsetRef + 'a) {
    #[inline]
    fn charset_name(&self) -> &'static str {
        (**self).charset_name()
    }

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (**self).size_hint()
    }
}

/// A charset implementation.
pub trait Charset {
    /// The internal name of the charset.
    /// If this is the actual charset name then the [`charset_name`] method can fall back on the default implementation.
    /// 
    /// [`charset_name`]: Charset::charset_name
    const CHARSET_NAME: &'static str;

    /// The name of the charset.
    /// 
    /// This method can be overridden to provide a different name than the internal name dependant on the specifics of `self`.
    #[inline]
    #[must_use]
    fn charset_name(&self) -> &'static str {
        Self::CHARSET_NAME
    }

    /// The name of the charset.
    /// 
    /// This method can be overridden to provide a different name than the internal name dependant on the specifics of `self`.
    #[inline]
    #[must_use]
    fn charset_alias(&self) -> &[&'static str] {
        &[Self::CHARSET_NAME]
    }

    /// The minimum and maximum number of bytes that represent a character in the charset.
    fn size_hint(&self) -> (u16, u16);
}

impl<T: Charset> CharsetRef for T {
    #[inline]
    fn charset_name(&self) -> &'static str {
        <T as Charset>::charset_name(self)
    }

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        <T as Charset>::size_hint(self)
    }
}

/// A charset that can decode bytes to characters.
pub trait CharsetDecoding: CharsetRef {
    /// Decode characters from the given bytes.
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult;
}

/// A charset that can encode characters to bytes.
pub trait CharsetEncoding: CharsetRef {
    /// Encode characters to bytes.
    fn encode(&self, chars: &str) -> crate::EncodeResult;
}
