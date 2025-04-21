use std::collections::VecDeque;

use proc_macro2::{Ident, Punct, Span, TokenStream, TokenTree};
use quote::quote_spanned;

pub(crate) fn parse_tokens(tokens: TokenStream) -> TokenStream {
    let mut items = tokens.into_iter().collect::<VecDeque<_>>();
    if items.is_empty() {
        return syn::Error::new(proc_macro2::Span::call_site(), "Empty pattern").to_compile_error();
    }
    let assign_data;
    if items.len() > 4 {
        match (&items[0], &items[1], &items[2], &items[3]) {
            (TokenTree::Ident(assign), TokenTree::Ident(ident), TokenTree::Punct(punct), _) if (assign == "let" || assign == "static") && punct.as_char() == '=' => {
                assign_data = Some((None, assign.clone(), ident.clone(), punct.clone()));
            }
            (TokenTree::Ident(vis), TokenTree::Ident(assign), TokenTree::Ident(ident), TokenTree::Punct(punct)) if vis == "pub" && (assign == "let" || assign == "static") && punct.as_char() == '=' => {
                assign_data = Some((Some(vis.clone()), assign.clone(), ident.clone(), punct.clone()));
            }
            _ => {
                assign_data = None;
            }
        }
        if let Some(assign_data) = assign_data.as_ref() {
            if assign_data.0.is_some() {
                items.pop_front();
            }
            items.pop_front();
            items.pop_front();
            items.pop_front();
        }
    } else {
        assign_data = None;
    }
    let ParseState {
        capture_groups,
        tree,
    } = match parse(items) {
        Ok(state) => state,
        Err(err) => return err,
    };
    let Some((vis, assign, ident, punct)) = assign_data else {
        return tree.into_pattern().into_stream();
    };
    let mut output = TokenStream::new();
    if let Some(vis) = vis {
        output.extend([TokenTree::from(vis)]);
    }
    output.extend([
        TokenTree::Ident(assign),
        TokenTree::Ident(ident),
        TokenTree::Punct(Punct::new(':', proc_macro2::Spacing::Alone)),
        TokenTree::Ident(Ident::new("bytedata_pattern_core", Span::call_site())),
        TokenTree::Punct(Punct::new(':', proc_macro2::Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', proc_macro2::Spacing::Joint)),
        TokenTree::Ident(Ident::new("Pattern", Span::call_site())),
        TokenTree::Punct(punct),
    ]);
    output.extend(tree.into_pattern().into_stream());
    output

}

struct ParseState {
    capture_groups: Vec<Option<Ident>>,
    tree: RetItem,
}

fn parse(mut tokens: VecDeque<TokenTree>) -> Result<ParseState, TokenStream> {
    let mut chain = Vec::with_capacity(8);
    let mut capture_groups = Vec::new();
    while let Some(peek) = tokens.front() {
        match peek {
            lit@TokenTree::Literal(_) => {
                let lit = syn::parse2::<syn::Lit>(lit.clone().into()).map_err(|err| err.into_compile_error())?;
                match lit {
                    syn::Lit::Str(lit) => {
                        chain.push(wrap_lit_str(lit.span(), lit.value().as_str()));
                    }
                    syn::Lit::Char(lit) => {
                        let mut ch = [0; 4];
                        let ch = lit.value().encode_utf8(&mut ch);
                        chain.push(wrap_lit_str(lit.span(), ch));
                    }
                    _ => {
                        return Err(syn::Error::new(lit.span(), "Unexpected literal in pattern").into_compile_error());
                    }
                }
            }
            TokenTree::Group(group) => {
                let span = group.span();
                let mut tokens = group.stream().into_iter().collect::<VecDeque<_>>();
                tokens.pop_front();
                let ParseState {
                    capture_groups: inner_capture_groups,
                    tree,
                } = parse(tokens)?;
                let ident = syn::Ident::new(&format!("group_{}", chain.len()), span);
                capture_groups.push(Some(ident.clone()));
                let tree = tree.into_stream();
                chain.push(RetItem::Group(span, quote_spanned!(span => bytedata_pattern_core::Group::new(#tree).capturing())));
            }
            peek => {
                return Err(syn::Error::new(peek.span(), "Unexpected token in pattern").into_compile_error());
            }
        }
    }

    if chain.is_empty() {
        return Err(syn::Error::new(proc_macro2::Span::call_site(), "Empty pattern").into_compile_error());
    }

    if chain.len() == 1 {
        return Ok(ParseState {
            capture_groups,
            tree: chain.pop().unwrap(),
        });
    }

    let only_tests = chain.iter().all(|item| matches!(item, RetItem::Test(_, _)));
    let tree = if only_tests {
        let chain = chain.into_iter().map(RetItem::into_stream);
        let stream = quote_spanned! {Span::call_site() =>
            bytedata_pattern_core::Test::Join(&[#(#chain),*])
        };
        RetItem::Test(Span::call_site(), stream)
    } else {
        let chain = chain.into_iter().map(RetItem::into_pattern).map(RetItem::into_stream);
        let stream = quote_spanned! {Span::call_site() =>
            bytedata_pattern_core::Pattern::Join(&[#(#chain),*])
        };
        RetItem::Pattern(Span::call_site(), stream)
    };
    Ok(ParseState {
        capture_groups,
        tree,
    })
}

enum RetItem {
    Pattern(Span, TokenStream),
    Test(Span, TokenStream),
    Group(Span, TokenStream),
}

impl RetItem {
    fn into_pattern(self) -> RetItem {
        match self {
            p@RetItem::Pattern(_, _) => p,
            RetItem::Test(span, stream) => {
                let stream = quote_spanned!(span => bytedata_pattern_core::Pattern::Test(#stream));
                RetItem::Pattern(span, stream)
            }
            RetItem::Group(span, stream) => {
                let stream = quote_spanned!(span => bytedata_pattern_core::Pattern::Group(#stream));
                RetItem::Pattern(span, stream)
            }
        }
    }

    fn into_stream(self) -> TokenStream {
        match self {
            RetItem::Pattern(_, stream) => stream,
            RetItem::Test(_, stream) => stream,
            RetItem::Group(_, stream) => stream,
        }
    }
}

fn wrap_lit_str(span: Span, value: &str) -> RetItem {
    let stream = quote_spanned!(span => bytedata_pattern_core::Test::verbatim(#value));
    RetItem::Test(span, stream)
}
