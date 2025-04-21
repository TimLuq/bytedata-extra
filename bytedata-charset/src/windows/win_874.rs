use crate::ascii7_compat::AsciiCompatible;

const WIN_874_CHARSET: [char; 128] = {
    let mut charset = crate::iso_8859::ISO_8859_11_CHARSET;

    charset[0x80 & 0x7F] = '\u{20AC}';
    charset[0x85 & 0x7F] = '\u{2026}';

    charset[0x91 & 0x7F] = '\u{2018}';
    charset[0x92 & 0x7F] = '\u{2019}';
    charset[0x93 & 0x7F] = '\u{201C}';
    charset[0x94 & 0x7F] = '\u{201D}';
    charset[0x95 & 0x7F] = '\u{2022}';
    charset[0x96 & 0x7F] = '\u{2013}';
    charset[0x97 & 0x7F] = '\u{2014}';

    charset
};

/// An encoding for windows-874.
/// 
/// This can be seen as a replacement for `iso-8859-11`.
/// The html specification remaps `iso-8859-11` to `windows-874`.
#[cfg_attr(docsrs, doc(cfg(feature = "windows-874")))]
pub static WINDOWS_874: Windows874 = Windows874::new();

/// A mapper from bytes over 128 to the corresponding unicode character.
const ENCODER: AsciiCompatible = AsciiCompatible::new(<Windows874 as crate::Charset>::CHARSET_NAME, &WIN_874_CHARSET);

/// An encoding for windows-874.
/// If possible use [`UTF-8`] instead.
/// 
/// [`UTF-8`]: crate::Utf8Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::exhaustive_structs)]
#[cfg_attr(docsrs, doc(cfg(feature = "windows-874")))]
pub struct Windows874;

impl Windows874 {

    /// Create a new windows-874 encoding instance.
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

#[cfg_attr(docsrs, doc(cfg(feature = "windows-874")))]
impl core::default::Default for Windows874 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "windows-874")))]
impl crate::Charset for Windows874 {
    const CHARSET_NAME: &'static str = "windows-874";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 1)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // IANA
            Self::CHARSET_NAME,
            "cswindows874",
            
            // code pages
            "cp1162", "ibm1162",

            // other
            "dos-874",

            // html extensions
            "iso-8859-11",
            "iso8859-11",
            "iso885911",
            "tis-620",
        ]
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "windows-874")))]
impl crate::CharsetDecoding for Windows874 {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "windows-874")))]
impl crate::CharsetEncoding for Windows874 {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}
