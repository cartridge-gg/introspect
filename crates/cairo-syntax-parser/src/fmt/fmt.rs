use cairo_lang_macro::{ProcMacroResult, TextSpan, Token, TokenStream, TokenTree};
use std::fmt::{Display, Result};

use crate::CairoWrite;

pub trait CairoFormat {
    fn stringify(&self) -> String;
    fn cfmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result;
    fn to_token(&self) -> Token {
        Token::new(self.stringify(), TextSpan::call_site())
    }
    fn to_token_tree(&self) -> TokenTree {
        TokenTree::Ident(self.to_token())
    }
    fn to_token_stream(&self) -> TokenStream {
        TokenStream::new(vec![self.to_token_tree()])
    }
    fn to_static_str(&self) -> &str {
        Box::leak(Box::new(self.stringify()))
    }
    fn to_proc_macro_result(&self) -> ProcMacroResult {
        ProcMacroResult::new(self.to_token_stream())
    }
}

impl<T: CairoWrite> CairoFormat for T {
    fn cfmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        self.cwrite(f)
    }
    fn stringify(&self) -> String {
        let size = self.size_hint();
        let mut s = String::with_capacity(size);
        self.cwrite(&mut s).unwrap();
        s
    }
}
