use super::{Spanned, SpannedString};

pub struct ElementDecl<'a> {
    pub spanned: Spanned<'a>,
    pub name: SpannedString<'a>,
    pub content: ContentSpec<'a>,
}

pub enum ContentSpec<'a> {
    Empty,
    Any,
    Mixed(Mixed<'a>),
    Children(Children<'a>),
}
