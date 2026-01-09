use crate::params::GenericParams;
use crate::{
    AsCairo, AstInto, AstToString, Attribute, CollectionsAsCairo, Derives, FromAst,
    IntrospectError, Result, SyntaxItemTrait, TryFromAst, Ty, Visibility, vec_from_element_list,
};
use cairo_lang_syntax::node::ast::{ItemStruct, Member as MemberAst};
use cairo_lang_syntax::node::kind::SyntaxKind;
use salsa::Database;

pub struct Struct {
    pub visibility: Visibility,
    pub attributes: Vec<Attribute>,
    pub derives: Derives,
    pub name: String,
    pub generic_params: GenericParams,
    pub members: Vec<Member>,
}

pub struct Member {
    pub visibility: Visibility,
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub ty: Ty,
}

impl<'db> FromAst<'db, MemberAst<'db>> for Member {
    fn from_ast(member: MemberAst<'db>, db: &'db dyn Database) -> Self {
        Self {
            visibility: member.visibility(db).into(),
            name: member.name(db).to_string(db),
            attributes: vec![],
            ty: member.type_clause(db).ast_into(db),
        }
    }
}

vec_from_element_list!(MemberList, Member);

impl<'db> TryFromAst<'db, ItemStruct<'db>> for Struct {
    fn try_from_ast(item: ItemStruct<'db>, db: &'db dyn Database) -> Result<Self> {
        let all_attributes: Vec<Attribute> = item.attributes(db).ast_into(db);
        let (attributes, derives) = Derives::split_derives(all_attributes)?;
        Ok(Self {
            visibility: item.visibility(db).into(),
            attributes,
            derives,
            name: item.name(db).to_string(db),
            generic_params: item.generic_params(db).ast_into(db),
            members: item.members(db).ast_into(db),
        })
    }
}

impl AsCairo for Member {
    fn as_cairo(&self) -> String {
        format!(
            "{attributes}{vis}{name}: {ty},",
            attributes = self.attributes.as_cairo_block(),
            vis = self.visibility.as_cairo(),
            name = self.name,
            ty = self.ty.as_cairo()
        )
    }
}

impl AsCairo for Struct {
    fn as_cairo(&self) -> String {
        format!(
            "{derives}{attributes}{vis}struct {name}{params}{{{members}}}
            ",
            derives = self.derives.as_cairo(),
            attributes = self.attributes.as_cairo_block(),
            vis = self.visibility.as_cairo(),
            name = self.name,
            params = self.generic_params.as_cairo(),
            members = self.members.as_cairo_block_section()
        )
    }
}

impl SyntaxItemTrait for Struct {
    fn from_file_node<'db>(
        db: &'db dyn Database,
        node: cairo_lang_syntax::node::SyntaxNode<'db>,
    ) -> Result<Self> {
        for child in node.get_children(db)[0].get_children(db) {
            let kind = child.kind(db);
            match kind {
                SyntaxKind::ItemStruct => {
                    return Struct::try_from_syntax_node(db, *child);
                }
                _ => continue,
            }
        }
        Err(IntrospectError::NoStruct())
    }
}
