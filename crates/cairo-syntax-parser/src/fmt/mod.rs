mod attribute;
mod common;
mod expr;
mod generic_param;
mod item;
mod statement;

use cairo_lang_macro::{TextSpan, Token, TokenStream, TokenTree};

pub trait CodeBuffer {
    fn new() -> Self
    where
        Self: Sized;
    fn push_token_str(&mut self, s: &str);
    fn push_token_char(&mut self, c: char);
}

impl CodeBuffer for String {
    fn new() -> Self
    where
        Self: Sized,
    {
        String::new()
    }
    fn push_token_str(&mut self, s: &str) {
        self.push_str(s);
    }
    fn push_token_char(&mut self, c: char) {
        self.push(c);
    }
}

// impl CodeBuffer for TokenStream {
//     fn new() -> Self
//     where
//         Self: Sized,
//     {
//         TokenStream::empty()
//     }
//     fn push_token_str(&mut self, string: &str) {
//         let string = string.trim();
//         if !string.is_empty() {
//             self.push_token(TokenTree::Ident(Token::new(string, TextSpan::call_site())))
//         }
//     }
//     fn push_token_char(&mut self, c: char) {
//         if !c.is_whitespace() {
//             self.push_token(TokenTree::Ident(Token::new(
//                 &c.to_string(),
//                 TextSpan::call_site(),
//             )))
//         }
//     }
// }

impl CodeBuffer for TokenStream {
    fn new() -> Self
    where
        Self: Sized,
    {
        TokenStream::empty()
    }
    fn push_token_str(&mut self, string: &str) {
        self.push_token(TokenTree::Ident(Token::new(string, TextSpan::call_site())))
    }
    fn push_token_char(&mut self, c: char) {
        self.push_token_str(&c.to_string());
    }
}

pub trait CairoFormat<T: CodeBuffer> {
    fn cfmt(&self, buf: &mut T);
    fn cfmt_suffixed_str(&self, buf: &mut T, suffix: &str) {
        self.cfmt(buf);
        buf.push_token_str(suffix);
    }
    fn cfmt_prefixed(&self, buf: &mut T, prefix: char) {
        buf.push_token_char(prefix);
        self.cfmt(buf);
    }
    fn cfmt_prefixed_str(&self, buf: &mut T, prefix: &str) {
        buf.push_token_str(prefix);
        self.cfmt(buf);
    }
    fn cfmt_suffixed(&self, buf: &mut T, suffix: char) {
        self.cfmt(buf);
        buf.push_token_char(suffix);
    }
    fn cfmt_wrapped(&self, buf: &mut T, prefix: char, suffix: char) {
        buf.push_token_char(prefix);
        self.cfmt(buf);
        buf.push_token_char(suffix);
    }
    fn cfmt_wrapped_str(&self, buf: &mut T, prefix: &str, suffix: &str) {
        buf.push_token_str(prefix);
        self.cfmt(buf);
        buf.push_token_str(suffix);
    }
    fn cfmt_parenthesized(&self, buf: &mut T) {
        self.cfmt_wrapped(buf, '(', ')');
    }
    fn cfmt_braced(&self, buf: &mut T) {
        self.cfmt_wrapped(buf, '{', '}');
    }
    fn cfmt_bracketed(&self, buf: &mut T) {
        self.cfmt_wrapped(buf, '[', ']');
    }

    fn to_cairo(&self) -> T {
        let mut buf = CodeBuffer::new();
        self.cfmt(&mut buf);
        buf
    }
}

pub trait CairoCollectionFormat<T: CodeBuffer> {
    type Element: CairoFormat<T>;
    fn cfmt_elements(&self) -> &[Self::Element];
    fn cfmt_join(&self, buf: &mut T, delimiter: &str) {
        let elements = self.cfmt_elements();
        if let Some((first, rest)) = elements.split_first() {
            first.cfmt(buf);
            rest.iter()
                .for_each(|e| e.cfmt_prefixed_str(buf, delimiter));
        }
    }
    fn cfmt_delimited(&self, buf: &mut T, delimiter: char) {
        let elements = self.cfmt_elements();
        if let Some((first, rest)) = elements.split_first() {
            first.cfmt(buf);
            rest.iter().for_each(|e| e.cfmt_prefixed(buf, delimiter));
        }
    }
    fn cfmt_terminated(&self, buf: &mut T, terminator: char) {
        self.cfmt_elements()
            .iter()
            .for_each(|e| e.cfmt_suffixed(buf, terminator));
    }
    fn cfmt_terminated_str(&self, buf: &mut T, terminator: &str) {
        self.cfmt_elements()
            .iter()
            .for_each(|e| e.cfmt_suffixed_str(buf, terminator));
    }

    fn cfmt_concatenated(&self, buf: &mut T) {
        self.cfmt_elements().iter().for_each(|e| e.cfmt(buf));
    }

    fn cfmt_csv(&self, buf: &mut T) {
        self.cfmt_join(buf, ", ");
    }
    fn cfmt_block(&self, buf: &mut T) {
        let elements = self.cfmt_elements();
        if !elements.is_empty() {
            buf.push_token_char('\n');
            elements.cfmt_terminated(buf, '\n');
        }
    }
    fn cfmt_block_braced(&self, buf: &mut T) {
        buf.push_token_char('{');
        self.cfmt_block(buf);
        buf.push_token_char('}');
    }
    fn cfmt_tuple(&self, buf: &mut T) {
        buf.push_token_char('(');
        let elements = self.cfmt_elements();
        match elements.len() {
            0 => {}
            1 => {
                elements[0].cfmt_suffixed_str(buf, ", ");
            }
            _ => self.cfmt_join(buf, ", "),
        }
        buf.push_token_char(')');
    }
    fn cfmt_csv_braced(&self, buf: &mut T) {
        buf.push_token_char('{');
        self.cfmt_csv(buf);
        buf.push_token_char('}');
    }
    fn cfmt_csv_parenthesized(&self, buf: &mut T) {
        buf.push_token_char('(');
        self.cfmt_csv(buf);
        buf.push_token_char(')');
    }
    fn cfmt_csv_bracketed(&self, buf: &mut T) {
        buf.push_token_char('[');
        self.cfmt_csv(buf);
        buf.push_token_char(']');
    }
    fn cfmt_csv_angled(&self, buf: &mut T) {
        buf.push_token_char('<');
        self.cfmt_csv(buf);
        buf.push_token_char('>');
    }
    fn cfmt_csv_barred(&self, buf: &mut T) {
        buf.push_token_char('|');
        self.cfmt_csv(buf);
        buf.push_token_char('|');
    }
    fn cfmt_array(&self, buf: &mut T) {
        buf.push_token_str("array![");
        self.cfmt_csv(buf);
        buf.push_token_char(']');
    }
    fn cfmt_span(&self, buf: &mut T) {
        buf.push_token_char('[');
        self.cfmt_csv(buf);
        buf.push_token_str("].span()");
    }

    fn cfmt_fields(&self, buf: &mut T) {
        let elements = self.cfmt_elements();
        if !elements.is_empty() {
            buf.push_token_char('\n');
            elements.cfmt_terminated_str(buf, ",\n");
        }
    }
    fn cfmt_fields_braced(&self, buf: &mut T) {
        buf.push_token_char('{');
        self.cfmt_fields(buf);
        buf.push_token_char('}');
    }

    fn to_cairo_block(&self) -> T {
        let mut buf = CodeBuffer::new();
        self.cfmt_block(&mut buf);
        buf
    }
}

impl<T: CodeBuffer> CairoFormat<T> for String {
    fn cfmt(&self, buf: &mut T) {
        buf.push_token_str(self);
    }
}

impl<T: CodeBuffer, E: CairoFormat<T>> CairoCollectionFormat<T> for Vec<E> {
    type Element = E;
    fn cfmt_elements(&self) -> &[Self::Element] {
        self
    }
}

impl<T: CodeBuffer, E: CairoFormat<T>> CairoCollectionFormat<T> for [E] {
    type Element = E;
    fn cfmt_elements(&self) -> &[Self::Element] {
        self
    }
}
