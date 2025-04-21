use crate::ascii7_compat::AsciiCompatible;

const WIN_1254_CHARSET: [char; 128] = {
    let mut charset = super::WIN_1252_CHARSET;

    charset[0x8e & 0x7F] = '\0';

    charset[0x9e & 0x7F] = '\0';

    charset[0xd0 & 0x7F] = '\u{011E}';
    charset[0xdd & 0x7F] = '\u{0130}';
    charset[0xde & 0x7F] = '\u{015E}';

    charset[0xf0 & 0x7F] = '\u{011F}';
    charset[0xfd & 0x7F] = '\u{0131}';
    charset[0xfe & 0x7F] = '\u{015F}';

    charset
};

/// An encoding for windows-1254.
#[cfg_attr(docsrs, doc(cfg(feature = "windows-1254")))]
pub static WINDOWS_1254: Windows1254 = Windows1254::new();

/// A mapper from bytes over 128 to the corresponding unicode character.
const ENCODER: AsciiCompatible = AsciiCompatible::new(<Windows1254 as crate::Charset>::CHARSET_NAME, &WIN_1254_CHARSET);

/// An encoding for windows-1254.
/// If possible use [`UTF-8`] instead.
/// 
/// [`UTF-8`]: crate::Utf8Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::exhaustive_structs)]
#[cfg_attr(docsrs, doc(cfg(feature = "windows-1254")))]
pub struct Windows1254;

impl Windows1254 {

    /// Create a new windows-1254 encoding instance.
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

#[cfg_attr(docsrs, doc(cfg(feature = "windows-1254")))]
impl core::default::Default for Windows1254 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "windows-1254")))]
impl crate::Charset for Windows1254 {
    const CHARSET_NAME: &'static str = "windows-1254";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 1)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // IANA
            Self::CHARSET_NAME,
            "cswindows1254",
            
            // code pages
            "cp1254", "x-cp1254", "ibm1254",

            // inherit most aliases from iso-8859-9
            "csisolatin5", "iso-8859-9", "iso-ir-148", "iso8859-9", "iso88599", "iso_8859-9", "iso_8859-9:1989", "l5", "latin5",
        ]
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "windows-1254")))]
impl crate::CharsetDecoding for Windows1254 {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "windows-1254")))]
impl crate::CharsetEncoding for Windows1254 {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}
