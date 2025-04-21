#![no_std]

extern crate alloc;

mod error;
mod line_source;

pub use error::ParseError;
pub use line_source::LineSource;

mod names_list;
mod title_page;

pub use names_list::NamesList;
pub use title_page::TitlePage;
