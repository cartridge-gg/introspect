use crate::attribute::{Attribute, parse_attributes};
use crate::{IntrospectError, Result};
use cairo_lang_syntax::attribute::structured::AttributeArgVariant;
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::ast::{AttributeList, Expr};
use salsa::Database;

const DERIVE_MACRO_NAME: &str = "derive";

fn get_derives_from_macro<'db>(attr: &Attribute<'db>) -> Result<Vec<String>> {
    match &attr.args {
        Some(args) => args
            .iter()
            .map(|a| match &a.variant {
                AttributeArgVariant::Unnamed(Expr::Path(path)) => Ok(path
                    .as_syntax_node()
                    .get_text_without_all_comment_trivia(attr.db)),
                _ => Err(IntrospectError::WrongDeriveVariant(format!(
                    "{:?}",
                    a.variant
                ))),
            })
            .collect(),
        None => Err(IntrospectError::DeriveMacroMissingArgs),
    }
}

pub fn make_derives_attributes_line(derives: &[String]) -> String {
    if derives.is_empty() {
        "".to_string()
    } else {
        format!("#[derive({})]\n", derives.join(", "))
    }
}

pub fn split_derives_attribute<'db>(
    attrs: AttributeList<'db>,
    db: &'db dyn Database,
) -> Result<(Vec<Attribute<'db>>, Vec<String>)> {
    let attrs = parse_attributes(attrs, db);
    let mut other_attrs = Vec::new();
    let mut derives = Vec::new();
    for attr in attrs {
        if attr.name == DERIVE_MACRO_NAME {
            for derive in get_derives_from_macro(&attr)? {
                derives.push(derive)
            }
        } else {
            other_attrs.push(attr);
        }
    }
    Ok((other_attrs, derives))
}
