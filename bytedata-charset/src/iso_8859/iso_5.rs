use crate::ascii7_compat::AsciiCompatible;

/// The ISO-8859-5 charset.
const ISO_8859_5_CHARSET: [char; 128] = {
    let mut charset = super::ISO_8859_1_CHARSET;

    charset[0xA0 & 0x7F] = '\u{00A0}';
    charset[0xA1 & 0x7F] = '\u{0401}';
    charset[0xA2 & 0x7F] = '\u{0402}';
    charset[0xA3 & 0x7F] = '\u{0403}';
    charset[0xA4 & 0x7F] = '\u{0404}';
    charset[0xA5 & 0x7F] = '\u{0405}';
    charset[0xA6 & 0x7F] = '\u{0406}';
    charset[0xA7 & 0x7F] = '\u{0407}';
    charset[0xA8 & 0x7F] = '\u{0408}';
    charset[0xA9 & 0x7F] = '\u{0409}';
    charset[0xAA & 0x7F] = '\u{040A}';
    charset[0xAB & 0x7F] = '\u{040B}';
    charset[0xAC & 0x7F] = '\u{040C}';
    charset[0xAD & 0x7F] = '\u{00AD}';
    charset[0xAE & 0x7F] = '\u{040E}';
    charset[0xAF & 0x7F] = '\u{040F}';

    charset[0xB0 & 0x7F] = '\u{0410}';
    charset[0xB1 & 0x7F] = '\u{0411}';
    charset[0xB2 & 0x7F] = '\u{0412}';
    charset[0xB3 & 0x7F] = '\u{0413}';
    charset[0xB4 & 0x7F] = '\u{0414}';
    charset[0xB5 & 0x7F] = '\u{0415}';
    charset[0xB6 & 0x7F] = '\u{0416}';
    charset[0xB7 & 0x7F] = '\u{0417}';
    charset[0xB8 & 0x7F] = '\u{0418}';
    charset[0xB9 & 0x7F] = '\u{0419}';
    charset[0xBA & 0x7F] = '\u{041A}';
    charset[0xBB & 0x7F] = '\u{041B}';
    charset[0xBC & 0x7F] = '\u{041C}';
    charset[0xBD & 0x7F] = '\u{041D}';
    charset[0xBE & 0x7F] = '\u{041E}';
    charset[0xBF & 0x7F] = '\u{041F}';

    charset[0xC0 & 0x7F] = '\u{0420}';
    charset[0xC1 & 0x7F] = '\u{0421}';
    charset[0xC2 & 0x7F] = '\u{0422}';
    charset[0xC3 & 0x7F] = '\u{0423}';
    charset[0xC4 & 0x7F] = '\u{0424}';
    charset[0xC5 & 0x7F] = '\u{0425}';
    charset[0xC6 & 0x7F] = '\u{0426}';
    charset[0xC7 & 0x7F] = '\u{0427}';
    charset[0xC8 & 0x7F] = '\u{0428}';
    charset[0xC9 & 0x7F] = '\u{0429}';
    charset[0xCA & 0x7F] = '\u{042A}';
    charset[0xCB & 0x7F] = '\u{042B}';
    charset[0xCC & 0x7F] = '\u{042C}';
    charset[0xCD & 0x7F] = '\u{042D}';
    charset[0xCE & 0x7F] = '\u{042E}';
    charset[0xCF & 0x7F] = '\u{042F}';

    charset[0xD0 & 0x7F] = '\u{0430}';
    charset[0xD1 & 0x7F] = '\u{0431}';
    charset[0xD2 & 0x7F] = '\u{0432}';
    charset[0xD3 & 0x7F] = '\u{0433}';
    charset[0xD4 & 0x7F] = '\u{0434}';
    charset[0xD5 & 0x7F] = '\u{0435}';
    charset[0xD6 & 0x7F] = '\u{0436}';
    charset[0xD7 & 0x7F] = '\u{0437}';
    charset[0xD8 & 0x7F] = '\u{0438}';
    charset[0xD9 & 0x7F] = '\u{0439}';
    charset[0xDA & 0x7F] = '\u{043A}';
    charset[0xDB & 0x7F] = '\u{043B}';
    charset[0xDC & 0x7F] = '\u{043C}';
    charset[0xDD & 0x7F] = '\u{043D}';
    charset[0xDE & 0x7F] = '\u{043E}';
    charset[0xDF & 0x7F] = '\u{043F}';

    charset[0xE0 & 0x7F] = '\u{0440}';
    charset[0xE1 & 0x7F] = '\u{0441}';
    charset[0xE2 & 0x7F] = '\u{0442}';
    charset[0xE3 & 0x7F] = '\u{0443}';
    charset[0xE4 & 0x7F] = '\u{0444}';
    charset[0xE5 & 0x7F] = '\u{0445}';
    charset[0xE6 & 0x7F] = '\u{0446}';
    charset[0xE7 & 0x7F] = '\u{0447}';
    charset[0xE8 & 0x7F] = '\u{0448}';
    charset[0xE9 & 0x7F] = '\u{0449}';
    charset[0xEA & 0x7F] = '\u{044A}';
    charset[0xEB & 0x7F] = '\u{044B}';
    charset[0xEC & 0x7F] = '\u{044C}';
    charset[0xED & 0x7F] = '\u{044D}';
    charset[0xEE & 0x7F] = '\u{044E}';
    charset[0xEF & 0x7F] = '\u{044F}';

    charset[0xF0 & 0x7F] = '\u{2116}';
    charset[0xF1 & 0x7F] = '\u{0451}';
    charset[0xF2 & 0x7F] = '\u{0452}';
    charset[0xF3 & 0x7F] = '\u{0453}';
    charset[0xF4 & 0x7F] = '\u{0454}';
    charset[0xF5 & 0x7F] = '\u{0455}';
    charset[0xF6 & 0x7F] = '\u{0456}';
    charset[0xF7 & 0x7F] = '\u{0457}';
    charset[0xF8 & 0x7F] = '\u{0458}';
    charset[0xF9 & 0x7F] = '\u{0459}';
    charset[0xFA & 0x7F] = '\u{045A}';
    charset[0xFB & 0x7F] = '\u{045B}';
    charset[0xFC & 0x7F] = '\u{045C}';
    charset[0xFD & 0x7F] = '\u{00A7}';
    charset[0xFE & 0x7F] = '\u{045E}';
    charset[0xFF & 0x7F] = '\u{045F}';

    charset
};

/// An encoding for ISO-8859-5.
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-5")))]
pub static ISO_8859_5: Iso8859_5 = Iso8859_5::new();

/// A mapper from bytes over 128 to the corresponding unicode character.
const ENCODER: AsciiCompatible = AsciiCompatible::new(<Iso8859_5 as crate::Charset>::CHARSET_NAME, &ISO_8859_5_CHARSET);

/// An encoding for ISO-8859-5.
/// If possible use [`UTF-8`] instead.
/// 
/// [`UTF-8`]: crate::Utf8Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::exhaustive_structs)]
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-5")))]
pub struct Iso8859_5;

impl Iso8859_5 {

    /// Create a new ISO-8859-5 encoding instance.
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

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-5")))]
impl core::default::Default for Iso8859_5 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-5")))]
impl crate::Charset for Iso8859_5 {
    const CHARSET_NAME: &'static str = "iso-8859-5";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 1)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // IANA
            Self::CHARSET_NAME,
            "iso-ir-144",
            "iso_8859-5",
            "cyrillic",
            "csisolatincyrillic",
            "iso_8859-5:1988",
            
            // code pages
            "cp28595", "windows-28595",

            // other
            "iso8859-5",
            "iso88595",
        ]
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-5")))]
impl crate::CharsetDecoding for Iso8859_5 {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-5")))]
impl crate::CharsetEncoding for Iso8859_5 {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}
