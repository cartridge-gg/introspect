use crate::{terminal_to_string, typed_syntax_node_to_string_without_trivia};
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};
use salsa::Database;

pub trait FromAst<'db, T>
where
    T: TypedSyntaxNode<'db>,
    Self: Sized,
{
    fn from_ast(ast: T, db: &'db dyn Database) -> Self;
    fn from_syntax_node(db: &'db dyn Database, node: SyntaxNode<'db>) -> Self {
        Self::from_ast(T::from_syntax_node(db, node), db)
    }
}

pub trait AstInto<'db, T>
where
    Self: TypedSyntaxNode<'db>,
{
    fn ast_into(self, db: &'db dyn Database) -> T;
}

impl<'db, T, U> AstInto<'db, U> for T
where
    T: TypedSyntaxNode<'db>,
    U: FromAst<'db, T>,
{
    fn ast_into(self, db: &'db dyn Database) -> U {
        U::from_ast(self, db)
    }
}

pub trait AstToString<'db> {
    fn to_string(&self, db: &'db dyn Database) -> String;
}

impl<'db, T> FromAst<'db, T> for String
where
    T: AstToString<'db> + TypedSyntaxNode<'db>,
{
    fn from_ast(ast: T, db: &'db dyn Database) -> Self {
        ast.to_string(db)
    }
}

impl<'db, T, S> FromAst<'db, T> for Box<S>
where
    S: FromAst<'db, T>,
    T: TypedSyntaxNode<'db>,
{
    fn from_ast(ast: T, db: &'db dyn salsa::Database) -> Self {
        Box::new(S::from_ast(ast, db))
    }
}

typed_syntax_node_to_string_without_trivia! { Expr }
typed_syntax_node_to_string_without_trivia! { TypeClause.ty }

terminal_to_string! {
    TerminalIdentifier,
    TerminalLiteralNumber,
    TerminalShortString,
    TerminalString,
    TerminalAs,
    TerminalConst,
    TerminalElse,
    TerminalEnum,
    TerminalExtern,
    TerminalFalse,
    TerminalFunction,
    TerminalIf,
    TerminalWhile,
    TerminalFor,
    TerminalLoop,
    TerminalImpl,
    TerminalImplicits,
    TerminalLet,
    TerminalMacro,
    TerminalMatch,
    TerminalModule,
    TerminalMut,
    TerminalNoPanic,
    TerminalOf,
    TerminalRef,
    TerminalContinue,
    TerminalReturn,
    TerminalBreak,
    TerminalStruct,
    TerminalTrait,
    TerminalTrue,
    TerminalType,
    TerminalUse,
    TerminalPub,
    TerminalAnd,
    TerminalAndAnd,
    TerminalArrow,
    TerminalAt,
    TerminalBadCharacters,
    TerminalColon,
    TerminalColonColon,
    TerminalComma,
    TerminalDiv,
    TerminalDivEq,
    TerminalDollar,
    TerminalDot,
    TerminalDotDot,
    TerminalDotDotEq,
    TerminalEndOfFile,
    TerminalEq,
    TerminalEqEq,
    TerminalGE,
    TerminalGT,
    TerminalHash,
    TerminalLBrace,
    TerminalLBrack,
    TerminalLE,
    TerminalLParen,
    TerminalLT,
    TerminalMatchArrow,
    TerminalMinus,
    TerminalMinusEq,
    TerminalMod,
    TerminalModEq,
    TerminalMul,
    TerminalMulEq,
    TerminalNeq,
    TerminalNot,
    TerminalBitNot,
    TerminalOr,
    TerminalOrOr,
    TerminalPlus,
    TerminalPlusEq,
    TerminalQuestionMark,
    TerminalRBrace,
    TerminalRBrack,
    TerminalRParen,
    TerminalSemicolon,
    TerminalUnderscore,
    TerminalXor,
    TerminalEmpty,
}
