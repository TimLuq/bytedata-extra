

/// The result of a charset decoding operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::exhaustive_enums)]
pub enum DecodeResult {
    /// The next number of bytes is a valid character. The character and the number of bytes consumed are returned.
    Char(char, u32),
    /// The next number of bytes is not a valid character. The invalid character data and the number of bytes consumed is returned.
    InvalidChar(u32, u32),
    /// The returned number of bytes are all valid utf-8 compatible characters.
    Utf8(u64),
    /// The data input was too short to determine if the next number of bytes represent a valid character.
    Incomplete,
    /// The data input was empty and no more data is expected.
    Empty,
}

/// The result of a charset decoding operation.
#[derive(Debug, Clone, Copy)]
#[expect(clippy::exhaustive_enums)]
pub enum EncodeResult {
    /// The next number of bytes from the str is encoded into a buffer. The encoded sequence and the number of bytes consumed are returned.
    Chunk(bytedata::ByteChunk, u16),
    /// The next number of bytes does not encode into valid character in the target charset. The invalid character data and the number of bytes consumed is returned.
    InvalidChar(char, u16),
    /// The returned number of bytes are all valid utf-8/ascii7 compatible characters in the target charset.
    Utf8(u64),
    /// The data input was too short to determine if the next number of bytes represent a valid character.
    Incomplete,
    /// The data input was empty and no more data is expected.
    Empty,
}
