use crate::ascii7_compat::AsciiCompatible;

/// The ISO-8859-13 charset.
const ISO_8859_13_CHARSET: [char; 128] = {
    let mut charset = super::ISO_8859_1_CHARSET;
    
    charset[0xA1 & 0x7F] = '\u{201D}';
    charset[0xA5 & 0x7F] = '\u{201E}';
    charset[0xA8 & 0x7F] = '\u{00D8}';
    charset[0xAA & 0x7F] = '\u{0156}';
    charset[0xAF & 0x7F] = '\u{00C6}';

    charset[0xB4 & 0x7F] = '\u{201C}';
    charset[0xB8 & 0x7F] = '\u{00F8}';
    charset[0xBA & 0x7F] = '\u{0157}';
    charset[0xBF & 0x7F] = '\u{00E6}';

    charset[0xC0 & 0x7F] = '\u{0104}';
    charset[0xC1 & 0x7F] = '\u{012E}';
    charset[0xC2 & 0x7F] = '\u{0100}';
    charset[0xC3 & 0x7F] = '\u{0106}';
    charset[0xC6 & 0x7F] = '\u{0118}';
    charset[0xC7 & 0x7F] = '\u{0112}';
    charset[0xC8 & 0x7F] = '\u{010C}';
    charset[0xCA & 0x7F] = '\u{0179}';
    charset[0xCB & 0x7F] = '\u{0116}';
    charset[0xCC & 0x7F] = '\u{0122}';
    charset[0xCD & 0x7F] = '\u{0136}';
    charset[0xCE & 0x7F] = '\u{012A}';
    charset[0xCF & 0x7F] = '\u{013B}';

    charset[0xD0 & 0x7F] = '\u{0160}';
    charset[0xD1 & 0x7F] = '\u{0143}';
    charset[0xD2 & 0x7F] = '\u{0145}';
    charset[0xD4 & 0x7F] = '\u{014C}';
    charset[0xD8 & 0x7F] = '\u{0172}';
    charset[0xD9 & 0x7F] = '\u{0141}';
    charset[0xDA & 0x7F] = '\u{015A}';
    charset[0xDB & 0x7F] = '\u{016A}';
    charset[0xDD & 0x7F] = '\u{017B}';
    charset[0xDE & 0x7F] = '\u{017D}';

    charset[0xE0 & 0x7F] = '\u{0105}';
    charset[0xE1 & 0x7F] = '\u{012F}';
    charset[0xE2 & 0x7F] = '\u{0101}';
    charset[0xE3 & 0x7F] = '\u{0107}';
    charset[0xE6 & 0x7F] = '\u{0119}';
    charset[0xE7 & 0x7F] = '\u{0113}';
    charset[0xE8 & 0x7F] = '\u{010D}';
    charset[0xEA & 0x7F] = '\u{017A}';
    charset[0xEB & 0x7F] = '\u{0117}';
    charset[0xEC & 0x7F] = '\u{0123}';
    charset[0xED & 0x7F] = '\u{0137}';
    charset[0xEE & 0x7F] = '\u{012B}';
    charset[0xEF & 0x7F] = '\u{013C}';

    charset[0xF0 & 0x7F] = '\u{0161}';
    charset[0xF1 & 0x7F] = '\u{0144}';
    charset[0xF2 & 0x7F] = '\u{0146}';
    charset[0xF4 & 0x7F] = '\u{014D}';
    charset[0xF8 & 0x7F] = '\u{0173}';
    charset[0xF9 & 0x7F] = '\u{0142}';
    charset[0xFA & 0x7F] = '\u{015B}';
    charset[0xFB & 0x7F] = '\u{016B}';
    charset[0xFD & 0x7F] = '\u{017C}';
    charset[0xFE & 0x7F] = '\u{017E}';
    charset[0xFF & 0x7F] = '\u{2019}';

    charset
};

/// An encoding for ISO-8859-13.
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-13")))]
pub static ISO_8859_13: Iso8859_13 = Iso8859_13::new();

/// A mapper from bytes over 128 to the corresponding unicode character.
const ENCODER: AsciiCompatible = AsciiCompatible::new(<Iso8859_13 as crate::Charset>::CHARSET_NAME, &ISO_8859_13_CHARSET);

/// An encoding for ISO-8859-13.
/// If possible use [`UTF-8`] instead.
/// 
/// [`UTF-8`]: crate::Utf8Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::exhaustive_structs)]
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-13")))]
pub struct Iso8859_13;

impl Iso8859_13 {

    /// Create a new ISO-8859-13 encoding instance.
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

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-13")))]
impl core::default::Default for Iso8859_13 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-13")))]
impl crate::Charset for Iso8859_13 {
    const CHARSET_NAME: &'static str = "iso-8859-13";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 1)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // IANA
            Self::CHARSET_NAME,
            "csiso885913",
            
            // code pages
            "cp28603", "windows-28603",

            // other
            "iso8859-13",
            "iso885913",
        ]
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-13")))]
impl crate::CharsetDecoding for Iso8859_13 {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-13")))]
impl crate::CharsetEncoding for Iso8859_13 {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}
