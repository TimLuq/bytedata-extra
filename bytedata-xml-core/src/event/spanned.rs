use core::ops::Deref;

use bytedata::{StringData, StringQueue};

use crate::parser::Location;

pub struct Spanned<'a> {
    pub start: Location<'a>,
    pub len: usize,
}

pub struct SpannedString<'a> {
    pub span: Spanned<'a>,
    pub data: StringData<'a>,
}

impl<'a> Deref for SpannedString<'a> {
    type Target = StringData<'a>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub struct SpannedStringQueue<'a> {
    pub span: Spanned<'a>,
    pub data: StringQueue<'a>,
}

impl<'a> Deref for SpannedStringQueue<'a> {
    type Target = StringQueue<'a>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
