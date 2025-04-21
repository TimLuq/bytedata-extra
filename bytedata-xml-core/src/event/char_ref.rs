use super::Spanned;

pub struct CharRef<'a> {
    pub spanned: Spanned<'a>,
    pub value: u32,
}
