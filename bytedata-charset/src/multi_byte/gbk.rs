
/// GBK encoding. A legacy encoding for simplified Chinese characters.
///
/// Use [`UTF-8`], [`UTF-16`], or [`GB18030`] instead if possible.
/// (GB18030 is an encoding backward compatible with GBK but support a much larger mapping against Unicode.)
/// 
/// [`UTF-8`]: crate::Utf8Encoding
/// [`UTF-16`]: crate::Utf16Encoding
/// [`GB18030`]: crate::multi_byte::Gb18030Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[non_exhaustive]
#[repr(transparent)]
#[cfg_attr(docsrs, doc(cfg(feature = "gbk")))]
pub struct GbkEncoding(super::Gb18030Encoding);

/// GBK encoding.
#[cfg_attr(docsrs, doc(cfg(feature = "gbk")))]
pub static GBK: GbkEncoding = GbkEncoding::new();

impl GbkEncoding {

    /// Create a new GBK encoding instance.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self(super::Gb18030Encoding { gbk: true })
    }

    /// Decode a GBK byte sequence.
    #[inline]
    #[must_use]
    pub const fn decode_const(&self, bytes: &[u8]) -> crate::DecodeResult {
        if bytes.is_empty() {
            return crate::DecodeResult::Empty;
        }
        match core::str::from_utf8(bytes) {
            Ok(st) => crate::DecodeResult::Utf8(st.len() as u64),
            Err(err) => {
                let vp = err.valid_up_to();
                if vp != 0 {
                    return crate::DecodeResult::Utf8(vp as u64);
                }
                if err.error_len().is_some() {
                    crate::DecodeResult::InvalidChar(bytes[0] as u32, 1)
                } else {
                    crate::DecodeResult::Incomplete
                }
            }
        }
    }

    /// Encode a GBK character sequence.
    #[inline]
    #[must_use]
    pub const fn encode_const(&self, chars: &str) -> crate::EncodeResult {
        if chars.is_empty() {
            return crate::EncodeResult::Empty;
        }
        crate::EncodeResult::Utf8(chars.len() as u64)
    }

    /// Detect if the given bytes are GBK encoded.
    #[inline]
    #[must_use]
    pub const fn detect_const(bytes: &[u8]) -> crate::detect::DetectionResult {
        if bytes.is_empty() {
            return crate::detect::DetectionResult::Incomplete;
        }
        GBK.0.detect_const_self(bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "gbk")))]
impl crate::Charset for GbkEncoding {
    const CHARSET_NAME: &'static str = "gbk";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 2)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // IANA
            Self::CHARSET_NAME,
            "csgbk",
            "cp936",
            "ms936",
            "windows-936",

            // other
            "chinese",
            "csgb2312",
            "csiso58gb231280",
            "gb2312",
            "gb_2312",
            "gb_2312-80",
            "iso-ir-58",
            "x-gbk",
        ]
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "gbk")))]
impl crate::detect::CharsetDetector for GbkEncoding {
    #[inline]
    fn detect(&self, bytes: &[u8]) -> crate::detect::DetectionResult {
        self.0.detect(bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "gbk")))]
impl crate::CharsetDecoding for GbkEncoding {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        self.0.decode(bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "gbk")))]
impl crate::CharsetEncoding for GbkEncoding {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        self.0.encode(chars)
    }
}
