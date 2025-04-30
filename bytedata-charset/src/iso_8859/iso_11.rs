use crate::ascii7_compat::AsciiCompatible;

/// The ISO-8859-11 charset.
#[expect(clippy::redundant_pub_crate)]
pub(crate) const ISO_8859_11_CHARSET: [char; 128] = {
    let mut charset = super::ISO_8859_1_CHARSET;

    charset[0xA0 & 0x7F] = '\u{00A0}';
    charset[0xA1 & 0x7F] = '\u{0E01}';
    charset[0xA2 & 0x7F] = '\u{0E02}';
    charset[0xA3 & 0x7F] = '\u{0E03}';
    charset[0xA4 & 0x7F] = '\u{0E04}';
    charset[0xA5 & 0x7F] = '\u{0E05}';
    charset[0xA6 & 0x7F] = '\u{0E06}';
    charset[0xA7 & 0x7F] = '\u{0E07}';
    charset[0xA8 & 0x7F] = '\u{0E08}';
    charset[0xA9 & 0x7F] = '\u{0E09}';
    charset[0xAA & 0x7F] = '\u{0E0A}';
    charset[0xAB & 0x7F] = '\u{0E0B}';
    charset[0xAC & 0x7F] = '\u{0E0C}';
    charset[0xAD & 0x7F] = '\u{0E0D}';
    charset[0xAE & 0x7F] = '\u{0E0E}';
    charset[0xAF & 0x7F] = '\u{0E0F}';

    charset[0xB0 & 0x7F] = '\u{0E10}';
    charset[0xB1 & 0x7F] = '\u{0E11}';
    charset[0xB2 & 0x7F] = '\u{0E12}';
    charset[0xB3 & 0x7F] = '\u{0E13}';
    charset[0xB4 & 0x7F] = '\u{0E14}';
    charset[0xB5 & 0x7F] = '\u{0E15}';
    charset[0xB6 & 0x7F] = '\u{0E16}';
    charset[0xB7 & 0x7F] = '\u{0E17}';
    charset[0xB8 & 0x7F] = '\u{0E18}';
    charset[0xB9 & 0x7F] = '\u{0E19}';
    charset[0xBA & 0x7F] = '\u{0E1A}';
    charset[0xBB & 0x7F] = '\u{0E1B}';
    charset[0xBC & 0x7F] = '\u{0E1C}';
    charset[0xBD & 0x7F] = '\u{0E1D}';
    charset[0xBE & 0x7F] = '\u{0E1E}';
    charset[0xBF & 0x7F] = '\u{0E1F}';

    charset[0xC0 & 0x7F] = '\u{0E20}';
    charset[0xC1 & 0x7F] = '\u{0E21}';
    charset[0xC2 & 0x7F] = '\u{0E22}';
    charset[0xC3 & 0x7F] = '\u{0E23}';
    charset[0xC4 & 0x7F] = '\u{0E24}';
    charset[0xC5 & 0x7F] = '\u{0E25}';
    charset[0xC6 & 0x7F] = '\u{0E26}';
    charset[0xC7 & 0x7F] = '\u{0E27}';
    charset[0xC8 & 0x7F] = '\u{0E28}';
    charset[0xC9 & 0x7F] = '\u{0E29}';
    charset[0xCA & 0x7F] = '\u{0E2A}';
    charset[0xCB & 0x7F] = '\u{0E2B}';
    charset[0xCC & 0x7F] = '\u{0E2C}';
    charset[0xCD & 0x7F] = '\u{0E2D}';
    charset[0xCE & 0x7F] = '\u{0E2E}';
    charset[0xCF & 0x7F] = '\u{0E2F}';

    charset[0xD0 & 0x7F] = '\u{0E30}';
    charset[0xD1 & 0x7F] = '\u{0E31}';
    charset[0xD2 & 0x7F] = '\u{0E32}';
    charset[0xD3 & 0x7F] = '\u{0E33}';
    charset[0xD4 & 0x7F] = '\u{0E34}';
    charset[0xD5 & 0x7F] = '\u{0E35}';
    charset[0xD6 & 0x7F] = '\u{0E36}';
    charset[0xD7 & 0x7F] = '\u{0E37}';
    charset[0xD8 & 0x7F] = '\u{0E38}';
    charset[0xD9 & 0x7F] = '\u{0E39}';
    charset[0xDA & 0x7F] = '\u{0E3A}';

    charset[0xDF & 0x7F] = '\u{0E3F}';

    charset[0xE0 & 0x7F] = '\u{0E40}';
    charset[0xE1 & 0x7F] = '\u{0E41}';
    charset[0xE2 & 0x7F] = '\u{0E42}';
    charset[0xE3 & 0x7F] = '\u{0E43}';
    charset[0xE4 & 0x7F] = '\u{0E44}';
    charset[0xE5 & 0x7F] = '\u{0E45}';
    charset[0xE6 & 0x7F] = '\u{0E46}';
    charset[0xE7 & 0x7F] = '\u{0E47}';
    charset[0xE8 & 0x7F] = '\u{0E48}';
    charset[0xE9 & 0x7F] = '\u{0E49}';
    charset[0xEA & 0x7F] = '\u{0E4A}';
    charset[0xEB & 0x7F] = '\u{0E4B}';
    charset[0xEC & 0x7F] = '\u{0E4C}';
    charset[0xED & 0x7F] = '\u{0E4D}';
    charset[0xEE & 0x7F] = '\u{0E4E}';
    charset[0xEF & 0x7F] = '\u{0E4F}';

    charset[0xF0 & 0x7F] = '\u{0E50}';
    charset[0xF1 & 0x7F] = '\u{0E51}';
    charset[0xF2 & 0x7F] = '\u{0E52}';
    charset[0xF3 & 0x7F] = '\u{0E53}';
    charset[0xF4 & 0x7F] = '\u{0E54}';
    charset[0xF5 & 0x7F] = '\u{0E55}';
    charset[0xF6 & 0x7F] = '\u{0E56}';
    charset[0xF7 & 0x7F] = '\u{0E57}';
    charset[0xF8 & 0x7F] = '\u{0E58}';
    charset[0xF9 & 0x7F] = '\u{0E59}';
    charset[0xFA & 0x7F] = '\u{0E5A}';
    charset[0xFB & 0x7F] = '\u{0E5B}';

    charset
};

/// An encoding for ISO-8859-11.
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-11")))]
pub static ISO_8859_11: Iso8859_11 = Iso8859_11::new();

/// A mapper from bytes over 128 to the corresponding unicode character.
const ENCODER: AsciiCompatible = AsciiCompatible::new(
    <Iso8859_11 as crate::Charset>::CHARSET_NAME,
    &ISO_8859_11_CHARSET,
);

/// An encoding for ISO-8859-11.
/// If possible use [`UTF-8`] instead.
///
/// [`UTF-8`]: crate::Utf8Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::exhaustive_structs)]
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-11")))]
pub struct Iso8859_11;

impl Iso8859_11 {
    /// Create a new ISO-8859-11 encoding instance.
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

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-11")))]
impl core::default::Default for Iso8859_11 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-11")))]
impl crate::Charset for Iso8859_11 {
    const CHARSET_NAME: &'static str = "iso-8859-11";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 1)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // IANA
            Self::CHARSET_NAME,
            "tis-620",
            "cstis620",
            // code pages
            "cp28601",
            "windows-28601",
        ]
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-11")))]
impl crate::CharsetDecoding for Iso8859_11 {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-11")))]
impl crate::CharsetEncoding for Iso8859_11 {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}
