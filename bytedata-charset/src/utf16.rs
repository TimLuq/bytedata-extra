use core::mem::MaybeUninit;

use crate::{CharsetDecoding, CharsetEndian, DecodeResult};

/// An encoding for UTF-16LE and UTF-16BE.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Utf16Encoding(CharsetEndian);

/// UTF-16 little endian decoder.
pub static UTF16_LE: Utf16Encoding = Utf16Encoding::UTF16_LE;
/// UTF-16 big endian decoder.
pub static UTF16_BE: Utf16Encoding = Utf16Encoding::UTF16_BE;

impl Utf16Encoding {
    /// UTF-16 little endian decoder.
    pub const UTF16_LE: Self = Self::new(CharsetEndian::Little);
    /// UTF-16 big endian decoder.
    pub const UTF16_BE: Self = Self::new(CharsetEndian::Big);
    /// UTF-16 decoder with system endianness.
    pub const UTF16_NATIVE: Self = Self::new(CharsetEndian::NATIVE);

    /// Create a new UTF-16 decoder with the specified endianness.
    #[inline]
    #[must_use]
    pub const fn new(endian: CharsetEndian) -> Self {
        Self(endian)
    }

    /// Decode a UTF-16 byte sequence.
    #[must_use]
    #[inline]
    pub const fn decode_const(self, bytes: &[u8]) -> DecodeResult {
        if bytes.is_empty() {
            return DecodeResult::Empty;
        }
        if bytes.len() == 1 {
            return DecodeResult::Incomplete;
        }
        self.decode_const_inner(bytes)
    }

    /// Decode a UTF-16 byte sequence.
    #[must_use]
    #[inline]
    pub fn decode(self, bytes: &[u8]) -> DecodeResult {
        if bytes.is_empty() {
            return DecodeResult::Empty;
        }
        if bytes.len() == 1 {
            return DecodeResult::Incomplete;
        }
        if self.0.is_native() && (bytes.as_ptr() as usize) & 1 == 0 {
            // SAFETY: bytes is a valid pointer to u16
            #[expect(clippy::cast_ptr_alignment)]
            let native = unsafe {
                core::slice::from_raw_parts(bytes.as_ptr().cast::<u16>(), bytes.len() >> 1)
            };
            Self::decode_native_inner_16(native)
        } else {
            self.decode_const_inner(bytes)
        }
    }

    /// Decode a UTF-16 sequence.
    #[must_use]
    #[inline]
    pub const fn decode_native_const(native: &[u16]) -> DecodeResult {
        if native.is_empty() {
            return DecodeResult::Empty;
        }
        if native.len() == 1 {
            return DecodeResult::Incomplete;
        }
        Self::decode_native_inner_16(native)
    }

    /// Decode a UTF-16 sequence into a `bytedata::SharedStrBuilder`.
    /// The `Decoded` result variant contains the number of bytes consumed.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    #[inline]
    pub fn decode_native_into(
        seq: &[u16],
        chars: &mut bytedata::SharedStrBuilder,
    ) -> crate::result::ExhaustiveDecodeResult<usize> {
        if seq.is_empty() {
            return crate::result::ExhaustiveDecodeResult::Decoded(0);
        }
        Self::decode_native_into_inner(seq, chars)
    }

    #[cfg(feature = "alloc")]
    fn decode_native_into_inner(
        mut seq: &[u16],
        chars: &mut bytedata::SharedStrBuilder,
    ) -> crate::result::ExhaustiveDecodeResult<usize> {
        let mut consumed = 0;
        while !seq.is_empty() {
            let res = Self::decode_native_inner_16(seq);
            match res {
                DecodeResult::Char(ch, len) => {
                    let len = len as usize >> 1_i32;
                    chars.push(ch);
                    consumed += len;
                    // SAFETY: the length is returned from the decode function
                    seq = unsafe { seq.get_unchecked(len..) };
                }
                _ if consumed != 0 => {
                    return crate::result::ExhaustiveDecodeResult::Decoded(consumed);
                }
                DecodeResult::InvalidChar(base, len) => {
                    return crate::result::ExhaustiveDecodeResult::InvalidChar(base, len >> 1);
                }
                DecodeResult::Incomplete => {
                    return crate::result::ExhaustiveDecodeResult::Incomplete;
                }
                DecodeResult::Empty => {
                    return crate::result::ExhaustiveDecodeResult::Empty;
                }
                DecodeResult::Utf8(_len) => {
                    unreachable!("UTF-8 is not supported in this function");
                }
            }
        }
        crate::result::ExhaustiveDecodeResult::Decoded(consumed)
    }

    /// Decode a UTF-16 byte sequence. This function assumes that the input is at least 2 bytes long.
    const fn decode_const_inner(self, bytes: &[u8]) -> DecodeResult {
        let len = bytes.len();
        debug_assert!(len >= 2, "The input must be at least 2 bytes long.");
        let len = len >> 1_u8;
        #[expect(clippy::cast_ptr_alignment)]
        let bytes = bytes.as_ptr().cast::<u16>();
        // SAFETY: bytes is a valid pointer to u16, though it may be unaligned
        let base = unsafe { bytes.read_unaligned() };
        let base = match self.0 {
            CharsetEndian::Big => base.to_be() as u32,
            CharsetEndian::Little => base.to_le() as u32,
        };
        if base < 0xD800 || base >= 0xE000 {
            // SAFETY: base is a valid char code point, transmuting to char is safe
            return DecodeResult::Char(unsafe { ::core::mem::transmute::<u32, char>(base) }, 2);
        }
        if base >= 0xDC00 {
            return DecodeResult::InvalidChar(base, 2);
        }
        if len < 2 {
            return DecodeResult::Incomplete;
        }

        // SAFETY: length is checked above
        let bytes = unsafe { bytes.add(1) };
        // SAFETY: bytes is a valid pointer to u16, though it may be unaligned
        let cont = unsafe { bytes.read_unaligned() };
        let cont = match self.0 {
            CharsetEndian::Big => cont.to_be() as u32,
            CharsetEndian::Little => cont.to_le() as u32,
        };
        if cont < 0xDC00 || cont >= 0xE000 {
            return DecodeResult::InvalidChar(base, 2);
        }
        let base = 0x1_0000 + (((base & 0x03FF) << 10_u32) | (cont & 0x03FF));
        match char::from_u32(base) {
            Some(ch) => DecodeResult::Char(ch, 4),
            None => DecodeResult::InvalidChar(base, 4),
        }
    }

    /// Convert a Latin-1 byte sequence to a UTF-16 slice.
    #[inline]
    pub const fn from_latin1_const(latin1: &[u8], utf16: &mut [u16]) -> usize {
        let len = min_usize(latin1.len(), utf16.len());
        if len == 0 {
            return 0;
        }
        // SAFETY: latin1 is a valid pointer to u8 and utf16 is a valid pointer to u16, and the length is checked
        unsafe { from_latin1_const_raw(latin1.as_ptr(), utf16.as_mut_ptr(), len) };
        len
    }

    /// Convert a Latin-1 byte sequence to a UTF-16 slice.
    #[inline]
    pub fn from_latin1(latin1: &[u8], utf16: &mut [u16]) -> usize {
        let len = min_usize(latin1.len(), utf16.len());
        if len == 0 {
            return 0;
        }
        let u8ptr = latin1.as_ptr();
        let u16ptr = utf16.as_mut_ptr();
        #[cfg(all(target_arch = "x86_64", feature = "avx"))]
        if len >= 16 && is_x86_feature_detected!("avx2") {
            // SAFETY: u8ptr is a valid pointer to u8 and u16ptr is a valid pointer to u16, and the length is checked
            unsafe { from_latin1_avx2(u8ptr, u16ptr, len) };
            return len;
        }
        // SAFETY: u8ptr is a valid pointer to u8 and u16ptr is a valid pointer to u16, and the length is checked
        unsafe { from_latin1_const_raw(u8ptr, u16ptr, len) };
        len
    }

    /// Convert a Latin-1 byte sequence to a UTF-16 slice.
    #[inline]
    pub fn from_latin1_uninit<'t>(latin1: &[u8], utf16: &'t mut [MaybeUninit<u16>]) -> &'t mut [u16] {
        let len = min_usize(latin1.len(), utf16.len());
        let u16ptr = utf16.as_mut_ptr().cast::<u16>();
        if len == 0 {
            // SAFETY: u16ptr is a valid pointer to u16
            return unsafe { core::slice::from_raw_parts_mut(u16ptr, 0) };
        }
        let u8ptr = latin1.as_ptr();
        #[cfg(all(target_arch = "x86_64", feature = "avx"))]
        if len >= 16 && is_x86_feature_detected!("avx2") {
            // SAFETY: u8ptr is a valid pointer to u8 and u16ptr is a valid pointer to u16, and the length is checked
            unsafe { from_latin1_avx2(u8ptr, u16ptr, len) };
            // SAFETY: u16ptr is a valid pointer to u16 and the length is checked
            return unsafe { core::slice::from_raw_parts_mut(u16ptr, len) };
        }
        // SAFETY: u8ptr is a valid pointer to u8 and u16ptr is a valid pointer to u16, and the length is checked
        unsafe { from_latin1_const_raw(u8ptr, u16ptr, len) };
        // SAFETY: u16ptr is a valid pointer to u16 and the length is checked
        unsafe { core::slice::from_raw_parts_mut(u16ptr, len) }
    }


    /// Decode a UTF-16 native sequence. This function assumes that the input is at least one u16 long.
    const fn decode_native_inner_16(bytes: &[u16]) -> DecodeResult {
        let len = bytes.len();
        debug_assert!(len >= 1, "The input must be at least 1 u16 long.");
        let bytes = bytes.as_ptr();
        // SAFETY: bytes is a valid pointer to u16, though it may be unaligned
        let base = unsafe { bytes.read() } as u32;
        if base < 0xD800 || base >= 0xE000 {
            // SAFETY: base is a valid char code point, transmuting to char is safe
            return DecodeResult::Char(unsafe { ::core::mem::transmute::<u32, char>(base) }, 2);
        }
        if base >= 0xDC00 {
            return DecodeResult::InvalidChar(base, 2);
        }
        if len < 2 {
            return DecodeResult::Incomplete;
        }

        // SAFETY: length is checked above
        let bytes = unsafe { bytes.add(1) };
        // SAFETY: bytes is a valid pointer to u16, though it may be unaligned
        let cont = unsafe { bytes.read() } as u32;
        if cont < 0xDC00 || cont >= 0xE000 {
            return DecodeResult::InvalidChar(base, 2);
        }
        let base = 0x1_0000 + (((base & 0x03FF) << 10_u32) | (cont & 0x03FF));
        match char::from_u32(base) {
            Some(ch) => DecodeResult::Char(ch, 4),
            None => DecodeResult::InvalidChar(base, 4),
        }
    }

    /// Detect the if the bytes are UTF-16 encoded.
    #[inline]
    #[must_use]
    pub const fn detect_const(self, bytes: &[u8]) -> crate::detect::DetectionResult {
        match self.0 {
            CharsetEndian::Big => detect_be(bytes),
            CharsetEndian::Little => detect_le(bytes),
        }
    }

    /// Encode UTF-16 characters.
    ///
    /// Returns the number of utf-8 bytes consumed.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    #[allow(clippy::allow_attributes)]
    #[allow(clippy::missing_inline_in_public_items)]
    pub fn encode_into(
        self,
        chars: &str,
        bytes: &mut bytedata::SharedBytesBuilder,
    ) -> crate::ExhaustiveEncodeResult<usize> {
        let mut len = 0;
        let mut chars = chars;
        while !chars.is_empty() {
            match self.encode_inner(chars) {
                crate::EncodeResult::Chunk(chunk, consumed) => {
                    let consumed = consumed as usize;
                    let end = bytes.len() + chunk.len();
                    if end > 0xFFFF_FFF0 {
                        if len != 0 {
                            return crate::ExhaustiveEncodeResult::Encoded(len);
                        }
                        return crate::ExhaustiveEncodeResult::Overflow;
                    }
                    bytes.extend_from_slice(&chunk);
                    len += consumed;
                    // SAFETY: the length is returned from the encode function
                    chars = unsafe { chars.get_unchecked(consumed..) };
                }
                _ if len != 0 => {
                    return crate::ExhaustiveEncodeResult::Encoded(len);
                }
                crate::EncodeResult::InvalidChar(ch, len_chunk) => {
                    return crate::ExhaustiveEncodeResult::InvalidChar(ch, len_chunk);
                }
                crate::EncodeResult::Incomplete => {
                    return crate::ExhaustiveEncodeResult::Incomplete;
                }
                crate::EncodeResult::Empty => {
                    return crate::ExhaustiveEncodeResult::Empty;
                }
                crate::EncodeResult::Utf8(_len) => {
                    unreachable!("UTF-8 is not supported in this function");
                }
            }
        }
        crate::ExhaustiveEncodeResult::Encoded(len)
    }

    /// Encode UTF-16 characters.
    ///
    /// Returns the number of utf-8 bytes consumed.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    #[allow(clippy::allow_attributes)]
    #[allow(clippy::missing_inline_in_public_items)]
    pub fn encode_native_into_vec(
        chars: &str,
        target: &mut alloc::vec::Vec<u16>,
    ) -> crate::ExhaustiveEncodeResult<usize> {
        let mut len = 0;
        let mut chars = chars;
        while !chars.is_empty() {
            match Self::UTF16_NATIVE.encode_inner(chars) {
                crate::EncodeResult::Chunk(chunk, consumed) => {
                    let consumed = consumed as usize;
                    let chunk = chunk.as_slice();
                    debug_assert!(
                        chunk.len() % 2 == 0,
                        "The chunk length must be a multiple of 2 bytes."
                    );
                    let clen = chunk.len() >> 1_u8;
                    target.reserve(clen);
                    #[allow(clippy::cast_ptr_alignment)]
                    let src = chunk.as_ptr();
                    // SAFETY: trg is a valid pointer to u16 that is at least `clen` u16 long
                    let trg = unsafe { target.as_mut_ptr().add(target.len()) };
                    let trg = trg.cast::<u8>();
                    // SAFETY: trg is a valid pointer to u8 that is at least `clen * 2` bytes long due to reserve
                    unsafe { trg.copy_from_nonoverlapping(src, chunk.len()) };
                    // SAFETY: `clen` number of u16 elements are written to the vector
                    unsafe { target.set_len(target.len() + clen) };
                    len += consumed;
                    // SAFETY: the length is returned from the encode function
                    chars = unsafe { chars.get_unchecked(consumed..) };
                }
                _ if len != 0 => {
                    return crate::ExhaustiveEncodeResult::Encoded(len);
                }
                crate::EncodeResult::InvalidChar(ch, len_chunk) => {
                    return crate::ExhaustiveEncodeResult::InvalidChar(ch, len_chunk);
                }
                crate::EncodeResult::Incomplete => {
                    return crate::ExhaustiveEncodeResult::Incomplete;
                }
                crate::EncodeResult::Empty => {
                    return crate::ExhaustiveEncodeResult::Empty;
                }
                crate::EncodeResult::Utf8(_len) => {
                    unreachable!("UTF-8 is not supported in this function");
                }
            }
        }
        crate::ExhaustiveEncodeResult::Encoded(len)
    }

    /// Encode UTF-16 characters.
    ///
    /// Returns the number of utf-8 bytes consumed and the number of `u16`s written as `(consumed, written)`.
    #[allow(clippy::allow_attributes)]
    #[allow(clippy::wildcard_enum_match_arm)]
    #[inline]
    pub fn encode_native_into_slice(
        chars: &str,
        slice: &mut [u16],
    ) -> crate::ExhaustiveEncodeResult<(usize, usize)> {
        let ptr = slice.as_mut_ptr().cast::<u8>();
        // SAFETY: slice is a valid pointer to u16 slice, so it is also a valid pointer to an u8 slice that is twice as long
        let bytes = unsafe { core::slice::from_raw_parts_mut(ptr, slice.len() << 1_u8) };
        match Self::UTF16_NATIVE.encode_into_slice_u8(chars, bytes) {
            crate::ExhaustiveEncodeResult::Encoded((consumed, written)) => {
                let written = written >> 1_u8;
                crate::ExhaustiveEncodeResult::Encoded((consumed, written))
            }
            other => other,
        }
    }

    /// Encode UTF-16 characters.
    ///
    /// Returns the number of utf-8 bytes consumed and the number of bytes written as `(consumed, written)`.
    #[allow(clippy::allow_attributes)]
    #[allow(clippy::missing_inline_in_public_items)]
    pub fn encode_into_slice_u8(
        self,
        chars: &str,
        slice: &mut [u8],
    ) -> crate::ExhaustiveEncodeResult<(usize, usize)> {
        let mut build = 0;
        let mut len = 0;
        let mut chars = chars;
        while !chars.is_empty() {
            match self.encode_inner(chars) {
                crate::EncodeResult::Chunk(chunk, consumed) => {
                    let chunk = chunk.as_slice();
                    let end = build + chunk.len();
                    if end > slice.len() {
                        if len != 0 {
                            return crate::ExhaustiveEncodeResult::Encoded((len, build));
                        }
                        return crate::ExhaustiveEncodeResult::Overflow;
                    }
                    let consumed = consumed as usize;
                    // SAFETY: slice is a valid pointer to u8 and has enough space for the chunk
                    let trg = unsafe { slice.as_mut_ptr().add(build) };
                    // SAFETY: trg is a valid pointer to u8 that is at least `chunk.len()` bytes long
                    unsafe { trg.copy_from_nonoverlapping(chunk.as_ptr(), chunk.len()) };
                    build = end;
                    len += consumed;
                    // SAFETY: the length is returned from the encode function
                    chars = unsafe { chars.get_unchecked(consumed..) };
                }
                _ if len != 0 => {
                    return crate::ExhaustiveEncodeResult::Encoded((len, build));
                }
                crate::EncodeResult::InvalidChar(ch, len_chunk) => {
                    return crate::ExhaustiveEncodeResult::InvalidChar(ch, len_chunk);
                }
                crate::EncodeResult::Incomplete => {
                    return crate::ExhaustiveEncodeResult::Incomplete;
                }
                crate::EncodeResult::Empty => {
                    return crate::ExhaustiveEncodeResult::Empty;
                }
                crate::EncodeResult::Utf8(_len) => {
                    unreachable!("UTF-8 is not supported in this function");
                }
            }
        }
        crate::ExhaustiveEncodeResult::Encoded((len, build))
    }

    /// Encode UTF-16 characters. `chars` must not be empty.
    fn encode_inner(self, chars: &str) -> crate::EncodeResult {
        let mut buf = [0_u16; 7];
        let mut buf_use = 0;
        for (idx, ch) in chars.char_indices() {
            if buf_use == 7 {
                // SAFETY: buf is a valid UTF-16 sequence and encoded according to the endianness
                let buf = unsafe { core::mem::transmute::<[u16; 7], [u8; 14]>(buf) };
                #[expect(clippy::cast_possible_truncation)]
                return crate::EncodeResult::Chunk(
                    bytedata::ByteChunk::from_array(&buf),
                    idx as u16,
                );
            }
            let ch = ch as u32;
            if ch < 0xD800 || (0xE000..0x1_0000).contains(&ch) {
                buf[buf_use] = match self.0 {
                    #[expect(clippy::cast_possible_truncation)]
                    CharsetEndian::Big => (ch as u16).to_be(),
                    #[expect(clippy::cast_possible_truncation)]
                    CharsetEndian::Little => (ch as u16).to_le(),
                };
                buf_use += 1;
                continue;
            }
            if buf_use == 6 {
                // SAFETY: buf is a valid UTF-16 sequence and encoded according to the endianness
                let buf = unsafe { core::mem::transmute::<[u16; 7], [u8; 14]>(buf) };
                #[expect(clippy::cast_possible_truncation)]
                return crate::EncodeResult::Chunk(
                    bytedata::ByteChunk::from_slice(&buf[0..12]),
                    idx as u16,
                );
            }
            let ch = ch - 0x1_0000;
            buf[buf_use] = match self.0 {
                #[expect(clippy::cast_possible_truncation)]
                CharsetEndian::Big => ((ch >> 10) as u16 | 0xD800).to_be(),
                #[expect(clippy::cast_possible_truncation)]
                CharsetEndian::Little => ((ch >> 10) as u16 | 0xD800).to_le(),
            };
            buf[buf_use + 1] = match self.0 {
                CharsetEndian::Big => ((ch & 0x3FF) as u16 | 0xDC00).to_be(),
                CharsetEndian::Little => ((ch & 0x3FF) as u16 | 0xDC00).to_le(),
            };
            buf_use += 2;
        }
        if buf_use == 0 {
            return crate::EncodeResult::Empty;
        }
        // SAFETY: buf is a valid UTF-16 sequence and encoded according to the endianness
        let buf = unsafe { core::mem::transmute::<[u16; 7], [u8; 14]>(buf) };
        #[expect(clippy::cast_possible_truncation)]
        crate::EncodeResult::Chunk(
            bytedata::ByteChunk::from_slice(&buf[0..(buf_use << 1)]),
            chars.len() as u16,
        )
    }
}

impl crate::Charset for Utf16Encoding {
    const CHARSET_NAME: &'static str = "utf-16";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (2, 4)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        match self.0 {
            CharsetEndian::Big => ["utf-16be", "unicodefffe", "csutf16be"].as_slice(),
            CharsetEndian::Little => [
                "utf-16le",
                "utf-16",
                "unicodefeff",
                "unicode",
                "ucs-2",
                "iso-10646-ucs-2",
                "csunicode",
                "csutf16le",
                "csutf16",
            ]
            .as_slice(),
        }
    }
}

impl CharsetDecoding for Utf16Encoding {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> DecodeResult {
        Self::decode(*self, bytes)
    }
}

impl crate::CharsetEncoding for Utf16Encoding {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        let len = chars.len();
        if len == 0 {
            return crate::EncodeResult::Empty;
        }
        self.encode_inner(chars)
    }
}

#[expect(clippy::missing_asserts_for_indexing)]
const fn detect_be(bytes: &[u8]) -> crate::detect::DetectionResult {
    let len = bytes.len();
    // Check for BOM
    if len < 2 {
        return crate::detect::DetectionResult::Incomplete;
    }
    if bytes[0] == 0xFE && bytes[1] == 0xFF {
        return crate::detect::DetectionResult::Certain;
    }

    // when no BOM is present, check for bytes matching UTF-16BE
    if len < 4 {
        return crate::detect::DetectionResult::Incomplete;
    }
    let mut i = 0;
    while i + 1 < len {
        if bytes[i] < 0xD8 || bytes[i] >= 0xE0 {
            i += 2;
            continue;
        }
        if bytes[i] >= 0xDC {
            return crate::detect::DetectionResult::Irrelevant;
        }
        if i + 2 >= len {
            break;
        }
        if bytes[i + 2] < 0xDC || bytes[i + 2] >= 0xE0 {
            return crate::detect::DetectionResult::Irrelevant;
        }
        if i + 3 >= len {
            break;
        }
        i += 4;
    }
    if i >= 4 {
        crate::detect::DetectionResult::Tentative
    } else {
        crate::detect::DetectionResult::Incomplete
    }
}

#[expect(clippy::missing_asserts_for_indexing)]
const fn detect_le(bytes: &[u8]) -> crate::detect::DetectionResult {
    let len = bytes.len();
    // Check for BOM
    if len < 2 {
        return crate::detect::DetectionResult::Incomplete;
    }
    if bytes[0] == 0xFF && bytes[1] == 0xFE {
        return crate::detect::DetectionResult::Certain;
    }

    // when no BOM is present, check for bytes matching UTF-16LE
    if len < 4 {
        return crate::detect::DetectionResult::Incomplete;
    }
    let mut i = 0;
    while i + 1 < len {
        if bytes[i + 1] < 0xD8 || bytes[i + 1] >= 0xE0 {
            i += 2;
            continue;
        }
        if bytes[i] >= 0xDC {
            return crate::detect::DetectionResult::Irrelevant;
        }
        if i + 2 >= len {
            break;
        }
        if bytes[i + 2] < 0xDC || bytes[i + 2] >= 0xE0 {
            return crate::detect::DetectionResult::Irrelevant;
        }
        if i + 3 >= len {
            break;
        }
        i += 4;
    }
    if i >= 4 {
        crate::detect::DetectionResult::Tentative
    } else {
        crate::detect::DetectionResult::Incomplete
    }
}

impl crate::detect::CharsetDetector for Utf16Encoding {
    #[inline]
    fn detect(&self, bytes: &[u8]) -> crate::detect::DetectionResult {
        match self.0 {
            CharsetEndian::Big => detect_be(bytes),
            CharsetEndian::Little => detect_le(bytes),
        }
    }
}

/// A UTF-16 character.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Utf16CodeUnit(u16);

impl Utf16CodeUnit {
    /// Create a new UTF-16 character.
    #[must_use]
    #[inline]
    pub const fn new(ch: u16) -> Self {
        Self(ch)
    }
    /// Create a new UTF-16 character from a little endian u16.
    #[must_use]
    #[inline]
    pub const fn from_le(ch: u16) -> Self {
        Self(ch.to_le())
    }
    /// Create a new UTF-16 character from a big endian u16.
    #[must_use]
    #[inline]
    pub const fn from_be(ch: u16) -> Self {
        Self(ch.to_be())
    }
    /// Encodes the character as a little endian u16.
    #[must_use]
    #[inline]
    pub const fn to_le(self) -> u16 {
        self.0.to_le()
    }
    /// Encodes the character as a big endian u16.
    #[must_use]
    #[inline]
    pub const fn to_be(self) -> u16 {
        self.0.to_be()
    }
    /// Converts the character to a char if it is a valid code point.
    /// Returns `None` if the character is part of a surrogate pair.
    #[must_use]
    #[inline]
    pub const fn to_char(self) -> Option<char> {
        if self.0 < 0xD800 || self.0 >= 0xE000 {
            // SAFETY: self.0 is a valid char code point
            Some(unsafe { core::mem::transmute::<u32, char>(self.0 as u32) })
        } else {
            None
        }
    }
    /// Converts the character to one or two UTF-16 characters.
    ///
    /// # Errors
    ///
    /// Returns an error if the character is part of a surrogate pair.
    #[inline]
    pub const fn from_char(ch: char) -> Result<Self, [Self; 2]> {
        let ch = ch as u32;
        #[expect(clippy::cast_possible_truncation)]
        if ch <= 0xFFFF {
            Ok(Self::new(ch as u16))
        } else {
            let ch = ch - 0x1_0000;
            let buff = [((ch >> 10) | 0xD800) as u16, ((ch & 0x3FF) | 0xDC00) as u16];
            // SAFETY: buff is a valid UTF-16 sequence that is transparent over u16.
            Err(unsafe { core::mem::transmute::<[u16; 2], [Self; 2]>(buff) })
        }
    }
}

/// An error that can occur when decoding specifically a UTF-16 sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Utf16Error {
    /// The input is not a valid UTF-16 sequence since all sequences should be multiples of `u16`.
    NotEvenLength,
    /// The input should be aligned to 2 bytes.
    NotAligned,
    /// The input ends with a high surrogate when expecting a low surrogate to follow.
    Incomplete,
    /// Expected something that is not a low surrogate, but found one.
    UnexpectedLow(usize),
    /// Expected a low surrogate, but found something else.
    ExpectedLow(usize),
}

impl core::fmt::Display for Utf16Error {
    #[inline]
    #[expect(clippy::min_ident_chars)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::NotEvenLength => write!(f, "The input is not a valid UTF-16 sequence since all sequences should be multiples of `u16`."),
            Self::NotAligned => write!(f, "The input should be aligned to 2 bytes."),
            Self::Incomplete => write!(f, "The input ends with a high surrogate when expecting a low surrogate to follow."),
            Self::UnexpectedLow(idx) => write!(f, "Expected something that is not a low surrogate, but found one at index {idx}."),
            Self::ExpectedLow(idx) => write!(f, "Expected a low surrogate, but found something else at index {idx}."),
        }
    }
}

impl core::error::Error for Utf16Error {}

/// A UTF-16 string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Utf16String<'a> {
    bytes: bytedata::ByteData<'a>,
    endian: CharsetEndian,
}

impl<'a> Utf16String<'a> {
    /// Create a new UTF-16 string from a byte sequence.
    /// The byte sequence must be a multiple of 2 bytes and contain valid UTF-16 format using the specified endianness.
    ///
    /// # Errors
    ///
    /// Returns the input byte sequence and the error that occurred.
    ///
    /// - `Utf16Error::NotEvenLength`: The input is not a valid UTF-16 sequence since all sequences should be multiples of `u16`.
    /// - `Utf16Error::NotAligned`: The input should be aligned to 2 bytes.
    /// - `Utf16Error::Incomplete`: The input ends with a high surrogate when expecting a low surrogate to follow.
    /// - `Utf16Error::UnexpectedLow`: Expected something that is not a low surrogate, but found one.
    /// - `Utf16Error::ExpectedLow`: Expected a low surrogate, but found something else.
    #[inline]
    pub const fn new(
        bytes: bytedata::ByteData<'a>,
        endian: CharsetEndian,
    ) -> Result<Self, (bytedata::ByteData<'a>, Utf16Error)> {
        let check_bytes = bytes.as_slice();
        let bytes_len = check_bytes.len();
        if bytes_len & 1 != 0 {
            return Err((bytes, Utf16Error::NotEvenLength));
        }
        // if !check_bytes.as_ptr().is_aligned() {
        //     return Err(Utf16Error::NotAligned);
        // }
        #[expect(clippy::pedantic)]
        let check_bytes = check_bytes.as_ptr().cast::<u16>();
        let u16_len = bytes_len >> 1_u8;
        let x = match endian {
            CharsetEndian::Big => Self::new_be(check_bytes, u16_len, bytes_len),
            CharsetEndian::Little => Self::new_le(check_bytes, u16_len, bytes_len),
        };
        match x {
            // TODO: when `const_ptr_write` is stable:
            //       Ok('\u{FEFF}') => Ok(Self { bytes., endian }),
            Ok(_) => Ok(Self { bytes, endian }),
            Err(err) => Err((bytes, err)),
        }
    }

    #[inline]
    const fn new_le(
        mut check_bytes: *const u16,
        mut u16_len: usize,
        bytes_len: usize,
    ) -> Result<char, Utf16Error> {
        let first = 'block: {
            // SAFETY: check_bytes is a valid pointer to u16
            let high = unsafe { check_bytes.read() }.to_le();
            if high < 0xD800 || high >= 0xE000 {
                // SAFETY: check_bytes is a valid pointer to u16
                check_bytes = unsafe { check_bytes.add(1) };
                u16_len -= 1;
                // SAFETY: high is a valid code point
                let codepoint = unsafe { core::mem::transmute::<u32, char>(high as u32) };
                break 'block codepoint;
            }
            if high >= 0xDC00 {
                return Err(Utf16Error::UnexpectedLow(bytes_len - (u16_len << 1)));
            }
            if u16_len == 1 {
                return Err(Utf16Error::Incomplete);
            }
            // SAFETY: check_bytes is a valid pointer to u16
            let low = unsafe { check_bytes.read() }.to_le();
            if low < 0xDC00 || low >= 0xE000 {
                return Err(Utf16Error::ExpectedLow(bytes_len - (u16_len << 1)));
            }
            // SAFETY: check_bytes is a valid pointer to u16
            check_bytes = unsafe { check_bytes.add(2) };
            u16_len -= 2;
            let uc = 0x1_0000 + ((((high as u32) & 0x03FF) << 10_u8) | (low & 0x03FF) as u32);
            // SAFETY: the codepoint is valid
            unsafe { core::mem::transmute::<u32, char>(uc) }
        };
        while u16_len != 0 {
            // SAFETY: check_bytes is a valid pointer to u16
            let high = unsafe { check_bytes.read() }.to_le();
            if high < 0xD800 || high >= 0xE000 {
                // SAFETY: check_bytes is a valid pointer to u16
                check_bytes = unsafe { check_bytes.add(1) };
                u16_len -= 1;
                continue;
            }
            if high >= 0xDC00 {
                return Err(Utf16Error::UnexpectedLow(bytes_len - (u16_len << 1)));
            }
            if u16_len == 1 {
                return Err(Utf16Error::Incomplete);
            }
            // SAFETY: check_bytes is a valid pointer to u16
            let low = unsafe { check_bytes.read() }.to_le();
            if low < 0xDC00 || low >= 0xE000 {
                return Err(Utf16Error::ExpectedLow(bytes_len - (u16_len << 1)));
            }
            // SAFETY: check_bytes is a valid pointer to u16
            check_bytes = unsafe { check_bytes.add(2) };
            u16_len -= 2;
        }
        Ok(first)
    }

    #[inline]
    const fn new_be(
        mut check_bytes: *const u16,
        mut u16_len: usize,
        bytes_len: usize,
    ) -> Result<char, Utf16Error> {
        let first = 'block: {
            // SAFETY: check_bytes is a valid pointer to u16
            let high = unsafe { check_bytes.read() }.to_be();
            if high < 0xD800 || high >= 0xE000 {
                // SAFETY: check_bytes is a valid pointer to u16
                check_bytes = unsafe { check_bytes.add(1) };
                u16_len -= 1;
                // SAFETY: high is a valid code point
                let codepoint = unsafe { core::mem::transmute::<u32, char>(high as u32) };
                break 'block codepoint;
            }
            if high >= 0xDC00 {
                return Err(Utf16Error::UnexpectedLow(bytes_len - (u16_len << 1)));
            }
            if u16_len == 1 {
                return Err(Utf16Error::Incomplete);
            }
            // SAFETY: check_bytes is a valid pointer to u16
            let low = unsafe { check_bytes.read() }.to_be();
            if low < 0xDC00 || low >= 0xE000 {
                return Err(Utf16Error::ExpectedLow(bytes_len - (u16_len << 1)));
            }
            // SAFETY: check_bytes is a valid pointer to u16
            check_bytes = unsafe { check_bytes.add(2) };
            u16_len -= 2;
            let uc = 0x1_0000 + ((((high as u32) & 0x03FF) << 10_u8) | (low & 0x03FF) as u32);
            // SAFETY: the codepoint is valid
            unsafe { core::mem::transmute::<u32, char>(uc) }
        };
        while u16_len != 0 {
            // SAFETY: check_bytes is a valid pointer to u16
            let high = unsafe { check_bytes.read() }.to_be();
            if high < 0xD800 || high >= 0xE000 {
                // SAFETY: check_bytes is a valid pointer to u16
                check_bytes = unsafe { check_bytes.add(1) };
                u16_len -= 1;
                continue;
            }
            if high >= 0xDC00 {
                return Err(Utf16Error::UnexpectedLow(bytes_len - (u16_len << 1)));
            }
            if u16_len == 1 {
                return Err(Utf16Error::Incomplete);
            }
            // SAFETY: check_bytes is a valid pointer to u16
            let low = unsafe { check_bytes.read() }.to_be();
            if low < 0xDC00 || low >= 0xE000 {
                return Err(Utf16Error::ExpectedLow(bytes_len - (u16_len << 1)));
            }
            // SAFETY: check_bytes is a valid pointer to u16
            check_bytes = unsafe { check_bytes.add(2) };
            u16_len -= 2;
        }
        Ok(first)
    }

    /// Returns the number of 16-bit chars in the string.
    /// This may not be the same as the number of unicode characters, as some codepoints are represented using two 16-bit surrogates.
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        self.bytes.len() >> 1
    }

    /// Return `true` if the string is empty.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Returns an iterator over the characters of the string.
    #[inline]
    #[must_use]
    pub fn chars(&self) -> Utf16Chars<'a> {
        Utf16Chars(self.code_units())
    }

    /// Returns an iterator over the 16-bit code units of the string.
    #[inline]
    #[must_use]
    pub fn code_units(&self) -> Utf16Units<'a> {
        Utf16Units {
            bytes: self.bytes.clone(),
            endian: self.endian,
        }
    }
}

/// An iterator over the 16-bit code units of a UTF-16 string.
pub struct Utf16Units<'a> {
    bytes: bytedata::ByteData<'a>,
    endian: CharsetEndian,
}

impl Utf16Units<'_> {
    /// Returns the number of 16-bit code units remaining.
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        self.bytes.len() >> 1_u8
    }

    /// Return `true` if there is no code units remaining.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

impl core::fmt::Debug for Utf16Units<'_> {
    #[inline]
    #[expect(clippy::min_ident_chars)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Utf16Units")
            .field(&self.bytes)
            .field(&self.endian)
            .finish()
    }
}

impl core::iter::Iterator for Utf16Units<'_> {
    type Item = Utf16CodeUnit;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.bytes.is_empty() {
            return None;
        }
        #[expect(clippy::cast_ptr_alignment)]
        let unit = self.bytes.as_slice().as_ptr().cast::<u16>();
        // SAFETY: unit is a valid pointer to u16
        let unit = unsafe { unit.read() };
        self.bytes.make_sliced(2..);
        let unit = match self.endian {
            CharsetEndian::Big => unit.to_be(),
            CharsetEndian::Little => unit.to_le(),
        };
        Some(Utf16CodeUnit(unit))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        if self.bytes.is_empty() {
            return None;
        }
        #[expect(clippy::cast_ptr_alignment)]
        let unit = self.bytes.as_slice().as_ptr().cast::<u16>();
        // SAFETY: unit is a valid pointer to u16
        let unit = unsafe { unit.add(self.len() - 1) };
        // SAFETY: unit is a valid pointer to u16
        let unit = unsafe { unit.read() };
        let unit = match self.endian {
            CharsetEndian::Big => unit.to_be(),
            CharsetEndian::Little => unit.to_le(),
        };
        Some(Utf16CodeUnit(unit))
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n >= self.len() {
            return None;
        }
        #[expect(clippy::cast_ptr_alignment)]
        let unit = self.bytes.as_slice().as_ptr().cast::<u16>();
        // SAFETY: unit is a valid pointer to u16
        let unit = unsafe { unit.add(n) };
        // SAFETY: unit is a valid pointer to u16
        let unit = unsafe { unit.read() };
        self.bytes.make_sliced((n << 1_u8)..);
        let unit = match self.endian {
            CharsetEndian::Big => unit.to_be(),
            CharsetEndian::Little => unit.to_le(),
        };
        Some(Utf16CodeUnit(unit))
    }
}

impl core::iter::ExactSizeIterator for Utf16Units<'_> {
    #[inline]
    fn len(&self) -> usize {
        Utf16Units::len(self)
    }
}

impl core::iter::FusedIterator for Utf16Units<'_> {}

impl core::iter::DoubleEndedIterator for Utf16Units<'_> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.bytes.is_empty() {
            return None;
        }
        let len = self.len();
        #[expect(clippy::cast_ptr_alignment)]
        let unit = self.bytes.as_slice().as_ptr().cast::<u16>();
        // SAFETY: unit is a valid pointer to u16
        let unit = unsafe { unit.add(len - 1) };
        // SAFETY: unit is a valid pointer to u16
        let unit = unsafe { unit.read() };
        self.bytes.make_sliced(0..(len - 1) << 1_u8);
        let unit = match self.endian {
            CharsetEndian::Big => unit.to_be(),
            CharsetEndian::Little => unit.to_le(),
        };
        Some(Utf16CodeUnit(unit))
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let len = self.len();
        if n >= len {
            return None;
        }
        #[expect(clippy::cast_ptr_alignment)]
        let unit = self.bytes.as_slice().as_ptr().cast::<u16>();
        // SAFETY: unit is a valid pointer to u16
        let unit = unsafe { unit.add(len - n - 1) };
        // SAFETY: unit is a valid pointer to u16
        let unit = unsafe { unit.read() };
        self.bytes.make_sliced(0..(len - n - 1) << 1_u8);
        let unit = match self.endian {
            CharsetEndian::Big => unit.to_be(),
            CharsetEndian::Little => unit.to_le(),
        };
        Some(Utf16CodeUnit(unit))
    }
}

/// An iterator over the characters of a UTF-16 string.
#[repr(transparent)]
pub struct Utf16Chars<'a>(Utf16Units<'a>);

impl core::fmt::Debug for Utf16Chars<'_> {
    #[inline]
    #[expect(clippy::min_ident_chars)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Utf16Chars")
            .field(&self.0.bytes)
            .field(&self.0.endian)
            .finish()
    }
}

impl core::iter::Iterator for Utf16Chars<'_> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let unit = self.0.next()?;
        if let Some(ch) = unit.to_char() {
            Some(ch)
        } else {
            #[expect(clippy::cast_lossless)]
            let high = (unit.0 & 0x03FF) as u32;
            #[expect(clippy::cast_lossless)]
            let low = (self.0.next()?.0 & 0x03FF) as u32;
            let uc = 0x1_0000 + ((high << 10) | low);
            #[expect(clippy::transmute_int_to_char)]
            // SAFETY: uc is a valid code point
            Some(unsafe { core::mem::transmute::<u32, char>(uc) })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = Utf16Units::len(&self.0);
        ((len + 1) >> 1_u8, Some(len))
    }
}

impl core::iter::FusedIterator for Utf16Chars<'_> {}

impl core::iter::DoubleEndedIterator for Utf16Chars<'_> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let unit = self.0.next_back()?;
        if let Some(ch) = unit.to_char() {
            Some(ch)
        } else {
            #[expect(clippy::cast_lossless)]
            let low = (unit.0 & 0x03FF) as u32;
            #[expect(clippy::cast_lossless)]
            let high = (self.0.next_back()?.0 & 0x03FF) as u32;
            let uc = 0x1_0000 + ((high << 10) | low);
            #[expect(clippy::transmute_int_to_char)]
            // SAFETY: uc is a valid code point
            Some(unsafe { core::mem::transmute::<u32, char>(uc) })
        }
    }
}



#[inline]
const unsafe fn from_latin1_const_raw(latin1: *const u8, utf16: *mut u16, mut len: usize) {
    loop {
        len -= 1;
        if len != 0 {
            let byte = *latin1.add(len);
            *utf16.add(len) = byte as u16;
            continue;
        }
        let byte = *latin1;
        *utf16.add(len) = byte as u16;
        return;
    }
}
#[inline]
const fn min_usize(val0: usize, val1: usize) -> usize {
    if val0 < val1 { val0 } else { val1 }
}

#[cfg(all(target_arch = "x86_64", feature = "avx"))]
#[target_feature(enable = "avx2")]
unsafe fn from_latin1_avx2(mut input: *const u8, mut output: *mut u16, mut len: usize) {
    use core::arch::x86_64::{__m128i, __m256i, _mm_loadu_si128, _mm256_cvtepu8_epi16, _mm256_storeu_si256};
    
    // Process 16 bytes at a time with AVX2
    #[expect(clippy::cast_ptr_alignment)]
    while len >= 16 {
        let data = _mm_loadu_si128(input.cast::<__m128i>());
        let data = _mm256_cvtepu8_epi16(data);
        _mm256_storeu_si256(output.cast::<__m256i>(), data);
        input = input.add(16);
        output = output.add(16);
        len -= 16;
    }
    
    // Handle remaining bytes with scalar code
    if len == 0 {
        return;
    }
    // SAFETY: input and output are valid pointers to u8 and u16 respectively, and len is defined and non-zero
    unsafe { from_latin1_const_raw(input, output, len) };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CharsetEncoding, ExhaustiveDecodeResult};

    #[test]
    fn test_utf16_le() {
        let utf16 = UTF16_LE;
        {
            let bytes = [0x61, 0x00, 0x62, 0x00, 0x63, 0x00];
            let res = utf16.decode(&bytes);
            assert_eq!(res, DecodeResult::Char('a', 2));
            assert_eq!(
                utf16.encode("abc"),
                crate::EncodeResult::Chunk(
                    bytedata::ByteChunk::from_slice(&[0x61, 0x00, 0x62, 0x00, 0x63, 0x00]),
                    3
                )
            );
        };
        {
            let bytes = [0x61];
            let res = utf16.decode(&bytes);
            assert_eq!(res, DecodeResult::Incomplete);
        };
        {
            let res = utf16.decode(&[]);
            assert_eq!(res, DecodeResult::Empty);
        };
        {
            let bytes = [0x00, 0xD8, 0x00, 0xDC];
            let res = utf16.decode(&bytes);
            assert_eq!(res, DecodeResult::Char('𐀀', 4));
        };
        {
            let bytes = [0x00, 0xD8];
            let res = utf16.decode(&bytes);
            assert_eq!(res, DecodeResult::Incomplete);
        };
        {
            let bytes = [0x00, 0xD8, 0x00, 0x00];
            let res = utf16.decode(&bytes);
            assert_eq!(res, DecodeResult::InvalidChar(0xD800, 2));
        };
    }

    #[test]
    fn test_utf16_be() {
        let utf16 = UTF16_BE;
        {
            let bytes = [0x00, 0x61, 0x00, 0x62, 0x00, 0x63];
            let res = utf16.decode(&bytes);
            assert_eq!(res, DecodeResult::Char('a', 2));
            assert_eq!(
                utf16.encode("abc"),
                crate::EncodeResult::Chunk(
                    bytedata::ByteChunk::from_slice(&[0x00, 0x61, 0x00, 0x62, 0x00, 0x63]),
                    3
                )
            );
        };
        {
            let bytes = [0x61];
            let res = utf16.decode(&bytes);
            assert_eq!(res, DecodeResult::Incomplete);
        };
        {
            let res = utf16.decode(&[]);
            assert_eq!(res, DecodeResult::Empty);
        };
        {
            let bytes = [0xD8, 0x00, 0xDC, 0x00];
            let res = utf16.decode(&bytes);
            assert_eq!(res, DecodeResult::Char('𐀀', 4));
        };
        {
            let bytes = [0xD8, 0x00];
            let res = utf16.decode(&bytes);
            assert_eq!(res, DecodeResult::Incomplete);
        };
        {
            let bytes = [0xD8, 0x00, 0x00, 0x00];
            let res = utf16.decode(&bytes);
            assert_eq!(res, DecodeResult::InvalidChar(0xD800, 2));
        };
    }

    #[test]
    fn test_utf16_native() {
        {
            let res = Utf16Encoding::decode_native_const(&[0x0061, 0x0062, 0x0063]);
            assert_eq!(res, DecodeResult::Char('a', 2));
        };
        {
            let res = Utf16Encoding::decode_native_const(&[0xD800, 0xDC00]);
            assert_eq!(res, DecodeResult::Char('𐀀', 4));
        };
        {
            let res = Utf16Encoding::decode_native_const(&[0xD800]);
            assert_eq!(res, DecodeResult::Incomplete);
        };
        {
            let res = Utf16Encoding::decode_native_const(&[0xD800, 0x0000]);
            assert_eq!(res, DecodeResult::InvalidChar(0xD800, 2));
        };
        #[cfg(feature = "alloc")]
        {
            let mut buff = bytedata::SharedStrBuilder::new();
            let res = Utf16Encoding::decode_native_into(
                &[0x0061, 0x0062, 0x0063, 0xD800, 0xDC00, 0x0064],
                &mut buff,
            );
            assert_eq!(res, ExhaustiveDecodeResult::Decoded(6));
            assert_eq!(buff.as_str(), "abc𐀀d");
        };
        {
            static DATA: &[u8; 15] = b"this is a test\x9C";
            let mut utf16 = [MaybeUninit::uninit(); 32];
            let encoded = Utf16Encoding::from_latin1_uninit(DATA, &mut utf16);
            assert_eq!(encoded.len(), DATA.len());
            for i in 0..DATA.len() {
                assert_eq!(encoded[i], u16::from(DATA[i]));
            }
        }
    }
}
