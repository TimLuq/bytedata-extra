use crate::parser::ParseItem;

use super::{Name, Spanned, SpannedString};

pub struct DoctypeOpen<'a> {
    pub spanned: Spanned<'a>,
    pub doctype: SpannedString<'a>,
    pub name: Name<'a>,
    pub public_id: Option<SpannedString<'a>>,
    pub system_id: Option<SpannedString<'a>>,
    pub internal_subset: Option<Spanned<'a>>,
}

pub struct DoctypeClose<'a> {
    pub spanned: Spanned<'a>,
}

impl<'a> ParseItem<'a> for DoctypeOpen<'a> {
    fn parse_item(input: &bytedata::StringQueue<'a>, location: &crate::parser::Location<'a>) -> crate::parser::ParseResult<Self> {
        todo!()
    }
    
    fn can_parse(input: &bytedata::StringQueue<'a>) -> Result<usize, crate::parser::ParseError> {
        let mut iter = input.chars_indecies();
        let Some((_, chr)) = iter.next() else {
            return Err(crate::parser::ParseError::Incomplete);
        };
        if chr != '<' {
            return Err(crate::parser::ParseError::NoMatch);
        }
        let Some((_, chr)) = iter.next() else {
            return Err(crate::parser::ParseError::Incomplete);
        };
        if chr != '!' {
            return Err(crate::parser::ParseError::NoMatch);
        }
        let Some((dt_start, chr)) = iter.next() else {
            return Err(crate::parser::ParseError::Incomplete);
        };
        if chr != 'D' && chr != 'd' {
            return Err(crate::parser::ParseError::NoMatch);
        }
        let Some((_, chr)) = iter.next() else {
            return Err(crate::parser::ParseError::Incomplete);
        };
        if chr != 'O' && chr != 'o' {
            return Err(crate::parser::ParseError::NoMatch);
        }
        let Some((_, chr)) = iter.next() else {
            return Err(crate::parser::ParseError::Incomplete);
        };
        if chr != 'C' && chr != 'c' {
            return Err(crate::parser::ParseError::NoMatch);
        }
        let Some((_, chr)) = iter.next() else {
            return Err(crate::parser::ParseError::Incomplete);
        };
        if chr != 'T' && chr != 't' {
            return Err(crate::parser::ParseError::NoMatch);
        }
        let Some((_, chr)) = iter.next() else {
            return Err(crate::parser::ParseError::Incomplete);
        };
        if chr != 'Y' && chr != 'y' {
            return Err(crate::parser::ParseError::NoMatch);
        }
        let Some((_, chr)) = iter.next() else {
            return Err(crate::parser::ParseError::Incomplete);
        };
        if chr != 'P' && chr != 'p' {
            return Err(crate::parser::ParseError::NoMatch);
        }
        let Some((_, chr)) = iter.next() else {
            return Err(crate::parser::ParseError::Incomplete);
        };
        if chr != 'E' && chr != 'e' {
            return Err(crate::parser::ParseError::NoMatch);
        }

        // Parse the doctype name
        let Some((dt_end, chr)) = iter.next() else {
            return Err(crate::parser::ParseError::Incomplete);
        };
        if !super::ws_char(chr) {
            return Err(crate::parser::ParseError::NoMatch);
        }
        let mut iter = iter.peekable();
        let end = loop {
            let Some((end, chr)) = iter.peek() else {
                return Err(crate::parser::ParseError::Incomplete);
            };
            if super::ws_char(chr) {
                iter.next();
                continue;
            }
            break end;
        };
        let mut input = input.slice(end..);
        let name = Name::can_parse(&input)?;
        drop(input.drain(..name));

        // Parse the external id
        let mut has_space = false;
        
    }
}
