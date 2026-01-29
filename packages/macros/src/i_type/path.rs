use cairo_lang_macro::TokenStream;

pub struct IPath(pub TokenStream);

impl IPath {
    pub fn with_segments(&self, segments: &TokenStream) -> TokenStream {
        quote! {
            #self::#segments
        }
    }
}
