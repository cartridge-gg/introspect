use super::{Expr, ExprPath, GenericParam, Param, PathSegment, Statement};
use crate::{
    AstInto, Attribute, FromAst, Visibility, syntax_enum, syntax_option, syntax_type,
    terminal_to_string, typed_syntax_node_to_string_without_trivia, vec_from_element_list,
};
use cairo_lang_macro::TokenStream;
use cairo_lang_parser::utils::SimpleParserDatabase;
use cairo_lang_syntax::node::ast::SyntaxFile;

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
    Struct[ItemStruct]{
        attributes: Vec<Attribute>,
        visibility: Visibility,
        name: String,
        generic_params: Option<Vec<GenericParam>>,
        members: Vec<Member>,
    }
}

syntax_type! {
    Member{
        attributes: Vec<Attribute>,
        visibility: Visibility,
        name: String,
        ty[type_clause]: Expr,
    }
}

syntax_type! {
    Enum[ItemEnum]{
        attributes: Vec<Attribute>,
        visibility: Visibility,
        name: String,
        generic_params: Option<Vec<GenericParam>>,
        variants: Vec<Variant>,
    }
}

syntax_type! {
    Variant{
        attributes: Vec<Attribute>,
        name: String,
        type_clause: Option<Expr>,
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
        name: String,
        generic_params: Option<Vec<GenericParam>>,
        trait_path: ExprPath,
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
vec_from_element_list!(SyntaxFile.items, Item);
vec_from_element_list!(MemberList, Member);
vec_from_element_list!(VariantList, Variant);

pub fn items_from_token_stream(token_stream: TokenStream) -> Vec<Item> {
    let db = SimpleParserDatabase::default();
    let (node, _diagnostics) = db.parse_virtual_with_diagnostics(token_stream);
    FromAst::<SyntaxFile>::from_syntax_node(&db, node)
}
