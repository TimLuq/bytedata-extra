use proc_macro::TokenStream;

mod parse_tokens;

/// Generate a list of Unicode character names from a `NamesList.txt` file.
/// The expected input is a string literal containing the path to the file relative to the project root.
#[proc_macro]
pub fn pattern(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let res = parse_tokens::parse_tokens(input);
    proc_macro::TokenStream::from(res)
}

#[derive(Debug, Clone)]
enum PatternContext {
    Root,
    Group(std::sync::Arc<(Option<String>, Vec<PatternContext>)>),
}
