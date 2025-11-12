use crate::attribute::{Attribute, attributes_to_string, parse_attributes};
use crate::derive::make_derives_attributes_line;
use crate::params::parse_params;
use crate::utils::string_to_keccak_hex;
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
    pub selector: String,
    pub attributes: Vec<Attribute<'db>>,
    pub name: String,
    pub ty: Option<String>,
}

fn parse_variant_type<'db>(variant: &VariantAst<'db>, db: &'db dyn Database) -> Option<String> {
    match variant.type_clause(db) {
        OptionTypeClause::Empty(_) => None,
        OptionTypeClause::TypeClause(ty) => {
            let ty_string = ty
                .ty(db)
                .as_syntax_node()
                .get_text_without_all_comment_trivia(db);
            if ty_string == "()" {
                None
            } else {
                Some(ty_string)
            }
        }
    }
}

impl<'db> Variant<'db> {
    pub fn new(variant: &VariantAst<'db>, db: &'db dyn Database) -> Self {
        let name = variant.name(db).text(db).to_string(db);
        Self {
            db,
            selector: string_to_keccak_hex(&name),
            name,
            attributes: parse_attributes(variant.attributes(db), db),
            ty: parse_variant_type(variant, db),
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
                .map(|m| Variant::new(&m, db))
                .collect(),
        }
    }

    pub fn parse_variant_selector(&self, n: usize) -> String {
        format!("{}", n)
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
            "{attributes}{name}{ty_str},",
            attributes = attributes_to_string(&self.attributes, 1),
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
            "{derives}{attributes}{vis}enum {name}{params_str}{{\n    {variants_str}\n}}",
            derives = make_derives_attributes_line(&self.derives),
            attributes = attributes_to_string(&self.attributes, 0),
            name = self.name,
            vis = self.visibility.to_code_string(),
        )
    }
}
