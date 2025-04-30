use crate::ascii7_compat::AsciiCompatible;

static WIN_1250_CHARSET: [char; 128] = {
    let mut charset = crate::iso_8859::ISO_8859_2_CHARSET;

    charset[0x80 & 0x7F] = '\u{20AC}';
    charset[0x82 & 0x7F] = '\u{201A}';
    charset[0x84 & 0x7F] = '\u{201E}';
    charset[0x85 & 0x7F] = '\u{2026}';
    charset[0x86 & 0x7F] = '\u{2020}';
    charset[0x87 & 0x7F] = '\u{2021}';

    charset[0x89 & 0x7F] = '\u{2030}';
    charset[0x8A & 0x7F] = '\u{0160}';
    charset[0x8B & 0x7F] = '\u{2039}';
    charset[0x8C & 0x7F] = '\u{015A}';
    charset[0x8D & 0x7F] = '\u{0164}';
    charset[0x8E & 0x7F] = '\u{017D}';
    charset[0x8F & 0x7F] = '\u{0179}';

    charset[0x91 & 0x7F] = '\u{2018}';
    charset[0x92 & 0x7F] = '\u{2019}';
    charset[0x93 & 0x7F] = '\u{201C}';
    charset[0x94 & 0x7F] = '\u{201D}';
    charset[0x95 & 0x7F] = '\u{2022}';
    charset[0x96 & 0x7F] = '\u{2013}';
    charset[0x97 & 0x7F] = '\u{2014}';

    charset[0x99 & 0x7F] = '\u{2122}';
    charset[0x9A & 0x7F] = '\u{0161}';
    charset[0x9B & 0x7F] = '\u{203A}';
    charset[0x9C & 0x7F] = '\u{015B}';
    charset[0x9D & 0x7F] = '\u{0165}';
    charset[0x9E & 0x7F] = '\u{017E}';
    charset[0x9F & 0x7F] = '\u{017A}';

    charset[0xA1 & 0x7F] = '\u{02C7}';
    charset[0xA5 & 0x7F] = '\u{0104}';
    charset[0xA6 & 0x7F] = '\u{00A6}';
    charset[0xA9 & 0x7F] = '\u{00A9}';
    charset[0xAB & 0x7F] = '\u{00AB}';
    charset[0xAC & 0x7F] = '\u{00AC}';
    charset[0xAE & 0x7F] = '\u{00AE}';

    charset[0xB1 & 0x7F] = '\u{00B1}';
    charset[0xB5 & 0x7F] = '\u{00B5}';
    charset[0xB6 & 0x7F] = '\u{00B6}';
    charset[0xB7 & 0x7F] = '\u{00B7}';

    charset[0xB9 & 0x7F] = '\u{0105}';
    charset[0xBB & 0x7F] = '\u{00BB}';
    charset[0xBC & 0x7F] = '\u{013D}';
    charset[0xBE & 0x7F] = '\u{013E}';

    charset
};

/// An encoding for windows-1250.
#[cfg_attr(docsrs, doc(cfg(feature = "windows-1250")))]
pub static WINDOWS_1250: Windows1250 = Windows1250::new();

/// A mapper from bytes over 128 to the corresponding unicode character.
const ENCODER: AsciiCompatible = AsciiCompatible::new(
    <Windows1250 as crate::Charset>::CHARSET_NAME,
    &WIN_1250_CHARSET,
);

/// An encoding for windows-1250.
/// If possible use [`UTF-8`] instead.
///
/// [`UTF-8`]: crate::Utf8Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::exhaustive_structs)]
#[cfg_attr(docsrs, doc(cfg(feature = "windows-1250")))]
pub struct Windows1250;

impl Windows1250 {
    /// Create a new windows-1250 encoding instance.
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

#[cfg_attr(docsrs, doc(cfg(feature = "windows-1250")))]
impl core::default::Default for Windows1250 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "windows-1250")))]
impl crate::Charset for Windows1250 {
    const CHARSET_NAME: &'static str = "windows-1250";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 1)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // IANA
            Self::CHARSET_NAME,
            "cswindows1250",
            // code pages
            "cp1250",
            "x-cp1250",
            "ibm1250",
        ]
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "windows-1250")))]
impl crate::CharsetDecoding for Windows1250 {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "windows-1250")))]
impl crate::CharsetEncoding for Windows1250 {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}
