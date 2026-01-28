use super::{Expr, ExprPath, GenericParam, Param, PathSegment, Statement};
use crate::{
    AsCairo, AstInto, Attribute, CollectionsAsCairo, Enum, Struct, Visibility, syntax_enum,
    syntax_option, syntax_type, terminal_to_string, typed_syntax_node_to_string_without_trivia,
    vec_from_element_list,
};

//TODO: implement other module items.
syntax_enum! {
    Item[ModuleItem]{
        Constant(Constant),
        Module(Module),
        Use(UseItem),
        FreeFunction(FunctionWithBody),
        ExternFunction(ExternFunction),
        ExternType(ExternType),
        Trait(Trait),
        Impl(Impl),
        ImplAlias(ImplAlias),
        Struct(Struct),
        Enum(Enum),
        TypeAlias(TypeAlias),
        InlineMacro(InlineMacroItem),
        MacroDeclaration,
        HeaderDoc(String),
        Missing,
    }
}

syntax_type! {
    Constant[ItemConstant]{
        attributes: Vec<Attribute>,
        visibility: Visibility,
        name: String,
        ty[type_clause]: Expr,
        value: Expr,
    }
}

syntax_type! { Module[ItemModule]{
        attributes: Vec<Attribute>,
        visibility: Visibility,
        name: String,
        body: Option<Vec<Item>>,

}}
syntax_type! { UseItem[ItemUse]{
    attributes: Vec<Attribute>,
        visibility: Visibility,
        dollar: bool,
        path[use_path]: UsePath,

}}
syntax_type! { FunctionWithBody{
        attributes: Vec<Attribute>,
        visibility: Visibility,
        declaration: FunctionDeclaration,
        body: Vec<Statement>,
}}

syntax_type! { FunctionDeclaration{
    is_const[optional_const]: bool,
    name: String,
    generic_params: Option<Vec<GenericParam>>,
    signature: FunctionSignature,
}}
syntax_type! {FunctionSignature{
    parameters: Vec<Param>,
    return_type[ret_ty]: Option<Expr>,
    implicits_clause: Option<Vec<ExprPath>>,
    no_panic[optional_no_panic]: bool,
}}

syntax_type! { ExternFunction[ItemExternFunction]{
        attributes: Vec<Attribute>,
        visibility: Visibility,
        declaration: FunctionDeclaration,
}}
syntax_type! { ExternType[ItemExternType]{
        attributes: Vec<Attribute>,
        visibility: Visibility,
        name: String,
        generic_params: Option<Vec<GenericParam>>,
}}
syntax_type! { Trait[ItemTrait]{
        attributes: Vec<Attribute>,
        visibility: Visibility,
        name: String,
        generic_params: Option<Vec<GenericParam>>,
        body: Option<Vec<TraitItem>>,
}}
syntax_type! { Impl[ItemImpl]{
        attributes: Vec<Attribute>,
        visibility: Visibility,
        generic_params: Option<Vec<GenericParam>>,
        path[trait_path]: ExprPath,
        body: Option<Vec<ImplItem>>,
}}
syntax_type! { ImplAlias[ItemImplAlias]{
        attributes: Vec<Attribute>,
        visibility: Visibility,
        name: String,
        generic_params: Option<Vec<GenericParam>>,
        path[impl_path]: ExprPath,
}}

syntax_type! { TypeAlias[ItemTypeAlias]{
        attributes: Vec<Attribute>,
        visibility: Visibility,
        name: String,
        generic_params: Option<Vec<GenericParam>>,
        ty: Expr,
}}
syntax_type! { InlineMacroItem[ItemInlineMacro]{
        attributes: Vec<Attribute>,
        path: ExprPath,
        arguments: String,
}}

syntax_type! {
    UsePathLeaf{
        ident: PathSegment,
        alias[alias_clause]: Option<String>,
    }
}

syntax_type! {
    UsePathSingle{
        ident: PathSegment,
        path[use_path]: Box<UsePath>,
    }
}

syntax_type! {
    TraitFunction[TraitItemFunction]{
        attributes: Vec<Attribute>,
        declaration: FunctionDeclaration,
        body: Option<Vec<Statement>>,
    }
}

syntax_type! {
    TraitConstant[TraitItemConstant]{
        attributes: Vec<Attribute>,
        name: String,
        ty[type_clause]: Expr,
    }
}

syntax_type! {
    TraitType[TraitItemType]{
        attributes: Vec<Attribute>,
        name: String,
        generic_params: Option<Vec<GenericParam>>,
    }
}

syntax_type! {
    TraitImpl[TraitItemImpl]{
        attributes: Vec<Attribute>,
        name: String,
        trait_path: ExprPath,
    }
}

syntax_enum! {
    TraitItem{
        Function(TraitFunction),
        Type(TraitType),
        Constant(TraitConstant),
        Impl(TraitImpl),
        Missing,
    }
}

syntax_enum! {
    ImplItem{
        Function(FunctionWithBody),
        Type(TypeAlias),
        Constant(Constant),
        Impl(ImplAlias),
        Module(Module),
        Use(UseItem),
        ExternFunction(ExternFunction),
        ExternType(ExternType),
        Trait(Trait),
        Struct(Struct),
        Enum(Enum),
        Missing,
    }
}

syntax_enum! {
    UsePath{
        Leaf(UsePathLeaf),
        Single(UsePathSingle),
        Multi(Vec<UsePath>),
        Star,
    }
}

syntax_option! {OptionAliasClause{AliasClause: String}}
syntax_option! {OptionImplicitsClause{ImplicitsClause: Vec<ExprPath>}}
syntax_option! {MaybeModuleBody{Some: Vec<Item>, None}}
syntax_option! {MaybeTraitBody{Some: Vec<TraitItem>, None}}
syntax_option! {MaybeTraitFunctionBody{Some: Vec<Statement>, None}}
syntax_option! {MaybeImplBody{Some: Vec<ImplItem>, None}}

vec_from_element_list!(UsePathMulti.use_paths, UsePath);
typed_syntax_node_to_string_without_trivia! {AliasClause.alias}
terminal_to_string! {ItemHeaderDoc.empty}

vec_from_element_list!(ImplicitsClause.implicits, ExprPath);
vec_from_element_list!(ModuleBody.items, Item);
vec_from_element_list!(TraitBody.items, TraitItem);
vec_from_element_list!(ImplBody.items, ImplItem);

impl AsCairo for Item {
    fn as_cairo(&self) -> String {
        match self {
            Item::Constant(e) => e.as_cairo(),
            Item::Module(e) => e.as_cairo(),
            Item::Use(e) => e.as_cairo(),
            Item::FreeFunction(e) => e.as_cairo(),
            Item::ExternFunction(e) => e.as_cairo(),
            Item::ExternType(e) => e.as_cairo(),
            Item::Trait(e) => e.as_cairo(),
            Item::Impl(e) => e.as_cairo(),
            Item::ImplAlias(e) => e.as_cairo(),
            Item::Struct(e) => e.as_cairo(),
            Item::Enum(e) => e.as_cairo(),
            Item::TypeAlias(e) => e.as_cairo(),
            Item::InlineMacro(e) => e.as_cairo(),
            Item::MacroDeclaration => "".to_string(),
            Item::HeaderDoc(_) => "".to_string(),
            Item::Missing => "".to_string(),
        }
    }
}

impl AsCairo for Constant {
    fn as_cairo(&self) -> String {
        format!(
            "{attributes}{vis}const {name}: {ty} = {value};",
            attributes = self.attributes.as_cairo_block_section(),
            vis = self.visibility.as_cairo(),
            name = self.name,
            ty = self.ty.as_cairo(),
            value = self.value.as_cairo(),
        )
    }
}
impl AsCairo for Module {
    fn as_cairo(&self) -> String {
        let body = match &self.body {
            Some(items) => format!("{{{}}}", items.as_cairo_block_section()),
            None => ";".to_string(),
        };
        format!(
            "{attributes}{vis}mod {name} {body}",
            attributes = self.attributes.as_cairo_block_section(),
            vis = self.visibility.as_cairo(),
            name = self.name,
            body = body,
        )
    }
}

impl AsCairo for UseItem {
    fn as_cairo(&self) -> String {
        format!(
            "{attributes}{vis}use {dollar}{path};",
            attributes = self.attributes.as_cairo_block_section(),
            vis = self.visibility.as_cairo(),
            dollar = if self.dollar { "$" } else { "" },
            path = self.path.as_cairo(),
        )
    }
}

impl AsCairo for UsePath {
    fn as_cairo(&self) -> String {
        match self {
            UsePath::Leaf(e) => e.as_cairo(),
            UsePath::Single(e) => e.as_cairo(),
            UsePath::Multi(e) => e.as_cairo_csv_braced(),
            UsePath::Star => "*".to_string(),
        }
    }
}

impl AsCairo for UsePathLeaf {
    fn as_cairo(&self) -> String {
        match &self.alias {
            Some(alias) => format!("{} as {}", self.ident.as_cairo(), alias),
            None => self.ident.as_cairo(),
        }
    }
}

impl AsCairo for UsePathSingle {
    fn as_cairo(&self) -> String {
        format!("{}::{}", self.ident.as_cairo(), self.path.as_cairo())
    }
}

impl AsCairo for FunctionWithBody {
    fn as_cairo(&self) -> String {
        format!(
            "{attributes}{vis}{declaration} {{{body}}}",
            attributes = self.attributes.as_cairo_block_section(),
            vis = self.visibility.as_cairo(),
            declaration = self.declaration.as_cairo(),
            body = self.body.as_cairo_block_section(),
        )
    }
}

impl AsCairo for FunctionDeclaration {
    fn as_cairo(&self) -> String {
        let const_str = if self.is_const { "const " } else { "" };
        let generic_params = match &self.generic_params {
            Some(params) => format!("<{}>", params.as_cairo_csv()),
            None => "".to_string(),
        };
        format!(
            "{const_str}fn {name}{generic_params}{signature}",
            const_str = const_str,
            name = self.name,
            generic_params = generic_params,
            signature = self.signature.as_cairo(),
        )
    }
}

impl AsCairo for FunctionSignature {
    fn as_cairo(&self) -> String {
        let params = self.parameters.as_cairo_csv();
        let return_type = match &self.return_type {
            Some(ty) => format!(" -> {}", ty.as_cairo()),
            None => "".to_string(),
        };
        let implicits = match &self.implicits_clause {
            Some(implicits) => format!(" implicit({})", implicits.as_cairo_csv()),
            None => "".to_string(),
        };
        let no_panic_str = if self.no_panic { " no_panic" } else { "" };
        format!(
            "({params}){return_type}{implicits}{no_panic}",
            params = params,
            return_type = return_type,
            implicits = implicits,
            no_panic = no_panic_str,
        )
    }
}

impl AsCairo for ExternFunction {
    fn as_cairo(&self) -> String {
        format!(
            "{attributes}{vis}extern fn {declaration};",
            attributes = self.attributes.as_cairo_block_section(),
            vis = self.visibility.as_cairo(),
            declaration = self.declaration.as_cairo(),
        )
    }
}

impl AsCairo for ExternType {
    fn as_cairo(&self) -> String {
        let generic_params = match &self.generic_params {
            Some(params) => format!("<{}>", params.as_cairo_csv()),
            None => "".to_string(),
        };
        format!(
            "{attributes}{vis}extern type {name}{generic_params};",
            attributes = self.attributes.as_cairo_block_section(),
            vis = self.visibility.as_cairo(),
            name = self.name,
            generic_params = generic_params,
        )
    }
}

impl AsCairo for Trait {
    fn as_cairo(&self) -> String {
        let body = match &self.body {
            Some(items) => format!("{{{}}}", items.as_cairo_block_section()),
            None => ";".to_string(),
        };
        let generic_params = match &self.generic_params {
            Some(params) => format!("<{}>", params.as_cairo_csv()),
            None => "".to_string(),
        };
        format!(
            "{attributes}{vis}trait {name}{generic_params} {body}",
            attributes = self.attributes.as_cairo_block_section(),
            vis = self.visibility.as_cairo(),
            name = self.name,
            generic_params = generic_params,
            body = body,
        )
    }
}

impl AsCairo for TraitItem {
    fn as_cairo(&self) -> String {
        match self {
            TraitItem::Function(e) => e.as_cairo(),
            TraitItem::Type(e) => e.as_cairo(),
            TraitItem::Constant(e) => e.as_cairo(),
            TraitItem::Impl(e) => e.as_cairo(),
            TraitItem::Missing => "".to_string(),
        }
    }
}

impl AsCairo for TraitFunction {
    fn as_cairo(&self) -> String {
        format!(
            "{attributes}{declaration}{body}",
            attributes = self.attributes.as_cairo_block_section(),
            declaration = self.declaration.as_cairo(),
            body = self.body.as_cairo()
        )
    }
}

impl AsCairo for TraitConstant {
    fn as_cairo(&self) -> String {
        format!(
            "{attributes}const {name}: {ty};",
            attributes = self.attributes.as_cairo_block_section(),
            name = self.name,
            ty = self.ty.as_cairo(),
        )
    }
}

impl AsCairo for TraitType {
    fn as_cairo(&self) -> String {
        let generic_params = match &self.generic_params {
            Some(params) => format!("<{}>", params.as_cairo_csv()),
            None => "".to_string(),
        };
        format!(
            "{attributes}type {name}{generic_params};",
            attributes = self.attributes.as_cairo_block_section(),
            name = self.name,
            generic_params = generic_params,
        )
    }
}

impl AsCairo for TraitImpl {
    fn as_cairo(&self) -> String {
        format!(
            "{attributes}impl {name}: {trait_path};",
            attributes = self.attributes.as_cairo_block_section(),
            name = self.name,
            trait_path = self.trait_path.as_cairo(),
        )
    }
}

impl AsCairo for Impl {
    fn as_cairo(&self) -> String {
        let body = match &self.body {
            Some(items) => format!("{{{}}}", items.as_cairo_block_section()),
            None => ";".to_string(),
        };

        format!(
            "{attributes}{vis}impl{generic_params} of {trait_path} {body}",
            attributes = self.attributes.as_cairo_block_section(),
            vis = self.visibility.as_cairo(),
            generic_params = self.generic_params.as_cairo(),
            trait_path = self.path.as_cairo(),
        )
    }
}

impl AsCairo for ImplItem {
    fn as_cairo(&self) -> String {
        match self {
            ImplItem::Function(e) => e.as_cairo(),
            ImplItem::Type(e) => e.as_cairo(),
            ImplItem::Constant(e) => e.as_cairo(),
            ImplItem::Impl(e) => e.as_cairo(),
            ImplItem::Module(e) => e.as_cairo(),
            ImplItem::Use(e) => e.as_cairo(),
            ImplItem::ExternFunction(e) => e.as_cairo(),
            ImplItem::ExternType(e) => e.as_cairo(),
            ImplItem::Trait(e) => e.as_cairo(),
            ImplItem::Struct(e) => e.as_cairo(),
            ImplItem::Enum(e) => e.as_cairo(),
            ImplItem::Missing => "".to_string(),
        }
    }
}

impl AsCairo for TypeAlias {
    fn as_cairo(&self) -> String {
        format!(
            "{attributes}{vis}type {name}{generic_params} = {ty};",
            attributes = self.attributes.as_cairo_block_section(),
            vis = self.visibility.as_cairo(),
            name = self.name,
            generic_params = self.generic_params.as_cairo(),
            ty = self.ty.as_cairo(),
        )
    }
}

impl AsCairo for ImplAlias {
    fn as_cairo(&self) -> String {
        format!(
            "{attributes}{vis}impl {name}{generic_params} = {impl_path};",
            attributes = self.attributes.as_cairo_block_section(),
            vis = self.visibility.as_cairo(),
            name = self.name,
            generic_params = self.generic_params.as_cairo(),
            impl_path = self.path.as_cairo(),
        )
    }
}

impl AsCairo for InlineMacroItem {
    fn as_cairo(&self) -> String {
        format!(
            "{attributes}{path}!({arguments});",
            attributes = self.attributes.as_cairo_block_section(),
            path = self.path.as_cairo(),
            arguments = self.arguments,
        )
    }
}
