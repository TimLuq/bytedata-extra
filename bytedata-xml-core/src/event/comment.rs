use crate::parser::{Location, ParseError, ParseItem, ParseResult};

use super::{Spanned, SpannedStringQueue};

pub struct Comment<'a> {
    pub spanned: Spanned<'a>,
    pub text: SpannedStringQueue<'a>,
}

impl<'a> ParseItem<'a> for Comment<'a> {
    fn parse_item(input: &bytedata::StringQueue<'a>, location: &Location<'a>) -> ParseResult<Self> {
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
        if chr != '!' {
            return Err(ParseError::NoMatch);
        }
        let Some((_, chr)) = iter.next() else {
            return Err(ParseError::Incomplete);
        };
        if chr != '-' {
            return Err(ParseError::NoMatch);
        }
        let Some((_, chr)) = iter.next() else {
            return Err(ParseError::Incomplete);
        };
        if chr != '-' {
            return Err(ParseError::NoMatch);
        }

        let mut start = 0;
        let mut state = 0;
        let mut end = 0;
        for (pos, chr) in iter {
            match state {
                0 => {
                    if start == 0 {
                        start = pos;
                    }
                    if chr == '-' {
                        state = 1;
                        end = pos;
                    }
                }
                1 => {
                    if chr == '-' {
                        state = 2;
                    } else {
                        state = 0;
                    }
                }
                2 => {
                    if chr == '>' {
                        state = 3;
                    } else {
                        // according to the xml standard, comments can't contain '--'
                        // so there should be an error if this occurs in XML
                        state = 0;
                    }
                }
                3 => return Ok((
                    Comment {
                        spanned: Spanned {
                            start: location.clone(),
                            len: pos,
                        },
                        text: SpannedStringQueue {
                            span: Spanned {
                                start: {
                                    let mut loc = location.clone();
                                    loc.byte += start as u64;
                                    loc.column += 4;
                                    loc
                                },
                                len: end - start,
                            },
                            data: input.slice(start..end),
                        },
                    },
                    pos,
                )),
                _ => unreachable!(),
            }
        }

        if state != 3 {
            return Err(ParseError::Incomplete);
        }

        Ok((
            Comment {
                spanned: Spanned {
                    start: location.clone(),
                    len: input.len(),
                },
                text: SpannedStringQueue {
                    span: Spanned {
                        start: {
                            let mut loc = location.clone();
                            loc.byte += start as u64;
                            loc.column += 4;
                            loc
                        },
                        len: end - start,
                    },
                    data: input.slice(start..end),
                },
            },
            input.len(),
        ))
    }

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
        if chr != '!' {
            return Err(ParseError::NoMatch);
        }
        let Some((_, chr)) = iter.next() else {
            return Err(ParseError::Incomplete);
        };
        if chr != '-' {
            return Err(ParseError::NoMatch);
        }
        let Some((_, chr)) = iter.next() else {
            return Err(ParseError::Incomplete);
        };
        if chr != '-' {
            return Err(ParseError::NoMatch);
        }

        let mut state = 0;
        for (pos, chr) in iter {
            match state {
                0 => {
                    if chr == '-' {
                        state = 1;
                    }
                }
                1 => {
                    if chr == '-' {
                        state = 2;
                    } else {
                        state = 0;
                    }
                }
                2 => {
                    if chr == '>' {
                        state = 3;
                    } else {
                        // according to the xml standard, comments can't contain '--'
                        // so there should be an error if this occurs in XML
                        state = 0;
                    }
                }
                3 => {
                    return Ok(pos);
                }
                _ => unreachable!(),
            }
        }
        if state == 3 {
            Ok(input.len())
        } else {
            Err(ParseError::Incomplete)
        }
    }
}
