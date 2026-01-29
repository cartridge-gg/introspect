use std::fmt::{Display, Result, Write};

use cairo_lang_macro::{ProcMacroResult, TextSpan, Token, TokenStream, TokenTree};

pub trait CairoWrite: Sized {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result;
    fn cfmt_suffixed_str<W: Write>(&self, buf: &mut W, suffix: &str) -> Result {
        self.cfmt(buf)?;
        buf.write_str(suffix)
    }
    fn cfmt_prefixed<W: Write>(&self, buf: &mut W, prefix: char) -> Result {
        buf.write_char(prefix)?;
        self.cfmt(buf)
    }
    fn cfmt_prefixed_str<W: Write>(&self, buf: &mut W, prefix: &str) -> Result {
        buf.write_str(prefix)?;
        self.cfmt(buf)
    }
    fn cfmt_suffixed<W: Write>(&self, buf: &mut W, suffix: char) -> Result {
        self.cfmt(buf)?;
        buf.write_char(suffix)
    }
    fn cfmt_wrapped<W: Write>(&self, buf: &mut W, prefix: char, suffix: char) -> Result {
        buf.write_char(prefix)?;
        self.cfmt(buf)?;
        buf.write_char(suffix)
    }
    fn cfmt_wrapped_str<W: Write>(&self, buf: &mut W, prefix: &str, suffix: &str) -> Result {
        buf.write_str(prefix)?;
        self.cfmt(buf)?;
        buf.write_str(suffix)
    }
    fn cfmt_parenthesized<W: Write>(&self, buf: &mut W) -> Result {
        self.cfmt_wrapped(buf, '(', ')')
    }
    fn cfmt_braced<W: Write>(&self, buf: &mut W) -> Result {
        self.cfmt_wrapped(buf, '{', '}')
    }
    fn cfmt_bracketed<W: Write>(&self, buf: &mut W) -> Result {
        self.cfmt_wrapped(buf, '[', ']')
    }
}

pub trait CairoFormat {
    // fn fmt
    // fn display(&self) -> CairoDisplay<'_, Self> {
    //     CairoDisplay(self)
    // }
    fn to_string(&self) -> String {
        let mut buf = String::with_capacity(self.size_hint());
        self.cfmt(&mut buf).unwrap();
        buf
    }
    fn to_token(&self) -> Token {
        Token::new(self.to_string(), TextSpan::call_site())
    }
    fn to_token_tree(&self) -> TokenTree {
        TokenTree::Ident(self.to_token())
    }
    fn to_token_stream(&self) -> TokenStream {
        TokenStream::new(vec![self.to_token_tree()])
    }
    fn to_static_str(&self) -> &str {
        Box::leak(Box::new(self.to_string()))
    }
    fn to_proc_macro_result(&self) -> ProcMacroResult {
        ProcMacroResult::new(self.to_token_stream())
    }
}

// impl<T: CairoFormat> CairoFormat for Box<T> {
//     fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
//         (**self).cfmt(buf)
//     }
//     }

pub trait CairoSliceFormat {
    type Element: CairoWrite + SizeHint;
    fn cfmt_elements(&self) -> &[Self::Element];
    fn size_hint_slice<const DELIMITER_SIZE: usize>(&self) -> usize {
        let elements = self.cfmt_elements();
        elements.len().saturating_sub(1) * DELIMITER_SIZE
            + elements.iter().map(SizeHint::size_hint).sum::<usize>()
    }
    fn size_hint_tuple(&self) -> usize {
        let elements = self.cfmt_elements();
        match elements.len() {
            0 => 2,                               // ()
            1 => elements[0].size_hint() + 4,     // (elem, )
            _ => self.size_hint_slice::<2>() + 2, // (elem1, elem2, ...)
        }
    }
    fn size_hint_block<const DELIMITER_SIZE: usize>(&self) -> usize {
        if self.cfmt_elements().is_empty() {
            0
        } else {
            self.size_hint_slice::<DELIMITER_SIZE>() + 1
        }
    }
    fn cfmt_join<W: Write>(&self, buf: &mut W, delimiter: &str) -> Result {
        let elements = self.cfmt_elements();
        if let Some((first, rest)) = elements.split_first() {
            first.cfmt(buf)?;
            rest.iter()
                .map(|e| e.cfmt_prefixed_str(buf, delimiter))
                .collect::<Result>()?;
        }
        Ok(())
    }
    fn cfmt_delimited<W: Write>(&self, buf: &mut W, delimiter: char) -> Result {
        let elements = self.cfmt_elements();
        if let Some((first, rest)) = elements.split_first() {
            first.cfmt(buf)?;
            rest.iter()
                .map(|e| e.cfmt_prefixed(buf, delimiter))
                .collect::<Result>()?;
        }
        Ok(())
    }
    fn cfmt_terminated<W: Write>(&self, buf: &mut W, terminator: char) -> Result {
        self.cfmt_elements()
            .iter()
            .map(|e| e.cfmt_suffixed(buf, terminator))
            .collect::<Result>()?;
        Ok(())
    }
    fn cfmt_terminated_str<W: Write>(&self, buf: &mut W, terminator: &str) -> Result {
        self.cfmt_elements()
            .iter()
            .map(|e| e.cfmt_suffixed_str(buf, terminator))
            .collect::<Result>()?;
        Ok(())
    }

    fn cfmt_concatenated<W: Write>(&self, buf: &mut W) -> Result {
        self.cfmt_elements()
            .iter()
            .map(|e| e.cfmt(buf))
            .collect::<Result>()?;
        Ok(())
    }

    fn cfmt_csv<W: Write>(&self, buf: &mut W) -> Result {
        self.cfmt_join(buf, ", ")
    }
    fn cfmt_block<W: Write>(&self, buf: &mut W) -> Result {
        let elements = self.cfmt_elements();
        if !elements.is_empty() {
            buf.write_char('\n')?;
            elements.cfmt_terminated(buf, '\n')?;
        }
        Ok(())
    }
    fn cfmt_block_braced<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_char('{')?;
        self.cfmt_block(buf)?;
        buf.write_char('}')
    }
    fn cfmt_tuple<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_char('(')?;
        let elements = self.cfmt_elements();
        match elements.len() {
            0 => {}
            1 => {
                elements[0].cfmt_suffixed_str(buf, ", ")?;
            }
            _ => self.cfmt_join(buf, ", ")?,
        }
        buf.write_char(')')
    }
    fn cfmt_csv_wrapped<W: Write>(&self, buf: &mut W, prefix: char, suffix: char) -> Result {
        buf.write_char(prefix)?;
        self.cfmt_csv(buf)?;
        buf.write_char(suffix)
    }
    fn cfmt_csv_braced<W: Write>(&self, buf: &mut W) -> Result {
        self.cfmt_csv_wrapped(buf, '{', '}')
    }
    fn cfmt_csv_parenthesized<W: Write>(&self, buf: &mut W) -> Result {
        self.cfmt_csv_wrapped(buf, '(', ')')
    }
    fn cfmt_csv_bracketed<W: Write>(&self, buf: &mut W) -> Result {
        self.cfmt_csv_wrapped(buf, '[', ']')
    }
    fn cfmt_csv_angled<W: Write>(&self, buf: &mut W) -> Result {
        self.cfmt_csv_wrapped(buf, '<', '>')
    }
    fn cfmt_csv_barred<W: Write>(&self, buf: &mut W) -> Result {
        self.cfmt_csv_wrapped(buf, '|', '|')
    }
    fn cfmt_array<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_str("array![")?;
        self.cfmt_csv(buf)?;
        buf.write_char(']')
    }
    fn cfmt_span<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_char('[')?;
        self.cfmt_csv(buf)?;
        buf.write_str("].span()")
    }

    fn cfmt_fields<W: Write>(&self, buf: &mut W) -> Result {
        let elements = self.cfmt_elements();
        if !elements.is_empty() {
            buf.write_char('\n')?;
            elements.cfmt_terminated_str(buf, ",\n")?;
        }
        Ok(())
    }
    fn cfmt_fields_braced<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_char('{')?;
        self.cfmt_fields(buf)?;
        buf.write_char('}')
    }
}

impl CairoWrite for String {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_str(self)
    }
}
impl SizeHint for String {
    fn size_hint(&self) -> usize {
        self.len()
    }
}

impl<E: CairoWrite + SizeHint> CairoSliceFormat for Vec<E> {
    type Element = E;
    fn cfmt_elements(&self) -> &[Self::Element] {
        self
    }
}

impl<E: CairoWrite + SizeHint> CairoSliceFormat for [E] {
    type Element = E;
    fn cfmt_elements(&self) -> &[Self::Element] {
        self
    }
}

pub trait SizeHint {
    fn size_hint(&self) -> usize;
}

pub trait OptionSizeHint {
    fn size_hint_option<const SOME_WRAPPER: usize, const NONE: usize>(&self) -> usize;
}

// impl<T, U> SizeHint for T
// where
//     T: AsRef<U>,
//     U: SizeHint + ?Sized,
//     T: ?Sized,
// {
//     fn size_hint(&self) -> usize {
//         self.as_ref().size_hint()
//     }
// }

// impl<T: CairoFormat> SizeHint for T {
//     fn size_hint(&self) -> usize {
//         CairoFormat::cfmt_size_hint(self)
//     }
// }

// impl<T: SizeHint> SizeHint for Box<T> {
//     fn size_hint(&self) -> usize {
//         self.as_ref().size_hint()
//     }
// }

impl<T: SizeHint> OptionSizeHint for Option<T> {
    fn size_hint_option<const SOME_WRAPPER: usize, const NONE: usize>(&self) -> usize {
        match self {
            Some(v) => v.size_hint() + SOME_WRAPPER,
            None => NONE,
        }
    }
}

pub struct CairoDisplay<'a, T: CairoWrite>(&'a T);

impl<'a, T: CairoWrite> Display for CairoDisplay<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        self.0.cfmt(f)
    }
}
