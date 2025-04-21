use crate::ascii7_compat::AsciiCompatible;

/// The ISO-8859-8 charset.
const ISO_8859_8_CHARSET: [char; 128] = {
    let mut charset = super::ISO_8859_1_CHARSET;

    charset[0xA0 & 0x7F] = '\u{00A0}';
    
    charset[0xA2 & 0x7F] = '\u{00A2}';
    charset[0xA3 & 0x7F] = '\u{00A3}';
    charset[0xA4 & 0x7F] = '\u{00A4}';
    charset[0xA5 & 0x7F] = '\u{00A5}';
    charset[0xA6 & 0x7F] = '\u{00A6}';
    charset[0xA7 & 0x7F] = '\u{00A7}';
    charset[0xA8 & 0x7F] = '\u{00A8}';
    charset[0xA9 & 0x7F] = '\u{00A9}';
    charset[0xAA & 0x7F] = '\u{00D7}';
    charset[0xAB & 0x7F] = '\u{00AB}';
    charset[0xAC & 0x7F] = '\u{00AC}';
    charset[0xAD & 0x7F] = '\u{00AD}';
    charset[0xAE & 0x7F] = '\u{00AE}';
    charset[0xAF & 0x7F] = '\u{00AF}';

    charset[0xB0 & 0x7F] = '\u{00B0}';
    charset[0xB1 & 0x7F] = '\u{00B1}';
    charset[0xB2 & 0x7F] = '\u{00B2}';
    charset[0xB3 & 0x7F] = '\u{00B3}';
    charset[0xB4 & 0x7F] = '\u{00B4}';
    charset[0xB5 & 0x7F] = '\u{00B5}';
    charset[0xB6 & 0x7F] = '\u{00B6}';
    charset[0xB7 & 0x7F] = '\u{00B7}';
    charset[0xB8 & 0x7F] = '\u{00B8}';
    charset[0xB9 & 0x7F] = '\u{00B9}';
    charset[0xBA & 0x7F] = '\u{00F7}';
    charset[0xBB & 0x7F] = '\u{00BB}';
    charset[0xBC & 0x7F] = '\u{00BC}';
    charset[0xBD & 0x7F] = '\u{00BD}';
    charset[0xBE & 0x7F] = '\u{00BE}';
    
    charset[0xDF & 0x7F] = '\u{2017}';

    charset[0xE0 & 0x7F] = '\u{05D0}';
    charset[0xE1 & 0x7F] = '\u{05D1}';
    charset[0xE2 & 0x7F] = '\u{05D2}';
    charset[0xE3 & 0x7F] = '\u{05D3}';
    charset[0xE4 & 0x7F] = '\u{05D4}';
    charset[0xE5 & 0x7F] = '\u{05D5}';
    charset[0xE6 & 0x7F] = '\u{05D6}';
    charset[0xE7 & 0x7F] = '\u{05D7}';
    charset[0xE8 & 0x7F] = '\u{05D8}';
    charset[0xE9 & 0x7F] = '\u{05D9}';
    charset[0xEA & 0x7F] = '\u{05DA}';
    charset[0xEB & 0x7F] = '\u{05DB}';
    charset[0xEC & 0x7F] = '\u{05DC}';
    charset[0xED & 0x7F] = '\u{05DD}';
    charset[0xEE & 0x7F] = '\u{05DE}';
    charset[0xEF & 0x7F] = '\u{05DF}';
    
    charset[0xF0 & 0x7F] = '\u{05E0}';
    charset[0xF1 & 0x7F] = '\u{05E1}';
    charset[0xF2 & 0x7F] = '\u{05E2}';
    charset[0xF3 & 0x7F] = '\u{05E3}';
    charset[0xF4 & 0x7F] = '\u{05E4}';
    charset[0xF5 & 0x7F] = '\u{05E5}';
    charset[0xF6 & 0x7F] = '\u{05E6}';
    charset[0xF7 & 0x7F] = '\u{05E7}';
    charset[0xF8 & 0x7F] = '\u{05E8}';
    charset[0xF9 & 0x7F] = '\u{05E9}';
    charset[0xFA & 0x7F] = '\u{05EA}';
    
    charset[0xFD & 0x7F] = '\u{200E}';
    charset[0xFE & 0x7F] = '\u{200F}';

    charset
};

/// An encoding for ISO-8859-8.
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-8")))]
pub static ISO_8859_8: Iso8859_8 = Iso8859_8::new();

/// A mapper from bytes over 128 to the corresponding unicode character.
const ENCODER: AsciiCompatible = AsciiCompatible::new(<Iso8859_8 as crate::Charset>::CHARSET_NAME, &ISO_8859_8_CHARSET);

/// An encoding for ISO-8859-8.
/// If possible use [`UTF-8`] instead.
/// 
/// [`UTF-8`]: crate::Utf8Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::exhaustive_structs)]
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-8")))]
pub struct Iso8859_8;

impl Iso8859_8 {

    /// Create a new ISO-8859-8 encoding instance.
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

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-8")))]
impl core::default::Default for Iso8859_8 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-8")))]
impl crate::Charset for Iso8859_8 {
    const CHARSET_NAME: &'static str = "iso-8859-8";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 1)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // IANA
            Self::CHARSET_NAME,
            "iso-ir-138",
            "iso_8859-8",
            "hebrew",
            "csisolatinhebrew",
            "iso_8859-8:1988",
            
            // code pages
            "cp916", "ibm916",

            // other
            "iso-8859-8-e",
            "iso8859-8",
            "iso88598",
            "sun_eu_greek",
            //"visual",
        ]
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-8")))]
impl crate::CharsetDecoding for Iso8859_8 {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-8")))]
impl crate::CharsetEncoding for Iso8859_8 {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}
