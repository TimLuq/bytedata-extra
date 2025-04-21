use crate::ascii7_compat::AsciiCompatible;

/// The ISO-8859-3 charset.
const ISO_8859_3_CHARSET: [char; 128] = {
    let mut charset = super::ISO_8859_1_CHARSET;

    charset[0xA1 & 0x7F] = '\u{0126}';
    charset[0xA2 & 0x7F] = '\u{02D8}';
    charset[0xA6 & 0x7F] = '\u{0124}';
    charset[0xA9 & 0x7F] = '\u{0130}';
    charset[0xAA & 0x7F] = '\u{015E}';
    charset[0xAB & 0x7F] = '\u{011E}';
    charset[0xAC & 0x7F] = '\u{0134}';
    charset[0xAF & 0x7F] = '\u{017B}';
    
    charset[0xB1 & 0x7F] = '\u{0127}';
    charset[0xB6 & 0x7F] = '\u{0125}';
    charset[0xB9 & 0x7F] = '\u{0131}';
    charset[0xBA & 0x7F] = '\u{015F}';
    charset[0xBB & 0x7F] = '\u{011F}';
    charset[0xBC & 0x7F] = '\u{0135}';
    charset[0xBF & 0x7F] = '\u{017C}';

    charset[0xC5 & 0x7F] = '\u{010A}';
    charset[0xC6 & 0x7F] = '\u{0108}';

    charset[0xD5 & 0x7F] = '\u{0120}';
    charset[0xD8 & 0x7F] = '\u{011C}';
    charset[0xDD & 0x7F] = '\u{016C}';
    charset[0xDE & 0x7F] = '\u{015C}';

    charset[0xE5 & 0x7F] = '\u{010B}';
    charset[0xE6 & 0x7F] = '\u{0109}';

    charset[0xF5 & 0x7F] = '\u{0121}';
    charset[0xF8 & 0x7F] = '\u{011D}';
    charset[0xFD & 0x7F] = '\u{016D}';
    charset[0xFE & 0x7F] = '\u{015D}';
    charset[0xFF & 0x7F] = '\u{02D9}';

    charset
};

/// An encoding for ISO-8859-3.
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-3")))]
pub static ISO_8859_3: Iso8859_3 = Iso8859_3::new();

/// A mapper from bytes over 128 to the corresponding unicode character.
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-3")))]
const ENCODER: AsciiCompatible = AsciiCompatible::new(<Iso8859_3 as crate::Charset>::CHARSET_NAME, &ISO_8859_3_CHARSET);

/// An encoding for ISO-8859-3.
/// If possible use [`UTF-8`] instead.
/// 
/// [`UTF-8`]: crate::Utf8Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::exhaustive_structs)]
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-3")))]
pub struct Iso8859_3;

impl Iso8859_3 {

    /// Create a new ISO-8859-3 encoding instance.
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

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-3")))]
impl core::default::Default for Iso8859_3 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-3")))]
impl crate::Charset for Iso8859_3 {
    const CHARSET_NAME: &'static str = "iso-8859-3";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 1)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // IANA
            Self::CHARSET_NAME,
            "iso-ir-109",
            "iso_8859-3",
            "csisolatin3",
            "latin3",
            "l3",
            "iso_8859-3:1988",

            // code pages
            "cp913", "ibm913",
            "cp28593", "windows-28593",

            // other
            "iso8859-3",
            "iso88593",
        ]
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-3")))]
impl crate::CharsetDecoding for Iso8859_3 {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-3")))]
impl crate::CharsetEncoding for Iso8859_3 {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}
