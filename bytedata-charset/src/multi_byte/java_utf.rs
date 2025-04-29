/// An encoding for Java modified UTF-8.
///
/// Java JNI operations use a modified version of UTF-8 encoding to be able to be compatible with UTF-16 charpoints and cstrings.
///
/// The modifications are that null characters ('\0') are encoded as two bytes and chars above `'\uFFFF'` are encoded as separate surrogate pairs which would be invalid utf-8.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[non_exhaustive]
pub struct JavaModifiedUtf8Encoding;

/// Java modified UTF-8 encoding.
pub static JAVA_MUTF_8: JavaModifiedUtf8Encoding = JavaModifiedUtf8Encoding::new();

impl JavaModifiedUtf8Encoding {
    /// Create a new MUTF-8 encoding instance.
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
            let res = unsafe { super::super::ascii7::ascii7_decode_avx512(bytes_ptr, maxlen, 0) };
            if !matches!(res, crate::DecodeResult::Empty) {
                return res;
            }
        }

        #[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), feature = "avx"))]
        if maxlen >= 32 && is_x86_feature_detected!("avx") {
            // SAFETY: The pointer is valid and the length is correct, and avx has been checked.
            let res = unsafe { super::super::ascii7::ascii7_decode_avx(bytes_ptr, maxlen, 0) };
            if !matches!(res, crate::DecodeResult::Empty) {
                return res;
            }
        }

        // SAFETY: The pointer is valid and the length is correct.
        unsafe { decode_const_inner(bytes_ptr, maxlen) }
    }

    /// Decode characters from the given cstring.
    ///
    /// ## Safety
    ///
    /// The caller must ensure that the pointer is valid and points to a null-terminated string.
    #[inline]
    #[must_use]
    #[expect(clippy::missing_const_for_fn)]
    pub unsafe fn decode_cstr(bytes: *const i8) -> crate::DecodeResult {
        // SAFETY: forward the safety guarantee to the inner function.
        unsafe {
            decode_const_inner(bytes.cast::<u8>(), 0)
        }
    }

    /// Decodes a full MUTF-8 cstring into a `bytedata::SharedStrBuilder`.
    ///
    /// ## Safety
    ///
    /// The caller must ensure that the pointer is valid and points to a null-terminated string.
    #[cfg(feature = "alloc")]
    #[allow(clippy::missing_inline_in_public_items)]
    pub unsafe fn decode_cstr_into(
        bytes: *const i8,
        chars: &mut bytedata::SharedStrBuilder,
    ) -> crate::result::ExhaustiveDecodeResult<(u32, u32)> {
        let utflen_prefix = chars.len();
        let mut consumed = 0;
        loop {
            // SAFETY: The pointer is valid and the length is already processed.
            let bytes_offset = unsafe { bytes.add(consumed) }.cast::<u8>();
            // SAFETY: The pointer is valid and the caller has guaranteed it is a cstr.
            let result = unsafe { decode_const_inner(bytes_offset, 0) };
            match result {
                crate::DecodeResult::Char(ch, con) => {
                    chars.push(ch);
                    consumed += con as usize;
                }
                crate::DecodeResult::Utf8(utf8) => {
                    #[expect(clippy::cast_possible_truncation)]
                    let utf8 = utf8 as usize;
                    consumed += utf8;
                    // SAFETY: The pointer is valid and the length is correct.
                    let str = unsafe { core::slice::from_raw_parts(bytes_offset, utf8) };
                    // SAFETY: The slice is guaranteed to contain a valid UTF-8 string.
                    let str = unsafe { core::str::from_utf8_unchecked(str) };
                    chars.push_str(str);
                }
                _ if consumed != 0 => {
                    break;
                }
                crate::DecodeResult::Empty => return crate::result::ExhaustiveDecodeResult::Empty,
                crate::DecodeResult::Incomplete => {
                    return crate::result::ExhaustiveDecodeResult::Incomplete
                }
                crate::DecodeResult::InvalidChar(ch, con) => {
                    return crate::result::ExhaustiveDecodeResult::InvalidChar(ch, con)
                }
            }
        }
        let utf8 = chars.len() - utflen_prefix;
        #[expect(clippy::cast_possible_truncation)]
        crate::result::ExhaustiveDecodeResult::Decoded((consumed as u32, utf8 as u32))
    }

    /// Encodes a `str` into a MUTF-8 `bytedata::SharedStrBuilder`.
    #[cfg(feature = "alloc")]
    #[allow(clippy::missing_inline_in_public_items)]
    pub fn encode_into(
        chars: &str,
        bytes: &mut bytedata::SharedBytesBuilder,
    ) -> crate::result::ExhaustiveEncodeResult<(u32, u32)> {
        let bytes_prefix = bytes.len();
        let mut consumed = 0;
        loop {
            // SAFETY: The pointer is valid and the length is already processed.
            let chars_offset =
                unsafe { bytedata::const_slice_str_unchecked(chars, consumed..chars.len()) };
            let result = encode_const_inner(chars_offset);
            match result {
                crate::EncodeResult::Chunk(ch, con) => {
                    bytes.extend_from_slice(ch.as_slice());
                    consumed += con as usize;
                }
                crate::EncodeResult::Utf8(utf8) => {
                    #[expect(clippy::cast_possible_truncation)]
                    let utf8 = utf8 as usize;
                    // SAFETY: The pointer is valid and the length is correct.
                    let str = unsafe {
                        bytedata::const_slice_str_unchecked(chars, consumed..(consumed + utf8))
                    };
                    bytes.extend_from_slice(str.as_bytes());
                    consumed += utf8;
                }
                _ if consumed != 0 => {
                    break;
                }
                crate::EncodeResult::Empty => return crate::result::ExhaustiveEncodeResult::Empty,
                crate::EncodeResult::Incomplete => {
                    return crate::result::ExhaustiveEncodeResult::Incomplete
                }
                crate::EncodeResult::InvalidChar(ch, con) => {
                    return crate::result::ExhaustiveEncodeResult::InvalidChar(ch, con)
                }
            }
        }
        let encoded = bytes.len() - bytes_prefix;
        #[expect(clippy::cast_possible_truncation)]
        crate::result::ExhaustiveEncodeResult::Encoded((consumed as u32, encoded as u32))
    }

    /// Encode characters from the given bytes.
    #[inline]
    #[must_use]
    #[expect(clippy::missing_const_for_fn)]
    pub fn encode(&self, chars: &str) -> crate::EncodeResult {
        if chars.is_empty() {
            return crate::EncodeResult::Empty;
        }
        encode_const_inner(chars)
    }

    /// Decode a MUTF-8 byte sequence.
    #[inline]
    #[must_use]
    pub const fn decode_const(&self, bytes: &[u8]) -> crate::DecodeResult {
        if bytes.is_empty() {
            return crate::DecodeResult::Empty;
        }
        // SAFETY: The pointer is valid and the length is correct.
        unsafe { decode_const_inner(bytes.as_ptr(), bytes.len()) }
    }

    /// Encode a MUTF-8 character sequence.
    #[inline]
    #[must_use]
    pub const fn encode_const(&self, chars: &str) -> crate::EncodeResult {
        if chars.is_empty() {
            return crate::EncodeResult::Empty;
        }
        encode_const_inner(chars)
    }
}

impl crate::Charset for JavaModifiedUtf8Encoding {
    const CHARSET_NAME: &'static str = "x-mutf-8";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 6)
    }
}

impl crate::CharsetDecoding for JavaModifiedUtf8Encoding {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode(self, bytes)
    }
}

impl crate::CharsetEncoding for JavaModifiedUtf8Encoding {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode(self, chars)
    }
}

#[inline]
#[must_use]
#[expect(clippy::too_many_lines)]
const fn encode_const_inner(chars: &str) -> crate::EncodeResult {
    let mut bytes = chars.as_bytes();
    let mut consumed = 0;
    let mut prev = Some(loop {
        let (charcode, charlen) = bytedata::const_utf8_char_next(bytes);

        // encode ascii characters as single bytes
        if charlen == 1 && charcode > 0 {
            consumed += 1;
            // SAFETY: one byte is consumed, so the slice is valid.
            bytes = unsafe { bytedata::const_slice_unchecked(bytes, 1..bytes.len()) };
            continue;
        }
        if charcode != 0 && charcode < 0x80 {
            if charlen == 2 {
                consumed += 2;
                // SAFETY: two bytes are consumed, so the slice is valid.
                bytes = unsafe { bytedata::const_slice_unchecked(bytes, 2..bytes.len()) };
                continue;
            }
            break (charcode, charlen);
        }

        // encode null character as two bytes
        if charcode == 0 {
            if charlen == 0 {
                if consumed != 0 {
                    return crate::EncodeResult::Utf8(consumed as u64);
                }
                if bytes.is_empty() {
                    return crate::EncodeResult::Empty;
                }
                return crate::EncodeResult::InvalidChar(bytes[0] as char, 1);
            }
            if charlen == 2 {
                consumed += 2;
                // SAFETY: two bytes are consumed, so the slice is valid.
                bytes = unsafe { bytedata::const_slice_unchecked(bytes, 2..bytes.len()) };
                continue;
            }
            break (charcode, charlen);
        }

        // encode two-byte characters
        if charcode < 0x0800 {
            if charlen == 2 {
                consumed += 2;
                // SAFETY: two bytes are consumed, so the slice is valid.
                bytes = unsafe { bytedata::const_slice_unchecked(bytes, 2..bytes.len()) };
                continue;
            }
            break (charcode, charlen);
        }

        // encode three-byte characters
        if charcode < 0x10000 {
            if charlen == 3 {
                consumed += 3;
                // SAFETY: three bytes are consumed, so the slice is valid.
                bytes = unsafe { bytedata::const_slice_unchecked(bytes, 3..bytes.len()) };
                continue;
            }
            break (charcode, charlen);
        }
        break (charcode, charlen);
    });

    // SAFETY: `MaybeUninit` is used to create an uninitialized array of bytes.
    #[expect(invalid_value, clippy::uninit_assumed_init)]
    let mut chunk: [u8; 14] = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
    let mut outlen = if consumed != 0 {
        if consumed > 6 {
            return crate::EncodeResult::Utf8(consumed as u64);
        }
        // SAFETY: the chunk is uninitialized, so we can copy the utf-8 bytes into it.
        unsafe {
            core::ptr::copy_nonoverlapping(chars.as_bytes().as_ptr(), chunk.as_mut_ptr(), consumed);
        };
        consumed
    } else {
        0
    };

    #[expect(clippy::cast_possible_truncation)]
    loop {
        let (charcode, charlen) = match prev.take() {
            Some(prev) => prev,
            None => bytedata::const_utf8_char_next(bytes),
        };

        // encode ascii characters as single bytes
        if charlen == 1 && charcode > 0 {
            chunk[outlen] = charcode as u8;
            outlen += 1;
            consumed += 1;
            if outlen <= 12 {
                // SAFETY: one byte is consumed, so the slice is valid.
                bytes = unsafe { bytedata::const_slice_unchecked(bytes, 1..bytes.len()) };
                continue;
            }
            // SAFETY: the chunk contains `outlen` initialized bytes, so we can create a slice from it.
            let chunk = bytedata::ByteChunk::from_slice(unsafe {
                bytedata::const_slice_unchecked(&chunk, 0..outlen)
            });
            return crate::EncodeResult::Chunk(chunk, consumed as u16);
        }
        #[expect(clippy::cast_possible_truncation)]
        if charcode != 0 && charcode < 0x80 {
            chunk[outlen] = charcode as u8;
            outlen += 1;
            consumed += 1;
            if outlen <= 12 {
                // SAFETY: `b` bytes are consumed, so the slice is valid.
                bytes = unsafe {
                    bytedata::const_slice_unchecked(bytes, (charlen as usize)..bytes.len())
                };
                continue;
            }
            // SAFETY: the chunk contains `outlen` initialized bytes, so we can create a slice from it.
            let chunk = bytedata::ByteChunk::from_slice(unsafe {
                bytedata::const_slice_unchecked(&chunk, 0..outlen)
            });
            return crate::EncodeResult::Chunk(chunk, consumed as u16);
        }

        // encode null character as two bytes
        #[expect(clippy::cast_possible_truncation)]
        if charcode == 0 {
            if charlen == 0 {
                if outlen != 0 {
                    // SAFETY: the chunk contains `outlen` initialized bytes, so we can create a slice from it.
                    let chunk = bytedata::ByteChunk::from_slice(unsafe {
                        bytedata::const_slice_unchecked(&chunk, 0..outlen)
                    });
                    return crate::EncodeResult::Chunk(chunk, consumed as u16);
                }
                if bytes.is_empty() {
                    return crate::EncodeResult::Empty;
                }
                // SAFETY: `bytes` is a valid pointer, and the first byte exists.
                let ch = unsafe { bytes.as_ptr().read() };
                return crate::EncodeResult::InvalidChar(ch as char, 1);
            }
            // encode null byte as two bytes
            chunk[outlen] = 0xC0;
            chunk[outlen + 1] = 0x80;
            outlen += 2;
            consumed += charlen as usize;
            if outlen <= 12 {
                // SAFETY: `b` bytes are consumed, so the slice is valid.
                bytes = unsafe {
                    bytedata::const_slice_unchecked(bytes, (charlen as usize)..bytes.len())
                };
                continue;
            }
            // SAFETY: the chunk contains `outlen` initialized bytes, so we can create a slice from it.
            let chunk = bytedata::ByteChunk::from_slice(unsafe {
                bytedata::const_slice_unchecked(&chunk, 0..outlen)
            });
            return crate::EncodeResult::Chunk(chunk, consumed as u16);
        }

        // encode two-byte characters
        #[expect(clippy::cast_possible_truncation)]
        if charcode < 0x0800 {
            // encode characters above 0x7F as two bytes
            chunk[outlen] = 0xC0 | ((charcode >> 6) as u8);
            chunk[outlen + 1] = 0x80 | ((charcode & 0x3F) as u8);
            outlen += 2;
            consumed += charlen as usize;
            if outlen <= 12 {
                // SAFETY: `b` bytes are consumed, so the slice is valid.
                bytes = unsafe {
                    bytedata::const_slice_unchecked(bytes, (charlen as usize)..bytes.len())
                };
                continue;
            }
            // SAFETY: the chunk contains `outlen` initialized bytes, so we can create a slice from it.
            let chunk = bytedata::ByteChunk::from_slice(unsafe {
                bytedata::const_slice_unchecked(&chunk, 0..outlen)
            });
            return crate::EncodeResult::Chunk(chunk, consumed as u16);
        }

        // encode three-byte characters
        if charcode < 0x10000 {
            if outlen >= 12 {
                // SAFETY: the chunk contains `outlen` initialized bytes, so we can create a slice from it.
                let chunk = bytedata::ByteChunk::from_slice(unsafe {
                    bytedata::const_slice_unchecked(&chunk, 0..outlen)
                });
                return crate::EncodeResult::Chunk(chunk, consumed as u16);
            }
            // encode characters above 0x7F as three bytes
            chunk[outlen] = 0xE0 | ((charcode >> 12) as u8);
            chunk[outlen + 1] = 0x80 | (((charcode >> 6) & 0x3F) as u8);
            chunk[outlen + 2] = 0x80 | ((charcode & 0x3F) as u8);
            outlen += 3;
            consumed += charlen as usize;
            if outlen <= 12 {
                // SAFETY: `b` bytes are consumed, so the slice is valid.
                bytes = unsafe {
                    bytedata::const_slice_unchecked(bytes, (charlen as usize)..bytes.len())
                };
                continue;
            }
            // SAFETY: the chunk contains `outlen` initialized bytes, so we can create a slice from it.
            let chunk = bytedata::ByteChunk::from_slice(unsafe {
                bytedata::const_slice_unchecked(&chunk, 0..outlen)
            });
            return crate::EncodeResult::Chunk(chunk, consumed as u16);
        }

        // encode surrogate pairs as six bytes
        if outlen > 8 {
            // SAFETY: the chunk contains `outlen` initialized bytes, so we can create a slice from it.
            let chunk = bytedata::ByteChunk::from_slice(unsafe {
                bytedata::const_slice_unchecked(&chunk, 0..outlen)
            });
            return crate::EncodeResult::Chunk(chunk, consumed as u16);
        }
        let surrogate_sum = charcode - 0x10000;
        chunk[outlen] = 0xED;
        chunk[outlen + 1] = 0xA0 | ((surrogate_sum >> 16) as u8);
        chunk[outlen + 2] = 0x80 | (((surrogate_sum >> 10) & 0x3F) as u8);
        chunk[outlen + 3] = 0xED;
        chunk[outlen + 4] = 0xB0 | (((surrogate_sum >> 6) & 0x0F) as u8);
        chunk[outlen + 5] = 0x80 | ((surrogate_sum & 0x3F) as u8);
        outlen += 6;
        consumed += charlen as usize;
        if outlen <= 12 {
            // SAFETY: `b` bytes are consumed, so the slice is valid.
            bytes =
                unsafe { bytedata::const_slice_unchecked(bytes, (charlen as usize)..bytes.len()) };
            continue;
        }
        // SAFETY: the chunk contains `outlen` initialized bytes, so we can create a slice from it.
        let chunk = bytedata::ByteChunk::from_slice(unsafe {
            bytedata::const_slice_unchecked(&chunk, 0..outlen)
        });
        return crate::EncodeResult::Chunk(chunk, consumed as u16);
    }
}

#[must_use]
#[expect(clippy::too_many_lines)]
const unsafe fn decode_const_inner(bytes: *const u8, maxlen: usize) -> crate::DecodeResult {
    let has_end = maxlen > 0;
    let mut utflen = 0;
    loop {
        let data = {
            // SAFETY: The pointer is valid and has been read up to this point
            let bytes_offset = unsafe { bytes.add(utflen) };
            if has_end {
                // SAFETY: The pointer is valid and the length is assumed to be correct.
                unsafe { core::slice::from_raw_parts(bytes_offset, maxlen - utflen) }
            } else {
                // SAFETY: The pointer is valid and the length is assumed to be correct, though it may actually
                unsafe { core::slice::from_raw_parts(bytes_offset, 4) }
            }
        };
        let (charcode, charlen) = bytedata::const_utf8_char_next(data);
        if charcode == 0 {
            if charlen == 0 {
                if utflen != 0 {
                    return crate::DecodeResult::Utf8(utflen as u64);
                }
                if has_end {
                    return crate::DecodeResult::Empty;
                }
                return crate::DecodeResult::InvalidChar(data[0] as u32, 1);
            }
            if charlen == 1 {
                if has_end {
                    utflen += 1;
                    continue;
                }
                if utflen != 0 {
                    return crate::DecodeResult::Utf8(utflen as u64);
                }
                return crate::DecodeResult::Empty;
            }
            if utflen != 0 {
                return crate::DecodeResult::Utf8(utflen as u64);
            }
            return crate::DecodeResult::Char('\0', charlen);
        }
        if charcode < 0x80 {
            if charlen == 1 {
                utflen += 1;
                continue;
            }
            if utflen != 0 {
                return crate::DecodeResult::Utf8(utflen as u64);
            }
            // SAFETY: `a` is a valid ASCII character, so we can create a char from it.
            return crate::DecodeResult::Char(
                unsafe { char::from_u32_unchecked(charcode) },
                charlen,
            );
        }
        // process two-byte characters
        if charcode < 0x0800 {
            if charlen == 2 {
                utflen += 2;
                continue;
            }
            if utflen != 0 {
                return crate::DecodeResult::Utf8(utflen as u64);
            }
            // SAFETY: `a` is a valid low character, so we can create a char from it.
            return crate::DecodeResult::Char(
                unsafe { char::from_u32_unchecked(charcode) },
                charlen,
            );
        }
        // 4-byte characters should never occur in Java modified UTF-8, but we handle them anyway
        if charcode >= 0x10000 {
            if charlen == 4 {
                utflen += 4;
                continue;
            }
            if utflen != 0 {
                return crate::DecodeResult::Utf8(utflen as u64);
            }
            // SAFETY: `a` is a valid high character, so we can create a char from it.
            return crate::DecodeResult::Char(
                unsafe { char::from_u32_unchecked(charcode) },
                charlen,
            );
        }
        // process 3-byte characters
        if charcode < 0xD800 || charcode >= 0xE000 {
            if charlen == 3 {
                utflen += 3;
                continue;
            }
            if utflen != 0 {
                return crate::DecodeResult::Utf8(utflen as u64);
            }
            // SAFETY: `a` is a valid mid character, so we can create a char from it.
            return crate::DecodeResult::Char(
                unsafe { char::from_u32_unchecked(charcode) },
                charlen,
            );
        }
        if utflen != 0 {
            return crate::DecodeResult::Utf8(utflen as u64);
        }
        if charcode & 0xFC00 == 0xD800 {
            let maxrest = if has_end {
                maxlen - utflen - charlen as usize
            } else {
                4
            };
            // SAFETY: The pointer is valid and the length is assumed to be correct.
            let bytes_offset = unsafe { bytes.add(utflen + charlen as usize) };
            // SAFETY: The pointer is valid and the length is assumed to be correct.
            let (ap, bp) = bytedata::const_utf8_char_next(unsafe {
                core::slice::from_raw_parts(bytes_offset, maxrest)
            });
            if ap & 0xFFFF_FC00 == 0xDC00 {
                // decode surrogate pair
                let low = ap & 0x3FF;
                let high = charcode & 0x3FF;
                // SAFETY: `high` and `low` are valid surrogate pairs, so we can create a char from them.
                let ch = unsafe { char::from_u32_unchecked(((high << 10) | low) + 0x10000) };
                return crate::DecodeResult::Char(ch, charlen + bp);
            }
        }
        return crate::DecodeResult::InvalidChar(charcode, charlen);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const COMPLEX: &str = "test-€𐍈-🐷🫶";
    const COMPLEX_LEN: usize = COMPLEX.len();
    const COMPLEX_EXPECTED: &[u8] = b"test-\xe2\x82\xac\xed\xa0\x80\xed\xbd\x88-\xed\xa0\xbd\xed\xb0\xb7\xed\xa0\xbe\xed\xbb\xb6";

    #[test]
    #[expect(clippy::cast_possible_truncation)]
    fn test_java_mutf8() {
        {
            // SAFETY: real cstr
            let res =
                unsafe { JavaModifiedUtf8Encoding::decode_cstr(c"a simple ascii test".as_ptr()) };
            assert_eq!(res, crate::DecodeResult::Utf8(19));
        };
        {
            let res = JAVA_MUTF_8.decode_const(b"\0\0\0\0");
            assert_eq!(res, crate::DecodeResult::Utf8(4));
        };
        let mut buff = bytedata::SharedBytesBuilder::with_capacity(32);
        #[expect(clippy::cast_possible_truncation)]
        {
            let res = JavaModifiedUtf8Encoding::encode_into(COMPLEX, &mut buff);
            assert_eq!(
                res,
                crate::result::ExhaustiveEncodeResult::Encoded((
                    COMPLEX_LEN as u32,
                    COMPLEX_EXPECTED.len() as u32
                ))
            );
            assert!(
                buff.as_ref() == COMPLEX_EXPECTED,
                "Encoding failed: {:?}",
                buff.as_ref()
            );
            assert_eq!(buff.len(), COMPLEX_EXPECTED.len());
            buff.extend_from_slice(b"\0");
        };
        let mut strbuff = bytedata::SharedStrBuilder::with_capacity(32);
        {
            // SAFETY: The pointer is valid cstr.
            let res = unsafe {
                JavaModifiedUtf8Encoding::decode_cstr_into(buff.as_ptr().cast::<i8>(), &mut strbuff)
            };
            assert_eq!(
                res,
                crate::result::ExhaustiveDecodeResult::Decoded((
                    (buff.len() - 1) as u32,
                    COMPLEX_LEN as u32
                ))
            );
            assert_eq!(strbuff.as_str(), COMPLEX);
        }
    }
}
