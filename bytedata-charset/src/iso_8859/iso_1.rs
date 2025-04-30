use crate::ascii7_compat::AsciiCompatible;

/// A mapper from bytes over 128 to the corresponding unicode character.
const ENCODER: AsciiCompatible = AsciiCompatible::new(
    <Iso8859_1 as crate::Charset>::CHARSET_NAME,
    &super::ISO_8859_1_CHARSET,
);

/// An encoding for ISO-8859-1.
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-1")))]
pub static ISO_8859_1: Iso8859_1 = Iso8859_1::new();

/// An encoding for ISO-8859-1.
/// If possible use [`UTF-8`] instead.
///
/// [`UTF-8`]: crate::Utf8Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::exhaustive_structs)]
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-1")))]
pub struct Iso8859_1;

impl Iso8859_1 {
    /// Create a new ISO-8859-1 encoding instance.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Decode characters from the given bytes.
    #[must_use]
    #[inline]
    pub const fn decode_const(&self, bytes: &[u8]) -> crate::DecodeResult {
        if bytes.is_empty() {
            return crate::DecodeResult::Empty;
        }
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] < 128 {
                i += 1;
                continue;
            }
            break;
        }
        if i != 0 {
            return crate::DecodeResult::Utf8(i as u64);
        }
        let chr = bytes[0] as char;
        crate::DecodeResult::Char(chr, 1)
    }

    /// Decode characters from the given bytes.
    #[inline]
    #[must_use]
    pub fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        const fn fallback(bytes: &[u8]) -> crate::DecodeResult {
            Iso8859_1::new().decode_const(bytes)
        }

        let maxlen = bytes.len();
        if maxlen == 0 {
            return crate::DecodeResult::Empty;
        }
        let bytes_ptr = bytes.as_ptr();

        #[cfg(all(target_arch = "x86_64", feature = "avx512f"))]
        if maxlen >= 64 && is_x86_feature_detected!("avx512f") {
            // SAFETY: The pointer is valid and the length is correct, and avx512f has been checked.
            let res = unsafe { crate::ascii7::ascii7_decode_avx512(bytes_ptr, maxlen, 0) };
            if !matches!(res, crate::DecodeResult::Empty) {
                return res;
            }
        }

        #[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), feature = "avx"))]
        if maxlen >= 32 && is_x86_feature_detected!("avx") {
            // SAFETY: The pointer is valid and the length is correct, and avx has been checked.
            let res = unsafe { crate::ascii7::ascii7_decode_avx(bytes_ptr, maxlen, 0) };
            if !matches!(res, crate::DecodeResult::Empty) {
                return res;
            }
            // SAFETY: The pointer is valid and there is at least 32 bytes available.
            let byte = unsafe { bytes_ptr.read() };
            return crate::DecodeResult::Char(byte as char, 1);
        }

        fallback(bytes)
    }

    /// Encode characters from the given bytes.
    #[inline]
    #[must_use]
    pub const fn encode_const(&self, chars: &str) -> crate::EncodeResult {
        if chars.is_empty() {
            return crate::EncodeResult::Empty;
        }
        let bytes = chars.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] < 128 {
                i += 1;
                continue;
            }
            break;
        }
        if i != 0 {
            return crate::EncodeResult::Utf8(i as u64);
        }
        encode_const_inner(bytes.as_ptr(), bytes.len())
    }

    /// Encode characters from the given bytes.
    #[inline]
    #[must_use]
    pub fn encode(&self, chars: &str) -> crate::EncodeResult {
        const fn fallback(chars: &str) -> crate::EncodeResult {
            Iso8859_1::new().encode_const(chars)
        }

        let maxlen = chars.len();
        if maxlen == 0 {
            return crate::EncodeResult::Empty;
        }
        let chars_ptr = chars.as_bytes().as_ptr();

        #[cfg(all(target_arch = "x86_64", feature = "avx512f"))]
        if maxlen >= 64 && is_x86_feature_detected!("avx512f") {
            // SAFETY: The pointer is valid and the length is correct, and avx512f has been checked.
            let res = unsafe { crate::ascii7::ascii7_encode_avx512(chars_ptr, maxlen, 0) };
            if !matches!(res, crate::EncodeResult::Empty) {
                return res;
            }
        }

        #[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), feature = "avx"))]
        if maxlen >= 32 && is_x86_feature_detected!("avx") {
            // SAFETY: The pointer is valid and the length is correct, and avx has been checked.
            let res = unsafe { crate::ascii7::ascii7_encode_avx(chars_ptr, maxlen, 0) };
            if !matches!(res, crate::EncodeResult::Empty) {
                return res;
            }
            return encode_const_inner(chars_ptr, maxlen);
        }

        fallback(chars)
    }

    /// Get the generic ASCII-compatible charset encoder for this charset.
    #[must_use]
    #[inline]
    pub const fn ascii_compat(&self) -> &'static AsciiCompatible {
        &ENCODER
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-1")))]
impl core::default::Default for Iso8859_1 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-1")))]
impl crate::Charset for Iso8859_1 {
    const CHARSET_NAME: &'static str = "iso-8859-1";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 1)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // IANA
            Self::CHARSET_NAME,
            "iso-ir-100",
            "iso_8859-1",
            "csisolatin1",
            "latin1",
            "l1",
            "iso_8859-1:1987",
            // code pages
            "cp819",
            "ibm819",
            "cp28591",
            "windows-28591",
        ]
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-1")))]
impl crate::CharsetDecoding for Iso8859_1 {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859-1")))]
impl crate::CharsetEncoding for Iso8859_1 {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}

const fn encode_const_inner(mut data: *const u8, mut maxlen: usize) -> crate::EncodeResult {
    let mut consumed = 0_u16;
    let mut buf = [0_u8; 14];
    let mut buf_len = 0;
    let mut ascii_mode = false;
    let mut ascii_consumed = 0;
    let mut ascii_offset = 0;
    loop {
        let (ch, b_len) = {
            // SAFETY: The pointer is valid and the length is correct.
            let slic = unsafe { core::slice::from_raw_parts(data, maxlen) };
            bytedata::const_utf8_char_next(slic)
        };
        if b_len == 0 || ch > 255 {
            break;
        }
        #[expect(clippy::cast_possible_truncation)]
        let ch = ch as u8;
        buf[buf_len] = ch;
        buf_len += 1;
        #[expect(clippy::cast_possible_truncation)]
        let b_len_16 = b_len as u16;
        consumed += b_len_16;
        let b_len = b_len as usize;
        maxlen -= b_len;
        // SAFETY: The pointer is valid and the length is lte the buffer length.
        data = unsafe { data.add(b_len) };
        #[expect(clippy::else_if_without_else)]
        if ch <= 127 && b_len == 1 {
            if !ascii_mode {
                ascii_mode = true;
                ascii_consumed = consumed;
                ascii_offset = buf_len;
            }
        } else if ascii_mode {
            ascii_mode = false;
        }
        if buf_len != 14 && maxlen != 0 {
            continue;
        }
        break;
    }
    if ascii_mode && ascii_offset != 0 {
        let buf = bytedata::const_or_bytes(bytedata::const_slice(&buf, 0..ascii_offset), b"");
        return crate::EncodeResult::Chunk(bytedata::ByteChunk::from_slice(buf), ascii_consumed);
    }
    if buf_len == 0 {
        return crate::EncodeResult::Incomplete;
    }
    let buf = bytedata::const_or_bytes(bytedata::const_slice(&buf, 0..ascii_offset), b"");
    crate::EncodeResult::Chunk(bytedata::ByteChunk::from_slice(buf), consumed)
}
