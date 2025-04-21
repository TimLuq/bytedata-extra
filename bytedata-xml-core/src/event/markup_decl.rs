use super::{Comment, ElementDecl, PERef, Pi};

pub enum MarkupDecl<'a> {
    ElementDecl(ElementDecl<'a>),
    AttListDecl(AttListDecl<'a>),
    EntityDecl(EntityDecl<'a>),
    NotationDecl(NotationDecl<'a>),
    Pi(Pi<'a>),
    Comment(Comment<'a>),
    PERef(PERef<'a>),
}
