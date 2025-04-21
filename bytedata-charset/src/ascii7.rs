
/// An encoding for ASCII-7.
/// 
/// In general, you can use the UTF-8 encoding instead of ASCII-7.
/// The only reason to use ASCII-7 is if you need to *ensure* that the text is ASCII-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[non_exhaustive]
pub struct Ascii7Encoding;

/// ASCII-7 encoding.
pub static ASCII7: Ascii7Encoding = Ascii7Encoding::new();

impl Ascii7Encoding {

    /// Create a new ASCII-7 encoding instance.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
    
    /// Decode characters from the given bytes.
    #[inline]
    #[must_use]
    pub fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        let maxlen = bytes.len();
        if maxlen == 0 {
            return crate::DecodeResult::Empty;
        }
        let bytes_ptr = bytes.as_ptr();

        #[cfg(all(target_arch = "x86_64", feature = "avx512f"))]
        if maxlen >= 64 && is_x86_feature_detected!("avx512f") {
            // SAFETY: The pointer is valid and the length is correct, and avx512f has been checked.
            let res = unsafe { ascii7_decode_avx512(bytes_ptr, maxlen, 0) };
            if !matches!(res, crate::DecodeResult::Empty) {
                return res;
            }
        }

        #[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), feature = "avx"))]
        if maxlen >= 32 && is_x86_feature_detected!("avx") {
            // SAFETY: The pointer is valid and the length is correct, and avx has been checked.
            let res = unsafe { ascii7_decode_avx(bytes_ptr, maxlen, 0) };
            if !matches!(res, crate::DecodeResult::Empty) {
                return res;
            }
            // SAFETY: The pointer is valid and there is at least 32 bytes available.
            let byte = unsafe { bytes_ptr.read() };
            #[expect(clippy::cast_lossless)]
            let byte = byte as u32;
            return crate::DecodeResult::InvalidChar(byte, 1);
        }

        // SAFETY: The pointer is valid and the length is correct.
        unsafe { ascii7_decode_const(bytes_ptr, maxlen, 0) }
    }

    /// Decode a ASCII-7 sequence.
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

    /// Encode characters from the given bytes.
    #[inline]
    #[must_use]
    pub fn encode(&self, chars: &str) -> crate::EncodeResult {
        let maxlen = chars.len();
        if maxlen == 0 {
            return crate::EncodeResult::Empty;
        }
        let chars_ptr = chars.as_bytes().as_ptr();

        #[cfg(all(target_arch = "x86_64", feature = "avx512f"))]
        if maxlen >= 64 && is_x86_feature_detected!("avx512f") {
            // SAFETY: The pointer is valid and the length is correct, and avx512f has been checked.
            let res = unsafe { ascii7_encode_avx512(chars_ptr, maxlen, 0) };
            if !matches!(res, crate::EncodeResult::Empty) {
                return res;
            }
        }

        #[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), feature = "avx"))]
        if maxlen >= 32 && is_x86_feature_detected!("avx") {
            // SAFETY: The pointer is valid and the length is correct, and avx has been checked.
            let res = unsafe { ascii7_encode_avx(chars_ptr, maxlen, 0) };
            if !matches!(res, crate::EncodeResult::Empty) {
                return res;
            }
        }

        // SAFETY: The pointer is valid and the length is correct.
        unsafe { ascii7_encode_const(chars_ptr, maxlen, 0) }
    }

    /// Encode a ASCII-7 character sequence.
    #[inline]
    #[must_use]
    pub const fn encode_const(&self, chars: &str) -> crate::EncodeResult {
        if chars.is_empty() {
            return crate::EncodeResult::Empty;
        }

        // SAFETY: The pointer is valid and the length is correct.
        unsafe { ascii7_encode_const(chars.as_ptr(), chars.len(), 0) }
    }

    /// Detect if the given bytes are UTF-8 encoded.
    #[inline]
    #[must_use]
    pub fn detect(bytes: &[u8]) -> crate::detect::DetectionResult {
        let maxlen = bytes.len();
        if maxlen == 0 {
            return crate::DetectionResult::Incomplete;
        }
        let bytes_ptr = bytes.as_ptr();

        #[cfg(all(target_arch = "x86_64", feature = "avx512f"))]
        #[expect(clippy::wildcard_enum_match_arm)]
        if maxlen >= 64 && is_x86_feature_detected!("avx512f") {
            // SAFETY: The pointer is valid and the length is correct, and avx512f has been checked.
            let res = unsafe { ascii7_decode_avx512(bytes_ptr, maxlen, 0) };
            match res {
                crate::DecodeResult::Utf8(n) if n == maxlen as u64 => return crate::detect::DetectionResult::Tentative,
                _ => return crate::detect::DetectionResult::Irrelevant,
            }
        }

        #[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), feature = "avx"))]
        #[expect(clippy::wildcard_enum_match_arm)]
        if maxlen >= 32 && is_x86_feature_detected!("avx") {
            // SAFETY: The pointer is valid and the length is correct, and avx has been checked.
            let res = unsafe { ascii7_decode_avx(bytes_ptr, maxlen, 0) };
            match res {
                crate::DecodeResult::Utf8(n) if n == maxlen as u64 => return crate::detect::DetectionResult::Tentative,
                _ => return crate::detect::DetectionResult::Irrelevant,
            }
        }

        detect_const_inner(bytes)
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

impl crate::Charset for Ascii7Encoding {
    const CHARSET_NAME: &'static str = "ascii-7";

    #[inline]
    fn charset_name(&self) -> &'static str {
        "us-ascii"
    }

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 4)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &["us-ascii", "iso-ir-6", "ansi_x3.4-1968", "ansi_x3.4-1986", "iso_646.irv:1991", "iso646-us", "us", "ibm367", "cp367", "csascii"]
    }
}

impl crate::detect::CharsetDetector for Ascii7Encoding {
    #[inline]
    fn detect(&self, bytes: &[u8]) -> crate::detect::DetectionResult {
        Self::detect_const(bytes)
    }
}

impl crate::CharsetDecoding for Ascii7Encoding {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

impl crate::CharsetEncoding for Ascii7Encoding {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}

const fn detect_const_inner(bytes: &[u8]) -> crate::detect::DetectionResult {
    let len = bytes.len();
    let mut i = 0;
    while i < len {
        if bytes[i] == 0 {
            return crate::detect::DetectionResult::Irrelevant;
        }
        if bytes[i] < 128 {
            i += 1;
            continue;
        }
        return crate::detect::DetectionResult::Irrelevant;
    }
    if len > 1024 {
        return crate::detect::DetectionResult::Certain;
    }
    crate::detect::DetectionResult::Tentative
}

const unsafe fn ascii7_encode_const(mut val: *const u8, mut maxlen: usize, mut utflen: usize) -> crate::EncodeResult {
    while maxlen != 0 {
        let byte = val.read();
        if byte < 128 {
            val = val.add(1);
            utflen += 1;
            maxlen -= 1;
            continue;
        }
        if utflen != 0 {
            break;
        }
        let bytes = core::slice::from_raw_parts(val, maxlen);
        let (ch, blen) = bytedata::const_utf8_char_next(bytes);
        let ch = core::mem::transmute::<u32, char>(ch);
        #[expect(clippy::cast_possible_truncation)]
        let blen = blen as u16;
        return crate::EncodeResult::InvalidChar(ch, blen);
    }
    crate::EncodeResult::Utf8(utflen as u64)
}



#[cfg(all(target_arch = "x86_64", feature = "avx512f"))]
#[inline]
#[expect(clippy::redundant_pub_crate)]
pub(crate) unsafe fn ascii7_encode_avx512(val: *const u8, maxlen: usize, utflen: usize) -> crate::EncodeResult {
    #[cfg(feature = "avx512bw")]
    if is_x86_feature_detected!("avx512bw") {
        return ascii7_encode_avx512bw(val, maxlen, utflen);
    }
    ascii7_encode_avx512f(val, maxlen, utflen)
}
    
/// Encode characters from the given bytes.
#[cfg(all(target_arch = "x86_64", feature = "avx512bw"))]
#[target_feature(enable = "avx512bw", enable = "avx512f")]
unsafe fn ascii7_encode_avx512bw(mut val: *const u8, mut maxlen: usize, mut utflen: usize) -> crate::EncodeResult {
    loop {
        #[expect(clippy::cast_ptr_alignment)]
        let data = core::arch::x86_64::_mm512_loadu_si512(val.cast::<i32>());
        #[expect(clippy::cast_possible_wrap)]
        let masked = core::arch::x86_64::_mm512_test_epi8_mask(data, core::arch::x86_64::_mm512_set1_epi8(0x80_u8 as i8));
        let zc = masked.leading_zeros() as usize;
        utflen += zc;
        if zc == 64 {
            maxlen -= 64;
            val = val.add(64);
            if maxlen >= 64 {
                continue;
            }
            #[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), feature = "avx"))]
            if maxlen >= 32 && is_x86_feature_detected!("avx") {
                return ascii7_encode_avx(val, maxlen, utflen);
            }
            if maxlen != 0 {
                return ascii7_encode_const(val, maxlen, utflen);
            }
            return crate::EncodeResult::Utf8(utflen as u64);
        }
        if utflen != 0 {
            return crate::EncodeResult::Utf8(utflen as u64);
        }
        return crate::EncodeResult::Empty;
    }
}

/// Encode characters from the given bytes.
#[cfg(all(target_arch = "x86_64", feature = "avx512f"))]
#[target_feature(enable = "avx512f")]
unsafe fn ascii7_encode_avx512f(mut val: *const u8, mut maxlen: usize, mut utflen: usize) -> crate::EncodeResult {
    loop {
        #[expect(clippy::cast_ptr_alignment)]
        let data = core::arch::x86_64::_mm512_loadu_si512(val.cast::<i32>());
        #[expect(clippy::cast_possible_wrap)]
        let masked = core::arch::x86_64::_mm512_test_epi32_mask(data, core::arch::x86_64::_mm512_set1_epi8(0x80_u8 as i8));
        let zc = (masked.leading_zeros() as usize) << 2;
        utflen += zc;
        if zc == 64 {
            maxlen -= 64;
            val = val.add(64);
            if maxlen >= 64 {
                continue;
            }
            #[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), feature = "avx"))]
            if maxlen >= 32 && is_x86_feature_detected!("avx") {
                return ascii7_encode_avx(val, maxlen, utflen);
            }
            if maxlen != 0 {
                return ascii7_encode_const(val, maxlen, utflen);
            }
            return crate::EncodeResult::Utf8(utflen as u64);
        }
        if zc != 0 {
            maxlen -= zc;
            val = val.add(zc);
        }
        if utflen != 0 {
            return ascii7_encode_const(val, maxlen, utflen);
        }
        return crate::EncodeResult::Empty;
    }
}
    
/// Encode characters from the given bytes.
#[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), feature = "avx"))]
#[target_feature(enable = "avx")]
#[expect(clippy::redundant_pub_crate)]
pub(crate) unsafe fn ascii7_encode_avx(mut val: *const u8, mut maxlen: usize, mut utflen: usize) -> crate::EncodeResult {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::{__m256i, _mm256_loadu_si256, _mm256_testc_si256, _mm256_set1_epi8};
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::{__m256i, _mm256_loadu_si256, _mm256_testc_si256, _mm256_set1_epi8};

    loop {
        #[expect(clippy::cast_ptr_alignment)]
        let data = _mm256_loadu_si256(val.cast::<__m256i>());
        #[expect(clippy::cast_possible_wrap)]
        let masked = _mm256_testc_si256(data, _mm256_set1_epi8(0x80_u8 as i8));
        let zc = masked.leading_zeros();
        utflen += zc as usize;
        if zc == 32 {
            maxlen -= 32;
            val = val.add(32);
            if maxlen >= 32 {
                continue;
            }
            if maxlen != 0 {
                return ascii7_encode_const(val, maxlen, utflen);
            }
            return crate::EncodeResult::Utf8(utflen as u64);
        }
        if utflen != 0 {
            return crate::EncodeResult::Utf8(utflen as u64);
        }
        return crate::EncodeResult::Empty;
    }
}
    
/// Encode characters from the given bytes.
#[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), feature = "avx"))]
#[target_feature(enable = "avx")]
#[expect(clippy::redundant_pub_crate)]
pub(crate) unsafe fn ascii7_decode_avx(mut val: *const u8, mut maxlen: usize, mut utflen: usize) -> crate::DecodeResult {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::{__m256i, _mm256_loadu_si256, _mm256_testc_si256, _mm256_set1_epi8};
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::{__m256i, _mm256_loadu_si256, _mm256_testc_si256, _mm256_set1_epi8};

    loop {
        #[expect(clippy::cast_ptr_alignment)]
        let data = _mm256_loadu_si256(val.cast::<__m256i>());
        #[expect(clippy::cast_possible_wrap)]
        let masked = _mm256_testc_si256(data, _mm256_set1_epi8(0x80_u8 as i8));
        let zc = masked.leading_zeros();
        utflen += zc as usize;
        if zc == 32 && maxlen >= 64 {
            maxlen -= 32;
            val = val.add(32);
            continue;
        }
        if utflen != 0 {
            return crate::DecodeResult::Utf8(utflen as u64);
        }
        return crate::DecodeResult::Empty;
    }
}

#[cfg(all(target_arch = "x86_64", feature = "avx512f"))]
#[inline]
#[expect(clippy::redundant_pub_crate)]
pub(crate) unsafe fn ascii7_decode_avx512(val: *const u8, maxlen: usize, utflen: usize) -> crate::DecodeResult {
    #[cfg(feature = "avx512bw")]
    if is_x86_feature_detected!("avx512bw") {
        return ascii7_decode_avx512bw(val, maxlen, utflen);
    }
    ascii7_decode_avx512f(val, maxlen, utflen)
}
    
/// Encode characters from the given bytes.
#[cfg(all(target_arch = "x86_64", feature = "avx512bw"))]
#[target_feature(enable = "avx512bw", enable = "avx512f")]
unsafe fn ascii7_decode_avx512bw(mut val: *const u8, mut maxlen: usize, mut utflen: usize) -> crate::DecodeResult {
    loop {
        #[expect(clippy::cast_ptr_alignment)]
        let data = core::arch::x86_64::_mm512_loadu_si512(val.cast::<i32>());
        #[expect(clippy::cast_possible_wrap)]
        let masked = core::arch::x86_64::_mm512_test_epi8_mask(data, core::arch::x86_64::_mm512_set1_epi8(0x80_u8 as i8));
        let zc = masked.leading_zeros() as usize;
        utflen += zc;
        if zc == 64 {
            maxlen -= 64;
            val = val.add(64);
            if maxlen >= 64 {
                continue;
            }
            #[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), feature = "avx"))]
            if maxlen >= 32 && is_x86_feature_detected!("avx") {
                return ascii7_decode_avx(val, maxlen, utflen);
            }
            if maxlen != 0 {
                return ascii7_decode_const(val, maxlen, utflen);
            }
            return crate::DecodeResult::Utf8(utflen as u64);
        }
        if utflen != 0 {
            return crate::DecodeResult::Utf8(utflen as u64);
        }
        return crate::DecodeResult::Empty;
    }
}

/// Encode characters from the given bytes.
#[cfg(all(target_arch = "x86_64", feature = "avx512f"))]
#[target_feature(enable = "avx512f")]
unsafe fn ascii7_decode_avx512f(mut val: *const u8, mut maxlen: usize, mut utflen: usize) -> crate::DecodeResult {
    loop {
        #[expect(clippy::cast_ptr_alignment)]
        let data = core::arch::x86_64::_mm512_loadu_si512(val.cast::<i32>());
        #[expect(clippy::cast_possible_wrap)]
        let masked = core::arch::x86_64::_mm512_test_epi32_mask(data, core::arch::x86_64::_mm512_set1_epi8(0x80_u8 as i8));
        let zc = (masked.leading_zeros() as usize) << 2;
        utflen += zc;
        if zc == 64 {
            maxlen -= 64;
            val = val.add(64);
            if maxlen >= 64 {
                continue;
            }
            #[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), feature = "avx"))]
            if maxlen >= 32 && is_x86_feature_detected!("avx") {
                return ascii7_decode_avx(val, maxlen, utflen);
            }
            if maxlen != 0 {
                return ascii7_decode_const(val, maxlen, utflen);
            }
            return crate::DecodeResult::Utf8(utflen as u64);
        }
        if zc != 0 {
            maxlen -= zc;
            val = val.add(zc);
        }
        if utflen != 0 {
            return ascii7_decode_const(val, maxlen, utflen);
        }
        return crate::DecodeResult::Empty;
    }
}

const unsafe fn ascii7_decode_const(mut val: *const u8, mut maxlen: usize, mut utflen: usize) -> crate::DecodeResult {
    while maxlen != 0 {
        let byte = val.read();
        if byte < 128 {
            val = val.add(1);
            utflen += 1;
            maxlen -= 1;
            continue;
        }
        if utflen != 0 {
            break;
        }
        let bytes = core::slice::from_raw_parts(val, maxlen);
        let (ch, blen) = bytedata::const_utf8_char_next(bytes);
        return crate::DecodeResult::InvalidChar(ch, blen);
    }
    crate::DecodeResult::Utf8(utflen as u64)
}
