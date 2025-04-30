use crate::ascii7_compat::AsciiCompatible;

/// The ISO-8859-16 charset.
const ISO_8859_16_CHARSET: [char; 128] = {
    let mut charset = super::ISO_8859_1_CHARSET;

    charset[0xA1 & 0x7F] = '\u{0104}';
    charset[0xA2 & 0x7F] = '\u{0105}';
    charset[0xA3 & 0x7F] = '\u{0141}';
    charset[0xA4 & 0x7F] = '\u{20AC}';
    charset[0xA5 & 0x7F] = '\u{201E}';
    charset[0xA6 & 0x7F] = '\u{0160}';

    charset[0xA8 & 0x7F] = '\u{0161}';

    charset[0xAA & 0x7F] = '\u{0218}';

    charset[0xAC & 0x7F] = '\u{0179}';

    charset[0xAE & 0x7F] = '\u{017A}';
    charset[0xAF & 0x7F] = '\u{017B}';

    charset[0xB2 & 0x7F] = '\u{010C}';
    charset[0xB3 & 0x7F] = '\u{0142}';
    charset[0xB4 & 0x7F] = '\u{017D}';
    charset[0xB5 & 0x7F] = '\u{201D}';

    charset[0xB8 & 0x7F] = '\u{017E}';
    charset[0xB9 & 0x7F] = '\u{010D}';
    charset[0xBA & 0x7F] = '\u{0219}';

    charset[0xBC & 0x7F] = '\u{0152}';
    charset[0xBD & 0x7F] = '\u{0153}';
    charset[0xBE & 0x7F] = '\u{0178}';
    charset[0xBF & 0x7F] = '\u{017C}';

    charset[0xC3 & 0x7F] = '\u{0102}';

    charset[0xC5 & 0x7F] = '\u{0106}';

    charset[0xD0 & 0x7F] = '\u{0110}';
    charset[0xD1 & 0x7F] = '\u{0143}';

    charset[0xD5 & 0x7F] = '\u{0150}';

    charset[0xD7 & 0x7F] = '\u{015A}';
    charset[0xD8 & 0x7F] = '\u{0170}';

    charset[0xDD & 0x7F] = '\u{0118}';
    charset[0xDE & 0x7F] = '\u{021A}';

    charset[0xE3 & 0x7F] = '\u{0103}';

    charset[0xE5 & 0x7F] = '\u{0107}';

    charset[0xF0 & 0x7F] = '\u{0111}';
    charset[0xF1 & 0x7F] = '\u{0144}';

    charset[0xF5 & 0x7F] = '\u{0151}';

    charset[0xF7 & 0x7F] = '\u{015B}';
    charset[0xF8 & 0x7F] = '\u{0171}';

    charset[0xFD & 0x7F] = '\u{0119}';
    charset[0xFE & 0x7F] = '\u{021B}';

    charset
};

/// An encoding for ISO-8859-16.
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-16")))]
pub static ISO_8859_16: Iso8859_16 = Iso8859_16::new();

/// A mapper from bytes over 128 to the corresponding unicode character.
const ENCODER: AsciiCompatible = AsciiCompatible::new(
    <Iso8859_16 as crate::Charset>::CHARSET_NAME,
    &ISO_8859_16_CHARSET,
);

/// An encoding for ISO-8859-16.
/// If possible use [`UTF-8`] instead.
///
/// [`UTF-8`]: crate::Utf8Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::exhaustive_structs)]
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-16")))]
pub struct Iso8859_16;

impl Iso8859_16 {
    /// Create a new ISO-8859-16 encoding instance.
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

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-16")))]
impl core::default::Default for Iso8859_16 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-16")))]
impl crate::Charset for Iso8859_16 {
    const CHARSET_NAME: &'static str = "iso-8859-16";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 1)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // IANA
            Self::CHARSET_NAME,
            "iso-ir-226",
            "iso_8859-16:2001",
            "iso_8859-16",
            "latin10",
            "l10",
            "csiso885916",
            // code pages
            "cp28606",
            "windows-28606",
            // other
            "iso8859-16",
            "iso885916",
        ]
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-16")))]
impl crate::CharsetDecoding for Iso8859_16 {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-16")))]
impl crate::CharsetEncoding for Iso8859_16 {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}
