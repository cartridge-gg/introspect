use crate::as_cairo::CollectionsAsCairo;
use crate::ast::AstToString;
use crate::{
    AsCairo, AstInto, AstTryInto, Attribute, Derives, GenericParams, IntrospectError, ItemTrait,
    IntrospectResult, SyntaxItemTrait, TryFromAst, Ty, Visibility, impl_attributes_trait,
    vec_try_from_element_list,
};
use cairo_lang_syntax::node::ast::{ItemEnum, Variant as VariantAst};
use cairo_lang_syntax::node::kind::SyntaxKind;
use salsa::Database;

pub struct Enum {
    pub visibility: Visibility,
    pub attributes: Vec<Attribute>,
    pub derives: Derives,
    pub name: String,
    pub generic_params: GenericParams,
    pub variants: Vec<Variant>,
}

pub struct Variant {
    pub attributes: Vec<Attribute>,
    pub name: String,
    pub ty: Option<Ty>,
}

impl_attributes_trait!(Enum);
impl_attributes_trait!(Variant);

impl<'db> TryFromAst<'db, VariantAst<'db>> for Variant {
    fn try_from_ast(variant: VariantAst<'db>, db: &'db dyn Database) -> IntrospectResult<Self> {
        let name = variant.name(db).to_string(db);
        Ok(Self {
            name,
            attributes: variant.attributes(db).ast_into(db),
            ty: variant.type_clause(db).ast_try_into(db)?,
        })
    }
}

vec_try_from_element_list!(VariantList, Variant);

impl<'db> TryFromAst<'db, ItemEnum<'db>> for Enum {
    fn try_from_ast(item: ItemEnum<'db>, db: &'db dyn Database) -> IntrospectResult<Self> {
        let all_attributes: Vec<Attribute> = item.attributes(db).ast_into(db);
        let (attributes, derives) = Derives::split_derives(all_attributes)?;
        Ok(Self {
            visibility: item.visibility(db).into(),
            attributes,
            derives,
            name: item.name(db).to_string(db),
            generic_params: item.generic_params(db).ast_into(db),
            variants: item.variants(db).ast_try_into(db)?,
        })
    }
}

impl<'db> AsCairo for Variant {
    fn as_cairo(&self) -> String {
        let ty_str = match &self.ty {
            Some(ty) => format!(": {}", ty.as_cairo()),
            None => "".to_string(),
        };
        format!(
            "{attributes}{name}{ty_str},",
            attributes = self.attributes.as_cairo_block(),
            name = self.name,
        )
    }
}

impl<'db> AsCairo for Enum {
    fn as_cairo(&self) -> String {
        format!(
            "{derives}{attributes}{vis}enum {name}{params}{{{variants}}}",
            derives = self.derives.as_cairo(),
            attributes = self.attributes.as_cairo_block(),
            vis = self.visibility.as_cairo(),
            params = self.generic_params.as_cairo(),
            name = self.name,
            variants = self.variants.as_cairo_block_section()
        )
    }
}

impl SyntaxItemTrait for Enum {
    fn from_file_node<'db>(
        db: &'db dyn Database,
        node: cairo_lang_syntax::node::SyntaxNode<'db>,
    ) -> IntrospectResult<Self> {
        for child in node.get_children(db)[0].get_children(db) {
            let kind = child.kind(db);
            match kind {
                SyntaxKind::ItemEnum => {
                    return Enum::try_from_syntax_node(db, *child);
                }
                _ => continue,
            }
        }
        Err(IntrospectError::NoEnum())
    }
}

impl ItemTrait for Enum {
    fn name(&self) -> &str {
        &self.name
    }
    fn generic_params(&self) -> &GenericParams {
        &self.generic_params
    }
}
