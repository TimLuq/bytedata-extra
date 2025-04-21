use crate::ascii7_compat::AsciiCompatible;

/// The ISO-8859-2 charset.
#[expect(clippy::redundant_pub_crate)]
pub(crate) const ISO_8859_2_CHARSET: [char; 128] = {
    let mut charset = super::ISO_8859_1_CHARSET;

    charset[0xA1 & 0x7F] = '\u{0104}';
    charset[0xA2 & 0x7F] = '\u{02D8}';
    charset[0xA3 & 0x7F] = '\u{0141}';
    charset[0xA5 & 0x7F] = '\u{013D}';
    charset[0xA6 & 0x7F] = '\u{015A}';
    charset[0xA9 & 0x7F] = '\u{0160}';
    charset[0xAA & 0x7F] = '\u{015E}';
    charset[0xAB & 0x7F] = '\u{0164}';
    charset[0xAC & 0x7F] = '\u{0179}';
    charset[0xAE & 0x7F] = '\u{017D}';
    charset[0xAF & 0x7F] = '\u{017B}';

    charset[0xB1 & 0x7F] = '\u{0105}';
    charset[0xB2 & 0x7F] = '\u{02DB}';
    charset[0xB3 & 0x7F] = '\u{0142}';
    charset[0xB5 & 0x7F] = '\u{013E}';
    charset[0xB6 & 0x7F] = '\u{015B}';
    charset[0xB7 & 0x7F] = '\u{02C7}';
    charset[0xB9 & 0x7F] = '\u{0161}';
    charset[0xBA & 0x7F] = '\u{015F}';
    charset[0xBB & 0x7F] = '\u{0165}';
    charset[0xBC & 0x7F] = '\u{017A}';
    charset[0xBD & 0x7F] = '\u{02DD}';
    charset[0xBE & 0x7F] = '\u{017E}';
    charset[0xBF & 0x7F] = '\u{017C}';

    charset[0xC0 & 0x7F] = '\u{0154}';
    charset[0xC3 & 0x7F] = '\u{0102}';
    charset[0xC5 & 0x7F] = '\u{0139}';
    charset[0xC6 & 0x7F] = '\u{0106}';
    charset[0xC8 & 0x7F] = '\u{010C}';
    charset[0xCA & 0x7F] = '\u{0118}';
    charset[0xCC & 0x7F] = '\u{011A}';
    charset[0xCF & 0x7F] = '\u{010E}';

    charset[0xD0 & 0x7F] = '\u{0110}';
    charset[0xD1 & 0x7F] = '\u{0143}';
    charset[0xD2 & 0x7F] = '\u{0147}';
    charset[0xD5 & 0x7F] = '\u{0150}';
    charset[0xD8 & 0x7F] = '\u{0158}';
    charset[0xD9 & 0x7F] = '\u{016E}';
    charset[0xDB & 0x7F] = '\u{0170}';
    charset[0xDE & 0x7F] = '\u{0162}';

    charset[0xE0 & 0x7F] = '\u{0155}';
    charset[0xE3 & 0x7F] = '\u{0103}';
    charset[0xE5 & 0x7F] = '\u{013A}';
    charset[0xE6 & 0x7F] = '\u{0107}';
    charset[0xE8 & 0x7F] = '\u{010D}';
    charset[0xEA & 0x7F] = '\u{0119}';
    charset[0xEC & 0x7F] = '\u{011B}';
    charset[0xEF & 0x7F] = '\u{010F}';
    
    charset[0xF0 & 0x7F] = '\u{0111}';
    charset[0xF1 & 0x7F] = '\u{0144}';
    charset[0xF2 & 0x7F] = '\u{0148}';
    charset[0xF5 & 0x7F] = '\u{0151}';
    charset[0xF8 & 0x7F] = '\u{0159}';
    charset[0xF9 & 0x7F] = '\u{016F}';
    charset[0xFB & 0x7F] = '\u{0171}';
    charset[0xFE & 0x7F] = '\u{0163}';
    charset[0xFF & 0x7F] = '\u{02D9}';

    charset
};

/// An encoding for ISO-8859-2.
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-2")))]
pub static ISO_8859_2: Iso8859_2 = Iso8859_2::new();

/// A mapper from bytes over 128 to the corresponding unicode character.
const ENCODER: AsciiCompatible = AsciiCompatible::new(<Iso8859_2 as crate::Charset>::CHARSET_NAME, &ISO_8859_2_CHARSET);

/// An encoding for ISO-8859-2.
/// If possible use [`UTF-8`] instead.
/// 
/// [`UTF-8`]: crate::Utf8Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::exhaustive_structs)]
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-2")))]
pub struct Iso8859_2;

impl Iso8859_2 {

    /// Create a new ISO-8859-2 encoding instance.
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

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-2")))]
impl core::default::Default for Iso8859_2 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-2")))]
impl crate::Charset for Iso8859_2 {
    const CHARSET_NAME: &'static str = "iso-8859-2";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 1)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // IANA
            Self::CHARSET_NAME,
            "iso-ir-101",
            "iso_8859-2",
            "csisolatin2",
            "latin2",
            "l2",
            "iso_8859-2:1987",
            
            // code pages
            "cp819", "ibm819",
            "cp28592", "windows-28592",

            // other
            "iso8859-2",
            "iso88592",
        ]
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-2")))]
impl crate::CharsetDecoding for Iso8859_2 {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-2")))]
impl crate::CharsetEncoding for Iso8859_2 {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}
