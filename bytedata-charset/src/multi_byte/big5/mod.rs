mod big5_cp;

/// BIG5 encoding. A legacy encoding for simplified Chinese characters.
///
/// Use [`UTF-8`] or [`UTF-16`] instead if possible.
/// (GB18030 is an encoding backward compatible with BIG5 but support a much larger mapping against Unicode.)
///
/// [`UTF-8`]: crate::Utf8Encoding
/// [`UTF-16`]: crate::Utf16Encoding
/// [`GB18030`]: crate::multi_byte::Gb18030Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[non_exhaustive]
#[repr(transparent)]
#[cfg_attr(docsrs, doc(cfg(feature = "big5")))]
pub struct Big5Encoding;

/// BIG5 encoding.
#[cfg_attr(docsrs, doc(cfg(feature = "big5")))]
pub static BIG5: Big5Encoding = Big5Encoding::new();

impl Big5Encoding {
    /// Create a new BIG5 encoding instance.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Decode a BIG5 byte sequence.
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

    /// Encode a BIG5 character sequence.
    #[inline]
    #[must_use]
    pub const fn encode_const(&self, chars: &str) -> crate::EncodeResult {
        if chars.is_empty() {
            return crate::EncodeResult::Empty;
        }
        crate::EncodeResult::Utf8(chars.len() as u64)
    }

    /// Detect if the given bytes are BIG5 encoded.
    #[inline]
    #[must_use]
    pub const fn detect_const(bytes: &[u8]) -> crate::detect::DetectionResult {
        if bytes.is_empty() {
            return crate::detect::DetectionResult::Incomplete;
        }
        detect_const_inner(bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "big5")))]
impl crate::Charset for Big5Encoding {
    const CHARSET_NAME: &'static str = "big5";

    #[inline]
    fn size_hint(&self) -> (u16, u16) {
        (1, 2)
    }

    #[inline]
    fn charset_alias(&self) -> &[&'static str] {
        &[
            // IANA
            Self::CHARSET_NAME,
            "csbig5",
            // other
            "cn-big5",
            "x-x-big5",
            "big5-hkscs",
        ]
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "big5")))]
impl crate::detect::CharsetDetector for Big5Encoding {
    #[inline]
    fn detect(&self, bytes: &[u8]) -> crate::detect::DetectionResult {
        Self::detect_const(bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "big5")))]
impl crate::CharsetDecoding for Big5Encoding {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> crate::DecodeResult {
        self.decode_const(bytes)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "big5")))]
impl crate::CharsetEncoding for Big5Encoding {
    #[inline]
    fn encode(&self, chars: &str) -> crate::EncodeResult {
        self.encode_const(chars)
    }
}

const fn detect_const_inner(bytes: &[u8]) -> crate::detect::DetectionResult {
    crate::detect::DetectionResult::Incomplete // TODO: implement
}

#[cfg(test)]
#[test]
#[ignore]
fn gen_index() {
    use std::io::{BufRead, Write};
    let infile = "src/multi_byte/big5/index.big5.txt";
    let infile = match std::fs::File::open(&infile) {
        Ok(file) => file,
        Err(err) => panic!("{err:?} for file {infile:?}"),
    };
    let mut infile = std::io::BufReader::new(infile);
    let outfile = "src/multi_byte/big5/big5_cp_gen.rs";
    let mut outfile = match std::fs::File::create(outfile) {
        Ok(file) => file,
        Err(err) => panic!("{err:?} for file {outfile:?}"),
    };
    let mut buf = String::new();
    let mut prev_pointer = 0;
    loop {
        buf.clear();
        match infile.read_line(&mut buf) {
            Ok(0) => return,
            Ok(_) => {
                let line = buf.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                let mut line = line.split_whitespace();
                let pointer = line.next().unwrap();
                let codepoint = line.next().unwrap();
                assert_eq!(&codepoint[..2], "0x");
                let pointer = usize::from_str_radix(pointer, 10).unwrap();
                let codepoint = u32::from_str_radix(&codepoint[2..], 16).unwrap();

                while prev_pointer != 0 && prev_pointer + 1 != pointer {
                    assert!(
                        prev_pointer < pointer,
                        "expected: {prev_pointer} < {pointer}"
                    );
                    write!(outfile, "    '\\0',\n").unwrap();
                    prev_pointer += 1;
                }
                write!(outfile, "    '\\u{{{codepoint:04X}}}', // {pointer}\n").unwrap();
                prev_pointer = pointer;
            }
            Err(err) => panic!("{}", err),
        }
    }
}
