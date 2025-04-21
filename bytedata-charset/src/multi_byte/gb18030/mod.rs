mod cp_gb18030;
mod gb18030_cp;
mod gb18030_cp_ranges;

/// An encoding for GB18030.
///
/// This is a charset that is a superset of GBK (simplified Chinese) but should be able to encode all Unicode characters using the modern extensions.
/// If possible, use [`UTF-8`] or [`UTF-16`] instead.
///
/// [`UTF-8`]: crate::Utf8Encoding
/// [`UTF-16`]: crate::Utf16Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[non_exhaustive]
#[cfg_attr(docsrs, doc(cfg(feature = "gb18030")))]
pub struct Gb18030Encoding {
    pub(super) gbk: bool,
}

/// An encoding for GB18030.
#[cfg_attr(docsrs, doc(cfg(feature = "gb18030")))]
pub static GB18030: Gb18030Encoding = Gb18030Encoding::new();

impl Gb18030Encoding {

    /// Create a new GB18030 encoding instance.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { gbk: false }
    }

    /// Decode a GB18030 byte sequence.
    #[inline]
    #[must_use]
    pub const fn decode_const(&self, bytes: &[u8]) -> crate::DecodeResult {
        if bytes.is_empty() {
            return crate::DecodeResult::Empty;
        }
        let byt = bytes[0];
        if byt < 0x80 {
            let mut i = 1;
            while i < bytes.len() {
                if bytes[i] & 0b1000_0000 == 0b1000_0000 {
                    break;
                }
                i += 1;
            }
            return crate::DecodeResult::Utf8(i as u64);
        }
        decode_const_inner(bytes)
    }

    /// Encode a GB18030 character sequence.
    #[inline]
    #[must_use]
    pub const fn encode_const(&self, chars: &str) -> crate::EncodeResult {
        let bytes = chars.as_bytes();
        if bytes.is_empty() {
            return crate::EncodeResult::Empty;
        }
        let byt = bytes[0];
        if byt < 0x80 {
            let mut i = 1;
            while i < bytes.len() {
                if bytes[i] & 0b1000_0000 == 0b1000_0000 {
                    break;
                }
                i += 1;
            }
            return crate::EncodeResult::Utf8(i as u64);
        }
        encode_const_inner(bytes, self.gbk)
    }

    /// Detect if the given bytes are GB18030 encoded.
    #[inline]
    #[must_use]
    pub const fn detect_const(bytes: &[u8]) -> crate::detect::DetectionResult {
        GB18030.detect_const_self(bytes)
    }

    /// Detect if the given bytes are GB18030 encoded.
    #[inline]
    #[must_use]
    pub(super) const fn detect_const_self(self, bytes: &[u8]) -> crate::detect::DetectionResult {
        if bytes.is_empty() {
            return crate::detect::DetectionResult::Incomplete;
        }
        detect_const_inner(self.gbk, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "gb18030")))]
impl crate::Charset for Gb18030Encoding {
    const CHARSET_NAME: &'static str = "utf-8";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 2)
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

#[cfg_attr(docsrs, doc(cfg(feature = "gb18030")))]
impl crate::detect::CharsetDetector for Gb18030Encoding {
    #[inline]
    fn detect(&self, bytes: &[u8]) -> crate::detect::DetectionResult {
        Self::detect_const(bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "gb18030")))]
impl crate::CharsetDecoding for Gb18030Encoding {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        Self::decode_const(self, bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "gb18030")))]
impl crate::CharsetEncoding for Gb18030Encoding {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        Self::encode_const(self, chars)
    }
}

const fn detect_const_inner(gbk: bool, mut bytes: &[u8]) -> crate::detect::DetectionResult {
    // TODO: Implement efficient GB18030 detection
    const MIN_LEN: usize = 4;
    let mut offset = 0;
    loop {
        if bytes.is_empty() {
            if offset < MIN_LEN {
                return crate::detect::DetectionResult::Incomplete;
            }
            return crate::detect::DetectionResult::Tentative;
        }
        let b0 = bytes[0];
        if b0 < 0x80 {
            bytes = bytedata::const_or_bytes(bytedata::const_slice(bytes, 1..bytes.len()), b"");
            offset += 1;
            continue;
        }
        let res = decode_const_inner(bytes);
        match res {
            crate::DecodeResult::Char(_, len) => {
                let len = len as usize;
                offset += len;
                bytes = bytedata::const_or_bytes(bytedata::const_slice(bytes, len..bytes.len()), b"");
            }
            crate::DecodeResult::InvalidChar(_, _) => {
                return crate::detect::DetectionResult::Irrelevant;
            }
            crate::DecodeResult::Utf8(len) => {
                #[expect(clippy::cast_possible_truncation)]
                let len = len as usize;
                offset += len;
                bytes = bytedata::const_or_bytes(bytedata::const_slice(bytes, len..bytes.len()), b"");
            }
            crate::DecodeResult::Empty | crate::DecodeResult::Incomplete => {
                if offset < MIN_LEN {
                    return crate::detect::DetectionResult::Incomplete;
                }
                return crate::detect::DetectionResult::Tentative;
            }
        }
    }
}

/// Decodes a GB18030 byte sequence. This should not be called directly without first validating the fist byte isn't ASCII.
#[inline]
#[expect(clippy::missing_asserts_for_indexing)]
const fn decode_const_inner(bytes: &[u8]) -> crate::DecodeResult {
    let first = bytes[0];
    if first <= 0x80 {
        return crate::DecodeResult::Char('\u{20AC}', 1);
    }
    if first == 0xFF {
        return crate::DecodeResult::InvalidChar(first as u32, 1);
    }
    if bytes.len() < 2 {
        return crate::DecodeResult::Incomplete;
    }
    let second = bytes[1];
    if second < 0x30 || second >= 0x40 {
        let offset = if second < 0x7F { 0x40 } else { 0x41 };
        if second >= 0x40 && second < 0xFE && second != 0x7F {
            let ptr = (first as usize - 0x81) * 190 + second as usize - offset; // 0 - 23_939
            let x = gb18030_cp::GB18030_PTR_TO_CP[ptr];
            if let Some(x) = char::from_u32(x as u32) {
                return crate::DecodeResult::Char(x, 2);
            }
            return crate::DecodeResult::InvalidChar(x as u32, 2);
        }
        let (ch, len) = if second < 0x7F { (first as u32, 1) } else { (((first as u32) << 8_i32) | second as u32, 2) };
        return crate::DecodeResult::InvalidChar(ch, len);
    }
    if bytes.len() < 3 {
        return crate::DecodeResult::Incomplete;
    }
    let third = bytes[2];
    if third < 0x81 || third > 0xFE {
        return crate::DecodeResult::InvalidChar(first as u32, 1);
    }
    if bytes.len() < 4 {
        return crate::DecodeResult::Incomplete;
    }
    let fourth = bytes[3];
    if fourth < 0x30 || fourth > 0x39 {
        return crate::DecodeResult::InvalidChar(first as u32, 1);
    }
    let ptr = (((first as u32) - 0x81) * (10 * 126 * 10)) + (((second as u32) - 0x30) * (10 * 126)) + (((third as u32) - 0x81) * 10) + (fourth as u32) - 0x30;
    if (ptr > 39_419 && ptr < 189_000) || ptr > 1_237_575 {
        return crate::DecodeResult::InvalidChar(ptr, 4);
    }
    if ptr == 7457 {
        return crate::DecodeResult::Char('\u{E7C7}', 4);
    }
    let (offset, cp_offset) = if ptr >= 189_000	{
        (189_000, 0x10000)
    } else if ptr >= 39394 {
        (39_394, 0xFFE6)
    } else {
        #[expect(clippy::cast_possible_truncation)]
        let ptr = ptr as u16;
        let Some(&(offset, cp_offset)) = b_find_before(ptr, &gb18030_cp_ranges::GB18030_CP_RANGES) else {
            unreachable!();
        };
        (offset as u32, cp_offset as u32)
    };
    let cp = cp_offset + ptr - offset;
    if let Some(ch) = char::from_u32(cp) {
        crate::DecodeResult::Char(ch, 4)
    } else {
        crate::DecodeResult::InvalidChar(cp, 4)
    }
}

#[inline]
#[expect(clippy::too_many_lines)]
const fn encode_const_inner(mut bytes: &[u8], gbk: bool) -> crate::EncodeResult {
    let mut chunk = [0_u8; 14];
    let mut chunk_len = 0_usize;
    let mut ascii = 0_usize;
    let mut consumed = 0_usize;
    loop {
        let (ch, ch_len) = bytedata::const_utf8_char_next(bytes);
        if ch_len == 0 {
            if chunk_len != 0 {
                if !bytes.is_empty() && ascii != 0 {
                    chunk_len -= ascii;
                    consumed -= ascii;
                }
                return crate::EncodeResult::Chunk(bytedata::ByteChunk::from_slice(bytedata::const_or_bytes(bytedata::const_slice(&chunk, 0..chunk_len), b"")), consumed as u16);
            }
            if bytes.is_empty() {
                return crate::EncodeResult::Empty;
            }
            return crate::EncodeResult::InvalidChar(bytes[0] as char, 1);
        }

        #[expect(clippy::cast_possible_truncation)]
        if ch < 0x80 {
            if ch_len == 1 {
                ascii += 1;
            } else {
                ascii = 0;
            }
            consumed += ch_len as usize;
            chunk[chunk_len] = ch as u8;
            chunk_len += 1;
            if chunk_len > 10 {
                if ascii != 0 {
                    chunk_len -= ascii;
                    consumed -= ascii;
                }
                return crate::EncodeResult::Chunk(bytedata::ByteChunk::from_slice(bytedata::const_or_bytes(bytedata::const_slice(&chunk, 0..chunk_len), b"")), consumed as u16);
            }
            bytes = bytedata::const_or_bytes(bytedata::const_slice(bytes, (ch_len as usize)..bytes.len()), b"");
            continue;
        }

        #[expect(clippy::cast_possible_truncation)]
        if ch == 0xE5E5 {
            if chunk_len != 0 {
                return crate::EncodeResult::Chunk(bytedata::ByteChunk::from_slice(bytedata::const_or_bytes(bytedata::const_slice(&chunk, 0..chunk_len), b"")), consumed as u16);
            }
            let ch = unsafe { char::from_u32_unchecked(ch) };
            return crate::EncodeResult::InvalidChar(ch, ch_len as u16);
        }

        if gbk && ch == 0x20AC {
            consumed += ch_len as usize;
            chunk[chunk_len] = 0x80;
            chunk_len += 1;
            #[expect(clippy::cast_possible_truncation)]
            if chunk_len > 10 {
                return crate::EncodeResult::Chunk(bytedata::ByteChunk::from_slice(bytedata::const_or_bytes(bytedata::const_slice(&chunk, 0..chunk_len), b"")), consumed as u16);
            }
            bytes = bytedata::const_or_bytes(bytedata::const_slice(bytes, (ch_len as usize)..bytes.len()), b"");
            continue;
        }

        let n: u16 = match ch {
            0xE78D => 0xA6D9,
            0xE78E => 0xA6DA,
            0xE78F => 0xA6DB,
            0xE790 => 0xA6DC,
            0xE791 => 0xA6DD,
            0xE792 => 0xA6DE,
            0xE793 => 0xA6DF,
            0xE794 => 0xA6EC,
            0xE795 => 0xA6ED,
            0xE796 => 0xA6F3,
            0xE81E => 0xFE59,
            0xE826 => 0xFE61,
            0xE82B => 0xFE66,
            0xE82C => 0xFE67,
            0xE832 => 0xFE6D,
            0xE843 => 0xFE7E,
            0xE854 => 0xFE90,
            0xE864 => 0xFEA0,
            _ => 0,
        };
        if n != 0 {
            consumed += ch_len as usize;
            chunk[chunk_len] = (n >> 8) as u8;
            chunk[chunk_len + 1] = (n & 0xFF) as u8;
            chunk_len += 2;
            #[expect(clippy::cast_possible_truncation)]
            if chunk_len > 10 {
                return crate::EncodeResult::Chunk(bytedata::ByteChunk::from_slice(bytedata::const_or_bytes(bytedata::const_slice(&chunk, 0..chunk_len), b"")), consumed as u16);
            }
            bytes = bytedata::const_or_bytes(bytedata::const_slice(bytes, (ch_len as usize)..bytes.len()), b"");
            continue;
        }

        if ch <= 0xFFE5 {
            #[expect(clippy::cast_possible_truncation)]
            let ch = ch as u16;
            if let Some(&(cp, pos)) = b_find_before(ch, &cp_gb18030::GB18030_CP_TO_PTR) {
                #[expect(clippy::cast_possible_truncation, clippy::integer_division)]
                if cp == ch {
                    chunk[chunk_len] = ((pos / 190) + 0x81) as u8;
                    chunk_len += 1;
                    let trail = (pos % 190) as u8;
                    chunk[chunk_len] = trail + if trail < 0x3F { 0x40 } else { 0x41 };
                    chunk_len += 1;
                    consumed += ch_len as usize;
                    if chunk_len > 10 {
                        return crate::EncodeResult::Chunk(bytedata::ByteChunk::from_slice(bytedata::const_or_bytes(bytedata::const_slice(&chunk, 0..chunk_len), b"")), consumed as u16);
                    }
                    bytes = bytedata::const_or_bytes(bytedata::const_slice(bytes, (ch_len as usize)..bytes.len()), b"");
                    continue;
                }
            }
        }

        #[expect(clippy::cast_possible_truncation)]
        if gbk {
            if chunk_len != 0 {
                return crate::EncodeResult::Chunk(bytedata::ByteChunk::from_slice(bytedata::const_or_bytes(bytedata::const_slice(&chunk, 0..chunk_len), b"")), consumed as u16);
            }
            let ch = unsafe { char::from_u32_unchecked(ch) };
            return crate::EncodeResult::InvalidChar(ch, ch_len as u16);
        }

        let (offset, cp_offset) = if ch > 0x10000 {
            (189_000, 0x10000)
        } else if ch > 0xFFE6 {
            (39_394, 0xFFE6)
        } else {
            #[expect(clippy::cast_possible_truncation)]
            let ch = ch as u16;
            let Some(&(offset, cp_offset)) = b_find_before_val(ch, &gb18030_cp_ranges::GB18030_CP_RANGES) else {
                unreachable!();
            };
            (offset as u32, cp_offset as u32)
        };
        
        let ptr = offset + (ch - cp_offset);
        let div = 10 * 126 * 10;
        #[expect(clippy::integer_division)]
        let b1 = ptr / div;
        let ptr = ptr % div;
        #[expect(clippy::integer_division)]
        let b2 = ptr / (10 * 126);
        let ptr = ptr % (10 * 126);
        #[expect(clippy::integer_division)]
        let b3 = ptr / 10;
        let b4 = ptr % 10;
        #[expect(clippy::cast_possible_truncation)]
        {
            chunk[chunk_len] = b1 as u8 + 0x81;
            chunk[chunk_len + 1] = b2 as u8 + 0x30;
            chunk[chunk_len + 2] = b3 as u8 + 0x81;
            chunk[chunk_len + 3] = b4 as u8 + 0x30;
            chunk_len += 4;
        };
        consumed += ch_len as usize;
        if chunk_len > 10 {
            #[expect(clippy::cast_possible_truncation)]
            return crate::EncodeResult::Chunk(bytedata::ByteChunk::from_slice(bytedata::const_or_bytes(bytedata::const_slice(&chunk, 0..chunk_len), b"")), consumed as u16);
        }
    }
}

const fn b_find_before(key: u16, arr: &[(u16, u16)]) -> Option<&(u16, u16)> {
    let size = arr.len();
    if size == 0 || key < arr[0].0 {
        return None;
    }
    let mut base = 0;
    let mut lim = size;
    while base < lim {
        #[expect(clippy::integer_division)]
        let mid = base + (lim - base) / 2;
        let mid_val = arr[mid].0;
        if mid_val < key {
            base = mid + 1;
        } else if mid_val > key {
            lim = mid;
        } else {
            return Some(&arr[mid]);
        }
    }
    Some(&arr[base - 1])
}

const fn b_find_before_val(val: u16, arr: &[(u16, u16)]) -> Option<&(u16, u16)> {
    let size = arr.len();
    if size == 0 || val < arr[0].1 {
        return None;
    }
    let mut base = 0;
    let mut lim = size;
    while base < lim {
        #[expect(clippy::integer_division)]
        let mid = base + (lim - base) / 2;
        let mid_val = arr[mid].1;
        if mid_val < val {
            base = mid + 1;
        } else if mid_val > val {
            lim = mid;
        } else {
            return Some(&arr[mid]);
        }
    }
    Some(&arr[base - 1])
}
