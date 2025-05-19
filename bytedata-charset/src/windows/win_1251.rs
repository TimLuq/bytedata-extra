use crate::ascii7_compat::AsciiCompatible;

const WIN_1251_CHARSET: [char; 128] = {
    // SAFETY: This is a static array of 128 characters, initialized to zero.
    let mut charset: [char; 128] = unsafe { core::mem::zeroed() };

    charset[0x80 & 0x7F] = '\u{0402}';
    charset[0x81 & 0x7F] = '\u{0403}';
    charset[0x82 & 0x7F] = '\u{201A}';
    charset[0x83 & 0x7F] = '\u{0453}';
    charset[0x84 & 0x7F] = '\u{201E}';
    charset[0x85 & 0x7F] = '\u{2026}';
    charset[0x86 & 0x7F] = '\u{2020}';
    charset[0x87 & 0x7F] = '\u{2021}';
    charset[0x88 & 0x7F] = '\u{20AC}';
    charset[0x89 & 0x7F] = '\u{2030}';
    charset[0x8a & 0x7F] = '\u{0409}';
    charset[0x8b & 0x7F] = '\u{2039}';
    charset[0x8c & 0x7F] = '\u{040A}';
    charset[0x8d & 0x7F] = '\u{040C}';
    charset[0x8e & 0x7F] = '\u{040B}';
    charset[0x8f & 0x7F] = '\u{040F}';

    charset[0x90 & 0x7F] = '\u{0452}';
    charset[0x91 & 0x7F] = '\u{2018}';
    charset[0x92 & 0x7F] = '\u{2019}';
    charset[0x93 & 0x7F] = '\u{201C}';
    charset[0x94 & 0x7F] = '\u{201D}';
    charset[0x95 & 0x7F] = '\u{2022}';
    charset[0x96 & 0x7F] = '\u{2013}';
    charset[0x97 & 0x7F] = '\u{2014}';
    //charset[0x98 & 0x7F] = '\u{undefined}';
    charset[0x99 & 0x7F] = '\u{2122}';
    charset[0x9a & 0x7F] = '\u{0459}';
    charset[0x9b & 0x7F] = '\u{203A}';
    charset[0x9c & 0x7F] = '\u{045A}';
    charset[0x9d & 0x7F] = '\u{045C}';
    charset[0x9e & 0x7F] = '\u{045B}';
    charset[0x9f & 0x7F] = '\u{045F}';

    charset[0xa0 & 0x7F] = '\u{00A0}';
    charset[0xa1 & 0x7F] = '\u{040E}';
    charset[0xa2 & 0x7F] = '\u{045E}';
    charset[0xa3 & 0x7F] = '\u{0408}';
    charset[0xa4 & 0x7F] = '\u{00A4}';
    charset[0xa5 & 0x7F] = '\u{0490}';
    charset[0xa6 & 0x7F] = '\u{00A6}';
    charset[0xa7 & 0x7F] = '\u{00A7}';
    charset[0xa8 & 0x7F] = '\u{0401}';
    charset[0xa9 & 0x7F] = '\u{00A9}';
    charset[0xaa & 0x7F] = '\u{0404}';
    charset[0xab & 0x7F] = '\u{00AB}';
    charset[0xac & 0x7F] = '\u{00AC}';
    charset[0xad & 0x7F] = '\u{00AD}';
    charset[0xae & 0x7F] = '\u{00AE}';
    charset[0xaf & 0x7F] = '\u{0407}';

    charset[0xb0 & 0x7F] = '\u{00B0}';
    charset[0xb1 & 0x7F] = '\u{00B1}';
    charset[0xb2 & 0x7F] = '\u{0406}';
    charset[0xb3 & 0x7F] = '\u{0456}';
    charset[0xb4 & 0x7F] = '\u{0491}';
    charset[0xb5 & 0x7F] = '\u{00B5}';
    charset[0xb6 & 0x7F] = '\u{00B6}';
    charset[0xb7 & 0x7F] = '\u{00B7}';
    charset[0xb8 & 0x7F] = '\u{0451}';
    charset[0xb9 & 0x7F] = '\u{2116}';
    charset[0xba & 0x7F] = '\u{0454}';
    charset[0xbb & 0x7F] = '\u{00BB}';
    charset[0xbc & 0x7F] = '\u{0458}';
    charset[0xbd & 0x7F] = '\u{0405}';
    charset[0xbe & 0x7F] = '\u{0455}';
    charset[0xbf & 0x7F] = '\u{0457}';

    charset[0xc0 & 0x7F] = '\u{0410}';
    charset[0xc1 & 0x7F] = '\u{0411}';
    charset[0xc2 & 0x7F] = '\u{0412}';
    charset[0xc3 & 0x7F] = '\u{0413}';
    charset[0xc4 & 0x7F] = '\u{0414}';
    charset[0xc5 & 0x7F] = '\u{0415}';
    charset[0xc6 & 0x7F] = '\u{0416}';
    charset[0xc7 & 0x7F] = '\u{0417}';
    charset[0xc8 & 0x7F] = '\u{0418}';
    charset[0xc9 & 0x7F] = '\u{0419}';
    charset[0xca & 0x7F] = '\u{041A}';
    charset[0xcb & 0x7F] = '\u{041B}';
    charset[0xcc & 0x7F] = '\u{041C}';
    charset[0xcd & 0x7F] = '\u{041D}';
    charset[0xce & 0x7F] = '\u{041E}';
    charset[0xcf & 0x7F] = '\u{041F}';

    charset[0xd0 & 0x7F] = '\u{0420}';
    charset[0xd1 & 0x7F] = '\u{0421}';
    charset[0xd2 & 0x7F] = '\u{0422}';
    charset[0xd3 & 0x7F] = '\u{0423}';
    charset[0xd4 & 0x7F] = '\u{0424}';
    charset[0xd5 & 0x7F] = '\u{0425}';
    charset[0xd6 & 0x7F] = '\u{0426}';
    charset[0xd7 & 0x7F] = '\u{0427}';
    charset[0xd8 & 0x7F] = '\u{0428}';
    charset[0xd9 & 0x7F] = '\u{0429}';
    charset[0xda & 0x7F] = '\u{042A}';
    charset[0xdb & 0x7F] = '\u{042B}';
    charset[0xdc & 0x7F] = '\u{042C}';
    charset[0xdd & 0x7F] = '\u{042D}';
    charset[0xde & 0x7F] = '\u{042E}';
    charset[0xdf & 0x7F] = '\u{042F}';

    charset[0xe0 & 0x7F] = '\u{0430}';
    charset[0xe1 & 0x7F] = '\u{0431}';
    charset[0xe2 & 0x7F] = '\u{0432}';
    charset[0xe3 & 0x7F] = '\u{0433}';
    charset[0xe4 & 0x7F] = '\u{0434}';
    charset[0xe5 & 0x7F] = '\u{0435}';
    charset[0xe6 & 0x7F] = '\u{0436}';
    charset[0xe7 & 0x7F] = '\u{0437}';
    charset[0xe8 & 0x7F] = '\u{0438}';
    charset[0xe9 & 0x7F] = '\u{0439}';
    charset[0xea & 0x7F] = '\u{043A}';
    charset[0xeb & 0x7F] = '\u{043B}';
    charset[0xec & 0x7F] = '\u{043C}';
    charset[0xed & 0x7F] = '\u{043D}';
    charset[0xee & 0x7F] = '\u{043E}';
    charset[0xef & 0x7F] = '\u{043F}';

    charset[0xf0 & 0x7F] = '\u{0440}';
    charset[0xf1 & 0x7F] = '\u{0441}';
    charset[0xf2 & 0x7F] = '\u{0442}';
    charset[0xf3 & 0x7F] = '\u{0443}';
    charset[0xf4 & 0x7F] = '\u{0444}';
    charset[0xf5 & 0x7F] = '\u{0445}';
    charset[0xf6 & 0x7F] = '\u{0446}';
    charset[0xf7 & 0x7F] = '\u{0447}';
    charset[0xf8 & 0x7F] = '\u{0448}';
    charset[0xf9 & 0x7F] = '\u{0449}';
    charset[0xfa & 0x7F] = '\u{044A}';
    charset[0xfb & 0x7F] = '\u{044B}';
    charset[0xfc & 0x7F] = '\u{044C}';
    charset[0xfd & 0x7F] = '\u{044D}';
    charset[0xfe & 0x7F] = '\u{044E}';
    charset[0xff & 0x7F] = '\u{044F}';

    charset
};

/// An encoding for windows-1251.
#[cfg_attr(docsrs, doc(cfg(feature = "windows-1251")))]
pub static WINDOWS_1251: Windows1251 = Windows1251::new();

/// A mapper from bytes over 128 to the corresponding unicode character.
const ENCODER: AsciiCompatible = AsciiCompatible::new(
    <Windows1251 as crate::Charset>::CHARSET_NAME,
    &WIN_1251_CHARSET,
);

/// An encoding for windows-1251.
/// If possible use [`UTF-8`] instead.
///
/// [`UTF-8`]: crate::Utf8Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::exhaustive_structs)]
#[cfg_attr(docsrs, doc(cfg(feature = "windows-1251")))]
pub struct Windows1251;

impl Windows1251 {
    /// Create a new windows-1251 encoding instance.
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

#[cfg_attr(docsrs, doc(cfg(feature = "windows-1251")))]
impl core::default::Default for Windows1251 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "windows-1251")))]
impl crate::Charset for Windows1251 {
    const CHARSET_NAME: &'static str = "windows-1251";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 1)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // IANA
            Self::CHARSET_NAME,
            "cswindows1251",
            // code pages
            "cp1251",
            "x-cp1251",
            "ibm1251",
        ]
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "windows-1251")))]
impl crate::CharsetDecoding for Windows1251 {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "windows-1251")))]
impl crate::CharsetEncoding for Windows1251 {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}
