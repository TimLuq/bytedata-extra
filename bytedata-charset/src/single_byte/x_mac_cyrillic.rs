use crate::ascii7_compat::AsciiCompatible;

/// The x-mac-cyrillic charset.
const X_MAC_CYRILLIC_CHARSET: [char; 128] = {
    let mut charset = ['\0'; 128];

    charset[0x80 & 0x7F] = '\u{00C4}';
    charset[0x81 & 0x7F] = '\u{00C5}';
    charset[0x82 & 0x7F] = '\u{00C7}';
    charset[0x83 & 0x7F] = '\u{00C9}';
    charset[0x84 & 0x7F] = '\u{00D1}';
    charset[0x85 & 0x7F] = '\u{00D6}';
    charset[0x86 & 0x7F] = '\u{00DC}';
    charset[0x87 & 0x7F] = '\u{00E1}';
    charset[0x88 & 0x7F] = '\u{00E0}';
    charset[0x89 & 0x7F] = '\u{00E2}';
    charset[0x8a & 0x7F] = '\u{00E4}';
    charset[0x8b & 0x7F] = '\u{00E3}';
    charset[0x8c & 0x7F] = '\u{00E5}';
    charset[0x8d & 0x7F] = '\u{00E7}';
    charset[0x8e & 0x7F] = '\u{00E9}';
    charset[0x8f & 0x7F] = '\u{00E8}';

    charset[0x90 & 0x7F] = '\u{00EA}';
    charset[0x91 & 0x7F] = '\u{00EB}';
    charset[0x92 & 0x7F] = '\u{00ED}';
    charset[0x93 & 0x7F] = '\u{00EC}';
    charset[0x94 & 0x7F] = '\u{00EE}';
    charset[0x95 & 0x7F] = '\u{00EF}';
    charset[0x96 & 0x7F] = '\u{00F1}';
    charset[0x97 & 0x7F] = '\u{00F3}';
    charset[0x98 & 0x7F] = '\u{00F2}';
    charset[0x99 & 0x7F] = '\u{00F4}';
    charset[0x9a & 0x7F] = '\u{00F6}';
    charset[0x9b & 0x7F] = '\u{00F5}';
    charset[0x9c & 0x7F] = '\u{00FA}';
    charset[0x9d & 0x7F] = '\u{00F9}';
    charset[0x9e & 0x7F] = '\u{00FB}';
    charset[0x9f & 0x7F] = '\u{00FC}';

    charset[0xa0 & 0x7F] = '\u{2020}';
    charset[0xa1 & 0x7F] = '\u{00B0}';
    charset[0xa2 & 0x7F] = '\u{00A2}';
    charset[0xa3 & 0x7F] = '\u{00A3}';
    charset[0xa4 & 0x7F] = '\u{00A7}';
    charset[0xa5 & 0x7F] = '\u{2022}';
    charset[0xa6 & 0x7F] = '\u{00B6}';
    charset[0xa7 & 0x7F] = '\u{00DF}';
    charset[0xa8 & 0x7F] = '\u{00AE}';
    charset[0xa9 & 0x7F] = '\u{00A9}';
    charset[0xaa & 0x7F] = '\u{2122}';
    charset[0xab & 0x7F] = '\u{00B4}';
    charset[0xac & 0x7F] = '\u{00A8}';
    charset[0xad & 0x7F] = '\u{2260}';
    charset[0xae & 0x7F] = '\u{00C6}';
    charset[0xaf & 0x7F] = '\u{00D8}';

    charset[0xb0 & 0x7F] = '\u{221E}';
    charset[0xb1 & 0x7F] = '\u{00B1}';
    charset[0xb2 & 0x7F] = '\u{2264}';
    charset[0xb3 & 0x7F] = '\u{2265}';
    charset[0xb4 & 0x7F] = '\u{00A5}';
    charset[0xb5 & 0x7F] = '\u{00B5}';
    charset[0xb6 & 0x7F] = '\u{2202}';
    charset[0xb7 & 0x7F] = '\u{2211}';
    charset[0xb8 & 0x7F] = '\u{220F}';
    charset[0xb9 & 0x7F] = '\u{03C0}';
    charset[0xba & 0x7F] = '\u{222B}';
    charset[0xbb & 0x7F] = '\u{00AA}';
    charset[0xbc & 0x7F] = '\u{00BA}';
    charset[0xbd & 0x7F] = '\u{03A9}';
    charset[0xbe & 0x7F] = '\u{00E6}';
    charset[0xbf & 0x7F] = '\u{00F8}';

    charset[0xc0 & 0x7F] = '\u{00BF}';
    charset[0xc1 & 0x7F] = '\u{00A1}';
    charset[0xc2 & 0x7F] = '\u{00AC}';
    charset[0xc3 & 0x7F] = '\u{221A}';
    charset[0xc4 & 0x7F] = '\u{0192}';
    charset[0xc5 & 0x7F] = '\u{2248}';
    charset[0xc6 & 0x7F] = '\u{2206}';
    charset[0xc7 & 0x7F] = '\u{00AB}';
    charset[0xc8 & 0x7F] = '\u{00BB}';
    charset[0xc9 & 0x7F] = '\u{2026}';
    charset[0xca & 0x7F] = '\u{00A0}';
    charset[0xcb & 0x7F] = '\u{00C0}';
    charset[0xcc & 0x7F] = '\u{00C3}';
    charset[0xcd & 0x7F] = '\u{00D5}';
    charset[0xce & 0x7F] = '\u{0152}';
    charset[0xcf & 0x7F] = '\u{0153}';

    charset[0xd0 & 0x7F] = '\u{2013}';
    charset[0xd1 & 0x7F] = '\u{2014}';
    charset[0xd2 & 0x7F] = '\u{201C}';
    charset[0xd3 & 0x7F] = '\u{201D}';
    charset[0xd4 & 0x7F] = '\u{2018}';
    charset[0xd5 & 0x7F] = '\u{2019}';
    charset[0xd6 & 0x7F] = '\u{00F7}';
    charset[0xd7 & 0x7F] = '\u{25CA}';
    charset[0xd8 & 0x7F] = '\u{00FF}';
    charset[0xd9 & 0x7F] = '\u{0178}';
    charset[0xda & 0x7F] = '\u{2044}';
    charset[0xdb & 0x7F] = '\u{20AC}';
    charset[0xdc & 0x7F] = '\u{2039}';
    charset[0xdd & 0x7F] = '\u{203A}';
    charset[0xde & 0x7F] = '\u{FB01}';
    charset[0xdf & 0x7F] = '\u{FB02}';

    charset[0xe0 & 0x7F] = '\u{2021}';
    charset[0xe1 & 0x7F] = '\u{00B7}';
    charset[0xe2 & 0x7F] = '\u{201A}';
    charset[0xe3 & 0x7F] = '\u{201E}';
    charset[0xe4 & 0x7F] = '\u{2030}';
    charset[0xe5 & 0x7F] = '\u{00C2}';
    charset[0xe6 & 0x7F] = '\u{00CA}';
    charset[0xe7 & 0x7F] = '\u{00C1}';
    charset[0xe8 & 0x7F] = '\u{00CB}';
    charset[0xe9 & 0x7F] = '\u{00C8}';
    charset[0xea & 0x7F] = '\u{00CD}';
    charset[0xeb & 0x7F] = '\u{00CE}';
    charset[0xec & 0x7F] = '\u{00CF}';
    charset[0xed & 0x7F] = '\u{00CC}';
    charset[0xee & 0x7F] = '\u{00D3}';
    charset[0xef & 0x7F] = '\u{00D4}';

    charset[0xf0 & 0x7F] = '\u{F8FF}';
    charset[0xf1 & 0x7F] = '\u{00D2}';
    charset[0xf2 & 0x7F] = '\u{00DA}';
    charset[0xf3 & 0x7F] = '\u{00DB}';
    charset[0xf4 & 0x7F] = '\u{00D9}';
    charset[0xf5 & 0x7F] = '\u{0131}';
    charset[0xf6 & 0x7F] = '\u{02C6}';
    charset[0xf7 & 0x7F] = '\u{02DC}';
    charset[0xf8 & 0x7F] = '\u{00AF}';
    charset[0xf9 & 0x7F] = '\u{02D8}';
    charset[0xfa & 0x7F] = '\u{02D9}';
    charset[0xfb & 0x7F] = '\u{02DA}';
    charset[0xfc & 0x7F] = '\u{00B8}';
    charset[0xfd & 0x7F] = '\u{02DD}';
    charset[0xfe & 0x7F] = '\u{02DB}';
    charset[0xff & 0x7F] = '\u{02C7}';

    charset
};

/// An encoding for x-mac-cyrillic.
#[doc(alias = "x_mac_ukrainian")]
#[cfg_attr(docsrs, doc(cfg(feature = "x-mac-cyrillic")))]
pub static X_MAC_CYRILLIC: XMacCyrillic = XMacCyrillic::new();

/// A mapper from bytes over 128 to the corresponding unicode character.
const ENCODER: AsciiCompatible = AsciiCompatible::new(
    <XMacCyrillic as crate::Charset>::CHARSET_NAME,
    &X_MAC_CYRILLIC_CHARSET,
);

/// An encoding for x-mac-cyrillic.
/// If possible use [`UTF-8`] instead.
///
/// [`UTF-8`]: crate::Utf8Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::exhaustive_structs)]
#[cfg_attr(docsrs, doc(cfg(feature = "x-mac-cyrillic")))]
pub struct XMacCyrillic;

impl XMacCyrillic {
    /// Create a new x-mac-cyrillic encoding instance.
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

#[cfg_attr(docsrs, doc(cfg(feature = "x-mac-cyrillic")))]
impl core::default::Default for XMacCyrillic {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "x-mac-cyrillic")))]
impl crate::Charset for XMacCyrillic {
    const CHARSET_NAME: &'static str = "x-mac-cyrillic";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 1)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // other
            Self::CHARSET_NAME,
            "x-mac-ukrainian",
        ]
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "x-mac-cyrillic")))]
impl crate::CharsetDecoding for XMacCyrillic {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "x-mac-cyrillic")))]
impl crate::CharsetEncoding for XMacCyrillic {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}
