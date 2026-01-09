use crate::{
    AsCairo, AstInto, AstToString, CollectionsAsCairo, FromAst, IntrospectError, Modifier, Result,
    terminal_to_string, vec_from_element_list,
};
use cairo_lang_syntax::node::ast::{
    Arg as AstArg, ArgClause as AstArgClause, ArgClauseNamed as AstArgClauseNamed,
    ArgClauseUnnamed as AstArgClauseUnnamed, ArgListParenthesized, Attribute as AstAttribute,
    OptionArgListParenthesized,
};
use cairo_lang_syntax::node::{Terminal, TypedSyntaxNode};
use salsa::Database;

#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub args: Option<Vec<AttributeArg>>,
}

impl Attribute {
    pub fn format_error(&self) -> IntrospectError {
        IntrospectError::InvalidIntrospectAttributeFormat(self.name.clone())
    }
    pub fn format_err<T>(&self) -> Result<T> {
        Err(self.format_error())
    }
    pub fn single_unnamed_arg(&self) -> Result<String> {
        match &self.args {
            Some(args) if args.len() == 1 => args[0].as_unnamed().ok_or(self.format_error()),
            _ => Err(self.format_error()),
        }
    }
    pub fn all_unnamed_args(&self) -> Result<Vec<String>> {
        match &self.args {
            Some(args) => args
                .iter()
                .map(|arg| arg.as_unnamed().ok_or(self.format_error()))
                .collect(),
            None => Ok(vec![]),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AttributeArg {
    pub clause: AttributeArgClause,
    pub modifiers: Vec<Modifier>,
}

#[derive(Clone, Debug)]
pub struct AttributeArgNamed {
    pub value: String,
    pub name: String,
}
#[derive(Clone, Debug)]
pub enum AttributeArgClause {
    Unnamed(String),
    Named(AttributeArgNamed),
    Shorthand(String),
}

impl AttributeArg {
    pub fn as_unnamed(&self) -> Option<String> {
        self.clause.as_unnamed()
    }
    pub fn to_unnamed(self) -> Option<String> {
        self.clause.to_unnamed()
    }
}

impl AttributeArgClause {
    pub fn as_unnamed(&self) -> Option<String> {
        match self {
            AttributeArgClause::Unnamed(value) => Some(value.clone()),
            _ => None,
        }
    }
    pub fn to_unnamed(self) -> Option<String> {
        match self {
            AttributeArgClause::Unnamed(value) => Some(value.clone()),
            _ => None,
        }
    }
}

impl<'db> FromAst<'db, AstArgClauseNamed<'db>> for AttributeArgNamed {
    fn from_ast(ast: AstArgClauseNamed<'db>, db: &'db dyn Database) -> Self {
        AttributeArgNamed {
            value: ast
                .value(db)
                .as_syntax_node()
                .get_text_without_all_comment_trivia(db),
            name: ast.name(db).text(db).to_string(db),
        }
    }
}

impl<'db> AstToString<'db> for AstArgClauseUnnamed<'db> {
    fn to_string(&self, db: &'db dyn Database) -> String {
        self.value(db).to_string(db)
    }
}

terminal_to_string!(ArgClauseFieldInitShorthand.name.name,);

impl<'db> FromAst<'db, AstArgClause<'db>> for AttributeArgClause {
    fn from_ast(ast: AstArgClause<'db>, db: &'db dyn Database) -> Self {
        match ast {
            AstArgClause::Unnamed(clause) => AttributeArgClause::Unnamed(clause.to_string(db)),
            AstArgClause::Named(clause) => AttributeArgClause::Named(clause.ast_into(db)),
            AstArgClause::FieldInitShorthand(clause) => {
                AttributeArgClause::Shorthand(clause.to_string(db))
            }
        }
    }
}

impl<'db> FromAst<'db, AstArg<'db>> for AttributeArg {
    fn from_ast(ast_arg: AstArg<'db>, db: &'db dyn Database) -> Self {
        let arg = ast_arg.arg_clause(db).ast_into(db);
        let modifiers = ast_arg
            .modifiers(db)
            .elements(db)
            .map(|m| m.into())
            .collect();

        AttributeArg {
            clause: arg,
            modifiers,
        }
    }
}

impl<'db> FromAst<'db, OptionArgListParenthesized<'db>> for Option<Vec<AttributeArg>> {
    fn from_ast(ast: OptionArgListParenthesized<'db>, db: &'db dyn Database) -> Self {
        match ast {
            OptionArgListParenthesized::Empty(_) => None,
            OptionArgListParenthesized::ArgListParenthesized(arg_list) => {
                Some(arg_list.ast_into(db))
            }
        }
    }
}

impl<'db> FromAst<'db, ArgListParenthesized<'db>> for Vec<AttributeArg> {
    fn from_ast(ast: ArgListParenthesized<'db>, db: &'db dyn Database) -> Self {
        ast.arguments(db)
            .elements(db)
            .map(|arg| arg.ast_into(db))
            .collect()
    }
}

impl<'db> FromAst<'db, AstAttribute<'db>> for Attribute {
    fn from_ast(ast_attribute: AstAttribute<'db>, db: &'db dyn Database) -> Self {
        Self {
            name: ast_attribute
                .attr(db)
                .as_syntax_node()
                .get_text_without_trivia(db)
                .to_string(db),
            args: ast_attribute.arguments(db).ast_into(db),
        }
    }
}

vec_from_element_list!(AttributeList, Attribute);

impl AsCairo for AttributeArgNamed {
    fn as_cairo(&self) -> String {
        format!("{}: {}", self.name, self.value)
    }
}

impl AsCairo for AttributeArgClause {
    fn as_cairo(&self) -> String {
        match self {
            AttributeArgClause::Unnamed(value) => value.clone(),
            AttributeArgClause::Named(named) => named.as_cairo(),
            AttributeArgClause::Shorthand(name) => format!(":{}", name),
        }
    }
}

impl AsCairo for AttributeArg {
    fn as_cairo(&self) -> String {
        format!("{}{}", self.modifiers.as_cairo(), self.clause.as_cairo())
    }
}

impl AsCairo for Option<Vec<AttributeArg>> {
    fn as_cairo(&self) -> String {
        match self {
            Some(args) => args.as_cairo_csv_wrapped("(", ")"),
            None => "".to_string(),
        }
    }
}

impl AsCairo for Attribute {
    fn as_cairo(&self) -> String {
        format!("#[{}{}]", self.name, self.args.as_cairo())
    }
}
