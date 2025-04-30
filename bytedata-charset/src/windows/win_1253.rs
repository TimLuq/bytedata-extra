use crate::ascii7_compat::AsciiCompatible;

const WIN_1253_CHARSET: [char; 128] = {
    let mut charset = crate::iso_8859::ISO_8859_7_CHARSET;

    charset[0x80 & 0x7F] = '\u{20AC}';
    charset[0x81 & 0x7F] = '\0';
    charset[0x82 & 0x7F] = '\u{201A}';
    charset[0x83 & 0x7F] = '\u{0192}';
    charset[0x84 & 0x7F] = '\u{201E}';
    charset[0x85 & 0x7F] = '\u{2026}';
    charset[0x86 & 0x7F] = '\u{2020}';
    charset[0x87 & 0x7F] = '\u{2021}';
    charset[0x88 & 0x7F] = '\0';
    charset[0x89 & 0x7F] = '\u{2030}';
    charset[0x8a & 0x7F] = '\0';
    charset[0x8b & 0x7F] = '\u{2039}';
    charset[0x8c & 0x7F] = '\0';
    charset[0x8d & 0x7F] = '\0';
    charset[0x8e & 0x7F] = '\0';
    charset[0x8f & 0x7F] = '\0';

    charset[0x90 & 0x7F] = '\0';
    charset[0x91 & 0x7F] = '\u{2018}';
    charset[0x92 & 0x7F] = '\u{2019}';
    charset[0x93 & 0x7F] = '\u{201C}';
    charset[0x94 & 0x7F] = '\u{201D}';
    charset[0x95 & 0x7F] = '\u{2022}';
    charset[0x96 & 0x7F] = '\u{2013}';
    charset[0x97 & 0x7F] = '\u{2014}';
    charset[0x98 & 0x7F] = '\0';
    charset[0x99 & 0x7F] = '\u{2122}';
    charset[0x9a & 0x7F] = '\0';
    charset[0x9b & 0x7F] = '\u{203A}';
    charset[0x9c & 0x7F] = '\0';
    charset[0x9d & 0x7F] = '\0';
    charset[0x9e & 0x7F] = '\0';
    charset[0x9f & 0x7F] = '\0';

    charset[0xa1 & 0x7F] = '\u{0385}';
    charset[0xa2 & 0x7F] = '\u{0386}';
    charset[0xa4 & 0x7F] = '\u{00A4}';
    charset[0xa5 & 0x7F] = '\u{00A5}';
    charset[0xaa & 0x7F] = '\0';
    charset[0xae & 0x7F] = '\u{00AE}';

    charset[0xb5 & 0x7F] = '\u{00B5}';
    charset[0xb6 & 0x7F] = '\u{00B6}';

    charset
};

/// An encoding for windows-1253.
#[cfg_attr(docsrs, doc(cfg(feature = "windows-1253")))]
pub static WINDOWS_1253: Windows1253 = Windows1253::new();

/// A mapper from bytes over 128 to the corresponding unicode character.
const ENCODER: AsciiCompatible = AsciiCompatible::new(
    <Windows1253 as crate::Charset>::CHARSET_NAME,
    &WIN_1253_CHARSET,
);

/// An encoding for windows-1253.
/// If possible use [`UTF-8`] instead.
///
/// [`UTF-8`]: crate::Utf8Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::exhaustive_structs)]
#[cfg_attr(docsrs, doc(cfg(feature = "windows-1253")))]
pub struct Windows1253;

impl Windows1253 {
    /// Create a new windows-1253 encoding instance.
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

#[cfg_attr(docsrs, doc(cfg(feature = "windows-1253")))]
impl core::default::Default for Windows1253 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "windows-1253")))]
impl crate::Charset for Windows1253 {
    const CHARSET_NAME: &'static str = "windows-1253";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 1)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // IANA
            Self::CHARSET_NAME,
            "cswindows1253",
            // code pages
            "cp1253",
            "x-cp1253",
            "ibm1253",
        ]
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "windows-1253")))]
impl crate::CharsetDecoding for Windows1253 {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "windows-1253")))]
impl crate::CharsetEncoding for Windows1253 {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}
