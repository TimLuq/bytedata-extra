use crate::ascii7_compat::AsciiCompatible;

/// The ISO-8859-7 charset.
#[expect(clippy::redundant_pub_crate)]
pub(crate) const ISO_8859_7_CHARSET: [char; 128] = {
    let mut charset = super::ISO_8859_1_CHARSET;

    charset[0xA0 & 0x7F] = '\u{00A0}';
    charset[0xA1 & 0x7F] = '\u{2018}';
    charset[0xA2 & 0x7F] = '\u{2019}';
    charset[0xA3 & 0x7F] = '\u{00A3}';
    charset[0xA4 & 0x7F] = '\u{20AC}';
    charset[0xA5 & 0x7F] = '\u{20AF}';
    charset[0xA6 & 0x7F] = '\u{00A6}';
    charset[0xA7 & 0x7F] = '\u{00A7}';
    charset[0xA8 & 0x7F] = '\u{00A8}';
    charset[0xA9 & 0x7F] = '\u{00A9}';
    charset[0xAA & 0x7F] = '\u{037A}';
    charset[0xAB & 0x7F] = '\u{00AB}';
    charset[0xAC & 0x7F] = '\u{00AC}';
    charset[0xAD & 0x7F] = '\u{00AD}';
    charset[0xAE & 0x7F] = '\0';
    charset[0xAF & 0x7F] = '\u{2015}';

    charset[0xB0 & 0x7F] = '\u{00B0}';
    charset[0xB1 & 0x7F] = '\u{00B1}';
    charset[0xB2 & 0x7F] = '\u{00B2}';
    charset[0xB3 & 0x7F] = '\u{00B3}';
    charset[0xB4 & 0x7F] = '\u{0384}';
    charset[0xB5 & 0x7F] = '\u{0385}';
    charset[0xB6 & 0x7F] = '\u{0386}';
    charset[0xB7 & 0x7F] = '\u{00B7}';
    charset[0xB8 & 0x7F] = '\u{0388}';
    charset[0xB9 & 0x7F] = '\u{0389}';
    charset[0xBA & 0x7F] = '\u{038A}';
    charset[0xBB & 0x7F] = '\u{00BB}';
    charset[0xBC & 0x7F] = '\u{038C}';
    charset[0xBD & 0x7F] = '\u{00BD}';
    charset[0xBE & 0x7F] = '\u{038E}';
    charset[0xBF & 0x7F] = '\u{038F}';

    charset[0xC0 & 0x7F] = '\u{0390}';
    charset[0xC1 & 0x7F] = '\u{0391}';
    charset[0xC2 & 0x7F] = '\u{0392}';
    charset[0xC3 & 0x7F] = '\u{0393}';
    charset[0xC4 & 0x7F] = '\u{0394}';
    charset[0xC5 & 0x7F] = '\u{0395}';
    charset[0xC6 & 0x7F] = '\u{0396}';
    charset[0xC7 & 0x7F] = '\u{0397}';
    charset[0xC8 & 0x7F] = '\u{0398}';
    charset[0xC9 & 0x7F] = '\u{0399}';
    charset[0xCA & 0x7F] = '\u{039A}';
    charset[0xCB & 0x7F] = '\u{039B}';
    charset[0xCC & 0x7F] = '\u{039C}';
    charset[0xCD & 0x7F] = '\u{039D}';
    charset[0xCE & 0x7F] = '\u{039E}';
    charset[0xCF & 0x7F] = '\u{039F}';

    charset[0xD0 & 0x7F] = '\u{03A0}';
    charset[0xD1 & 0x7F] = '\u{03A1}';
    charset[0xD2 & 0x7F] = '\0';
    charset[0xD3 & 0x7F] = '\u{03A3}';
    charset[0xD4 & 0x7F] = '\u{03A4}';
    charset[0xD5 & 0x7F] = '\u{03A5}';
    charset[0xD6 & 0x7F] = '\u{03A6}';
    charset[0xD7 & 0x7F] = '\u{03A7}';
    charset[0xD8 & 0x7F] = '\u{03A8}';
    charset[0xD9 & 0x7F] = '\u{03A9}';
    charset[0xDA & 0x7F] = '\u{03AA}';
    charset[0xDB & 0x7F] = '\u{03AB}';
    charset[0xDC & 0x7F] = '\u{03AC}';
    charset[0xDD & 0x7F] = '\u{03AD}';
    charset[0xDE & 0x7F] = '\u{03AE}';
    charset[0xDF & 0x7F] = '\u{03AF}';

    charset[0xE0 & 0x7F] = '\u{03B0}';
    charset[0xE1 & 0x7F] = '\u{03B1}';
    charset[0xE2 & 0x7F] = '\u{03B2}';
    charset[0xE3 & 0x7F] = '\u{03B3}';
    charset[0xE4 & 0x7F] = '\u{03B4}';
    charset[0xE5 & 0x7F] = '\u{03B5}';
    charset[0xE6 & 0x7F] = '\u{03B6}';
    charset[0xE7 & 0x7F] = '\u{03B7}';
    charset[0xE8 & 0x7F] = '\u{03B8}';
    charset[0xE9 & 0x7F] = '\u{03B9}';
    charset[0xEA & 0x7F] = '\u{03BA}';
    charset[0xEB & 0x7F] = '\u{03BB}';
    charset[0xEC & 0x7F] = '\u{03BC}';
    charset[0xED & 0x7F] = '\u{03BD}';
    charset[0xEE & 0x7F] = '\u{03BE}';
    charset[0xEF & 0x7F] = '\u{03BF}';

    charset[0xF0 & 0x7F] = '\u{03C0}';
    charset[0xF1 & 0x7F] = '\u{03C1}';
    charset[0xF2 & 0x7F] = '\u{03C2}';
    charset[0xF3 & 0x7F] = '\u{03C3}';
    charset[0xF4 & 0x7F] = '\u{03C4}';
    charset[0xF5 & 0x7F] = '\u{03C5}';
    charset[0xF6 & 0x7F] = '\u{03C6}';
    charset[0xF7 & 0x7F] = '\u{03C7}';
    charset[0xF8 & 0x7F] = '\u{03C8}';
    charset[0xF9 & 0x7F] = '\u{03C9}';
    charset[0xFA & 0x7F] = '\u{03CA}';
    charset[0xFB & 0x7F] = '\u{03CB}';
    charset[0xFC & 0x7F] = '\u{03CC}';
    charset[0xFD & 0x7F] = '\u{03CD}';
    charset[0xFE & 0x7F] = '\u{03CE}';
    charset[0xFF & 0x7F] = '\0';

    charset
};

/// An encoding for ISO-8859-7.
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-7")))]
pub static ISO_8859_7: Iso8859_7 = Iso8859_7::new();

/// A mapper from bytes over 128 to the corresponding unicode character.
const ENCODER: AsciiCompatible = AsciiCompatible::new(
    <Iso8859_7 as crate::Charset>::CHARSET_NAME,
    &ISO_8859_7_CHARSET,
);

/// An encoding for ISO-8859-7.
/// If possible use [`UTF-8`] instead.
///
/// [`UTF-8`]: crate::Utf8Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::exhaustive_structs)]
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-7")))]
pub struct Iso8859_7;

impl Iso8859_7 {
    /// Create a new ISO-8859-7 encoding instance.
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

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-7")))]
impl core::default::Default for Iso8859_7 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-7")))]
impl crate::Charset for Iso8859_7 {
    const CHARSET_NAME: &'static str = "iso-8859-7";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 1)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // IANA
            Self::CHARSET_NAME,
            "iso-ir-126",
            "iso_8859-7",
            "elot_928",
            "ecma-118",
            "greek",
            "greek8",
            "csisolatingreek",
            "iso_8859-7:1987",
            // code pages
            "cp813",
            "ibm813",
            "cp28597",
            "windows-28597",
            // other
            "iso8859-7",
            "iso88597",
            "sun_eu_greek",
        ]
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-7")))]
impl crate::CharsetDecoding for Iso8859_7 {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-7")))]
impl crate::CharsetEncoding for Iso8859_7 {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}
