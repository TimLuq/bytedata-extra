use super::spanned::SpannedString;

pub struct Attribute<'a> {
    pub name: SpannedString<'a>,
    pub value: SpannedString<'a>,
}
