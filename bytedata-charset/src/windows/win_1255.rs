use crate::ascii7_compat::AsciiCompatible;

const WIN_1255_CHARSET: [char; 128] = {
    let mut charset = ['\0'; 128];

    charset[0x80 & 0x7F] = '\u{20AC}';
    charset[0x81 & 0x7F] = '\0';
    charset[0x82 & 0x7F] = '\u{201A}';
    charset[0x83 & 0x7F] = '\u{0192}';
    charset[0x84 & 0x7F] = '\u{201E}';
    charset[0x85 & 0x7F] = '\u{2026}';
    charset[0x86 & 0x7F] = '\u{2020}';
    charset[0x87 & 0x7F] = '\u{2021}';
    charset[0x88 & 0x7F] = '\u{02C6}';
    charset[0x89 & 0x7F] = '\u{2030}';
    charset[0x8a & 0x7F] = '\0';
    charset[0x8b & 0x7F] = '\u{2039}';
    charset[0x8c & 0x7F] = '\0';
    charset[0x8d & 0x7F] = '\0';
    charset[0x8e & 0x7F] = '\0';
    charset[0x8f & 0x7F] = '\0';

    charset[0x90 & 0x7F] = '\0';
    charset[0x91 & 0x7F] = '\u{2018}';
    charset[0x92 & 0x7F] = '\u{2019}';
    charset[0x93 & 0x7F] = '\u{201C}';
    charset[0x94 & 0x7F] = '\u{201D}';
    charset[0x95 & 0x7F] = '\u{2022}';
    charset[0x96 & 0x7F] = '\u{2013}';
    charset[0x97 & 0x7F] = '\u{2014}';
    charset[0x98 & 0x7F] = '\u{02DC}';
    charset[0x99 & 0x7F] = '\u{2122}';
    charset[0x9a & 0x7F] = '\0';
    charset[0x9b & 0x7F] = '\u{203A}';
    charset[0x9c & 0x7F] = '\0';
    charset[0x9d & 0x7F] = '\0';
    charset[0x9e & 0x7F] = '\0';
    charset[0x9f & 0x7F] = '\0';

    charset[0xa0 & 0x7F] = '\u{00A0}';
    charset[0xa1 & 0x7F] = '\u{00A1}';
    charset[0xa2 & 0x7F] = '\u{00A2}';
    charset[0xa3 & 0x7F] = '\u{00A3}';
    charset[0xa4 & 0x7F] = '\u{20AA}';
    charset[0xa5 & 0x7F] = '\u{00A5}';
    charset[0xa6 & 0x7F] = '\u{00A6}';
    charset[0xa7 & 0x7F] = '\u{00A7}';
    charset[0xa8 & 0x7F] = '\u{00A8}';
    charset[0xa9 & 0x7F] = '\u{00A9}';
    charset[0xaa & 0x7F] = '\u{00D7}';
    charset[0xab & 0x7F] = '\u{00AB}';
    charset[0xac & 0x7F] = '\u{00AC}';
    charset[0xad & 0x7F] = '\u{00AD}';
    charset[0xae & 0x7F] = '\u{00AE}';
    charset[0xaf & 0x7F] = '\u{00AF}';

    charset[0xb0 & 0x7F] = '\u{00B0}';
    charset[0xb1 & 0x7F] = '\u{00B1}';
    charset[0xb2 & 0x7F] = '\u{00B2}';
    charset[0xb3 & 0x7F] = '\u{00B3}';
    charset[0xb4 & 0x7F] = '\u{00B4}';
    charset[0xb5 & 0x7F] = '\u{00B5}';
    charset[0xb6 & 0x7F] = '\u{00B6}';
    charset[0xb7 & 0x7F] = '\u{00B7}';
    charset[0xb8 & 0x7F] = '\u{00B8}';
    charset[0xb9 & 0x7F] = '\u{00B9}';
    charset[0xba & 0x7F] = '\u{00F7}';
    charset[0xbb & 0x7F] = '\u{00BB}';
    charset[0xbc & 0x7F] = '\u{00BC}';
    charset[0xbd & 0x7F] = '\u{00BD}';
    charset[0xbe & 0x7F] = '\u{00BE}';
    charset[0xbf & 0x7F] = '\u{00BF}';

    charset[0xc0 & 0x7F] = '\u{05B0}';
    charset[0xc1 & 0x7F] = '\u{05B1}';
    charset[0xc2 & 0x7F] = '\u{05B2}';
    charset[0xc3 & 0x7F] = '\u{05B3}';
    charset[0xc4 & 0x7F] = '\u{05B4}';
    charset[0xc5 & 0x7F] = '\u{05B5}';
    charset[0xc6 & 0x7F] = '\u{05B6}';
    charset[0xc7 & 0x7F] = '\u{05B7}';
    charset[0xc8 & 0x7F] = '\u{05B8}';
    charset[0xc9 & 0x7F] = '\u{05B9}';
    charset[0xca & 0x7F] = '\u{05BA}';
    charset[0xcb & 0x7F] = '\u{05BB}';
    charset[0xcc & 0x7F] = '\u{05BC}';
    charset[0xcd & 0x7F] = '\u{05BD}';
    charset[0xce & 0x7F] = '\u{05BE}';
    charset[0xcf & 0x7F] = '\u{05BF}';

    charset[0xd0 & 0x7F] = '\u{05C0}';
    charset[0xd1 & 0x7F] = '\u{05C1}';
    charset[0xd2 & 0x7F] = '\u{05C2}';
    charset[0xd3 & 0x7F] = '\u{05C3}';
    charset[0xd4 & 0x7F] = '\u{05F0}';
    charset[0xd5 & 0x7F] = '\u{05F1}';
    charset[0xd6 & 0x7F] = '\u{05F2}';
    charset[0xd7 & 0x7F] = '\u{05F3}';
    charset[0xd8 & 0x7F] = '\u{05F4}';
    charset[0xd9 & 0x7F] = '\0';
    charset[0xda & 0x7F] = '\0';
    charset[0xdb & 0x7F] = '\0';
    charset[0xdc & 0x7F] = '\0';
    charset[0xdd & 0x7F] = '\0';
    charset[0xde & 0x7F] = '\0';
    charset[0xdf & 0x7F] = '\0';

    charset[0xe0 & 0x7F] = '\u{05D0}';
    charset[0xe1 & 0x7F] = '\u{05D1}';
    charset[0xe2 & 0x7F] = '\u{05D2}';
    charset[0xe3 & 0x7F] = '\u{05D3}';
    charset[0xe4 & 0x7F] = '\u{05D4}';
    charset[0xe5 & 0x7F] = '\u{05D5}';
    charset[0xe6 & 0x7F] = '\u{05D6}';
    charset[0xe7 & 0x7F] = '\u{05D7}';
    charset[0xe8 & 0x7F] = '\u{05D8}';
    charset[0xe9 & 0x7F] = '\u{05D9}';
    charset[0xea & 0x7F] = '\u{05DA}';
    charset[0xeb & 0x7F] = '\u{05DB}';
    charset[0xec & 0x7F] = '\u{05DC}';
    charset[0xed & 0x7F] = '\u{05DD}';
    charset[0xee & 0x7F] = '\u{05DE}';
    charset[0xef & 0x7F] = '\u{05DF}';

    charset[0xf0 & 0x7F] = '\u{05E0}';
    charset[0xf1 & 0x7F] = '\u{05E1}';
    charset[0xf2 & 0x7F] = '\u{05E2}';
    charset[0xf3 & 0x7F] = '\u{05E3}';
    charset[0xf4 & 0x7F] = '\u{05E4}';
    charset[0xf5 & 0x7F] = '\u{05E5}';
    charset[0xf6 & 0x7F] = '\u{05E6}';
    charset[0xf7 & 0x7F] = '\u{05E7}';
    charset[0xf8 & 0x7F] = '\u{05E8}';
    charset[0xf9 & 0x7F] = '\u{05E9}';
    charset[0xfa & 0x7F] = '\u{05EA}';
    charset[0xfb & 0x7F] = '\0';
    charset[0xfc & 0x7F] = '\0';
    charset[0xfd & 0x7F] = '\u{200E}';
    charset[0xfe & 0x7F] = '\u{200F}';
    charset[0xff & 0x7F] = '\0';

    charset
};

/// An encoding for windows-1255.
#[cfg_attr(docsrs, doc(cfg(feature = "windows-1255")))]
pub static WINDOWS_1255: Windows1255 = Windows1255::new();

/// A mapper from bytes over 128 to the corresponding unicode character.
const ENCODER: AsciiCompatible = AsciiCompatible::new(<Windows1255 as crate::Charset>::CHARSET_NAME, &WIN_1255_CHARSET);

/// An encoding for windows-1255.
/// If possible use [`UTF-8`] instead.
/// 
/// [`UTF-8`]: crate::Utf8Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::exhaustive_structs)]
#[cfg_attr(docsrs, doc(cfg(feature = "windows-1255")))]
pub struct Windows1255;

impl Windows1255 {

    /// Create a new windows-1255 encoding instance.
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

#[cfg_attr(docsrs, doc(cfg(feature = "windows-1255")))]
impl core::default::Default for Windows1255 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "windows-1255")))]
impl crate::Charset for Windows1255 {
    const CHARSET_NAME: &'static str = "windows-1255";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 1)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // IANA
            Self::CHARSET_NAME,
            "cswindows1255",
            
            // code pages
            "cp1255", "x-cp1255", "ibm1255",
        ]
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "windows-1255")))]
impl crate::CharsetDecoding for Windows1255 {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "windows-1255")))]
impl crate::CharsetEncoding for Windows1255 {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}
