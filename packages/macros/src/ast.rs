use crate::IntrospectResult;
use cairo_lang_syntax::node::{SyntaxNode, Terminal, TypedSyntaxNode};
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

pub trait TryFromAst<'db, T>
where
    Self: Sized,
    T: TypedSyntaxNode<'db>,
{
    fn try_from_ast(ast: T, db: &'db dyn Database) -> IntrospectResult<Self>;
    fn try_from_syntax_node(
        db: &'db dyn Database,
        node: SyntaxNode<'db>,
    ) -> IntrospectResult<Self> {
        Self::try_from_ast(T::from_syntax_node(db, node), db)
    }
}

pub trait AstInto<'db, T>
where
    Self: TypedSyntaxNode<'db>,
{
    fn ast_into(self, db: &'db dyn Database) -> T;
}

pub trait AstTryInto<'db, T>
where
    Self: TypedSyntaxNode<'db>,
{
    fn ast_try_into(self, db: &'db dyn Database) -> IntrospectResult<T>;
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

impl<'db, T, U> AstTryInto<'db, U> for T
where
    T: TypedSyntaxNode<'db>,
    U: TryFromAst<'db, T>,
{
    fn ast_try_into(self, db: &'db dyn Database) -> IntrospectResult<U> {
        U::try_from_ast(self, db)
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

#[macro_export]
macro_rules! terminal_to_string {

    ($($terminal:ident $(. $($methods:ident).+)?),* $(,)?) => {
        $(
            impl<'db> AstToString<'db> for cairo_lang_syntax::node::ast::$terminal<'db> {
                fn to_string(&self, db: &'db dyn Database) -> String {
                    self$(.$($methods(db)).+)?.text(db).to_string(db)
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! typed_syntax_node_to_string_without_trivia {
    ($typed_syntax_node:ident $(. $($methods:ident).+)?) => {
        impl<'db> AstToString<'db> for cairo_lang_syntax::node::ast::$typed_syntax_node<'db> {
            fn to_string(&self, db: &'db dyn Database) -> String {
                self$(.$($methods(db)).+)?.as_syntax_node().get_text_without_trivia(db).to_string(db)
            }
        }
    };
}

#[macro_export]
macro_rules! vec_from_element_list {
    ($list:ident $(. $($methods:ident).+)?, $element:ident) => {
        impl<'db> FromAst<'db, cairo_lang_syntax::node::ast::$list<'db>> for Vec<$element> {
            fn from_ast(ast: cairo_lang_syntax::node::ast::$list<'db>, db: &'db dyn Database) -> Vec<$element> {
                ast$(.$($methods(db)).+)?.elements(db).into_iter().map(|e| e.ast_into(db)).collect()
            }
        }
    };
}

#[macro_export]
macro_rules! vec_try_from_element_list {
    ($list:ident $(. $($methods:ident).+)?, $element:ident) => {
        impl<'db> TryFromAst<'db, cairo_lang_syntax::node::ast::$list<'db>> for Vec<$element> {
            fn try_from_ast(ast: cairo_lang_syntax::node::ast::$list<'db>, db: &'db dyn Database) -> IntrospectResult<Vec<$element>> {
                ast$(.$($methods(db)).+)?.elements(db).into_iter().map(|e| e.ast_try_into(db)).collect()
            }
        }
    };
}

typed_syntax_node_to_string_without_trivia!(Expr);
typed_syntax_node_to_string_without_trivia!(TypeClause.ty);

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
