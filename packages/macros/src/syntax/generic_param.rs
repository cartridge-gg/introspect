use crate::syntax::{Expr, ExprPath};
use crate::{
    from_typed_syntax_node, syntax_enum, syntax_option, syntax_type,
    typed_syntax_node_to_string_without_trivia, vec_from_element_list,
};

syntax_enum! {
    GenericParam {
        Type(String),
        Const(ConstGenericParam),
        ImplNamed(ImplNamedGenericParam),
        ImplAnonymous(ImplAnonymousGenericParam),
        NegativeImpl(ExprPath),
    }
}

syntax_type! {
    ConstGenericParam[GenericParamConst]{
        name: String,
        ty: Expr,
    }
}

syntax_type! {
    ImplNamedGenericParam[GenericParamImplNamed]{
        name: String,
        trait_path: ExprPath,
        type_constrains: Option<Vec<AssociatedItemConstraint>>,
    }
}

syntax_type! {
    AssociatedItemConstraint{
        item: String,
        value: Expr,
    }
}

syntax_type! {
   ImplAnonymousGenericParam[GenericParamImplAnonymous]{
        trait_path: ExprPath,
        type_constrains: Option<Vec<AssociatedItemConstraint>>,
    }
}

from_typed_syntax_node!(GenericParamNegativeImpl.trait_path, ExprPath);

typed_syntax_node_to_string_without_trivia!(GenericParamType.name);

syntax_option! {OptionAssociatedItemConstraints{AssociatedItemConstraints: Vec<AssociatedItemConstraint>}}
syntax_option! {OptionWrappedGenericParamList{WrappedGenericParamList: Vec<GenericParam>}}
vec_from_element_list! {AssociatedItemConstraints.associated_item_constraints,
AssociatedItemConstraint}

vec_from_element_list! {GenericParamList, GenericParam}
vec_from_element_list! {WrappedGenericParamList.generic_params, GenericParam}

// impl AsCairo for GenericParam {
//     fn as_cairo(&self) -> String {
//         match self {
//             GenericParam::Type(name) => name.to_string(),
//             GenericParam::Const(cgp) => cgp.as_cairo(),
//             GenericParam::ImplNamed(ingp) => ingp.as_cairo(),
//             GenericParam::ImplAnonymous(iagp) => iagp.as_cairo(),
//             GenericParam::NegativeImpl(trait_path) => {
//                 format!("-{}", trait_path.as_cairo())
//             }
//         }
//     }
// }

// impl AsCairo for ConstGenericParam {
//     fn as_cairo(&self) -> String {
//         format!("const {}: {}", self.name, self.ty.as_cairo())
//     }
// }

// impl AsCairo for ImplNamedGenericParam {
//     fn as_cairo(&self) -> String {
//         format!(
//             "impl {}: {}{}",
//             self.name,
//             self.trait_path.as_cairo(),
//             self.type_constrains.as_cairo()
//         )
//     }
// }

// impl AsCairo for ImplAnonymousGenericParam {
//     fn as_cairo(&self) -> String {
//         format!(
//             "+{}{}",
//             self.trait_path.as_cairo(),
//             self.type_constrains.as_cairo()
//         )
//     }
// }

// impl AsCairo for AssociatedItemConstraint {
//     fn as_cairo(&self) -> String {
//         format!("{}: {}", self.item, self.value.as_cairo())
//     }
// }

// impl AsCairo for Option<Vec<AssociatedItemConstraint>> {
//     fn as_cairo(&self) -> String {
//         match self {
//             Some(constraints) => constraints.as_cairo_csv_wrapped("[", "]"),
//             None => "".to_string(),
//         }
//     }
// }

// impl AsCairo for Option<Vec<GenericParam>> {
//     fn as_cairo(&self) -> String {
//         match self {
//             Some(params) => params.as_cairo_csv_wrapped("<", ">"),
//             None => "".to_string(),
//         }
//     }
// }
