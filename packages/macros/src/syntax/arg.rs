use crate::syntax::Expr;
use crate::{
    CairoFormat, Modifier, from_typed_syntax_node, syntax_enum, syntax_option, syntax_type,
    terminal_to_string, typed_syntax_node_to_string_without_trivia, vec_from_element_list,
};

syntax_type! {
    Arg {
        modifiers: Vec<Modifier>,
        clause[arg_clause]: ArgClause,
    }
}

syntax_enum! {
    ArgClause{
        Unnamed(Expr),
        Named(NamedArg),
        Shorthand[FieldInitShorthand](String),
    }
}

syntax_type! {
    NamedArg[ArgClauseNamed]{
        name: String,
        value: Expr,
    }
}

from_typed_syntax_node!(ArgClauseUnnamed.value, Expr);
terminal_to_string! {ArgClauseFieldInitShorthand.name.name,}
typed_syntax_node_to_string_without_trivia! {ArgClauseUnnamed.value}
vec_from_element_list!(ArgListParenthesized.arguments, Arg);
syntax_option! {OptionArgListParenthesized{ArgListParenthesized: Vec<Arg>}}

impl Arg {
    pub fn as_unnamed(&self) -> Option<String> {
        self.clause.as_unnamed()
    }
}

impl ArgClause {
    pub fn as_unnamed(&self) -> Option<String> {
        match self {
            ArgClause::Unnamed(value) => Some(value.to_cairo()),
            _ => None,
        }
    }
}

// impl AsCairo for NamedArg {
//     fn as_cairo(&self) -> String {
//         format!("{}: {}", self.name, self.value.as_cairo())
//     }
// }

// impl AsCairo for ArgClause {
//     fn as_cairo(&self) -> String {
//         match self {
//             ArgClause::Unnamed(value) => value.as_cairo(),
//             ArgClause::Named(named) => named.as_cairo(),
//             ArgClause::Shorthand(name) => format!(":{}", name),
//         }
//     }
// }

// impl AsCairo for Arg {
//     fn as_cairo(&self) -> String {
//         format!("{}{}", self.modifiers.as_cairo(), self.clause.as_cairo())
//     }
// }

// impl AsCairo for Option<Vec<Arg>> {
//     fn as_cairo(&self) -> String {
//         match self {
//             Some(args) => args.as_cairo_csv_parenthesized(),
//             None => "".to_string(),
//         }
//     }
// }

// pub fn parse_args(token_stream: TokenStream) -> Option<Vec<Arg>> {
//     let args_stream = quote! {
//         #[attr #token_stream]
//         mod rocker;
//     };
//     let db = SimpleParserDatabase::default();
//     let (parsed, _diag) = db.parse_virtual_with_diagnostics(args_stream);
//     let child = parsed.get_children(&db)[0].get_children(&db)[0].get_children(&db)[0]
//         .get_children(&db)[0]
//         .get_children(&db)[3];
//     println!("{}", print_tree(&db, &child, true, true));
//     Option::<Vec<Arg>>::from_syntax_node(&db, child)
// }
