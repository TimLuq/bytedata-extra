use alloc::{borrow::ToOwned, string::String, vec::Vec};

use crate::ParseError;

pub struct TitlePage {

}

impl TitlePage {
    pub const fn parser() -> TitlePageParser {
        TitlePageParser {
            title: None,
            lines: Vec::new(),
        }
    }
}

enum TitlePart {
    SubTitle(String),
    SubHeader(String),
    Notice(String),
    Comment(bool, ExpandLine),
    PageBreak(String),
    Summary(String),
}

impl TitlePart {
    fn parse_line(line_no: u64, line: &str) -> Result<Self, ParseError> {
        if let Some(line) = line.strip_prefix("@@@+\t") {
            Ok(TitlePart::SubTitle(line.to_owned()))
        } else if let Some(line) = line.strip_prefix("@\t") {
            Ok(TitlePart::SubHeader(line.to_owned()))
        } else if let Some(line) = line.strip_prefix("@+\t") {
            Ok(TitlePart::Notice(line.to_owned()))
        } else if let Some(line) = line.strip_prefix("\t* ") {
            Ok(TitlePart::Comment(true, ExpandLine::parse_line(line_no, line)?))
        } else if let Some(line) = line.strip_prefix("@@@-\t") {
            Ok(TitlePart::PageBreak(line.to_owned()))
        } else if let Some(line) = line.strip_prefix("@@@=\t") {
            Ok(TitlePart::Summary(line.to_owned()))
        } else if let Some(line) = line.strip_prefix("\t") {
            Ok(TitlePart::Comment(false, ExpandLine::parse_line(line_no, line)?))
        } else {
            Err(ParseError::UnexpectedLineType(line_no))
        }
    }
}

struct TitlePageParser {
    title: Option<String>,
    lines: Vec<TitlePart>,
}

impl TitlePageParser {
    #[inline]
    pub fn parse_line(&mut self, line_no: u64, line: &str) -> Result<(), ParseError> {
        if self.title.is_some() {
            let part = TitlePart::parse_line(line_no, line)?;
            self.lines.push(part);
            return Ok(());
        }
        if let Some(line) = line.strip_prefix("@@@\t") {
            self.title = Some(line.trim().to_owned());
            Ok(())
        } else {
            Err(ParseError::UnexpectedLineType(line_no))
        }
    }

    pub fn finish(self) -> Result<TitlePage, ParseError> {
        Ok(TitlePage {})
    }
}
