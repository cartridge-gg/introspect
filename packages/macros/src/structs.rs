use crate::attribute::{Attribute, attributes_to_string, parse_attributes};
use crate::derive::make_derives_attributes_line;
use crate::params::parse_params;
use crate::{IntrospectError, Result, Visibility, split_derives_attribute};
use cairo_lang_syntax::node::ast::{ItemStruct, Member as MemberAst};
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, Terminal, TypedSyntaxNode};
use salsa::Database;

pub struct Struct<'db> {
    pub db: &'db dyn Database,
    pub visibility: Visibility,
    pub attributes: Vec<Attribute<'db>>,
    pub derives: Vec<String>,
    pub name: String,
    pub generic_params: Option<Vec<String>>,
    pub members: Vec<Member<'db>>,
}

pub struct Member<'db> {
    pub db: &'db dyn Database,
    pub visibility: Visibility,
    pub name: String,
    pub attributes: Vec<Attribute<'db>>,
    pub ty: String,
}

impl<'db> Member<'db> {
    pub fn new(member: &MemberAst<'db>, db: &'db dyn Database) -> Self {
        Self {
            db,
            visibility: member.visibility(db).into(),
            name: member.name(db).text(db).to_string(db),
            attributes: parse_attributes(member.attributes(db), db),
            ty: member
                .type_clause(db)
                .ty(db)
                .as_syntax_node()
                .get_text_without_all_comment_trivia(db),
        }
    }
}

impl<'db> Struct<'db> {
    pub fn new(item: ItemStruct<'db>, db: &'db dyn Database) -> Self {
        let (attributes, derives) = split_derives_attribute(item.attributes(db), db).unwrap();
        Self {
            db,
            visibility: item.visibility(db).into(),
            attributes,
            derives,
            name: item.name(db).text(db).to_string(db),
            generic_params: parse_params(item.generic_params(db), db),
            members: item
                .members(db)
                .elements(db)
                .map(|m| Member::new(&m, db))
                .collect(),
        }
    }

    pub fn from_syntax_node(db: &'db dyn Database, node: SyntaxNode<'db>) -> Self {
        Self::new(ItemStruct::from_syntax_node(db, node), db)
    }
}

pub fn get_struct<'db>(db: &'db dyn Database, file: SyntaxNode<'db>) -> Result<Struct<'db>> {
    for child in file.get_children(db)[0].get_children(db) {
        if (&child).kind(db) == SyntaxKind::ItemStruct {
            return Ok(Struct::from_syntax_node(db, *child));
        }
    }
    Err(IntrospectError::NoItem())
}

impl<'db> ToString for Member<'db> {
    fn to_string(&self) -> String {
        format!(
            "{attributes}{vis}{name}: {ty},",
            attributes = attributes_to_string(&self.attributes, 1),
            vis = self.visibility.to_code_string(),
            name = self.name,
            ty = self.ty
        )
    }
}

impl<'db> ToString for Struct<'db> {
    fn to_string(&self) -> String {
        let params_str = match &self.generic_params {
            Some(p) => format!("<{}>", p.join(", ")),
            None => "".to_string(),
        };
        let members_str = self
            .members
            .iter()
            .map(Member::to_string)
            .collect::<Vec<String>>()
            .join("\n    ");

        format!(
            "{derives}{attributes}{vis}struct {name}{params_str}{{\n    {members_str}\n}}
            ",
            derives = make_derives_attributes_line(&self.derives),
            attributes = attributes_to_string(&self.attributes, 0),
            name = self.name,
            vis = self.visibility.to_code_string(),
        )
    }
}
