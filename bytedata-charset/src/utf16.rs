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

    /// Detect the if the bytes are UTF-16 encoded.
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
    
    /// Encode UTF-16 characters. `chars` must not be empty.
    fn encode_inner(self, chars: &str) -> crate::EncodeResult {
        let mut buf = [0_u16; 7];
        let mut buf_use = 0;
        for (idx, ch) in chars.char_indices() {
            if buf_use == 7 {
                // SAFETY: buf is a valid UTF-16 sequence and encoded according to the endianness
                let buf = unsafe { core::mem::transmute::<[u16; 7], [u8; 14]>(buf) };
                #[expect(clippy::cast_possible_truncation)]
                return crate::EncodeResult::Chunk(bytedata::ByteChunk::from_array(&buf), idx as u16);
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
                return crate::EncodeResult::Chunk(bytedata::ByteChunk::from_slice(&buf[0..12]), idx as u16);
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
        crate::EncodeResult::Chunk(bytedata::ByteChunk::from_slice(&buf[0..(buf_use << 1)]), chars.len() as u16)
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
            CharsetEndian::Little => ["utf-16le", "utf-16", "unicodefeff", "unicode", "ucs-2", "iso-10646-ucs-2", "csunicode", "csutf16le", "csutf16"].as_slice(),
        }
    }
}

impl CharsetDecoding for Utf16Encoding {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> DecodeResult {
        Self::decode_const(*self, bytes)
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
            CharsetEndian::Big => {
                detect_be(bytes)
            }
            CharsetEndian::Little => {
                detect_le(bytes)
            }
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
            let buff = [
                ((ch >> 10) | 0xD800) as u16,
                ((ch & 0x3FF) | 0xDC00) as u16,
            ];
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
    pub const fn new(bytes: bytedata::ByteData<'a>, endian: CharsetEndian) -> Result<Self, (bytedata::ByteData<'a>, Utf16Error)> {
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
    const fn new_le(mut check_bytes: *const u16, mut u16_len: usize, bytes_len: usize) -> Result<char, Utf16Error> {
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
    const fn new_be(mut check_bytes: *const u16, mut u16_len: usize, bytes_len: usize) -> Result<char, Utf16Error> {
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
        if let Some(ch) = unit.to_char() { Some(ch) } else {
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
        if let Some(ch) = unit.to_char() { Some(ch) } else {
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
