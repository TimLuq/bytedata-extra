use crate::ascii7_compat::AsciiCompatible;

/// The X-USER-DEFINED charset.
const X_USER_DEFINED_CHARSET: [char; 128] = {
    let mut charset = ['\0'; 128];

    let mut i = 0;
    while i < 0x80 {
        // SAFETY: The index is always in bounds of a valid codepoint.
        charset[i as usize] = unsafe { core::char::from_u32_unchecked(0xF780 + i) };
        i += 1;
    }

    charset
};

/// An encoding for X-USER-DEFINED.
#[cfg_attr(docsrs, doc(cfg(feature = "x-user-defined")))]
pub static X_USER_DEFINED: XUserDefined = XUserDefined::new();

/// A mapper from bytes over 128 to the corresponding unicode character.
const ENCODER: AsciiCompatible = AsciiCompatible::new(<XUserDefined as crate::Charset>::CHARSET_NAME, &X_USER_DEFINED_CHARSET);

/// An encoding for X-USER-DEFINED.
/// If possible use [`UTF-8`] instead.
/// 
/// [`UTF-8`]: crate::Utf8Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::exhaustive_structs)]
#[cfg_attr(docsrs, doc(cfg(feature = "x-user-defined")))]
pub struct XUserDefined;

impl XUserDefined {

    /// Create a new X-USER-DEFINED encoding instance.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Encode characters from the given bytes.
    #[must_use]
    #[inline]
    pub const fn encode_const(&self, chars: &str) -> crate::EncodeResult {
        ENCODER.encode_const(chars)
    }

    /// Encode characters from the given bytes.
    #[must_use]
    #[inline]
    pub fn encode(&self, chars: &str) -> crate::EncodeResult {
        // TODO: optimize runtime encode (0xF780 ..= 0xF7FF).contains(&ch) -> ch - 0xF780 + 0x80
        ENCODER.encode(chars)
    }

    /// Decode characters from the given bytes.
    #[must_use]
    #[inline]
    pub const fn decode_const(&self, bytes: &[u8]) -> crate::DecodeResult {
        ENCODER.decode_const(bytes)
    }

    /// Decode characters from the given bytes.
    #[must_use]
    #[inline]
    pub fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        ENCODER.decode(bytes)
    }

    /// Get the generic ASCII-compatible charset encoder for this charset.
    #[must_use]
    #[inline]
    pub const fn ascii_compat(&self) -> &'static AsciiCompatible {
        &ENCODER
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "x-user-defined")))]
impl core::default::Default for XUserDefined {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "x-user-defined")))]
impl crate::Charset for XUserDefined {
    const CHARSET_NAME: &'static str = "x-user-defined";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 1)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // IANA
            Self::CHARSET_NAME,
            "cskoi8r",
            
            // code pages
            "cp20866", "windows-20866",
            "cp878", "ibm878",

            // other
            "koi",
            "koi8",
            "koi8_r",
        ]
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "x-user-defined")))]
impl crate::CharsetDecoding for XUserDefined {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "x-user-defined")))]
impl crate::CharsetEncoding for XUserDefined {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}
