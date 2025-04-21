use core::ops::Deref;

use smallvec::SmallVec;

use crate::parser::{Location, ParseError, ParseItem, ParseResult};

use super::{Spanned, SpannedString};

pub struct Nmtoken<'a> {
    inner: SpannedString<'a>,
}

impl<'a> Nmtoken<'a> {
    pub fn new(value: SpannedString<'a>) -> Self {
        Self { inner: value }
    }
}

impl<'a> From<SpannedString<'a>> for Nmtoken<'a> {
    fn from(value: SpannedString<'a>) -> Self {
        Self::new(value)
    }
}

impl<'a> From<Nmtoken<'a>> for SpannedString<'a> {
    fn from(value: Nmtoken<'a>) -> Self {
        value.inner
    }
}

impl<'a> Deref for Nmtoken<'a> {
    type Target = SpannedString<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> ParseItem<'a> for Nmtoken<'a> {
    fn parse_item(input: &bytedata::StringQueue<'a>, location: &Location<'a>) -> ParseResult<Self> {
        let mut iter = input.chars_indecies();
        let Some((_, chr)) = iter.next() else {
            return Err(ParseError::Incomplete);
        };
        if !Name::name_char(chr) {
            return Err(ParseError::NoMatch);
        }
        
        while let Some((pos, chr)) = input.chars_indecies().next() {
            if Name::name_char(chr) {
                continue;
            }
            return Ok((Nmtoken { inner: SpannedString { span: Spanned { start: location.clone(), len: pos }, data: input.slice(0..pos).into() } }, pos));
        }
        Err(ParseError::Incomplete)
    }

    fn can_parse(input: &bytedata::StringQueue<'a>) -> Result<usize, ParseError> {
        let mut iter = input.chars_indecies();
        let Some((_, chr)) = iter.next() else {
            return Err(ParseError::Incomplete);
        };
        if !Name::name_char(chr) {
            return Err(ParseError::NoMatch);
        }
        
        while let Some((pos, chr)) = input.chars_indecies().next() {
            if Name::name_char(chr) {
                continue;
            }
            return Ok(pos);
        }
        Err(ParseError::Incomplete)
    }
}

impl<'a> Nmtoken<'a> {
    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }

    pub(crate) fn can_parse_nmtokens(input: &bytedata::StringQueue<'a>) -> Result<usize, ParseError> {
        let mut len = Self::can_parse(input)?;
        let mut input = input.slice(len..);
        while input.starts_with(b" ") {
            core::mem::drop(input.drain(0..1));
            match Self::can_parse(&input) {
                Ok(l) => {
                    core::mem::drop(input.drain(0..l + 1));
                    len += l + 1;
                    continue;
                }
                Err(ParseError::NoMatch) => return Ok(len),
                Err(ParseError::Incomplete) => return Err(ParseError::Incomplete),
            }
        }
        if input.is_empty() {
            return Err(ParseError::Incomplete);
        }
        Ok(len)
    }

    pub(crate) fn parse_nmtokens(input: &bytedata::StringQueue<'a>, location: &Location<'a>) -> ParseResult<SmallVec<[Nmtoken<'a>; 1]>> {
        let (fst, mut len) = Self::parse_item(input, location)?;
        let mut names = SmallVec::from_const([fst]);
        let mut location = Location::new(location.path.clone(), location.line, location.column + len as u64, location.byte + len as u64);
        let mut input = input.slice(len..);
        while input.starts_with(b" ") {
            core::mem::drop(input.drain(0..1));
            match Self::parse_item(&input, &location) {
                Ok((n, l)) => {
                    core::mem::drop(input.drain(0..l));
                    len += l + 1;
                    names.push(n);
                    location.column += l as u64 + 1;
                    location.byte += l as u64 + 1;
                    continue;
                }
                Err(ParseError::NoMatch) => return Ok((names, len)),
                Err(ParseError::Incomplete) => return Err(ParseError::Incomplete),
            }
        }
        if input.is_empty() {
            return Err(ParseError::Incomplete);
        }
        Ok((names, len))
    }
}

impl<'a> core::fmt::Debug for Nmtoken<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self.as_str())
    }
}

pub struct Name<'a> {
    inner: SpannedString<'a>,
}

impl<'a> Deref for Name<'a> {
    type Target = SpannedString<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> Name<'a> {
    pub fn new(value: SpannedString<'a>) -> Self {
        Self { inner: value }
    }

    pub(crate) fn name_start_char(chr: char) -> bool {
        matches!(chr, 'A'..='Z' | 'a'..='z' | '_' | ':' | '\u{C0}'..='\u{D6}' | '\u{D8}'..='\u{F6}' | '\u{F8}'..='\u{2FF}' | '\u{370}'..='\u{37D}' | '\u{37F}'..='\u{1FFF}' | '\u{200C}'..='\u{200D}' | '\u{2070}'..='\u{218F}' | '\u{2C00}'..='\u{2FEF}' | '\u{3001}'..='\u{D7FF}' | '\u{F900}'..='\u{FDCF}' | '\u{FDF0}'..='\u{FFFD}' | '\u{10000}'..='\u{EFFFF}')
    }

    pub(crate) fn name_char(chr: char) -> bool {
        Self::name_start_char(chr) || matches!(chr, '-' | '.' | '0'..='9' | '\u{B7}' | '\u{0300}'..='\u{036F}' | '\u{203F}'..='\u{2040}')
    }

    pub(crate) fn can_parse_names(input: &bytedata::StringQueue<'a>) -> Result<usize, ParseError> {
        let mut len = Self::can_parse(input)?;
        let mut input = input.slice(len..);
        while input.starts_with(b" ") {
            core::mem::drop(input.drain(0..1));
            match Self::can_parse(&input) {
                Ok(l) => {
                    core::mem::drop(input.drain(0..l + 1));
                    len += l + 1;
                    continue;
                }
                Err(ParseError::NoMatch) => return Ok(len),
                Err(ParseError::Incomplete) => return Err(ParseError::Incomplete),
            }
        }
        if input.is_empty() {
            return Err(ParseError::Incomplete);
        }
        Ok(len)
    }

    pub(crate) fn parse_names(input: &bytedata::StringQueue<'a>, location: &Location<'a>) -> ParseResult<SmallVec<[Name<'a>; 1]>> {
        let (fst, mut len) = Self::parse_item(input, location)?;
        let mut names = SmallVec::from_const([fst]);
        let mut location = Location::new(location.path.clone(), location.line, location.column + len as u64, location.byte + len as u64);
        let mut input = input.slice(len..);
        while input.starts_with(b" ") {
            core::mem::drop(input.drain(0..1));
            match Self::parse_item(&input, &location) {
                Ok((n, l)) => {
                    core::mem::drop(input.drain(0..l));
                    len += l + 1;
                    names.push(n);
                    location.column += l as u64 + 1;
                    location.byte += l as u64 + 1;
                    continue;
                }
                Err(ParseError::NoMatch) => return Ok((names, len)),
                Err(ParseError::Incomplete) => return Err(ParseError::Incomplete),
            }
        }
        if input.is_empty() {
            return Err(ParseError::Incomplete);
        }
        Ok((names, len))
    }
}

impl<'a> From<SpannedString<'a>> for Name<'a> {
    fn from(value: SpannedString<'a>) -> Self {
        Self::new(value)
    }
}

impl<'a> From<Name<'a>> for SpannedString<'a> {
    fn from(value: Name<'a>) -> Self {
        value.inner
    }
}

impl<'a> ParseItem<'a> for Name<'a> {
    fn parse_item(input: &bytedata::StringQueue<'a>, location: &Location<'a>) -> ParseResult<Self> {
        let mut iter = input.chars_indecies();
        let Some((_, chr)) = iter.next() else {
            return Err(ParseError::Incomplete);
        };
        if !Name::name_start_char(chr) {
            return Err(ParseError::NoMatch);
        }
        
        while let Some((pos, chr)) = input.chars_indecies().next() {
            if Self::name_char(chr) {
                continue;
            }
            return Ok((Name { inner: SpannedString { span: Spanned { start: location.clone(), len: pos }, data: input.slice(0..pos).into() } }, pos));
        }
        Err(ParseError::Incomplete)
    }

    fn can_parse(input: &bytedata::StringQueue<'a>) -> Result<usize, ParseError> {
        let mut iter = input.chars_indecies();
        let Some((_, chr)) = iter.next() else {
            return Err(ParseError::Incomplete);
        };
        if !Name::name_start_char(chr) {
            return Err(ParseError::NoMatch);
        }
        
        while let Some((pos, chr)) = input.chars_indecies().next() {
            if Self::name_char(chr) {
                continue;
            }
            return Ok(pos);
        }
        Err(ParseError::Incomplete)
    }
}
