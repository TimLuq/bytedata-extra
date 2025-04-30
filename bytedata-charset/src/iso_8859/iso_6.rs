use crate::ascii7_compat::AsciiCompatible;

/// The ISO-8859-6 charset.
const ISO_8859_6_CHARSET: [char; 128] = {
    let mut charset = super::ISO_8859_1_CHARSET;

    charset[0xA0 & 0x7F] = '\u{00A0}';

    charset[0xA4 & 0x7F] = '\u{00A4}';

    charset[0xAC & 0x7F] = '\u{060C}';
    charset[0xAD & 0x7F] = '\u{00AD}';

    charset[0xBB & 0x7F] = '\u{061B}';

    charset[0xBF & 0x7F] = '\u{061F}';

    charset[0xC1 & 0x7F] = '\u{0621}';
    charset[0xC2 & 0x7F] = '\u{0622}';
    charset[0xC3 & 0x7F] = '\u{0623}';
    charset[0xC4 & 0x7F] = '\u{0624}';
    charset[0xC5 & 0x7F] = '\u{0625}';
    charset[0xC6 & 0x7F] = '\u{0626}';
    charset[0xC7 & 0x7F] = '\u{0627}';
    charset[0xC8 & 0x7F] = '\u{0628}';
    charset[0xC9 & 0x7F] = '\u{0629}';
    charset[0xCA & 0x7F] = '\u{062A}';
    charset[0xCB & 0x7F] = '\u{062B}';
    charset[0xCC & 0x7F] = '\u{062C}';
    charset[0xCD & 0x7F] = '\u{062D}';
    charset[0xCE & 0x7F] = '\u{062E}';
    charset[0xCF & 0x7F] = '\u{062F}';
    charset[0xD0 & 0x7F] = '\u{0630}';
    charset[0xD1 & 0x7F] = '\u{0631}';
    charset[0xD2 & 0x7F] = '\u{0632}';
    charset[0xD3 & 0x7F] = '\u{0633}';
    charset[0xD4 & 0x7F] = '\u{0634}';
    charset[0xD5 & 0x7F] = '\u{0635}';
    charset[0xD6 & 0x7F] = '\u{0636}';
    charset[0xD7 & 0x7F] = '\u{0637}';
    charset[0xD8 & 0x7F] = '\u{0638}';
    charset[0xD9 & 0x7F] = '\u{0639}';
    charset[0xDA & 0x7F] = '\u{063A}';

    charset[0xE0 & 0x7F] = '\u{0640}';
    charset[0xE1 & 0x7F] = '\u{0641}';
    charset[0xE2 & 0x7F] = '\u{0642}';
    charset[0xE3 & 0x7F] = '\u{0643}';
    charset[0xE4 & 0x7F] = '\u{0644}';
    charset[0xE5 & 0x7F] = '\u{0645}';
    charset[0xE6 & 0x7F] = '\u{0646}';
    charset[0xE7 & 0x7F] = '\u{0647}';
    charset[0xE8 & 0x7F] = '\u{0648}';
    charset[0xE9 & 0x7F] = '\u{0649}';
    charset[0xEA & 0x7F] = '\u{064A}';
    charset[0xEB & 0x7F] = '\u{064B}';
    charset[0xEC & 0x7F] = '\u{064C}';
    charset[0xED & 0x7F] = '\u{064D}';
    charset[0xEE & 0x7F] = '\u{064E}';
    charset[0xEF & 0x7F] = '\u{064F}';

    charset[0xF0 & 0x7F] = '\u{0650}';
    charset[0xF1 & 0x7F] = '\u{0651}';
    charset[0xF2 & 0x7F] = '\u{0652}';

    charset
};

/// An encoding for ISO-8859-6.
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-6")))]
pub static ISO_8859_6: Iso8859_6 = Iso8859_6::new();

/// A mapper from bytes over 128 to the corresponding unicode character.
const ENCODER: AsciiCompatible = AsciiCompatible::new(
    <Iso8859_6 as crate::Charset>::CHARSET_NAME,
    &ISO_8859_6_CHARSET,
);

/// An encoding for ISO-8859-6.
/// If possible use [`UTF-8`] instead.
///
/// [`UTF-8`]: crate::Utf8Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::exhaustive_structs)]
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-6")))]
pub struct Iso8859_6;

impl Iso8859_6 {
    /// Create a new ISO-8859-6 encoding instance.
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

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-6")))]
impl core::default::Default for Iso8859_6 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-6")))]
impl crate::Charset for Iso8859_6 {
    const CHARSET_NAME: &'static str = "iso-8859-6";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 1)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // IANA
            Self::CHARSET_NAME,
            "iso-ir-127",
            "iso_8859-6",
            "arabic",
            "csisolatinarabic",
            "ecma-114",
            "asmo-708",
            "iso_8859-6:1987",
            // code pages
            "cp1089",
            "ibm1089",
            "cp28596",
            "windows-28596",
            // other
            "iso8859-6",
            "iso88596",
            "iso-8859-6-e",
            "iso-8859-6-i",
            "csiso88596e",
            "csiso88596i",
        ]
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-6")))]
impl crate::CharsetDecoding for Iso8859_6 {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-6")))]
impl crate::CharsetEncoding for Iso8859_6 {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}
