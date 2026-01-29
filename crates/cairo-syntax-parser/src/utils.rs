use cairo_lang_macro::{TextSpan, Token, TokenStream, TokenTree};

pub fn str_to_token_stream(s: &str) -> TokenStream {
    TokenStream::new(vec![TokenTree::Ident(Token::new(s, TextSpan::call_site()))])
}
