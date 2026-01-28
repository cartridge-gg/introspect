use crate::syntax::Expr;
use crate::{
    AsCairo, AstInto, AstToString, CollectionsAsCairo, FromAst, Modifier, syntax_enum, syntax_type,
    terminal_to_string,
};
use cairo_lang_macro::{TokenStream, quote};
use cairo_lang_parser::printer::print_tree;
use cairo_lang_parser::utils::SimpleParserDatabase;
use cairo_lang_syntax::node::ast::{
    ArgClauseUnnamed as AstArgClauseUnnamed, ArgListParenthesized, OptionArgListParenthesized,
};
use cairo_lang_syntax::node::kind::SyntaxKind;
use salsa::Database;

syntax_type! {
    Arg {
        clause[arg_clause]: ArgClause,
        modifiers: Vec<Modifier>,
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

impl Arg {
    pub fn as_unnamed(&self) -> Option<String> {
        self.clause.as_unnamed()
    }
    pub fn to_unnamed(self) -> Option<String> {
        self.clause.to_unnamed()
    }
}

impl ArgClause {
    pub fn as_unnamed(&self) -> Option<String> {
        match self {
            ArgClause::Unnamed(value) => Some(value.clone()),
            _ => None,
        }
    }
    pub fn to_unnamed(self) -> Option<String> {
        match self {
            ArgClause::Unnamed(value) => Some(value.clone()),
            _ => None,
        }
    }
}

impl<'db> AstToString<'db> for AstArgClauseUnnamed<'db> {
    fn to_string(&self, db: &'db dyn Database) -> String {
        self.value(db).to_string(db)
    }
}

// terminal_to_string!(ArgClauseUnnamed.value,);
terminal_to_string!(ArgClauseFieldInitShorthand.name.name,);

// impl<'db> FromAst<'db, AstArg<'db>> for Arg {
//     fn from_ast(ast_arg: AstArg<'db>, db: &'db dyn Database) -> Self {
//         let arg = ast_arg.arg_clause(db).ast_into(db);
//         let modifiers = ast_arg.modifiers(db).ast_into(db);
//         Arg {
//             clause: arg,
//             modifiers,
//         }
//     }
// }

impl<'db> FromAst<'db, OptionArgListParenthesized<'db>> for Option<Vec<Arg>> {
    fn from_ast(ast: OptionArgListParenthesized<'db>, db: &'db dyn Database) -> Self {
        match ast {
            OptionArgListParenthesized::Empty(_) => None,
            OptionArgListParenthesized::ArgListParenthesized(arg_list) => {
                Some(arg_list.ast_into(db))
            }
        }
    }
    fn from_syntax_node(
        db: &'db dyn Database,
        node: cairo_lang_syntax::node::SyntaxNode<'db>,
    ) -> Self {
        match node.kind(db) {
            SyntaxKind::OptionArgListParenthesizedEmpty => None,
            SyntaxKind::ArgListParenthesized => Vec::<Arg>::from_syntax_node(db, node).into(),
            _ => panic!("Not an ArgListParenthesized Or Empty"),
        }
    }
}

impl<'db> FromAst<'db, ArgListParenthesized<'db>> for Vec<Arg> {
    fn from_ast(ast: ArgListParenthesized<'db>, db: &'db dyn Database) -> Self {
        ast.arguments(db)
            .elements(db)
            .map(|arg| arg.ast_into(db))
            .collect()
    }
}

impl AsCairo for ArgNamed {
    fn as_cairo(&self) -> String {
        format!("{}: {}", self.name, self.value)
    }
}

impl AsCairo for ArgClause {
    fn as_cairo(&self) -> String {
        match self {
            ArgClause::Unnamed(value) => value.clone(),
            ArgClause::Named(named) => named.as_cairo(),
            ArgClause::Shorthand(name) => format!(":{}", name),
        }
    }
}

impl AsCairo for Arg {
    fn as_cairo(&self) -> String {
        format!("{}{}", self.modifiers.as_cairo(), self.clause.as_cairo())
    }
}

impl AsCairo for Option<Vec<Arg>> {
    fn as_cairo(&self) -> String {
        match self {
            Some(args) => args.as_cairo_csv_wrapped("(", ")"),
            None => "".to_string(),
        }
    }
}

pub fn parse_args(token_stream: TokenStream) -> Option<Vec<Arg>> {
    let args_stream = quote! {
        #[attr #token_stream]
        mod rocker;
    };
    let db = SimpleParserDatabase::default();
    let (parsed, _diag) = db.parse_virtual_with_diagnostics(args_stream);
    let child = parsed.get_children(&db)[0].get_children(&db)[0].get_children(&db)[0]
        .get_children(&db)[0]
        .get_children(&db)[3];
    println!("{}", print_tree(&db, &child, true, true));
    Option::<Vec<Arg>>::from_syntax_node(&db, child)
}
