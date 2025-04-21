use crate::ascii7_compat::AsciiCompatible;

/// The ISO-8859-14 charset.
const ISO_8859_14_CHARSET: [char; 128] = {
    let mut charset = super::ISO_8859_1_CHARSET;

    charset[0xA1 & 0x7F] = '\u{1E02}';
    charset[0xA2 & 0x7F] = '\u{1E03}';
    charset[0xA4 & 0x7F] = '\u{010A}';
    charset[0xA5 & 0x7F] = '\u{010B}';
    charset[0xA6 & 0x7F] = '\u{1E0A}';
    charset[0xA8 & 0x7F] = '\u{1E80}';
    charset[0xAA & 0x7F] = '\u{1E82}';
    charset[0xAB & 0x7F] = '\u{1E0B}';
    charset[0xAC & 0x7F] = '\u{1EF2}';
    charset[0xAF & 0x7F] = '\u{0178}';

    charset[0xB0 & 0x7F] = '\u{1E1E}';
    charset[0xB1 & 0x7F] = '\u{1E1F}';
    charset[0xB2 & 0x7F] = '\u{0120}';
    charset[0xB3 & 0x7F] = '\u{0121}';
    charset[0xB4 & 0x7F] = '\u{1E40}';
    charset[0xB5 & 0x7F] = '\u{1E41}';
    charset[0xB7 & 0x7F] = '\u{1E56}';
    charset[0xB8 & 0x7F] = '\u{1E81}';
    charset[0xB9 & 0x7F] = '\u{1E57}';
    charset[0xBA & 0x7F] = '\u{1E83}';
    charset[0xBB & 0x7F] = '\u{1E60}';
    charset[0xBC & 0x7F] = '\u{1EF3}';
    charset[0xBD & 0x7F] = '\u{1E84}';
    charset[0xBE & 0x7F] = '\u{1E85}';
    charset[0xBF & 0x7F] = '\u{1E61}';

    charset[0xD0 & 0x7F] = '\u{0174}';
    charset[0xD7 & 0x7F] = '\u{1E6A}';
    charset[0xDE & 0x7F] = '\u{0176}';

    charset[0xF0 & 0x7F] = '\u{0175}';
    charset[0xF7 & 0x7F] = '\u{1E6B}';
    charset[0xFE & 0x7F] = '\u{0177}';

    charset
};

/// An encoding for ISO-8859-14.
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-14")))]
pub static ISO_8859_14: Iso8859_14 = Iso8859_14::new();

/// A mapper from bytes over 128 to the corresponding unicode character.
const ENCODER: AsciiCompatible = AsciiCompatible::new(<Iso8859_14 as crate::Charset>::CHARSET_NAME, &ISO_8859_14_CHARSET);

/// An encoding for ISO-8859-14.
/// If possible use [`UTF-8`] instead.
/// 
/// [`UTF-8`]: crate::Utf8Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::exhaustive_structs)]
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-14")))]
pub struct Iso8859_14;

impl Iso8859_14 {

    /// Create a new ISO-8859-14 encoding instance.
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

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-14")))]
impl core::default::Default for Iso8859_14 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-14")))]
impl crate::Charset for Iso8859_14 {
    const CHARSET_NAME: &'static str = "iso-8859-14";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 1)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // IANA
            Self::CHARSET_NAME,
            "csiso885914",
            "iso-ir-199",
            "iso_8859-14",
            "iso_8859-14:1998",
            "l8",
            "latin8",
            "iso-celtic",
            
            // code pages
            "cp28604", "windows-28604",

            // other
            "iso8859-14",
            "iso885914",
        ]
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-14")))]
impl crate::CharsetDecoding for Iso8859_14 {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-14")))]
impl crate::CharsetEncoding for Iso8859_14 {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}
