use alloc::vec::Vec;

use crate::{line_source::LineSourceBuf, LineSource, ParseError, TitlePage};

pub struct NamesList {
    pub title_page: TitlePage,
    pub blocks: Vec<ExtendedBlock>,
}

impl NamesList {
    pub fn parse<S: LineSource, E: From<S::Error> + From<ParseError>>(source: S) -> Result<Self, E> {
        let mut source = LineSourceBuf::new(source);

        let mut line = loop {
            let mut parser = TitlePage::parser();
            let Some(line) = source.read_line()? else {
                return Err(E::from(ParseError::UnexpectedEndOfFile(source.line_num)));
            };
            let line = line.trim_end();
            if line.is_empty() || line.starts_with(";") {
                continue;
            }
            match parser.parse_line(source.line_num, &line) {
                Ok(()) => continue,
                Err(ParseError::UnexpectedLineType(_)) => break line,
                Err(e) => return Err(e.into()),
            }
        };
        let title_page = parser.finish()?;

        let line = 'outer: loop {
            match parser.parse_line(source.line_num, &line) {
                Ok(()) => (),
                Err(ParseError::UnexpectedLineType(_)) => break line,
                Err(e) => return Err(e.into()),
            }
            loop {
                let Some(nextline) = source.read_line()? else {
                    return Err(E::from(ParseError::UnexpectedEndOfFile(source.line_num)));
                };
                line = nextline;
                line = line.trim_end();
                if !line.is_empty() && !line.starts_with(";") {
                    continue 'outer;
                }
            }
        };
        let title_page = parser.finish()?;
    }
}

pub struct ExtendedBlock {
    block: Block,
    summary: Summary,
}
