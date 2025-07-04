//! ## ASCII-7 compatible charset encoding
//!
//! This module provides the base for an 8-bit ASCII-7 compatible charset encoder.
//! See the [`AsciiCompatible`] struct for more information.

/// An ASCII-7 compatible charset encoder.
///
/// This struct provides the base for an 8-bit ASCII-7 compatible charset encoder.
/// As the 128 first characters are the same as ASCII-7, this needs to map the 128 extended characters to the corresponding unicode character.
/// A list of the extended characters is provided as a static array to the [`AsciiCompatible::new`] function to allow this charset to substitute the bytes and characters.
///
/// If the encoding you are implementing does not have the full set of 128 extended characters defined, you can use the `'\0'` value to trigger [`DecodeResult::InvalidChar`] for the missing characters.
///
/// *The feature guarded ISO-8859 and windows charsets in this crate internally uses `AsciiCompatible` to provide their charset encoding.*
///
/// [`DecodeResult::InvalidChar`]: crate::DecodeResult::InvalidChar
#[cfg_attr(docsrs, doc(cfg(feature = "ascii7-compat")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AsciiCompatible {
    chars: &'static [char; 128],
    name: &'static str,
}

impl AsciiCompatible {
    /// Create a new ASCII-compatible charset encoding instance.
    #[inline]
    #[must_use]
    pub const fn new(name: &'static str, chars: &'static [char; 128]) -> Self {
        Self { chars, name }
    }

    /// Decode characters from the given bytes.
    #[inline]
    #[must_use]
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
        // SAFETY: The pointer is valid and there is at least 1 byte available.
        let byte = unsafe { bytes.as_ptr().read() };
        decode_const_inner(self.chars, byte)
    }

    /// Decode characters from the given bytes.
    #[inline]
    #[must_use]
    pub fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        const fn fallback(this: &AsciiCompatible, bytes: &[u8]) -> crate::DecodeResult {
            this.decode_const(bytes)
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
            // SAFETY: The pointer is valid and there is at least 32 bytes available.
            let byte = unsafe { bytes_ptr.read() };
            return decode_const_inner(self.chars, byte);
        }

        fallback(self, bytes)
    }

    /// Decode characters from the given bytes.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    #[inline]
    #[must_use]
    pub fn decode_into(
        &self,
        bytes: &[u8],
        chars: &mut bytedata::SharedStrBuilder,
    ) -> crate::ExhaustiveDecodeResult<(u32, u32)> {
        if bytes.is_empty() {
            return crate::ExhaustiveDecodeResult::Empty;
        }
        Self::decode_into_inner(self, bytes, chars)
    }

    /// Decode characters from the given bytes.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    #[must_use]
    fn decode_into_inner(
        &self,
        mut bytes: &[u8],
        chars: &mut bytedata::SharedStrBuilder,
    ) -> crate::ExhaustiveDecodeResult<(u32, u32)> {
        let mut consumed = 0;
        let chars_prefix = chars.len();
        loop {
            let res = self.decode(bytes);
            match res {
                crate::DecodeResult::Char(ch, len) => {
                    chars.push(ch);
                    consumed += len;
                    // SAFETY: The pointer is valid and the length is correct.
                    bytes = unsafe { bytes.get_unchecked(len as usize..) };
                }
                #[expect(clippy::cast_possible_truncation)]
                crate::DecodeResult::Utf8(len) => {
                    let len = len as usize;
                    // SAFETY: The pointer is valid and the length is correct.
                    let utf8 = unsafe { core::str::from_utf8_unchecked(&bytes[..len]) };
                    chars.push_str(utf8);
                    // SAFETY: The pointer is valid and the length is correct.
                    bytes = unsafe { bytes.get_unchecked(len..) };
                    consumed += len as u32;
                }
                _ if consumed != 0 => break,
                crate::DecodeResult::InvalidChar(ch, len) => {
                    return crate::ExhaustiveDecodeResult::InvalidChar(ch, len)
                }
                crate::DecodeResult::Incomplete => {
                    return crate::ExhaustiveDecodeResult::Incomplete
                }
                crate::DecodeResult::Empty => return crate::ExhaustiveDecodeResult::Empty,
            }
        }
        let chars_len = chars.len() - chars_prefix;
        #[expect(clippy::cast_possible_truncation)]
        crate::ExhaustiveDecodeResult::Decoded((consumed, chars_len as u32))
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
        encode_const_inner(self.chars, chars)
    }

    /// Encode characters from the given bytes.
    #[inline]
    #[must_use]
    pub fn encode(&self, chars: &str) -> crate::EncodeResult {
        const fn fallback(this: &AsciiCompatible, chars: &str) -> crate::EncodeResult {
            this.encode_const(chars)
        }

        let maxlen = chars.len();
        if maxlen == 0 {
            return crate::EncodeResult::Empty;
        }
        let chars_ptr = chars.as_bytes().as_ptr();

        #[cfg(all(target_arch = "x86_64", feature = "avx512f"))]
        if maxlen >= 64 && is_x86_feature_detected!("avx512f") {
            // SAFETY: The pointer is valid and the length is correct, and avx512f has been checked.
            let res = unsafe { super::ascii7::ascii7_encode_avx512(chars_ptr, maxlen, 0) };
            if !matches!(res, crate::EncodeResult::Empty) {
                return res;
            }
        }

        #[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), feature = "avx"))]
        if maxlen >= 32 && is_x86_feature_detected!("avx") {
            // SAFETY: The pointer is valid and the length is correct, and avx has been checked.
            let res = unsafe { super::ascii7::ascii7_encode_avx(chars_ptr, maxlen, 0) };
            if !matches!(res, crate::EncodeResult::Empty) {
                return res;
            }
            return encode_const_inner(self.chars, chars);
        }

        fallback(self, chars)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "ascii7-compat")))]
impl crate::Charset for AsciiCompatible {
    const CHARSET_NAME: &'static str = "ASCII-compatible";
    #[inline]
    fn charset_name(&self) -> &'static str {
        self.name
    }

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 1)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "ascii7-compat")))]
impl crate::CharsetDecoding for AsciiCompatible {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "ascii7-compat")))]
impl crate::CharsetEncoding for AsciiCompatible {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}

#[inline]
const fn decode_const_inner(chars: &[char; 128], byte: u8) -> crate::DecodeResult {
    let by = (byte & 0x7F) as usize;
    // SAFETY: The index is always in bounds.
    let ptr = unsafe { chars.as_ptr().add(by) };
    // SAFETY: The pointer is always valid.
    let ch = unsafe { ptr.read() };
    if ch == '\0' {
        crate::DecodeResult::InvalidChar(byte as u32, 1)
    } else {
        crate::DecodeResult::Char(ch, 1)
    }
}

#[expect(clippy::too_many_lines)]
const fn encode_const_inner(chars: &'static [char; 128], data: &str) -> crate::EncodeResult {
    /// Find a character in the extended charset. Returns `0` if not found or if the value was actually `0`.
    #[inline]
    const fn find_char(chars: &'static [char; 128], ch: u32) -> u8 {
        if ch < 128 {
            #[expect(clippy::cast_possible_truncation)]
            return ch as u8;
        }
        let mut i = 0;
        while i < 128 {
            if chars[i] as u32 != ch {
                i += 1;
                continue;
            }
            #[expect(clippy::cast_possible_truncation)]
            return (128 + i) as u8;
        }
        0
    }
    let mut buf = [0_u8; 14];
    let mut consumed = 0;
    let mut buff_off = 0;
    let mut ascii_off = 0;
    let mut ascii_con = 0;
    let mut mode_ascii = false;
    let mut chardata = data;
    loop {
        let (cp, b_len) = bytedata::const_utf8_char_next(chardata.as_bytes());
        if b_len == 0 {
            if chardata.is_empty() || consumed != 0 {
                break;
            }
            return crate::EncodeResult::Incomplete;
        }
        if cp < 128 {
            #[expect(clippy::cast_possible_truncation)]
            let cp = cp as u8;
            buf[buff_off] = cp;
            if b_len != 1 {
                mode_ascii = false;
                buff_off += 1;
                consumed += b_len;
                if buff_off < buf.len() {
                    chardata =
                        match bytedata::const_slice_str(chardata, (b_len as usize)..chardata.len())
                        {
                            bytedata::StrSliceResult::OutOfBounds
                            | bytedata::StrSliceResult::InvalidUtf8 => break,
                            bytedata::StrSliceResult::Success(x) => x,
                        };
                    continue;
                }
                break;
            }
            if !mode_ascii {
                mode_ascii = true;
                ascii_off = buff_off;
                ascii_con = consumed;
            }
            buff_off += 1;
            consumed += 1;
            if buff_off < buf.len() {
                chardata =
                    match bytedata::const_slice_str(chardata, (b_len as usize)..chardata.len()) {
                        bytedata::StrSliceResult::OutOfBounds
                        | bytedata::StrSliceResult::InvalidUtf8 => break,
                        bytedata::StrSliceResult::Success(x) => x,
                    };
                continue;
            }
            break;
        }
        let found = find_char(chars, cp);
        if found == 0 && cp != 0 {
            if consumed != 0 {
                break;
            }
            if let Some(x) = char::from_u32(cp) {
                #[expect(clippy::cast_possible_truncation)]
                return crate::EncodeResult::InvalidChar(x, b_len as u16);
            }
            return crate::EncodeResult::Incomplete;
        }
        if mode_ascii {
            mode_ascii = false;
        }
        buf[buff_off] = found;
        buff_off += 1;
        consumed += b_len;
        if buff_off != buf.len() {
            chardata = match bytedata::const_slice_str(chardata, (b_len as usize)..chardata.len()) {
                bytedata::StrSliceResult::OutOfBounds | bytedata::StrSliceResult::InvalidUtf8 => {
                    break
                }
                bytedata::StrSliceResult::Success(x) => x,
            };
            continue;
        }
        break;
    }
    if buff_off == 0 {
        return crate::EncodeResult::Empty;
    }
    let buff_off = if mode_ascii { ascii_off } else { buff_off };
    let consumed = if mode_ascii { ascii_con } else { consumed };
    let slice = bytedata::const_or_bytes(bytedata::const_slice(buf.as_slice(), 0..buff_off), b"");
    #[expect(clippy::cast_possible_truncation)]
    let consumed = consumed as u16;
    crate::EncodeResult::Chunk(bytedata::ByteChunk::from_slice(slice), consumed)
}
