use crate::ascii7_compat::AsciiCompatible;

/// The ISO-8859-4 charset.
const ISO_8859_4_CHARSET: [char; 128] = {
    let mut charset = super::ISO_8859_1_CHARSET;

    charset[0xA1 & 0x7F] = '\u{0104}';
    charset[0xA2 & 0x7F] = '\u{0138}';
    charset[0xA3 & 0x7F] = '\u{0156}';
    charset[0xA5 & 0x7F] = '\u{0128}';
    charset[0xA6 & 0x7F] = '\u{013B}';
    charset[0xA9 & 0x7F] = '\u{0160}';
    charset[0xAA & 0x7F] = '\u{0112}';
    charset[0xAB & 0x7F] = '\u{0122}';
    charset[0xAC & 0x7F] = '\u{0166}';
    charset[0xAE & 0x7F] = '\u{017D}';

    charset[0xB1 & 0x7F] = '\u{0105}';
    charset[0xB2 & 0x7F] = '\u{02DB}';
    charset[0xB3 & 0x7F] = '\u{0157}';
    charset[0xB5 & 0x7F] = '\u{0129}';
    charset[0xB6 & 0x7F] = '\u{013C}';
    charset[0xB7 & 0x7F] = '\u{02C7}';
    charset[0xB9 & 0x7F] = '\u{0161}';
    charset[0xBA & 0x7F] = '\u{0113}';
    charset[0xBB & 0x7F] = '\u{0123}';
    charset[0xBC & 0x7F] = '\u{0167}';
    charset[0xBD & 0x7F] = '\u{014A}';
    charset[0xBE & 0x7F] = '\u{017E}';
    charset[0xBF & 0x7F] = '\u{014B}';

    charset[0xC0 & 0x7F] = '\u{0100}';
    charset[0xC7 & 0x7F] = '\u{012E}';
    charset[0xC8 & 0x7F] = '\u{010C}';
    charset[0xCA & 0x7F] = '\u{0118}';
    charset[0xCC & 0x7F] = '\u{0116}';
    charset[0xCF & 0x7F] = '\u{012A}';

    charset[0xD0 & 0x7F] = '\u{0110}';
    charset[0xD1 & 0x7F] = '\u{0145}';
    charset[0xD2 & 0x7F] = '\u{014C}';
    charset[0xD3 & 0x7F] = '\u{0136}';
    charset[0xD9 & 0x7F] = '\u{0172}';
    charset[0xDD & 0x7F] = '\u{0168}';

    charset[0xDE & 0x7F] = '\u{016A}';
    charset[0xE0 & 0x7F] = '\u{0101}';
    charset[0xE7 & 0x7F] = '\u{012F}';
    charset[0xE8 & 0x7F] = '\u{010D}';
    charset[0xEA & 0x7F] = '\u{0119}';
    charset[0xEC & 0x7F] = '\u{0117}';
    charset[0xEF & 0x7F] = '\u{012B}';
    
    charset[0xF0 & 0x7F] = '\u{0111}';
    charset[0xF1 & 0x7F] = '\u{0146}';
    charset[0xF2 & 0x7F] = '\u{014D}';
    charset[0xF3 & 0x7F] = '\u{0137}';
    charset[0xF9 & 0x7F] = '\u{0173}';
    charset[0xFD & 0x7F] = '\u{0169}';
    charset[0xFE & 0x7F] = '\u{016B}';
    charset[0xFF & 0x7F] = '\u{02D9}';

    charset
};

/// An encoding for ISO-8859-4.
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-4")))]
pub static ISO_8859_4: Iso8859_4 = Iso8859_4::new();

/// A mapper from bytes over 128 to the corresponding unicode character.
const ENCODER: AsciiCompatible = AsciiCompatible::new(<Iso8859_4 as crate::Charset>::CHARSET_NAME, &ISO_8859_4_CHARSET);

/// An encoding for ISO-8859-4.
/// If possible use [`UTF-8`] instead.
/// 
/// [`UTF-8`]: crate::Utf8Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::exhaustive_structs)]
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-4")))]
pub struct Iso8859_4;

impl Iso8859_4 {

    /// Create a new ISO-8859-4 encoding instance.
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

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-4")))]
impl core::default::Default for Iso8859_4 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-4")))]
impl crate::Charset for Iso8859_4 {
    const CHARSET_NAME: &'static str = "iso-8859-4";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 1)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // IANA
            Self::CHARSET_NAME,
            "iso-ir-110",
            "iso_8859-4",
            "csisolatin4",
            "latin4",
            "l4",
            "iso_8859-4:1988",

            // code pages
            "cp914", "ibm914",
            "cp28594", "windows-28594",

            // other
            "iso8859-4",
            "iso88594",
        ]
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-4")))]
impl crate::CharsetDecoding for Iso8859_4 {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-4")))]
impl crate::CharsetEncoding for Iso8859_4 {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}
