//! ## ASCII-7 compatible charset encoding
//! 
//! This module provides the base for an 8-bit ASCII-7 compatible charset encoder.
//! See the [`SingleByteEncoding`] struct for more information.


/// An ASCII-7 compatible charset encoder.
/// 
/// This struct provides the base for an 8-bit ASCII-7 compatible charset encoder.
/// As the 128 first characters are the same as ASCII-7, this needs to map the 128 extended characters to the corresponding unicode character.
/// A list of the extended characters is provided as a static array to the [`SingleByteEncoding::new`] function to allow this charset to substitute the bytes and characters.
/// 
/// If the encoding you are implementing does not have the full set of 128 extended characters defined, you can use the `'\0'` value to trigger [`DecodeResult::InvalidChar`] for all missing characters except for at position `0`.
/// 
/// *The feature guarded ISO-8859 and windows charsets in this crate internally uses `SingleByteEncoding` to provide their charset encoding.*
/// 
/// [`DecodeResult::InvalidChar`]: crate::DecodeResult::InvalidChar
#[cfg_attr(docsrs, doc(cfg(feature = "single-byte")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SingleByteEncoding {
    chars: &'static [char; 256],
    name: &'static str,
}

impl SingleByteEncoding {
    /// Create a new ASCII-compatible charset encoding instance.
    #[inline]
    #[must_use]
    pub const fn new(name: &'static str, chars: &'static [char; 256]) -> Self {
        Self { chars, name }
    }
    
    /// Decode characters from the given bytes.
    #[inline]
    #[must_use]
    pub const fn decode_const(&self, bytes: &[u8]) -> crate::DecodeResult {
        if bytes.is_empty() {
            return crate::DecodeResult::Empty;
        }

        // some sprinkled characters, such as digits and space, may be in ASCII position.
        let mut i = 0;
        while i < bytes.len() {
            let bval = bytes[i] as u32;
            if bval < 128 && bval == self.chars[bval as usize] as u32 {
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
    #[expect(clippy::missing_const_for_fn)]
    pub fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        self.decode_const(bytes)
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
            let bval = bytes[i] as u32;
            if bval < 128 && bval == self.chars[bval as usize] as u32 {
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
    #[expect(clippy::missing_const_for_fn)]
    pub fn encode(&self, chars: &str) -> crate::EncodeResult {
        self.encode_const(chars)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "single-byte")))]
impl crate::Charset for SingleByteEncoding {
    const CHARSET_NAME: &'static str = "single-byte";
    #[inline]
    fn charset_name(&self) -> &'static str {
        self.name
    }

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 1)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "single-byte")))]
impl crate::CharsetDecoding for SingleByteEncoding {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode_const(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "single-byte")))]
impl crate::CharsetEncoding for SingleByteEncoding {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode_const(self, chars)
    }
}

#[inline]
const fn decode_const_inner(chars: &[char; 256], byte: u8) -> crate::DecodeResult {
    let by = byte as usize;
    // SAFETY: The index is always in bounds.
    let ptr = unsafe { chars.as_ptr().add(by) };
    // SAFETY: The pointer is always valid.
    let ch = unsafe { ptr.read() };
    if ch == '\0' && byte != 0 {
        crate::DecodeResult::InvalidChar(byte as u32, 1)
    } else {
        crate::DecodeResult::Char(ch, 1)
    }
}

const fn encode_const_inner(chars: &'static [char; 256], data: &str) -> crate::EncodeResult {
    /// Find a character in the extended charset. Returns `0` if not found or if the value was actually `0`.
    #[inline]
    const fn find_char(chars: &'static [char; 256], ch: u32) -> u8 {
        let mut i = 0;
        while i < 256 {
            if chars[i] as u32 != ch {
                i += 1;
                continue;
            }
            #[expect(clippy::cast_possible_truncation)]
            return i as u8;
        }
        0
    }
    let mut buf = [0_u8; 14];
    let mut consumed = 0;
    let mut buff_off = 0;
    let mut chardata = data;
    loop {
        let (cp, b_len) = bytedata::const_utf8_char_next(chardata.as_bytes());
        if b_len == 0 {
            if chardata.is_empty() || consumed != 0 {
                break;
            }
            return crate::EncodeResult::Incomplete;
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
        buf[buff_off] = found;
        buff_off += 1;
        consumed += b_len;
        if buff_off != buf.len() {
            chardata = match bytedata::const_slice_str(chardata, (b_len as usize)..chardata.len()) {
                bytedata::StrSliceResult::OutOfBounds | bytedata::StrSliceResult::InvalidUtf8 => break,
                bytedata::StrSliceResult::Success(x) => x,
            };
            continue;
        }
        break;
    }
    if buff_off == 0 {
        return crate::EncodeResult::Empty;
    }
    let slice = bytedata::const_or_bytes(bytedata::const_slice(buf.as_slice(), 0..buff_off), b"");
    #[expect(clippy::cast_possible_truncation)]
    let consumed = consumed as u16;
    crate::EncodeResult::Chunk(bytedata::ByteChunk::from_slice(slice), consumed)
}
