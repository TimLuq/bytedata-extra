use crate::{CharsetDecoding, CharsetEndian, DecodeResult};

/// An encoding for UTF-32LE and UTF-32BE.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
#[cfg_attr(docsrs, doc(cfg(feature = "utf-32")))]
pub struct Utf32Encoding(CharsetEndian);

/// UTF-32 little endian decoder.
#[cfg_attr(docsrs, doc(cfg(feature = "utf-32")))]
pub static UTF32_LE: Utf32Encoding = Utf32Encoding::UTF32_LE;
/// UTF-32 big endian decoder.
#[cfg_attr(docsrs, doc(cfg(feature = "utf-32")))]
pub static UTF32_BE: Utf32Encoding = Utf32Encoding::UTF32_BE;

impl Utf32Encoding {
    /// UTF-32 little endian decoder.
    pub const UTF32_LE: Self = Self::new(CharsetEndian::Little);
    /// UTF-32 big endian decoder.
    pub const UTF32_BE: Self = Self::new(CharsetEndian::Big);

    /// Create a new UTF-32 decoder with the specified endianness.
    #[inline]
    #[must_use]
    pub const fn new(endian: CharsetEndian) -> Self {
        Self(endian)
    }

    /// Decode a UTF-32 byte sequence.
    #[must_use]
    #[inline]
    pub const fn decode_const(self, bytes: &[u8]) -> DecodeResult {
        if bytes.is_empty() {
            return DecodeResult::Empty;
        }
        if bytes.len() < 4 {
            return DecodeResult::Incomplete;
        }
        let bytes: *const u8 = bytes.as_ptr();
        // SAFETY: bytes is at least 4 bytes long, so it is safe to read 4 unaligned bytes
        let base = unsafe { core::ptr::read_unaligned(bytes.cast::<u32>()) };
        let base = match self.0 {
            CharsetEndian::Big => u32::from_be(base),
            CharsetEndian::Little => u32::from_le(base),
        };
        match char::from_u32(base) {
            Some(ch) => DecodeResult::Char(ch, 4),
            None => DecodeResult::InvalidChar(base, 4),
        }
    }

    
    fn encode_inner(self, chars: &str) -> crate::EncodeResult {
        let mut buf = [0_u32; 3];
        let mut buf_use = 0;
        for (idx, ch) in chars.char_indices() {
            if buf_use == 3 {
                // SAFETY: buf is filled with valid u32 values in the correct endianness
                let buf = unsafe { core::mem::transmute::<[u32; 3], [u8; 12]>(buf) };
                #[expect(clippy::cast_possible_truncation)]
                return crate::EncodeResult::Chunk(bytedata::ByteChunk::from_array(&buf), idx as u16);
            }
            let ch = ch as u32;
            buf[buf_use] = match self.0 {
                CharsetEndian::Big => ch.to_be(),
                CharsetEndian::Little => ch.to_le(),
            };
            buf_use += 1;
        }
        if buf_use == 0 {
            return crate::EncodeResult::Empty;
        }
        // SAFETY: buf is partially(?) filled with valid u32 values in the correct endianness
        let buf = unsafe { core::mem::transmute::<[u32; 3], [u8; 12]>(buf) };
        #[expect(clippy::cast_possible_truncation)]
        crate::EncodeResult::Chunk(bytedata::ByteChunk::from_slice(&buf[0..(buf_use << 2)]), chars.len() as u16)
    }

    /// Detect the if the bytes are UTF-32 encoded.
    #[inline]
    #[must_use]
    pub const fn detect_const(self, bytes: &[u8]) -> crate::detect::DetectionResult {
        match self.0 {
            CharsetEndian::Big => {
                detect_be(bytes)
            }
            CharsetEndian::Little => {
                detect_le(bytes)
            }
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "utf-32")))]
impl crate::Charset for Utf32Encoding {
    const CHARSET_NAME: &'static str = "utf-32";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (4, 4)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "utf-32")))]
impl CharsetDecoding for Utf32Encoding {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> DecodeResult {
        Self::decode_const(*self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "utf-32")))]
impl crate::CharsetEncoding for Utf32Encoding {
    #[inline]
    #[must_use]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        let len = chars.len();
        if len == 0 {
            return crate::EncodeResult::Empty;
        }
        self.encode_inner(chars)
    }
}

#[must_use]
#[expect(clippy::missing_asserts_for_indexing)]
const fn detect_be(bytes: &[u8]) -> crate::detect::DetectionResult {
    let len = bytes.len();
    if len < 4 {
        return crate::detect::DetectionResult::Incomplete;
    }
    // UTF-32BE BOM
    if bytes[0] == 0x00 && bytes[1] == 0x00 && bytes[2] == 0xFE && bytes[3] == 0xFF {
        return crate::detect::DetectionResult::Certain;
    }
    
    let bytes: *const u8 = bytes.as_ptr();

    // when no BOM is present, check for bytes matching UTF-32BE
    let mut i = 0;
    while i + 4 <= len {
        // SAFETY: the length check guarantees that the `base+i`` pointer is valid and has at least 4 bytes
        let bytes = unsafe { bytes.add(i) };
        // SAFETY: bytes is at least 4 bytes long, so it is safe to read 4 unaligned bytes
        let ch = u32::from_be(unsafe { core::ptr::read_unaligned(bytes.cast::<u32>()) });
        if char::from_u32(ch).is_none() {
            return crate::detect::DetectionResult::Irrelevant;
        }
        i += 4;
    }
    if i < 16 {
        crate::detect::DetectionResult::Incomplete
    } else {
        crate::detect::DetectionResult::Tentative
    }
}

#[expect(clippy::missing_asserts_for_indexing)]
const fn detect_le(bytes: &[u8]) -> crate::detect::DetectionResult {
    let len = bytes.len();
    if len < 4 {
        return crate::detect::DetectionResult::Incomplete;
    }
    // UTF-32LE BOM
    if bytes[0] == 0xFF && bytes[1] == 0xFE && bytes[2] == 0x00 && bytes[3] == 0x00 {
        return crate::detect::DetectionResult::Certain;
    }

    let bytes: *const u8 = bytes.as_ptr();

    // when no BOM is present, check for bytes matching UTF-32LE
    let mut i = 0;
    while i + 4 <= len {
        // SAFETY: the length check guarantees that the `base+i`` pointer is valid and has at least 4 bytes
        let bytes = unsafe { bytes.add(i) };
        // SAFETY: bytes is at least 4 bytes long, so it is safe to read 4 unaligned bytes
        let ch = u32::from_le(unsafe { core::ptr::read_unaligned(bytes.cast::<u32>()) });
        if char::from_u32(ch).is_none() {
            return crate::detect::DetectionResult::Irrelevant;
        }
        i += 4;
    }
    if i < 16 {
        crate::detect::DetectionResult::Incomplete
    } else {
        crate::detect::DetectionResult::Tentative
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "utf-32")))]
impl crate::detect::CharsetDetector for Utf32Encoding {
    #[inline]
    fn detect(&self, bytes: &[u8]) -> crate::detect::DetectionResult {
        match self.0 {
            CharsetEndian::Big => {
                detect_be(bytes)
            }
            CharsetEndian::Little => {
                detect_le(bytes)
            }
        }
    }
}
