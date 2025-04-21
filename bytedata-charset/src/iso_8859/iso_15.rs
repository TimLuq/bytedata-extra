use crate::ascii7_compat::AsciiCompatible;

/// The ISO-8859-15 charset.
const ISO_8859_15_CHARSET: [char; 128] = {
    let mut charset = super::ISO_8859_1_CHARSET;
    charset[0xA4 & 0x7F] = '\u{20AC}';
    charset[0xA6 & 0x7F] = '\u{0160}';
    charset[0xA8 & 0x7F] = '\u{0161}';
    charset[0xB4 & 0x7F] = '\u{017D}';
    charset[0xB8 & 0x7F] = '\u{017E}';
    charset[0xBC & 0x7F] = '\u{0152}';
    charset[0xBD & 0x7F] = '\u{0153}';
    charset[0xBE & 0x7F] = '\u{0178}';
    charset
};

/// An encoding for ISO-8859-15.
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-15")))]
pub static ISO_8859_15: Iso8859_15 = Iso8859_15::new();

/// A mapper from bytes over 128 to the corresponding unicode character.
const ENCODER: AsciiCompatible = AsciiCompatible::new(<Iso8859_15 as crate::Charset>::CHARSET_NAME, &ISO_8859_15_CHARSET);

/// An encoding for ISO-8859-15.
/// If possible use [`UTF-8`] instead.
/// 
/// [`UTF-8`]: crate::Utf8Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::exhaustive_structs)]
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-15")))]
pub struct Iso8859_15;

impl Iso8859_15 {

    /// Create a new ISO-8859-15 encoding instance.
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

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-15")))]
impl core::default::Default for Iso8859_15 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-15")))]
impl crate::Charset for Iso8859_15 {
    const CHARSET_NAME: &'static str = "iso-8859-15";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 1)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // IANA
            Self::CHARSET_NAME,
            "csiso885915",
            "iso_8859-15",
            "latin-9",
            
            // code pages
            "cp923", "ibm923",
            "cp28605", "windows-28605",

            // other
            "iso8859-15",
            "iso885915",
            "latin9",
            "l9",
            "csisolatin9",
        ]
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-15")))]
impl crate::CharsetDecoding for Iso8859_15 {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-15")))]
impl crate::CharsetEncoding for Iso8859_15 {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}
