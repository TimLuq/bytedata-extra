mod event_enum;
pub use event_enum::*;

mod spanned;
pub use spanned::*;

mod attlist_decl;
pub use attlist_decl::*;
mod attribute;
pub use attribute::*;
mod cdata;
pub use cdata::*;
mod char_ref;
pub use char_ref::*;
mod comment;
pub use comment::*;
mod doctype;
pub use doctype::*;
mod element_decl;
pub use element_decl::*;
mod enitity_ref;
pub use enitity_ref::*;
mod markup_decl;
pub use markup_decl::*;
mod nmtoken;
pub use nmtoken::*;
mod pe_ref;
pub use pe_ref::*;
mod pi;
pub use pi::*;
mod tag_end;
pub use tag_end::*;
mod tag_start;
pub use tag_start::*;

fn ws_char(chr: char) -> bool {
    matches!(chr, ' ' | '\t' | '\n' | '\r')
}
