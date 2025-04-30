use super::SingleByteEncoding;

const IBM866_CHARSET: [char; 256] = {
    let mut charset = ['\0'; 256];

    charset[0x00] = '\u{0000}';
    charset[0x01] = '\u{263A}';
    charset[0x02] = '\u{263B}';
    charset[0x03] = '\u{2665}';
    charset[0x04] = '\u{2666}';
    charset[0x05] = '\u{2663}';
    charset[0x06] = '\u{2660}';
    charset[0x07] = '\u{2022}';
    charset[0x08] = '\u{25D8}';
    charset[0x09] = '\u{25CB}';
    charset[0x0a] = '\u{25D9}';
    charset[0x0b] = '\u{2642}';
    charset[0x0c] = '\u{2640}';
    charset[0x0d] = '\u{266A}';
    charset[0x0e] = '\u{266B}';
    charset[0x0f] = '\u{263C}';

    charset[0x10] = '\u{25BA}';
    charset[0x11] = '\u{25C4}';
    charset[0x12] = '\u{2195}';
    charset[0x13] = '\u{203C}';
    charset[0x14] = '\u{00B6}';
    charset[0x15] = '\u{00A7}';
    charset[0x16] = '\u{25AC}';
    charset[0x17] = '\u{21A8}';
    charset[0x18] = '\u{2191}';
    charset[0x19] = '\u{2193}';
    charset[0x1a] = '\u{2192}';
    charset[0x1b] = '\u{2190}';
    charset[0x1c] = '\u{221F}';
    charset[0x1d] = '\u{2194}';
    charset[0x1e] = '\u{25B2}';
    charset[0x1f] = '\u{25BC}';

    charset[0x20] = '\u{0020}';
    charset[0x21] = '\u{0021}';
    charset[0x22] = '\u{0022}';
    charset[0x23] = '\u{0023}';
    charset[0x24] = '\u{0024}';
    charset[0x25] = '\u{0025}';
    charset[0x26] = '\u{0026}';
    charset[0x27] = '\u{0027}';
    charset[0x28] = '\u{0028}';
    charset[0x29] = '\u{0029}';
    charset[0x2a] = '\u{002A}';
    charset[0x2b] = '\u{002B}';
    charset[0x2c] = '\u{002C}';
    charset[0x2d] = '\u{002D}';
    charset[0x2e] = '\u{002E}';
    charset[0x2f] = '\u{002F}';

    charset[0x30] = '\u{0030}';
    charset[0x31] = '\u{0031}';
    charset[0x32] = '\u{0032}';
    charset[0x33] = '\u{0033}';
    charset[0x34] = '\u{0034}';
    charset[0x35] = '\u{0035}';
    charset[0x36] = '\u{0036}';
    charset[0x37] = '\u{0037}';
    charset[0x38] = '\u{0038}';
    charset[0x39] = '\u{0039}';
    charset[0x3a] = '\u{003A}';
    charset[0x3b] = '\u{003B}';
    charset[0x3c] = '\u{003C}';
    charset[0x3d] = '\u{003D}';
    charset[0x3e] = '\u{003E}';
    charset[0x3f] = '\u{003F}';

    charset[0x40] = '\u{0040}';
    charset[0x41] = '\u{0041}';
    charset[0x42] = '\u{0042}';
    charset[0x43] = '\u{0043}';
    charset[0x44] = '\u{0044}';
    charset[0x45] = '\u{0045}';
    charset[0x46] = '\u{0046}';
    charset[0x47] = '\u{0047}';
    charset[0x48] = '\u{0048}';
    charset[0x49] = '\u{0049}';
    charset[0x4a] = '\u{004A}';
    charset[0x4b] = '\u{004B}';
    charset[0x4c] = '\u{004C}';
    charset[0x4d] = '\u{004D}';
    charset[0x4e] = '\u{004E}';
    charset[0x4f] = '\u{004F}';

    charset[0x50] = '\u{0050}';
    charset[0x51] = '\u{0051}';
    charset[0x52] = '\u{0052}';
    charset[0x53] = '\u{0053}';
    charset[0x54] = '\u{0054}';
    charset[0x55] = '\u{0055}';
    charset[0x56] = '\u{0056}';
    charset[0x57] = '\u{0057}';
    charset[0x58] = '\u{0058}';
    charset[0x59] = '\u{0059}';
    charset[0x5a] = '\u{005A}';
    charset[0x5b] = '\u{005B}';
    charset[0x5c] = '\u{005C}';
    charset[0x5d] = '\u{005D}';
    charset[0x5e] = '\u{005E}';
    charset[0x5f] = '\u{005F}';

    charset[0x60] = '\u{0060}';
    charset[0x61] = '\u{0061}';
    charset[0x62] = '\u{0062}';
    charset[0x63] = '\u{0063}';
    charset[0x64] = '\u{0064}';
    charset[0x65] = '\u{0065}';
    charset[0x66] = '\u{0066}';
    charset[0x67] = '\u{0067}';
    charset[0x68] = '\u{0068}';
    charset[0x69] = '\u{0069}';
    charset[0x6a] = '\u{006A}';
    charset[0x6b] = '\u{006B}';
    charset[0x6c] = '\u{006C}';
    charset[0x6d] = '\u{006D}';
    charset[0x6e] = '\u{006E}';
    charset[0x6f] = '\u{006F}';

    charset[0x70] = '\u{0070}';
    charset[0x71] = '\u{0071}';
    charset[0x72] = '\u{0072}';
    charset[0x73] = '\u{0073}';
    charset[0x74] = '\u{0074}';
    charset[0x75] = '\u{0075}';
    charset[0x76] = '\u{0076}';
    charset[0x77] = '\u{0077}';
    charset[0x78] = '\u{0078}';
    charset[0x79] = '\u{0079}';
    charset[0x7a] = '\u{007A}';
    charset[0x7b] = '\u{007B}';
    charset[0x7c] = '\u{007C}';
    charset[0x7d] = '\u{007D}';
    charset[0x7e] = '\u{007E}';
    charset[0x7f] = '\u{2302}';

    charset[0x80] = '\u{0410}';
    charset[0x81] = '\u{0411}';
    charset[0x82] = '\u{0412}';
    charset[0x83] = '\u{0413}';
    charset[0x84] = '\u{0414}';
    charset[0x85] = '\u{0415}';
    charset[0x86] = '\u{0416}';
    charset[0x87] = '\u{0417}';
    charset[0x88] = '\u{0418}';
    charset[0x89] = '\u{0419}';
    charset[0x8a] = '\u{041A}';
    charset[0x8b] = '\u{041B}';
    charset[0x8c] = '\u{041C}';
    charset[0x8d] = '\u{041D}';
    charset[0x8e] = '\u{041E}';
    charset[0x8f] = '\u{041F}';

    charset[0x90] = '\u{0420}';
    charset[0x91] = '\u{0421}';
    charset[0x92] = '\u{0422}';
    charset[0x93] = '\u{0423}';
    charset[0x94] = '\u{0424}';
    charset[0x95] = '\u{0425}';
    charset[0x96] = '\u{0426}';
    charset[0x97] = '\u{0427}';
    charset[0x98] = '\u{0428}';
    charset[0x99] = '\u{0429}';
    charset[0x9a] = '\u{042A}';
    charset[0x9b] = '\u{042B}';
    charset[0x9c] = '\u{042C}';
    charset[0x9d] = '\u{042D}';
    charset[0x9e] = '\u{042E}';
    charset[0x9f] = '\u{042F}';

    charset[0xa0] = '\u{0430}';
    charset[0xa1] = '\u{0431}';
    charset[0xa2] = '\u{0432}';
    charset[0xa3] = '\u{0433}';
    charset[0xa4] = '\u{0434}';
    charset[0xa5] = '\u{0435}';
    charset[0xa6] = '\u{0436}';
    charset[0xa7] = '\u{0437}';
    charset[0xa8] = '\u{0438}';
    charset[0xa9] = '\u{0439}';
    charset[0xaa] = '\u{043A}';
    charset[0xab] = '\u{043B}';
    charset[0xac] = '\u{043C}';
    charset[0xad] = '\u{043D}';
    charset[0xae] = '\u{043E}';
    charset[0xaf] = '\u{043F}';

    charset[0xb0] = '\u{2591}';
    charset[0xb1] = '\u{2592}';
    charset[0xb2] = '\u{2593}';
    charset[0xb3] = '\u{2502}';
    charset[0xb4] = '\u{2524}';
    charset[0xb5] = '\u{2561}';
    charset[0xb6] = '\u{2562}';
    charset[0xb7] = '\u{2556}';
    charset[0xb8] = '\u{2555}';
    charset[0xb9] = '\u{2563}';
    charset[0xba] = '\u{2551}';
    charset[0xbb] = '\u{2557}';
    charset[0xbc] = '\u{255D}';
    charset[0xbd] = '\u{255C}';
    charset[0xbe] = '\u{255B}';
    charset[0xbf] = '\u{2510}';

    charset[0xc0] = '\u{2514}';
    charset[0xc1] = '\u{2534}';
    charset[0xc2] = '\u{252C}';
    charset[0xc3] = '\u{251C}';
    charset[0xc4] = '\u{2500}';
    charset[0xc5] = '\u{253C}';
    charset[0xc6] = '\u{255E}';
    charset[0xc7] = '\u{255F}';
    charset[0xc8] = '\u{255A}';
    charset[0xc9] = '\u{2554}';
    charset[0xca] = '\u{2569}';
    charset[0xcb] = '\u{2566}';
    charset[0xcc] = '\u{2560}';
    charset[0xcd] = '\u{2550}';
    charset[0xce] = '\u{256C}';
    charset[0xcf] = '\u{2567}';

    charset[0xd0] = '\u{2568}';
    charset[0xd1] = '\u{2564}';
    charset[0xd2] = '\u{2565}';
    charset[0xd3] = '\u{2559}';
    charset[0xd4] = '\u{2558}';
    charset[0xd5] = '\u{2552}';
    charset[0xd6] = '\u{2553}';
    charset[0xd7] = '\u{256B}';
    charset[0xd8] = '\u{256A}';
    charset[0xd9] = '\u{2518}';
    charset[0xda] = '\u{250C}';
    charset[0xdb] = '\u{2588}';
    charset[0xdc] = '\u{2584}';
    charset[0xdd] = '\u{258C}';
    charset[0xde] = '\u{2590}';
    charset[0xdf] = '\u{2580}';

    charset[0xe0] = '\u{0440}';
    charset[0xe1] = '\u{0441}';
    charset[0xe2] = '\u{0442}';
    charset[0xe3] = '\u{0443}';
    charset[0xe4] = '\u{0444}';
    charset[0xe5] = '\u{0445}';
    charset[0xe6] = '\u{0446}';
    charset[0xe7] = '\u{0447}';
    charset[0xe8] = '\u{0448}';
    charset[0xe9] = '\u{0449}';
    charset[0xea] = '\u{044A}';
    charset[0xeb] = '\u{044B}';
    charset[0xec] = '\u{044C}';
    charset[0xed] = '\u{044D}';
    charset[0xee] = '\u{044E}';
    charset[0xef] = '\u{044F}';

    charset[0xf0] = '\u{0401}';
    charset[0xf1] = '\u{0451}';
    charset[0xf2] = '\u{0404}';
    charset[0xf3] = '\u{0454}';
    charset[0xf4] = '\u{0407}';
    charset[0xf5] = '\u{0457}';
    charset[0xf6] = '\u{040E}';
    charset[0xf7] = '\u{045E}';
    charset[0xf8] = '\u{00B0}';
    charset[0xf9] = '\u{2219}';
    charset[0xfa] = '\u{00B7}';
    charset[0xfb] = '\u{221A}';
    charset[0xfc] = '\u{2116}';
    charset[0xfd] = '\u{00A4}';
    charset[0xfe] = '\u{25A0}';
    charset[0xff] = '\u{00A0}';

    charset
};

/// An encoding for ISO-8859-4.
#[cfg_attr(docsrs, doc(cfg(feature = "ibm866")))]
pub static IBM866: Ibm866 = Ibm866::new();

/// A mapper from bytes over 128 to the corresponding unicode character.
const ENCODER: SingleByteEncoding =
    SingleByteEncoding::new(<Ibm866 as crate::Charset>::CHARSET_NAME, &IBM866_CHARSET);

/// An encoding for ISO-8859-4.
/// If possible use [`UTF-8`] instead.
///
/// [`UTF-8`]: crate::Utf8Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::exhaustive_structs)]
#[cfg_attr(docsrs, doc(cfg(feature = "ibm866")))]
pub struct Ibm866;

impl Ibm866 {
    /// Create a new ISO-8859-4 encoding instance.
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
    pub const fn single_byte(&self) -> &'static SingleByteEncoding {
        &ENCODER
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "ibm866")))]
impl core::default::Default for Ibm866 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "ibm866")))]
impl crate::Charset for Ibm866 {
    const CHARSET_NAME: &'static str = "ibm866";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 1)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // IANA
            Self::CHARSET_NAME,
            "866",
            "cp866",
            "csibm866",
        ]
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "ibm866")))]
impl crate::CharsetDecoding for Ibm866 {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "ibm866")))]
impl crate::CharsetEncoding for Ibm866 {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}
