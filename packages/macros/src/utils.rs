use crate::{IntrospectError, Result};
use cairo_lang_macro::{ProcMacroResult, TextSpan, Token, TokenStream, TokenTree, derive_macro};
use cairo_lang_parser::db;
use cairo_lang_parser::printer::print_tree;
use cairo_lang_parser::utils::SimpleParserDatabase;
use cairo_lang_syntax::attribute::structured::{AttributeArgVariant, AttributeStructurize};
use cairo_lang_syntax::node::ast::{
    ArgClause, Attribute, AttributeList, Expr, ExprPath, ItemEnum, ItemStruct,
    OptionArgListParenthesized,
};
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, Terminal, TypedSyntaxNode};
use salsa::Database;

pub fn str_to_token_stream(s: &str) -> TokenStream {
    TokenStream::new(vec![TokenTree::Ident(Token::new(s, TextSpan::call_site()))])
}

const DERIVE_MACRO_NAME: &str = "derive";

pub struct Member {
    pub visibility: Visibility,
    pub name: String,
    pub attributes: Vec<String>,
    pub ty: String,
}

pub struct Struct {
    pub visibility: Visibility,
    pub attributes: Vec<String>,
    pub derives: Option<Vec<String>>,
    pub name: String,
    pub generic_params: Option<Vec<String>>,
    pub members: Vec<Member>,
}

// impl Struct {
//     pub fn new<'db>(item: ItemStruct, db: &'db dyn Database) -> Self {}
// }

impl ToString for Struct {
    fn to_string(&self) -> String {
        let params_str = match &self.generic_params {
            Some(p) => format!("<{}>", p.join(", ")),
            None => "".to_string(),
        };
        let members_str = self
            .members
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<String>>()
            .join("\n            ");
        let attributes_str = if self.attributes.is_empty() {
            "".to_string()
        } else {
            self.attributes.join("\n") + "\n"
        };
        format!(
            r#"
        {attributes_str}{vis}struct {name}{params_str}{{
            {members_str}
        }}
        "#,
            name = self.name,
            vis = self.visibility.to_code_string(),
        )
    }
}

#[derive(Clone, Debug)]
pub enum Visibility {
    Default,
    Pub,
}

#[derive(Clone, Debug)]
pub enum Modifier {
    Ref,
    Mut,
}

impl Visibility {
    pub fn to_code_string(&self) -> String {
        match self {
            Visibility::Default => "".to_string(),
            Visibility::Pub => "pub ".to_string(),
        }
    }
}

pub enum IntrospectType<'db> {
    Struct(ItemStruct<'db>),
    Enum(ItemEnum<'db>),
}

fn get_derives_from_macro<'db>(
    attr: Attribute<'db>,
    db: &'db dyn Database,
) -> Vec<Result<&'db str>> {
    let structured = attr.structurize(db);
    structured
        .args
        .iter()
        .map(|a| match &a.variant {
            AttributeArgVariant::Unnamed(Expr::Path(path)) => {
                Ok(path.as_syntax_node().get_text(db))
            }
            _ => Err(IntrospectError::WrongDeriveVariant(format!(
                "{:?}",
                a.variant
            ))),
        })
        .collect()
}

pub fn get_derives<'db>(attrs: AttributeList<'db>, db: &'db dyn Database) -> Result<Vec<&'db str>> {
    attrs
        .elements(db)
        .filter(|x| x.attr(db).as_syntax_node().get_text(db) == DERIVE_MACRO_NAME)
        .flat_map(|attr| get_derives_from_macro(attr, db))
        .collect()
}

pub fn get_introspect_type<'db>(
    db: &'db dyn Database,
    file: SyntaxNode<'db>,
) -> Result<IntrospectType<'db>> {
    let item = file.get_children(db)[0].get_children(db)[0];
    let kind = item.kind(db);
    match kind {
        SyntaxKind::ItemStruct => Ok(IntrospectType::Struct(ItemStruct::from_syntax_node(
            db, item,
        ))),
        SyntaxKind::ItemEnum => Ok(IntrospectType::Enum(ItemEnum::from_syntax_node(db, item))),
        _ => Err(IntrospectError::UnsupportedItem(kind.to_string())),
    }
}

pub fn get_struct<'db>(db: &'db dyn Database, file: SyntaxNode<'db>) -> Result<ItemStruct<'db>> {
    let item = file.get_children(db)[0].get_children(db)[0];
    let kind = item.kind(db);
    match kind {
        SyntaxKind::ItemStruct => Ok(ItemStruct::from_syntax_node(db, item)),
        _ => Err(IntrospectError::NotAStruct(kind.to_string())),
    }
}

impl ToString for Member {
    fn to_string(&self) -> String {
        let attrs_str = if self.attributes.is_empty() {
            "".to_string()
        } else {
            format!("{}\n", self.attributes.join("\n"))
        };
        format!(
            "{attrs_str}{vis}{name}: {ty},",
            vis = self.visibility.to_code_string(),
            name = self.name,
            ty = self.ty
        )
    }
}

pub fn make_derives_attribute(derives: &[&str]) -> String {
    format!("#[derive({})]", derives.join(", "))
}

#[derive_macro]
pub fn print_all(token_stream: TokenStream) -> ProcMacroResult {
    let db = SimpleParserDatabase::default();
    let (parsed, _diag) = db.parse_virtual_with_diagnostics(token_stream.clone());
    let struct_item = get_struct(&db, parsed.clone()).unwrap();

    // let derives = get_derives(struct_item.attributes(&db), &db).unwrap();
    println!(
        "{}",
        print_tree(&db, &struct_item.as_syntax_node(), true, false)
    );
    let derives = get_derives(struct_item.attributes(&db), &db).unwrap();
    let filtered_derives = derives
        .iter()
        .filter_map(|d| if *d != "PrintAll" { Some(*d) } else { None })
        .collect::<Vec<_>>();
    let attrs = [make_derives_attribute(&filtered_derives)];

    // println!("Generated struct:\n{}", struct_str);

    ProcMacroResult::new(str_to_token_stream("mod something {}"))
}
