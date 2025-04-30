use crate::ascii7_compat::AsciiCompatible;

/// The ISO-8859-9 charset.
const ISO_8859_9_CHARSET: [char; 128] = {
    let mut charset = super::ISO_8859_1_CHARSET;
    charset[0xD0 & 0x7F] = '\u{011E}';
    charset[0xDD & 0x7F] = '\u{0130}';
    charset[0xDE & 0x7F] = '\u{015E}';
    charset[0xF0 & 0x7F] = '\u{011F}';
    charset[0xFD & 0x7F] = '\u{0131}';
    charset[0xFE & 0x7F] = '\u{015F}';
    charset
};

/// An encoding for ISO-8859-9.
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-9")))]
pub static ISO_8859_9: Iso8859_9 = Iso8859_9::new();

/// A mapper from bytes over 128 to the corresponding unicode character.
const ENCODER: AsciiCompatible = AsciiCompatible::new(
    <Iso8859_9 as crate::Charset>::CHARSET_NAME,
    &ISO_8859_9_CHARSET,
);

/// An encoding for ISO-8859-9.
/// If possible use [`UTF-8`] instead.
///
/// [`UTF-8`]: crate::Utf8Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::exhaustive_structs)]
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-9")))]
pub struct Iso8859_9;

impl Iso8859_9 {
    /// Create a new ISO-8859-9 encoding instance.
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

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-9")))]
impl core::default::Default for Iso8859_9 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-9")))]
impl crate::Charset for Iso8859_9 {
    const CHARSET_NAME: &'static str = "iso-8859-9";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 1)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // IANA
            Self::CHARSET_NAME,
            "iso-ir-148",
            "iso_8859-9",
            "latin5",
            "l5",
            "csisolatin5",
            "iso_8859-9:1989",
            // code pages
            "cp920",
            "ibm920",
            "cp28599",
            "windows-28599",
            // other
            "iso8859-9",
            "iso88599",
            "ecma-128",
        ]
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-9")))]
impl crate::CharsetDecoding for Iso8859_9 {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-9")))]
impl crate::CharsetEncoding for Iso8859_9 {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}
