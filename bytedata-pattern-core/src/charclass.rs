use crate::test_exec::{ExecFlag, ExecResult, TestExec};


#[derive(Clone, Copy)]
#[non_exhaustive]
pub enum CharacterClass {
    /// Matches any character except a newline.
    Any,
    /// Matches a whitespace character.
    Whitespace,
    /// Matches a non-whitespace character.
    NonWhitespace,
    /// Matches a whitespace character that is not a line separator
    Space,
    /// Matches a word character. (Alphanumeric or `_`)
    Word,
    /// Matches a non-word character. (Non-alphanumeric nor `_`)
    NonWord,
    /// Matches a alphanumeric character.
    Alphanumeric,
    /// Matches an alphabetic character.
    Alpha,
    /// Matches a digit character.
    Digit,
    /// Matches a non-digit character.
    NonDigit,
    /// Matches a hex-digit character.
    HexDigit,
    /// Matches a punctuation character.
    Punctuation,
    /// Matches a control character.
    Control,
    /// Matches a graphic character.
    Graphic,
    /// Matches a printable character
    Print,
    /// Matches a lower case character
    Lower,
    /// Matches an upper case character
    Upper,
}

impl core::fmt::Debug for CharacterClass {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Any => f.write_str("CharacterClass::Any"),
            Self::Whitespace => f.write_str("CharacterClass::Whitespace"),
            Self::NonWhitespace => f.write_str("CharacterClass::NonWhitespace"),
            Self::Space => f.write_str("CharacterClass::Space"),
            Self::Word => f.write_str("CharacterClass::Word"),
            Self::NonWord => f.write_str("CharacterClass::NonWord"),
            Self::Alphanumeric => f.write_str("CharacterClass::Alphanumeric"),
            Self::Alpha => f.write_str("CharacterClass::Alpha"),
            Self::Digit => f.write_str("CharacterClass::Digit"),
            Self::NonDigit => f.write_str("CharacterClass::NonDigit"),
            Self::HexDigit => f.write_str("CharacterClass::HexDigit"),
            Self::Punctuation => f.write_str("CharacterClass::Punctuation"),
            Self::Control => f.write_str("CharacterClass::Control"),
            Self::Graphic => f.write_str("CharacterClass::Graphic"),
            Self::Print => f.write_str("CharacterClass::Print"),
            Self::Lower => f.write_str("CharacterClass::Lower"),
            Self::Upper => f.write_str("CharacterClass::Upper"),
        }
    }
}

impl core::fmt::Display for CharacterClass {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Any => f.write_str("."),
            Self::Whitespace => f.write_str(r"\s"),
            Self::NonWhitespace => f.write_str(r"\S"),
            Self::Space => f.write_str("[[:space:]]"),
            Self::Word => f.write_str(r"\w"),
            Self::NonWord => f.write_str(r"\W"),
            Self::Alphanumeric => f.write_str("[[:alnum:]]"),
            Self::Alpha => f.write_str("[[:alpha:]]"),
            Self::Digit => f.write_str(r"\d"),
            Self::NonDigit => f.write_str(r"\D"),
            Self::HexDigit => f.write_str("[[:xdigit:]]"),
            Self::Punctuation => f.write_str("[[:punct:]]"),
            Self::Control => f.write_str("[[:cntrl:]]"),
            Self::Graphic => f.write_str("[[:graph:]]"),
            Self::Print => f.write_str("[[:print:]]"),
            Self::Lower => f.write_str("[[::lower:]]"),
            Self::Upper => f.write_str("[[::upper:]]"),
        }
    }
}

#[inline(always)]
#[must_use]
const fn is_non_nl_ws(chr: char) -> bool {
    if chr < '\u{2000}' {
        chr == '\t' || chr == ' ' || chr == '\u{00A0}' || chr == '\u{1680}'
    } else {
        chr <= '\u{200A}' || chr == '\u{202F}' || chr == '\u{205F}' || chr == '\u{3000}'
    }
}

#[inline(always)]
#[must_use]
pub(crate) const fn is_full_ws(chr: char) -> bool {
    is_non_nl_ws(chr) || (chr >= '\n' && chr <= '\r') || chr == '\u{0085}' || chr == '\u{2028}' || chr == '\u{2029}'
}

#[inline(always)]
#[must_use]
const fn is_alphabetic(chr: char) -> bool {
    (chr >= 'a' && chr <= 'z') || (chr >= 'A' && chr <= 'Z') // TODO: Unicode
}

#[inline(always)]
#[must_use]
const fn is_numeric(chr: char) -> bool {
    chr.is_ascii_digit() // TODO: Unicode
}

#[inline(always)]
#[must_use]
const fn is_alphanumeric(chr: char) -> bool {
    is_alphabetic(chr) || is_numeric(chr)
}

#[inline(always)]
#[must_use]
const fn is_control(chr: char) -> bool {
    chr.is_ascii_control() || (chr >= '\u{007F}' && chr <= '\u{009F}')
}

impl CharacterClass {
    #[inline]
    pub const fn check_char(&self, chr: char) -> bool {
        match self {
            Self::Any => chr != '\n',
            Self::Whitespace => is_full_ws(chr),
            Self::NonWhitespace => !is_full_ws(chr),
            Self::Space => is_non_nl_ws(chr),
            Self::Word => is_alphanumeric(chr) || chr == '_',
            Self::NonWord => !is_alphanumeric(chr) && chr != '_',
            Self::Alphanumeric => is_alphanumeric(chr),
            Self::Alpha => is_alphabetic(chr),
            Self::Digit => chr.is_ascii_digit(),
            Self::NonDigit => !chr.is_ascii_digit(),
            Self::HexDigit => chr.is_ascii_hexdigit(),
            Self::Punctuation => chr.is_ascii_punctuation(),
            Self::Control => is_control(chr),
            Self::Graphic => chr.is_ascii_graphic(),
            Self::Print => chr.is_ascii_graphic() || is_full_ws(chr),
            Self::Lower => chr.is_lowercase(),
            Self::Upper => chr.is_uppercase(),
        }
    }

    pub(crate) const fn run_test(self, mut exec: TestExec) -> TestExec {
        if exec.chunk.is_empty() {
            exec.result = Some(ExecResult::Incomplete);
            return exec;
        }
        let (chr, len) = bytedata::const_utf8_char_next(exec.chunk);
        if len == 0 {
            exec.result = Some(ExecResult::Incomplete);
            return exec;
        }
        let chr = char::from_u32(chr).unwrap();
        if self.check_char(chr) {
            let len = len as usize;
            let new_chunk_len = exec.chunk.len() - len;
            exec.chunk = unsafe { core::slice::from_raw_parts(exec.chunk.as_ptr().add(len), new_chunk_len) };
            exec.offset += len;
            if CharacterClass::Word.check_char(chr) {
                exec.flags.set(ExecFlag::PrevWord);
            } else {
                exec.flags.unset(ExecFlag::PrevWord);
            }
        } else {
            exec.result = Some(ExecResult::Mismatch);
        }
        exec
    }
}
