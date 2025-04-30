use proc_macro2::{Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use std::io::BufRead;

struct CodePoint {
    codepoint: char,
    kind: String,
    names: Vec<String>,
    see_also: Vec<(String, char)>,
    alternative: Vec<Vec<char>>,
    notes: Vec<String>,
}

impl CodePoint {
    fn clear(&mut self) {
        self.codepoint = '\0';
        self.kind.clear();
        self.names.clear();
        self.see_also.clear();
        self.alternative.clear();
        self.notes.clear();
    }

    fn append_to(&self, out: &mut TokenStream) {
        let codepoint = self.codepoint as u32;
        let u_codepoint = format!("U{codepoint:04X}");
        let mut has_doc_empty = true;
        for note in &self.notes {
            out.extend(TokenStream::from_iter([
                TokenTree::Punct(Punct::new('#', Spacing::Joint)),
                TokenTree::Punct(Punct::new('[', Spacing::Joint)),
                TokenTree::Ident(Ident::new("doc", Span::call_site())),
                TokenTree::Punct(Punct::new('=', Spacing::Alone)),
                TokenTree::Literal(Literal::string(note)),
                TokenTree::Punct(Punct::new(']', Spacing::Alone)),
            ]));
            has_doc_empty = false;
        }
        if !self.alternative.is_empty() {
            use std::fmt::Write;
            if !has_doc_empty {
                out.extend(TokenStream::from_iter([
                    TokenTree::Punct(Punct::new('#', Spacing::Joint)),
                    TokenTree::Punct(Punct::new('[', Spacing::Joint)),
                    TokenTree::Ident(Ident::new("doc", Span::call_site())),
                    TokenTree::Punct(Punct::new('=', Spacing::Alone)),
                    TokenTree::Literal(Literal::string("")),
                    TokenTree::Punct(Punct::new(']', Spacing::Alone)),
                ]));
            }
            out.extend(TokenStream::from_iter([
                TokenTree::Punct(Punct::new('#', Spacing::Joint)),
                TokenTree::Punct(Punct::new('[', Spacing::Joint)),
                TokenTree::Ident(Ident::new("doc", Span::call_site())),
                TokenTree::Punct(Punct::new('=', Spacing::Alone)),
                TokenTree::Literal(Literal::string("## Alternative representations:")),
                TokenTree::Punct(Punct::new(']', Spacing::Alone)),
            ]));

            let mut alt_str = String::new();
            for alt in &self.alternative {
                alt_str.clear();
                alt_str.reserve(alt.len() * 8 + 8);
                alt_str.push_str("- `\"");
                for ch in alt {
                    write!(alt_str, "\\u{{{:04X}}}", *ch as u32).unwrap();
                }
                alt_str.push_str("`\"");
                out.extend(TokenStream::from_iter([
                    TokenTree::Punct(Punct::new('#', Spacing::Joint)),
                    TokenTree::Punct(Punct::new('[', Spacing::Joint)),
                    TokenTree::Ident(Ident::new("doc", Span::call_site())),
                    TokenTree::Punct(Punct::new('=', Spacing::Alone)),
                    TokenTree::Literal(Literal::string(&alt_str)),
                    TokenTree::Punct(Punct::new(']', Spacing::Alone)),
                ]));
            }
            has_doc_empty = false;
        }

        out.extend(TokenStream::from_iter([
            TokenTree::Ident(Ident::new("pub", Span::call_site())),
            TokenTree::Ident(Ident::new("const", Span::call_site())),
            TokenTree::Ident(Ident::new(&u_codepoint, Span::call_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("char", Span::call_site())),
            TokenTree::Punct(Punct::new('=', Spacing::Alone)),
            TokenTree::Literal(Literal::character(self.codepoint)),
            TokenTree::Punct(Punct::new(';', Spacing::Alone)),
        ]));

        let alias_doc = format!("Alias for [`{}`].", u_codepoint);
        for alias in &self.names {
            out.extend(TokenStream::from_iter([
                TokenTree::Punct(Punct::new('#', Spacing::Joint)),
                TokenTree::Punct(Punct::new('[', Spacing::Joint)),
                TokenTree::Ident(Ident::new("doc", Span::call_site())),
                TokenTree::Punct(Punct::new('=', Spacing::Alone)),
                TokenTree::Literal(Literal::string(&alias_doc)),
                TokenTree::Punct(Punct::new(']', Spacing::Alone)),
            ]));
            out.extend(TokenStream::from_iter([
                TokenTree::Ident(Ident::new("pub", Span::call_site())),
                TokenTree::Ident(Ident::new("const", Span::call_site())),
                TokenTree::Ident(Ident::new(alias, Span::call_site())),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("const_char", Span::call_site())),
                TokenTree::Punct(Punct::new('=', Spacing::Alone)),
                TokenTree::Ident(Ident::new(&u_codepoint, Span::call_site())),
                TokenTree::Punct(Punct::new(';', Spacing::Alone)),
            ]));
        }
    }
}

fn fail<D: core::fmt::Debug>(
    file_path: &str,
    line_count: u32,
    msg: &str,
    context: D,
) -> TokenStream {
    #[cfg(test)]
    panic!("{msg} (location = {file_path}:{line_count}, context = {context:?})");

    TokenStream::from_iter([
        TokenTree::Ident(Ident::new("compile_error", Span::call_site())),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Literal(Literal::string(&format!(
            "{msg} (location = {file_path}:{line_count}, context = {context:?})"
        ))),
    ])
}

const MAKE_IDENT_CONTINUATION_UPPER: u8 = 0b0001;
const MAKE_IDENT_INITIAL_UPPER: u8 = 0b0010;
const MAKE_IDENT_JOIN_UPPER: u8 = 0b0100;
const MAKE_IDENT_JOIN_UNDERSCORE: u8 = 0b1000;

const MAKE_IDENT_CONTINUATION: u8 = 0b0001_0000;
const MAKE_IDENT_WAS_NON_ALPHANUMERIC: u8 = 0b0010_0000;

#[derive(Clone, Copy)]
struct CharBuf {
    char0: char,
    char1: char,
    data: u8,
}
impl CharBuf {
    #[inline]
    const fn new(flags: u8) -> Self {
        Self {
            char0: '\0',
            char1: '\0',
            data: flags & 0b0011_1111,
        }
    }
    #[inline]
    const fn is_empty(&self) -> bool {
        self.data & 0b1100_0000 == 0
    }

    #[inline]
    fn take_char(&mut self) -> Option<char> {
        match self.data >> 6 {
            0b11 => {
                self.data &= 0b0111_1111;
                Some(self.char1)
            }
            0b01 => {
                self.data &= 0b0011_1111;
                Some(self.char0)
            }
            0b00 => None,
            _ => unreachable!(),
        }
    }

    #[inline]
    fn set_chars<I: Iterator<Item = char>>(&mut self, mut chars: I) {
        let Some(ch1) = chars.next() else {
            self.data &= 0b0011_1111;
            return;
        };
        let Some(ch0) = chars.next() else {
            self.data &= 0b0111_1111;
            self.char0 = ch1;
            return;
        };
        self.char0 = ch0;
        self.char1 = ch1;
        if chars.next().is_none() {
            return;
        }
        panic!("unexpected char sequence of 3 or longer");
    }

    #[inline]
    fn has_flag(&self, flag: u8) -> bool {
        self.data & flag == flag
    }

    #[inline]
    fn set_flag(&mut self, flag: u8) {
        self.data |= flag;
    }

    #[inline]
    fn set_masked(&mut self, mask: u8, flags: u8) {
        self.data = (self.data & mask) | flags;
    }
}

struct MakeIdent<'a>(std::str::Chars<'a>, CharBuf);
impl<'a> MakeIdent<'a> {
    fn camel(s: std::str::Chars<'a>) -> Self {
        const STATE: CharBuf = CharBuf::new(MAKE_IDENT_JOIN_UPPER);
        Self(s, STATE)
    }
    fn pascal(s: std::str::Chars<'a>) -> Self {
        const STATE: CharBuf = CharBuf::new(MAKE_IDENT_INITIAL_UPPER | MAKE_IDENT_JOIN_UPPER);
        Self(s, STATE)
    }
    fn snake(s: std::str::Chars<'a>) -> Self {
        const STATE: CharBuf = CharBuf::new(MAKE_IDENT_JOIN_UNDERSCORE);
        Self(s, STATE)
    }
    fn upper_snake(s: std::str::Chars<'a>) -> Self {
        const STATE: CharBuf = CharBuf::new(
            MAKE_IDENT_JOIN_UNDERSCORE | MAKE_IDENT_CONTINUATION_UPPER | MAKE_IDENT_INITIAL_UPPER,
        );
        Self(s, STATE)
    }
}
impl Iterator for MakeIdent<'_> {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        let ch = self.1.take_char();
        if ch.is_some() {
            return ch;
        }
        let ch = self.0.next()?;
        if ch.is_alphanumeric() {
            if self.1.has_flag(MAKE_IDENT_WAS_NON_ALPHANUMERIC) {
                self.1.set_flag(MAKE_IDENT_CONTINUATION);
                let ret_us = self.1.has_flag(MAKE_IDENT_JOIN_UNDERSCORE);
                if self.1.has_flag(MAKE_IDENT_JOIN_UPPER) {
                    self.1.set_chars(ch.to_uppercase());
                    if ret_us {
                        return Some('_');
                    } else {
                        return Self::next(self);
                    }
                } else if ret_us {
                    return Some('_');
                }
            } else if !self.1.has_flag(MAKE_IDENT_CONTINUATION) {
                self.1.set_flag(MAKE_IDENT_CONTINUATION);
                if self.1.has_flag(MAKE_IDENT_INITIAL_UPPER) {
                    self.1.set_chars(ch.to_uppercase());
                    return Self::next(self);
                }
            } else if self.1.has_flag(MAKE_IDENT_CONTINUATION_UPPER) {
                self.1.set_chars(ch.to_uppercase());
                return Self::next(self);
            }
            self.1.set_chars(ch.to_lowercase());
        } else {
            const MASK: u8 = !(MAKE_IDENT_WAS_NON_ALPHANUMERIC | MAKE_IDENT_CONTINUATION);
            self.1.set_masked(MASK, MAKE_IDENT_WAS_NON_ALPHANUMERIC);
        }
        Self::next(self)
    }
}

pub(super) fn names_list(
    file_path: String,
    mut file: std::io::BufReader<std::fs::File>,
) -> TokenStream {
    let mut out = TokenStream::new();
    let mut current_codepoint = CodePoint {
        codepoint: '\0',
        kind: String::new(),
        names: Vec::new(),
        see_also: Vec::new(),
        notes: Vec::new(),
        alternative: Vec::new(),
    };
    let mut depth = 0_u8;
    let mut buffer = String::new();
    let mut line_count = 0_u32;

    loop {
        buffer.clear();
        match file.read_line(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                line_count += 1;
                let line = buffer.trim_end();
                if line.starts_with(";") {
                    // comment
                    continue;
                }
                if let Some(line) = line.strip_prefix("\t= ") {
                    if depth != 3 {
                        return fail(
                            &file_path,
                            line_count,
                            "Unexpected definition location",
                            depth,
                        );
                    }
                    for part in line.split(',') {
                        let part = part.trim();
                        current_codepoint
                            .names
                            .push(String::from_iter(MakeIdent::upper_snake(part.chars())));
                    }
                    continue;
                }

                if let Some(line) = line.strip_prefix("\t* ") {
                    if depth != 3 {
                        return fail(
                            &file_path,
                            line_count,
                            "Unexpected codepoint documentation",
                            depth,
                        );
                    }
                    current_codepoint.notes.push(line.to_owned());
                    continue;
                }

                if let Some(line) = line.strip_prefix("\tx ") {
                    if depth != 3 {
                        return fail(
                            &file_path,
                            line_count,
                            "Unexpected codepoint see also",
                            depth,
                        );
                    }
                    let Some(part) = line
                        .strip_prefix('(')
                        .and_then(|part| part.strip_suffix(')'))
                    else {
                        let Ok(codepoint) = u32::from_str_radix(line, 16) else {
                            return fail(&file_path, line_count, "Invalid see also syntax", line);
                        };
                        let Some(codepoint) = std::char::from_u32(codepoint) else {
                            return fail(
                                &file_path,
                                line_count,
                                "Invalid codepoint in see also",
                                codepoint,
                            );
                        };
                        current_codepoint
                            .see_also
                            .push((line.to_owned(), codepoint));
                        continue;
                    };
                    let Some(p) = part.rfind(" - ") else {
                        return fail(&file_path, line_count, "Invalid see also syntax", line);
                    };
                    let (name, codepoint) = part.split_at(p);
                    let codepoint = codepoint.strip_prefix(" - ").unwrap();
                    let Ok(codepoint) = u32::from_str_radix(codepoint, 16) else {
                        return fail(&file_path, line_count, "Invalid see also syntax", line);
                    };
                    let Some(codepoint) = std::char::from_u32(codepoint) else {
                        return fail(
                            &file_path,
                            line_count,
                            "Invalid codepoint in see also",
                            codepoint,
                        );
                    };
                    current_codepoint
                        .see_also
                        .push((name.to_owned(), codepoint));
                    continue;
                }

                if let Some(line) = line.strip_prefix("\t: ") {
                    if depth != 3 {
                        return fail(
                            &file_path,
                            line_count,
                            "Unexpected codepoint alternative",
                            depth,
                        );
                    }
                    let mut alternative = Vec::new();
                    for part in line.split(' ') {
                        let part = part.trim();
                        let codepoint = u32::from_str_radix(part, 16).unwrap();
                        let Some(codepoint) = std::char::from_u32(codepoint) else {
                            return fail(
                                &file_path,
                                line_count,
                                "Invalid codepoint in alternative",
                                codepoint,
                            );
                        };
                        alternative.push(codepoint);
                    }
                    current_codepoint.alternative.push(alternative);
                    continue;
                }

                if depth == 3 {
                    current_codepoint.append_to(&mut out);
                    current_codepoint.clear();
                    depth = 2;
                }

                if let Some(line) = line.strip_prefix("@@\t") {
                    // block header
                    if depth == 2 {
                        out.extend([TokenTree::Punct(Punct::new('}', Spacing::Alone))]);
                        depth = 1;
                    }
                    if depth == 1 {
                        out.extend([TokenTree::Punct(Punct::new('}', Spacing::Alone))]);
                        depth = 0;
                    }

                    let Some((codepoint_start, rest)) = line.split_once('\t') else {
                        return fail(&file_path, line_count, "Invalid codepoint line", line);
                    };
                    let Ok(codepoint_start) = u32::from_str_radix(codepoint_start, 16) else {
                        return fail(&file_path, line_count, "Invalid codepoint", codepoint_start);
                    };
                    let Some((rest, codepoint_end)) = rest.split_once('\t') else {
                        return fail(&file_path, line_count, "Invalid codepoint line", line);
                    };
                    let Ok(codepoint_end) = u32::from_str_radix(codepoint_end, 16) else {
                        return fail(&file_path, line_count, "Invalid codepoint", codepoint_end);
                    };

                    out.extend([
                        TokenTree::Punct(Punct::new('#', Spacing::Joint)),
                        TokenTree::Punct(Punct::new('[', Spacing::Joint)),
                        TokenTree::Ident(Ident::new("doc", Span::call_site())),
                        TokenTree::Punct(Punct::new('=', Spacing::Alone)),
                        TokenTree::Literal(Literal::string(rest.trim())),
                        TokenTree::Punct(Punct::new(']', Spacing::Alone)),
                        TokenTree::Punct(Punct::new('#', Spacing::Joint)),
                        TokenTree::Punct(Punct::new('[', Spacing::Joint)),
                        TokenTree::Ident(Ident::new("doc", Span::call_site())),
                        TokenTree::Punct(Punct::new('=', Spacing::Alone)),
                        TokenTree::Literal(Literal::string(&format!(
                            "- Range: 0x{codepoint_start:06X} - 0x{codepoint_end:06X}"
                        ))),
                        TokenTree::Punct(Punct::new(']', Spacing::Alone)),
                        TokenTree::Ident(Ident::new("pub", Span::call_site())),
                        TokenTree::Ident(Ident::new("mod", Span::call_site())),
                        TokenTree::Ident(Ident::new(
                            &String::from_iter(MakeIdent::snake(rest.chars())),
                            Span::call_site(),
                        )),
                        TokenTree::Punct(Punct::new('{', Spacing::Alone)),
                    ]);
                    depth = 1;
                    continue;
                }

                if let Some(line) = line.strip_prefix("@\t") {
                    if depth == 2 {
                        out.extend([TokenTree::Punct(Punct::new('}', Spacing::Alone))]);
                        depth = 1;
                    }
                    if depth != 1 {
                        return fail(&file_path, line_count, "Unexpected section line", depth);
                    }
                    let Some(line) = line.strip_prefix("\t") else {
                        return fail(&file_path, line_count, "Unexpected section line", line);
                    };
                    let name = line.split_once('\t').map(|(x, _)| x).unwrap_or(line);
                    let name = name
                        .split_once('(')
                        .and_then(|(_, b)| b.split_once(')').map(|(a, _)| a))
                        .unwrap_or(name);

                    out.extend([
                        TokenTree::Punct(Punct::new('#', Spacing::Joint)),
                        TokenTree::Punct(Punct::new('[', Spacing::Joint)),
                        TokenTree::Ident(Ident::new("doc", Span::call_site())),
                        TokenTree::Punct(Punct::new('=', Spacing::Alone)),
                        TokenTree::Literal(Literal::string(line.trim())),
                        TokenTree::Punct(Punct::new(']', Spacing::Alone)),
                        TokenTree::Ident(Ident::new("pub", Span::call_site())),
                        TokenTree::Ident(Ident::new("mod", Span::call_site())),
                        TokenTree::Ident(Ident::new(
                            &String::from_iter(MakeIdent::snake(name.chars())),
                            Span::call_site(),
                        )),
                        TokenTree::Punct(Punct::new('{', Spacing::Alone)),
                    ]);
                    depth = 2;
                    continue;
                }

                if let Some(line) = line.strip_prefix("@+\t\t") {
                    if depth == 0 {
                        continue;
                    }
                    if depth == 3 {
                        current_codepoint.notes.push(line.to_owned());
                        continue;
                    }
                    out.extend([
                        TokenTree::Punct(Punct::new('#', Spacing::Joint)),
                        TokenTree::Punct(Punct::new('[', Spacing::Joint)),
                        TokenTree::Ident(Ident::new("doc", Span::call_site())),
                        TokenTree::Punct(Punct::new('=', Spacing::Alone)),
                        TokenTree::Literal(Literal::string(line.trim())),
                        TokenTree::Punct(Punct::new(']', Spacing::Alone)),
                    ]);
                    continue;
                }

                if line.len() < 5
                    || depth != 2
                    || line
                        .split_once('\t')
                        .map(|(x, _)| x.len())
                        .unwrap_or_default()
                        < 4
                {
                    eprintln!("Skipping line: [{depth}] {line:?}");
                    continue;
                }

                let codepoint = &line[..4];
                let Ok(codepoint) = u32::from_str_radix(codepoint, 16) else {
                    return fail(&file_path, line_count, "Invalid codepoint", codepoint);
                };
                let Some(ch) = std::char::from_u32(codepoint) else {
                    return fail(&file_path, line_count, "Invalid codepoint", codepoint);
                };
                let line = &line[5..];
                current_codepoint.codepoint = ch;
                if line.starts_with('<') {
                    let Some(kind) = line
                        .strip_prefix('<')
                        .and_then(|kind| kind.strip_suffix('>'))
                    else {
                        return fail(&file_path, line_count, "Invalid codepoint kind", line);
                    };
                    current_codepoint.kind.push_str(kind);
                    depth = 3;
                    continue;
                }
                current_codepoint
                    .names
                    .push(String::from_iter(MakeIdent::upper_snake(line.chars())));
                depth = 3;
                continue;
            }
            Err(err) => {
                out.extend(TokenStream::from_iter([
                    TokenTree::Ident(Ident::new("compile_error", Span::call_site())),
                    TokenTree::Punct(Punct::new('!', Spacing::Alone)),
                    TokenTree::Literal(Literal::string(&format!(
                        "Error while reading file: {err}"
                    ))),
                ]));
                break;
            }
        }
    }

    out
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_names_list() {
        static FILE_PATH: &str = "./bytedata-unicode/unicode/NamesList.txt";
        let file = std::fs::File::open("../bytedata-unicode/unicode/NamesList.txt").unwrap();
        let file = std::io::BufReader::new(file);
        let as_str = names_list(FILE_PATH.to_owned(), file).to_string();
        panic!("Result:\n{}", as_str);
    }
}
