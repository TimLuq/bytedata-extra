use proc_macro::{Ident, Punct, TokenStream, TokenTree};

mod names_list;
mod util;

/// Generate a list of Unicode character names from a `NamesList.txt` file.
/// The expected input is a string literal containing the path to the file relative to the project root.
#[proc_macro]
pub fn names_list(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let (path, file) = match util::parse_file_param(input) {
        Ok((path, file)) => (path, std::io::BufReader::new(file)),
        Err((span, err)) => return TokenStream::from_iter([
            TokenTree::Ident(Ident::new("compile_error", span.unwrap())),
            TokenTree::Punct(Punct::new('!', proc_macro::Spacing::Alone)),
            TokenTree::Literal(proc_macro::Literal::string(err)),
        ]),
    };

    proc_macro::TokenStream::from(names_list::names_list(path, file))
}