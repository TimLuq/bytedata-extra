use crate::parser::{ParseError, ParseItem, ParseResult};

use super::{Name, Spanned, SpannedString};

pub struct Pi<'a> {
    pub spanned: Spanned<'a>,
    pub target: Name<'a>,
    pub data: Option<SpannedString<'a>>,
}

impl<'a> ParseItem<'a> for Pi<'a> {
    fn can_parse(input: &bytedata::StringQueue<'a>) -> Result<usize, ParseError> {
        let mut iter = input.chars_indecies();
        let Some((_, chr)) = iter.next() else {
            return Err(ParseError::Incomplete);
        };
        if chr != '<' {
            return Err(ParseError::NoMatch);
        }
        let Some((_, chr)) = iter.next() else {
            return Err(ParseError::Incomplete);
        };
        if chr != '?' {
            return Err(ParseError::NoMatch);
        }
        let Some((pos, _)) = iter.next() else {
            return Err(ParseError::Incomplete);
        };
        core::mem::drop(iter);

        let mut input = input.slice(pos..);
        let target = Name::can_parse(&input)?;
        input.drain(..target);

        let iter = input.chars_indecies();
        let mut space = 0;
        let mut state = 0;
        for (pos, chr) in iter {
            match state {
                0 => {
                    if chr == '?' {
                        if space == 1 {
                            space = pos;
                        }
                        state = 1;
                    } else if space <= 1 && super::ws_char(chr) {
                        space = 1;
                    } else if space == 0 {
                        return Err(ParseError::NoMatch);
                    }
                }
                1 => {
                    state = if chr == '>' {
                        2
                    } else {
                        if space == 0 {
                            return Err(ParseError::NoMatch);
                        }
                        if chr == '?' {
                            1
                        } else {
                            0
                        }
                    };
                }
                2 => {
                    return Ok(pos);
                }
                _ => unreachable!(),
            }
        }

        if state != 2 {
            Err(ParseError::Incomplete)
        } else {
            Ok(input.len())
        }
    }
    
    fn parse_item(input: &bytedata::StringQueue<'a>, location: &crate::parser::Location<'a>) -> ParseResult<Self> {
        let mut iter = input.chars_indecies();
        let Some((_, chr)) = iter.next() else {
            return Err(ParseError::Incomplete);
        };
        if chr != '<' {
            return Err(ParseError::NoMatch);
        }
        let Some((_, chr)) = iter.next() else {
            return Err(ParseError::Incomplete);
        };
        if chr != '?' {
            return Err(ParseError::NoMatch);
        }
        let Some((pos, _)) = iter.next() else {
            return Err(ParseError::Incomplete);
        };
        core::mem::drop(iter);

        let mut input = input.slice(pos..);
        let mut live_loc = location.clone();
        live_loc.byte += pos as u64;
        live_loc.column += 2;
        let target = Name::parse_item(&input, &live_loc)?;
        input.drain(..target.1);
        live_loc.column += target.0.chars().count() as u64;
        live_loc.byte += target.1 as u64;
        let prefix_len = target.1 + pos;

        let iter = input.chars_indecies();
        let mut nl = false;
        let mut space = 0;
        let mut state = 0;
        let mut end = 0;
        for (pos, chr) in iter {
            match state {
                0 => {
                    if space == 1 {
                        space = pos;
                    }
                    if chr == '?' {
                        end = pos;
                        state = 1;
                    } else if space == 0 && super::ws_char(chr) {
                        space = 1;
                        nl = chr == '\n' || chr == '\r';
                    } else if space == 0 {
                        return Err(ParseError::NoMatch);
                    }
                }
                1 => {
                    state = if chr == '>' {
                        2
                    } else {
                        if space == 0 {
                            return Err(ParseError::NoMatch);
                        }
                        if chr == '?' {
                            end = pos;
                            1
                        } else {
                            0
                        }
                    };
                }
                2 => {
                    let data = if space != 0 {
                        live_loc.byte += space as u64;
                        if nl {
                            live_loc.line += 1;
                            live_loc.column = 1;
                        } else {
                            live_loc.column += 1;
                        }
                        core::mem::drop(input.drain(end..));
                        Some(SpannedString {
                            span: Spanned { start: live_loc, len: end - space },
                            data: input.into(),
                        })
                    } else {
                        None
                    };
                    return Ok((Pi {
                        spanned: Spanned { start: location.clone(), len: pos + prefix_len },
                        target: target.0,
                        data,
                    }, pos));
                }
                _ => unreachable!(),
            }
        }

        if state != 2 {
            return Err(ParseError::Incomplete);
        }

        let len = prefix_len + input.len();
        let data = if space != 0 {
            live_loc.byte += space as u64;
            if nl {
                live_loc.line += 1;
                live_loc.column = 1;
            } else {
                live_loc.column += 1;
            }
            core::mem::drop(input.drain(end..));
            Some(SpannedString {
                span: Spanned { start: live_loc, len: end - space },
                data: input.into(),
            })
        } else {
            None
        };
        Ok((Pi {
            spanned: Spanned { start: location.clone(), len },
            target: target.0,
            data,
        }, len))
    }
}
