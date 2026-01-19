use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, Ident, ItemStruct, parse_macro_input};

/// Attribute macro that automatically converts all fields to `Option<T>` and generates
/// `set_*` and getter methods that check if the field is already set.
///
/// # Field Attributes:
/// - `#[skip_accessors]` - Wraps field in Option but skips getter/setter generation
/// - `#[skip]` - Leaves field unchanged (no Option wrapping, no accessors)
///
/// # Example
///
/// ```ignore
/// #[macro_attributes]
/// pub struct MyStruct {
///     name: String,              // → Option<String> with accessors
///     #[skip_accessors]
///     internal: String,          // → Option<String> without accessors
///     #[skip]
///     raw: u32,                  // → u32 (unchanged)
/// }
/// ```
#[proc_macro_attribute]
pub fn macro_attributes(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;
    let vis = &input.vis;
    let attrs = &input.attrs;
    let generics = &input.generics;
    let fields = &input.fields;

    let (wrapped_fields, setters) = match fields {
        Fields::Named(fields) => {
            let mut wrapped = Vec::new();
            let mut setters = Vec::new();
            for f in &fields.named {
                let field_name = f.ident.as_ref().unwrap();
                let field_vis = &f.vis;
                let field_attrs = &f.attrs;
                let original_ty = &f.ty;

                // Check for skip attributes
                let has_skip = field_attrs.iter().any(|attr| attr.path().is_ident("skip"));
                let has_skip_accessors = field_attrs
                    .iter()
                    .any(|attr| attr.path().is_ident("skip_accessors"));

                // Filter out our custom attributes from the output
                let filtered_attrs: Vec<_> = field_attrs
                    .iter()
                    .filter(|attr| {
                        !attr.path().is_ident("skip") && !attr.path().is_ident("skip_accessors")
                    })
                    .collect();
                if has_skip {
                    // Leave field completely unchanged
                    wrapped.push(quote! {
                        #(#filtered_attrs)*
                        #field_vis #field_name: #original_ty
                    });
                    // No accessors
                } else if has_skip_accessors {
                    // Wrap in Option but no accessors
                    let wrapped_ty = quote! { Option<#original_ty> };
                    wrapped.push(quote! {
                        #(#filtered_attrs)*
                        #field_vis #field_name: #wrapped_ty
                    });
                    // No accessors
                } else {
                    // Normal behavior: wrap in Option and generate accessors
                    let wrapped_ty = quote! { Option<#original_ty> };
                    let inner_ty = original_ty.clone();

                    wrapped.push(quote! {
                        #(#filtered_attrs)*
                        #field_vis #field_name: #wrapped_ty
                    });

                    let setter_name = Ident::new(&format!("set_{field_name}"), field_name.span());
                    let setter_return_empty_name =
                        Ident::new(&format!("set_{field_name}_return_empty"), field_name.span());
                    let field_name_str = field_name.to_string();

                    setters.push(quote! {
                        pub fn #setter_name<E: From<IntrospectError>>(&mut self, #field_name: #inner_ty) -> Result<(), E> {
                            match &self.#field_name.replace(#field_name) {
                                None => Ok(()),
                                Some(_) => Err(IntrospectError::DuplicateAttribute(#field_name_str.to_string()).into()),
                            }
                        }

                        pub fn #setter_return_empty_name<T, E: From<IntrospectError>>(&mut self, #field_name: #inner_ty) -> Result<Vec<T>, E> {
                            self.#setter_name(#field_name).map(|_| Vec::new())
                        }
                    });
                }
            }

            (wrapped, setters)
        }
        _ => (vec![], vec![]), // Only named fields are supported
    };

    let expanded = quote! {
        #(#attrs)*
        #vis struct #name #generics {
            #(#wrapped_fields),*
        }

        impl #generics #name #generics {
            #(#setters)*
        }
    };

    TokenStream::from(expanded)
}
