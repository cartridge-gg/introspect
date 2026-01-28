use crate::params::GenericParams;
use crate::syntax::CairoFormat;
use crate::{
    AstInto, AstToString, AstTryInto, Attribute, AttributesTrait, CairoCollectionFormat, Derives,
    FromAst, IntrospectError, IntrospectResult, ItemTrait, SyntaxItemTrait, TryFromAst, Ty,
    Visibility, impl_attributes_trait, vec_try_from_element_list,
};
use cairo_lang_syntax::node::ast::{ItemStruct, Member as MemberAst};
use cairo_lang_syntax::node::kind::SyntaxKind;
use salsa::Database;

#[derive(Clone, Debug, PartialEq)]
pub struct Struct {
    pub visibility: Visibility,
    pub attributes: Vec<Attribute>,
    pub derives: Derives,
    pub name: String,
    pub generic_params: GenericParams,
    pub members: Vec<Member>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Member {
    pub visibility: Visibility,
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub ty: Ty,
}

impl_attributes_trait!(Struct);
impl_attributes_trait!(Member);

impl Struct {
    pub fn get_key_members(&self) -> Vec<&Member> {
        self.members
            .iter()
            .filter(|m| m.has_name_only_attribute("key"))
            .collect()
    }
}

impl<'db> TryFromAst<'db, MemberAst<'db>> for Member {
    fn try_from_ast(member: MemberAst<'db>, db: &'db dyn Database) -> IntrospectResult<Self> {
        Ok(Self {
            visibility: member.visibility(db).ast_into(db),
            name: member.name(db).to_string(db),
            attributes: member.attributes(db).ast_into(db),
            ty: member.type_clause(db).ast_try_into(db)?,
        })
    }
}

vec_try_from_element_list!(MemberList, Member);

impl<'db> TryFromAst<'db, ItemStruct<'db>> for Struct {
    fn try_from_ast(item: ItemStruct<'db>, db: &'db dyn Database) -> IntrospectResult<Self> {
        let all_attributes: Vec<Attribute> = item.attributes(db).ast_into(db);
        let (attributes, derives) = Derives::split_derives(all_attributes)?;
        Ok(Self {
            visibility: item.visibility(db).ast_into(db),
            attributes,
            derives,
            name: item.name(db).to_string(db),
            generic_params: item.generic_params(db).ast_into(db),
            members: item.members(db).ast_try_into(db)?,
        })
    }
}

impl<'db> FromAst<'db, ItemStruct<'db>> for Struct {
    fn from_ast(item: ItemStruct<'db>, db: &'db dyn Database) -> Self {
        Self::try_from_ast(item, db).unwrap()
    }
}

impl CairoFormat for Struct {
    fn cfmt(&self, buf: &mut String) {
        self.attributes.cfmt(buf);
        self.derives.cfmt(buf);
        self.visibility.cfmt(buf);
        self.name.cfmt_prefixed_str(buf, "struct ");
        self.generic_params.cfmt(buf);
        self.members.cfmt_csv_braced(buf);
    }
}

impl CairoFormat for Member {
    fn cfmt(&self, buf: &mut String) {
        self.attributes.cfmt(buf);
        self.visibility.cfmt(buf);
        self.name.cfmt(buf);
        self.ty.cfmt_prefixed_str(buf, ": ");
    }
}

impl SyntaxItemTrait for Struct {
    fn from_file_node<'db>(
        db: &'db dyn Database,
        node: cairo_lang_syntax::node::SyntaxNode<'db>,
    ) -> IntrospectResult<Self> {
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

impl ItemTrait for Struct {
    fn name(&self) -> &str {
        &self.name
    }
    fn generic_params(&self) -> &GenericParams {
        &self.generic_params
    }
}
