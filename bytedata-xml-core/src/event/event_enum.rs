use crate::parser::Location;

use super::{Attribute, CData, CharRef, Comment, DoctypeClose, DoctypeOpen, EntityRef, MarkupDecl, PERef, Pi, SpannedString, TagEnd, TagStart};


pub enum Event<'a> {
    /// `TagStart { name: $name }`
    /// - (`Attr`)\*
    /// - 1. `TagEnd { name: None }`
    ///   2. `TagOpen`
    ///       - (`Character` | `Comment` | `Pi` | `TagStart ..`)\*
    ///       - `TagEnd { name: Some($name) }`?
    TagStart(TagStart<'a>),
    Attr(Attribute<'a>),
    TagOpen(Location<'a>),
    TagEnd(TagEnd<'a>),
    Pi(Pi<'a>),
    Comment(Comment<'a>),
    Text(SpannedString<'a>),
    CData(CData<'a>),

    CharRef(CharRef<'a>),
    EntityRef(EntityRef<'a>),
    PERef(PERef<'a>),
    
    /// `DoctypeOpen { internal_subset: $subset, .. }`
    /// - *if `internal_subset` is `Some`*: (`DoctypeInternalSubset`)\*
    /// - `DoctypeClose`
    DoctypeOpen(DoctypeOpen<'a>),
    /// Should only appear after a `DoctypeOpen` with `internal_subset` set to `Some`.
    DoctypeInternalSubset(MarkupDecl<'a>),
    DoctypeClose(DoctypeClose<'a>),

    Eof,
}
