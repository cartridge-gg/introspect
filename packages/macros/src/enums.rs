use crate::attribute::{Attribute, attributes_to_string, parse_attributes};
use crate::derive::make_derives_attributes_line;
use crate::params::parse_params;
use crate::{Visibility, split_derives_attribute};
use cairo_lang_syntax::node::ast::{ItemEnum, OptionTypeClause, Variant as VariantAst};
use cairo_lang_syntax::node::{SyntaxNode, Terminal, TypedSyntaxNode};
use salsa::Database;

pub struct Enum<'db> {
    pub db: &'db dyn Database,
    pub visibility: Visibility,
    pub attributes: Vec<Attribute<'db>>,
    pub derives: Vec<String>,
    pub name: String,
    pub generic_params: Option<Vec<String>>,
    pub variants: Vec<Variant<'db>>,
}

pub struct Variant<'db> {
    pub db: &'db dyn Database,
    pub n: u32,

    pub attributes: Vec<Attribute<'db>>,
    pub name: String,
    pub ty: Option<String>,
}

impl<'db> Variant<'db> {
    pub fn new(variant: &VariantAst<'db>, db: &'db dyn Database, n: u32) -> Self {
        Self {
            db,
            n,
            name: variant.name(db).text(db).to_string(db),
            attributes: parse_attributes(variant.attributes(db), db),
            ty: match variant.type_clause(db) {
                OptionTypeClause::Empty(_) => None,
                OptionTypeClause::TypeClause(ty) => Some(
                    ty.ty(db)
                        .as_syntax_node()
                        .get_text_without_all_comment_trivia(db),
                ),
            },
        }
    }
}

impl<'db> Enum<'db> {
    pub fn new(item: ItemEnum<'db>, db: &'db dyn Database) -> Self {
        let (attributes, derives) = split_derives_attribute(item.attributes(db), db).unwrap();
        Self {
            db,
            visibility: item.visibility(db).into(),
            attributes,
            derives,
            name: item.name(db).text(db).to_string(db),
            generic_params: parse_params(item.generic_params(db), db),
            variants: item
                .variants(db)
                .elements(db)
                .enumerate()
                .map(|(n, m)| Variant::new(&m, db, n as u32))
                .collect(),
        }
    }

    pub fn from_syntax_node(db: &'db dyn Database, node: SyntaxNode<'db>) -> Self {
        Self::new(ItemEnum::from_syntax_node(db, node), db)
    }
}

impl<'db> ToString for Variant<'db> {
    fn to_string(&self) -> String {
        let ty_str = match &self.ty {
            Some(ty) => format!(": {}", ty),
            None => "".to_string(),
        };
        format!(
            "{attrs}{name}{ty_str},",
            attrs = attributes_to_string(&self.attributes, 1),
            name = self.name,
        )
    }
}

impl<'db> ToString for Enum<'db> {
    fn to_string(&self) -> String {
        let params_str = match &self.generic_params {
            Some(p) => format!("<{}>", p.join(", ")),
            None => "".to_string(),
        };
        let variants_str = self
            .variants
            .iter()
            .map(Variant::to_string)
            .collect::<Vec<String>>()
            .join("\n    ");

        format!(
            "{derives}{attrs}{vis}enum {name}{params_str}{{\n    {variants_str}\n}}",
            derives = make_derives_attributes_line(&self.derives),
            attrs = attributes_to_string(&self.attributes, 0),
            name = self.name,
            vis = self.visibility.to_code_string(),
        )
    }
}
