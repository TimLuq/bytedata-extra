use crate::ascii7_compat::AsciiCompatible;

/// The KOI8-U charset.
const KOI8_U_CHARSET: [char; 128] = {
    let mut charset = ['\0'; 128];

    charset[0x80 & 0x7F] = '\u{2500}';
    charset[0x81 & 0x7F] = '\u{2502}';
    charset[0x82 & 0x7F] = '\u{250C}';
    charset[0x83 & 0x7F] = '\u{2510}';
    charset[0x84 & 0x7F] = '\u{2514}';
    charset[0x85 & 0x7F] = '\u{2518}';
    charset[0x86 & 0x7F] = '\u{251C}';
    charset[0x87 & 0x7F] = '\u{2524}';
    charset[0x88 & 0x7F] = '\u{252C}';
    charset[0x89 & 0x7F] = '\u{2534}';
    charset[0x8a & 0x7F] = '\u{253C}';
    charset[0x8b & 0x7F] = '\u{2580}';
    charset[0x8c & 0x7F] = '\u{2584}';
    charset[0x8d & 0x7F] = '\u{2588}';
    charset[0x8e & 0x7F] = '\u{258C}';
    charset[0x8f & 0x7F] = '\u{2590}';

    charset[0x90 & 0x7F] = '\u{2591}';
    charset[0x91 & 0x7F] = '\u{2592}';
    charset[0x92 & 0x7F] = '\u{2593}';
    charset[0x93 & 0x7F] = '\u{2320}';
    charset[0x94 & 0x7F] = '\u{25A0}';
    charset[0x95 & 0x7F] = '\u{2219}';
    charset[0x96 & 0x7F] = '\u{221A}';
    charset[0x97 & 0x7F] = '\u{2248}';
    charset[0x98 & 0x7F] = '\u{2264}';
    charset[0x99 & 0x7F] = '\u{2265}';
    charset[0x9a & 0x7F] = '\u{00A0}';
    charset[0x9b & 0x7F] = '\u{2321}';
    charset[0x9c & 0x7F] = '\u{00B0}';
    charset[0x9d & 0x7F] = '\u{00B2}';
    charset[0x9e & 0x7F] = '\u{00B7}';
    charset[0x9f & 0x7F] = '\u{00F7}';

    charset[0xa0 & 0x7F] = '\u{2550}';
    charset[0xa1 & 0x7F] = '\u{2551}';
    charset[0xa2 & 0x7F] = '\u{2552}';
    charset[0xa3 & 0x7F] = '\u{0451}';
    charset[0xa4 & 0x7F] = '\u{0454}';
    charset[0xa5 & 0x7F] = '\u{2554}';
    charset[0xa6 & 0x7F] = '\u{0456}';
    charset[0xa7 & 0x7F] = '\u{0457}';
    charset[0xa8 & 0x7F] = '\u{2557}';
    charset[0xa9 & 0x7F] = '\u{2558}';
    charset[0xaa & 0x7F] = '\u{2559}';
    charset[0xab & 0x7F] = '\u{255A}';
    charset[0xac & 0x7F] = '\u{255B}';
    charset[0xad & 0x7F] = '\u{0491}';
    charset[0xae & 0x7F] = '\u{255D}';
    charset[0xaf & 0x7F] = '\u{255E}';

    charset[0xb0 & 0x7F] = '\u{255F}';
    charset[0xb1 & 0x7F] = '\u{2560}';
    charset[0xb2 & 0x7F] = '\u{2561}';
    charset[0xb3 & 0x7F] = '\u{0401}';
    charset[0xb4 & 0x7F] = '\u{0404}';
    charset[0xb5 & 0x7F] = '\u{2563}';
    charset[0xb6 & 0x7F] = '\u{0406}';
    charset[0xb7 & 0x7F] = '\u{0407}';
    charset[0xb8 & 0x7F] = '\u{2566}';
    charset[0xb9 & 0x7F] = '\u{2567}';
    charset[0xba & 0x7F] = '\u{2568}';
    charset[0xbb & 0x7F] = '\u{2569}';
    charset[0xbc & 0x7F] = '\u{256A}';
    charset[0xbd & 0x7F] = '\u{0490}';
    charset[0xbe & 0x7F] = '\u{256C}';
    charset[0xbf & 0x7F] = '\u{00A9}';

    charset[0xc0 & 0x7F] = '\u{044E}';
    charset[0xc1 & 0x7F] = '\u{0430}';
    charset[0xc2 & 0x7F] = '\u{0431}';
    charset[0xc3 & 0x7F] = '\u{0446}';
    charset[0xc4 & 0x7F] = '\u{0434}';
    charset[0xc5 & 0x7F] = '\u{0435}';
    charset[0xc6 & 0x7F] = '\u{0444}';
    charset[0xc7 & 0x7F] = '\u{0433}';
    charset[0xc8 & 0x7F] = '\u{0445}';
    charset[0xc9 & 0x7F] = '\u{0438}';
    charset[0xca & 0x7F] = '\u{0439}';
    charset[0xcb & 0x7F] = '\u{043A}';
    charset[0xcc & 0x7F] = '\u{043B}';
    charset[0xcd & 0x7F] = '\u{043C}';
    charset[0xce & 0x7F] = '\u{043D}';
    charset[0xcf & 0x7F] = '\u{043E}';

    charset[0xd0 & 0x7F] = '\u{043F}';
    charset[0xd1 & 0x7F] = '\u{044F}';
    charset[0xd2 & 0x7F] = '\u{0440}';
    charset[0xd3 & 0x7F] = '\u{0441}';
    charset[0xd4 & 0x7F] = '\u{0442}';
    charset[0xd5 & 0x7F] = '\u{0443}';
    charset[0xd6 & 0x7F] = '\u{0436}';
    charset[0xd7 & 0x7F] = '\u{0432}';
    charset[0xd8 & 0x7F] = '\u{044C}';
    charset[0xd9 & 0x7F] = '\u{044B}';
    charset[0xda & 0x7F] = '\u{0437}';
    charset[0xdb & 0x7F] = '\u{0448}';
    charset[0xdc & 0x7F] = '\u{044D}';
    charset[0xdd & 0x7F] = '\u{0449}';
    charset[0xde & 0x7F] = '\u{0447}';
    charset[0xdf & 0x7F] = '\u{044A}';

    charset[0xe0 & 0x7F] = '\u{042E}';
    charset[0xe1 & 0x7F] = '\u{0410}';
    charset[0xe2 & 0x7F] = '\u{0411}';
    charset[0xe3 & 0x7F] = '\u{0426}';
    charset[0xe4 & 0x7F] = '\u{0414}';
    charset[0xe5 & 0x7F] = '\u{0415}';
    charset[0xe6 & 0x7F] = '\u{0424}';
    charset[0xe7 & 0x7F] = '\u{0413}';
    charset[0xe8 & 0x7F] = '\u{0425}';
    charset[0xe9 & 0x7F] = '\u{0418}';
    charset[0xea & 0x7F] = '\u{0419}';
    charset[0xeb & 0x7F] = '\u{041A}';
    charset[0xec & 0x7F] = '\u{041B}';
    charset[0xed & 0x7F] = '\u{041C}';
    charset[0xee & 0x7F] = '\u{041D}';
    charset[0xef & 0x7F] = '\u{041E}';

    charset[0xf0 & 0x7F] = '\u{041F}';
    charset[0xf1 & 0x7F] = '\u{042F}';
    charset[0xf2 & 0x7F] = '\u{0420}';
    charset[0xf3 & 0x7F] = '\u{0421}';
    charset[0xf4 & 0x7F] = '\u{0422}';
    charset[0xf5 & 0x7F] = '\u{0423}';
    charset[0xf6 & 0x7F] = '\u{0416}';
    charset[0xf7 & 0x7F] = '\u{0412}';
    charset[0xf8 & 0x7F] = '\u{042C}';
    charset[0xf9 & 0x7F] = '\u{042B}';
    charset[0xfa & 0x7F] = '\u{0417}';
    charset[0xfb & 0x7F] = '\u{0428}';
    charset[0xfc & 0x7F] = '\u{042D}';
    charset[0xfd & 0x7F] = '\u{0429}';
    charset[0xfe & 0x7F] = '\u{0427}';
    charset[0xff & 0x7F] = '\u{042A}';

    charset
};

/// An encoding for KOI8-U.
#[doc(alias = "cp21866")]
#[doc(alias = "cp1168")]
#[cfg_attr(docsrs, doc(cfg(feature = "koi8-r")))]
pub static KOI8_U: Koi8U = Koi8U::new();

/// A mapper from bytes over 128 to the corresponding unicode character.
const ENCODER: AsciiCompatible = AsciiCompatible::new(<Koi8U as crate::Charset>::CHARSET_NAME, &KOI8_U_CHARSET);

/// An encoding for KOI8-U.
/// If possible use [`UTF-8`] instead.
/// 
/// [`UTF-8`]: crate::Utf8Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::exhaustive_structs)]
#[cfg_attr(docsrs, doc(cfg(feature = "koi8-r")))]
pub struct Koi8U;

impl Koi8U {

    /// Create a new KOI8-U encoding instance.
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

#[cfg_attr(docsrs, doc(cfg(feature = "koi8-r")))]
impl core::default::Default for Koi8U {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "koi8-r")))]
impl crate::Charset for Koi8U {
    const CHARSET_NAME: &'static str = "koi8-r";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 1)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // IANA
            Self::CHARSET_NAME,
            "cskoi8u",
            
            // code pages
            "cp21866", "windows-21866",
            "cp1168", "ibm1168",

            // other
            "koi8-ru",
        ]
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "koi8-r")))]
impl crate::CharsetDecoding for Koi8U {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "koi8-r")))]
impl crate::CharsetEncoding for Koi8U {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}
