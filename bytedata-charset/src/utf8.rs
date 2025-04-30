/// An encoding for UTF-8.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[non_exhaustive]
pub struct Utf8Encoding;

/// UTF-8 encoding.
pub static UTF8: Utf8Encoding = Utf8Encoding::new();

impl Utf8Encoding {
    /// Create a new UTF-8 encoding instance.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Decode characters from the given bytes.
    #[inline]
    #[must_use]
    pub fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        const fn fallback(bytes: &[u8]) -> crate::DecodeResult {
            decode_const_inner(bytes)
        }

        let maxlen = bytes.len();
        if maxlen == 0 {
            return crate::DecodeResult::Empty;
        }
        let bytes_ptr = bytes.as_ptr();

        #[cfg(all(target_arch = "x86_64", feature = "avx512f"))]
        if maxlen >= 64 && is_x86_feature_detected!("avx512f") {
            // SAFETY: The pointer is valid and the length is correct, and avx512f has been checked.
            let res = unsafe { super::ascii7::ascii7_decode_avx512(bytes_ptr, maxlen, 0) };
            if !matches!(res, crate::DecodeResult::Empty) {
                return res;
            }
        }

        #[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), feature = "avx"))]
        if maxlen >= 32 && is_x86_feature_detected!("avx") {
            // SAFETY: The pointer is valid and the length is correct, and avx has been checked.
            let res = unsafe { super::ascii7::ascii7_decode_avx(bytes_ptr, maxlen, 0) };
            if !matches!(res, crate::DecodeResult::Empty) {
                return res;
            }
        }

        fallback(bytes)
    }

    /// Encode characters from the given bytes.
    #[inline]
    #[must_use]
    #[expect(clippy::missing_const_for_fn)]
    pub fn encode(&self, chars: &str) -> crate::EncodeResult {
        if chars.is_empty() {
            return crate::EncodeResult::Empty;
        }
        crate::EncodeResult::Utf8(chars.len() as u64)
    }

    /// Decode a UTF-8 byte sequence.
    #[inline]
    #[must_use]
    pub const fn decode_const(&self, bytes: &[u8]) -> crate::DecodeResult {
        if bytes.is_empty() {
            return crate::DecodeResult::Empty;
        }
        decode_const_inner(bytes)
    }

    /// Encode a UTF-8 character sequence.
    #[inline]
    #[must_use]
    pub const fn encode_const(&self, chars: &str) -> crate::EncodeResult {
        if chars.is_empty() {
            return crate::EncodeResult::Empty;
        }
        crate::EncodeResult::Utf8(chars.len() as u64)
    }

    /// Detect if the given bytes are UTF-8 encoded.
    #[inline]
    #[must_use]
    pub const fn detect_const(bytes: &[u8]) -> crate::detect::DetectionResult {
        if bytes.is_empty() {
            return crate::detect::DetectionResult::Incomplete;
        }
        detect_const_inner(bytes)
    }
}

impl crate::Charset for Utf8Encoding {
    const CHARSET_NAME: &'static str = "utf-8";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 4)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // IANA
            Self::CHARSET_NAME,
            "csutf8",
            // extra
            "utf8",
            "unicode-1-1-utf-8",
            "unicode11utf8",
            "unicode20utf8",
            "x-unicode20utf8",
        ]
    }
}

impl crate::detect::CharsetDetector for Utf8Encoding {
    #[inline]
    fn detect(&self, bytes: &[u8]) -> crate::detect::DetectionResult {
        Self::detect_const(bytes)
    }
}

impl crate::CharsetDecoding for Utf8Encoding {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode_const(self, bytes)
    }
}

impl crate::CharsetEncoding for Utf8Encoding {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode_const(self, chars)
    }
}

#[expect(clippy::missing_asserts_for_indexing)]
const fn detect_const_inner(bytes: &[u8]) -> crate::detect::DetectionResult {
    let len = bytes.len();
    if bytes.is_empty() {
        return crate::detect::DetectionResult::Incomplete;
    }
    if bytes[0] == 0xEF {
        if len == 1 {
            return crate::detect::DetectionResult::Incomplete;
        }
        if bytes[1] == 0xBB {
            if len == 2 {
                return crate::detect::DetectionResult::Incomplete;
            }
            if bytes[2] == 0xBF {
                return crate::detect::DetectionResult::Certain;
            }
        }
        return crate::detect::DetectionResult::Irrelevant;
    }
    let mut i = 0;
    while i < len {
        if bytes[i] == 0 {
            return crate::detect::DetectionResult::Irrelevant;
        }
        if bytes[i] < 128 {
            i += 1;
            continue;
        }
        if bytes[i] & 0b1110_0000 == 0b1100_0000 {
            if i + 1 < len {
                if bytes[i + 1] & 0b1100_0000 == 0b1000_0000 {
                    i += 2;
                    continue;
                }
                return crate::detect::DetectionResult::Irrelevant;
            }
            if len >= 1024 {
                return crate::detect::DetectionResult::Certain;
            }
            return crate::detect::DetectionResult::Tentative;
        }

        return crate::detect::DetectionResult::Irrelevant;
    }
    crate::detect::DetectionResult::Tentative
}

#[inline]
#[must_use]
const fn decode_const_inner(bytes: &[u8]) -> crate::DecodeResult {
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
