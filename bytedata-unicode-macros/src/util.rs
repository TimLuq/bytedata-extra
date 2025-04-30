use proc_macro2::{Span, TokenStream, TokenTree};

pub(crate) fn parse_file_param(
    input: TokenStream,
) -> Result<(String, std::fs::File), (Span, &'static str)> {
    let mut input = input.into_iter();
    let Some(path) = input.next() else {
        return Err((Span::call_site(), "Expected a quoted file path"));
    };
    let (path_span, path) = match path {
        TokenTree::Literal(lit) => (lit.span(), lit.to_string()),
        tok => return Err((tok.span(), "Expected a string literal")),
    };
    let Some(path) = path
        .strip_prefix('"')
        .and_then(|path| path.strip_suffix('"'))
    else {
        return Err((path_span, "Expected a string literal"));
    };
    if let Some(p) = input.next() {
        return Err((p.span(), "Unexpected token after file path"));
    };
    std::fs::File::open(path)
        .map_err(|_| (path_span, "Failed to open file"))
        .map(|x| (path.to_owned(), x))
}
