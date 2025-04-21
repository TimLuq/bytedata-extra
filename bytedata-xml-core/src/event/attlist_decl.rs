use super::{Spanned, SpannedString};

pub struct AttlistDecl<'a> {
    pub spanned: Spanned<'a>,
    pub name: SpannedString<'a>,
    pub att_defs: smallvec::SmallVec<[AttDef<'a>; 2]>,
}

pub struct AttDef<'a> {
    pub spanned: Spanned<'a>,
    pub name: SpannedString<'a>,
    pub att_type: AttType<'a>,
    pub default: DefaultDecl<'a>,
}

pub enum AttType<'a> {
    CData,
    ID,
    IDRef,
    IDRefs,
    Entity,
    Entities,
    Nmtoken,
    Nmtokens,
    Notation(Notation<'a>),
    Enumeration(Enumeration<'a>),
}

pub struct Notation<'a> {
    pub spanned: Spanned<'a>,
    pub names: smallvec::SmallVec<[SpannedString<'a>; 2]>,
}

pub struct Enumeration<'a> {
    pub spanned: Spanned<'a>,
    pub names: smallvec::SmallVec<[SpannedString<'a>; 2]>,
}
