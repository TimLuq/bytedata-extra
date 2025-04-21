use bytedata::StringQueue;

use super::Spanned;

pub struct CData<'a> {
    pub spanned: Spanned<'a>,
    pub text: StringQueue<'a>,
}
