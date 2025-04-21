use crate::parser::ParseItem;

use super::{Name, Spanned};

pub struct TagEnd<'a> {
    pub spanned: Spanned<'a>,
    pub name: Option<Name<'a>>,
}

impl<'a> ParseItem<'a> for TagEnd<'a> {
    fn parse_item(input: &bytedata::StringQueue<'a>, location: &crate::parser::Location<'a>) -> crate::parser::ParseResult<Self> {
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
        if chr != '/' {
            return Err(crate::parser::ParseError::NoMatch);
        }
        let Some((pos, _)) = iter.next() else {
            return Err(crate::parser::ParseError::Incomplete);
        };
        core::mem::drop(iter);

        let mut input = input.slice(pos..);
        let mut name_loc = location.clone();
        name_loc.byte += pos as u64;
        name_loc.column += 2;
        let name = super::Name::parse_item(&input, &name_loc)?;
        drop(input.drain(..name.1));
        
        let mut iter = input.chars_indecies();
        loop {
            let Some((_, chr)) = iter.next() else {
                return Err(crate::parser::ParseError::Incomplete);
            };
            if chr == '>' {
                break;
            }
            if super::ws_char(chr) {
                continue;
            }
            return Err(crate::parser::ParseError::NoMatch);
        }
        let pos = iter.next().map(|(pos, _)| pos).unwrap_or_else(|| input.len()) + pos + name.1;
        let spanned = Spanned { start: location.clone(), len: pos };
        Ok((TagEnd { spanned, name: Some(name.0) }, pos))
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
        if chr != '/' {
            return Err(crate::parser::ParseError::NoMatch);
        }
        let Some((pos, _)) = iter.next() else {
            return Err(crate::parser::ParseError::Incomplete);
        };
        core::mem::drop(iter);

        let mut input = input.slice(pos..);
        let name = super::Name::can_parse(&input)?;
        drop(input.drain(..name));
        
        let mut iter = input.chars_indecies();
        loop {
            let Some((_, chr)) = iter.next() else {
                return Err(crate::parser::ParseError::Incomplete);
            };
            if chr == '>' {
                break;
            }
            if super::ws_char(chr) {
                continue;
            }
            return Err(crate::parser::ParseError::NoMatch);
        }
        let pos = iter.next().map(|(pos, _)| pos).unwrap_or_else(|| input.len()) + pos + name;
        Ok(pos)
    }
}
