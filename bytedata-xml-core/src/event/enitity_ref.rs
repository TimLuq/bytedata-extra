use super::{Spanned, Name};

pub struct EntityRef<'a> {
    pub spanned: Spanned<'a>,
    pub entity: Name<'a>,
}
