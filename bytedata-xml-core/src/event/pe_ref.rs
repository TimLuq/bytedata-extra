use super::{Spanned, SpannedString};

pub struct PERef<'a> {
    pub spanned: Spanned<'a>,
    pub entity: SpannedString<'a>,
}
