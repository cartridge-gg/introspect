use crate::syntax::expr::{ExprPath, PathSegment};
use crate::syntax::{Expr, Param, Statement};
use crate::{
    AstInto, Attribute, Enum, Struct, Visibility, syntax_enum, syntax_option, syntax_type,
    terminal_to_string, typed_syntax_node_to_string_without_trivia, vec_from_element_list,
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
        InlineMacro,
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
    generics[generic_params]: Option<Vec<String>>,
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
        generics[generic_params]: Option<Vec<String>>,
}}
syntax_type! { Trait[ItemTrait]{
        attributes: Vec<Attribute>,
        visibility: Visibility,
        name: String,
        generics[generic_params]: Option<Vec<String>>,
        body: Option<Vec<TraitItem>>,
}}
syntax_type! { Impl[ItemImpl]{
        attributes: Vec<Attribute>,
        visibility: Visibility,
        generics[generic_params]: Option<Vec<String>>,
        path[trait_path]: ExprPath,
        body: Option<Vec<ImplItem>>,
}}
syntax_type! { ImplAlias[ItemImplAlias]{
        attributes: Vec<Attribute>,
        visibility: Visibility,
        name: String,
        generics[generic_params]: Option<Vec<String>>,
        path[impl_path]: ExprPath,
}}

syntax_type! { TypeAlias[ItemTypeAlias]{
        attributes: Vec<Attribute>,
        visibility: Visibility,
        name: String,
        generics[generic_params]: Option<Vec<String>>,
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
        generics[generic_params]: Option<Vec<String>>,
    }
}

syntax_type! {
    TraitImpl[TraitItemImpl]{
        attributes: Vec<Attribute>,
        name: String,
        path[trait_path]: ExprPath,
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
