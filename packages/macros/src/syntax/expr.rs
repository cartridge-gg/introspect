use cairo_lang_syntax::node::ast;
use salsa::Database;

use super::{Condition, Param, Pattern, Statement};
use crate::{
    Arg, AstInto, FromAst, from_typed_syntax_node, syntax_enum, syntax_option,
    syntax_terminal_bool, syntax_terminal_enum, syntax_type,
    typed_syntax_node_to_string_without_trivia, vec_from_element_list,
};
syntax_enum! {
    Expr {
        Path(ExprPath),
        Literal(String),
        ShortString(String),
        String(String),
        False,
        True,
        Parenthesized(Box<Expr>),
        Unary(UnaryExpr),
        Binary(BinaryExpr),
        Tuple(Vec<Expr>),
        FunctionCall(FunctionCall),
        StructConstructorCall[StructCtorCall](StructConstructorCall),
        Block(Vec<Statement>),
        Match(MatchExpr),
        If(IfExpr),
        Loop(LoopExpr),
        While(WhileExpr),
        For(ForExpr),
        Closure(Closure),
        ErrorPropagate(Box<Expr>),
        FieldInitShorthand(String),
        Indexed(IndexExpr),
        InlineMacro(InlineMacroExpr),
        FixedSizeArray(FixedSizeArray),
        Underscore,
        Missing,
    }
}

syntax_type! {
    InlineMacro[ExprInlineMacro]{
        path: ExprPath,
        arguments: String,
    }
}

syntax_type! {
    ExprPath {
        dollar: bool,
        path[segments]: Vec<PathSegment>,
    }
}
syntax_type! {
    PathSegmentWithGenerics[PathSegmentWithGenericArgs] {
        ident: String,
        separator: bool,
        args[generic_args]: Vec<GenericArg>,
    }
}
syntax_type! {
    FunctionCall[ExprFunctionCall] {
        path: ExprPath,
        args[arguments]: Vec<Arg>,
    }
}
syntax_type! {
    StructConstructorCall[ExprStructCtorCall] {
        path: ExprPath,
        args[arguments]: Vec<StructArg>,
    }
}
syntax_type! {
    MatchExpr[ExprMatch] {
        expr: Box<Expr>,
        arms: Vec<MatchArm>,
    }
}

syntax_type! {
    ElseIfBlock[ExprIf] {
        conditions: Vec<Condition>,
        body[if_block]: Vec<Statement>,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IfExpr {
    pub conditions: Vec<Condition>,
    pub if_block: Vec<Statement>,
    pub else_if_clauses: Vec<ElseIfBlock>,
    pub else_clause: Option<Vec<Statement>>,
}

impl<'db> FromAst<'db, ast::ExprIf<'db>> for IfExpr {
    fn from_ast(ast: ast::ExprIf<'db>, db: &'db dyn Database) -> Self {
        let mut else_if_clauses = vec![];
        let else_block = unpack_if_else_clause(ast.else_clause(db), &mut else_if_clauses, db);
        IfExpr {
            conditions: ast.conditions(db).ast_into(db),
            if_block: ast.if_block(db).statements(db).ast_into(db),
            else_if_clauses,
            else_clause: else_block,
        }
    }
}

fn unpack_if_else_clause<'db>(
    clause: ast::OptionElseClause<'db>,
    clauses: &mut Vec<ElseIfBlock>,
    db: &'db dyn Database,
) -> Option<Vec<Statement>> {
    match clause {
        ast::OptionElseClause::Empty(_) => None,
        ast::OptionElseClause::ElseClause(c) => match c.else_block_or_if(db) {
            ast::BlockOrIf::Block(b) => Some(b.statements(db).ast_into(db)),
            ast::BlockOrIf::If(i) => {
                let else_clause = i.else_clause(db);
                clauses.push(i.ast_into(db));
                unpack_if_else_clause(else_clause, clauses, db)
            }
        },
    }
}

syntax_type! {
    LoopExpr[ExprLoop] {
        body: Vec<Statement>,
    }
}

syntax_type! {
    WhileExpr[ExprWhile] {
        conditions: Vec<Condition>,
        body: Vec<Statement>,
    }
}

syntax_type! {
    ForExpr[ExprFor] {
        pattern: Pattern,
        identifier: String,
        expr: Box<Expr>,
        body: Vec<Statement>,
    }
}

syntax_type! {
    FixedSizeArray[ExprFixedSizeArray] {
        exprs: Vec<Expr>,
        size: Option<Box<Expr>>,
    }
}

syntax_type! {
    IndexExpr[ExprIndexed] {
        expr: Box<Expr>,
        index_expr: Box<Expr>,
    }
}
syntax_type! {
    BinaryExpr[ExprBinary] {
        lhs: Box<Expr>,
        op: BinaryOp,
        rhs: Box<Expr>,
    }
}
syntax_type! {
    UnaryExpr[ExprUnary] {
        op: UnaryOp,
        expr: Box<Expr>,
    }
}

syntax_type! {
    GenericArgNamed {
        name: String,
        value: Box<Expr>,
    }
}

syntax_type! {
    StructArgSingle{
        identifier: String,
        arg_expr: Option<Expr>,
    }
}
syntax_type! {
    Closure[ExprClosure]{
        params: Vec<Param>,
        ret_ty: Option<Box<Expr>>,
        no_panic[optional_no_panic]: bool,
        expr: Box<Expr>,
    }
}

syntax_type! {
    InlineMacroExpr[ExprInlineMacro]{
        path: ExprPath,
        arguments: String,
    }
}

syntax_type! {
    MatchArm{
        patterns: Vec<Pattern>,
        expr[expression]: Expr,
    }
}

from_typed_syntax_node! {StructArgTail.expression, Expr}
from_typed_syntax_node! {ReturnTypeClause.ty, Expr}
from_typed_syntax_node! {GenericArgUnnamed.value, Expr}
from_typed_syntax_node! {ExprErrorPropagate.expr, Expr}
from_typed_syntax_node! {ExprParenthesized.expr, Expr}
from_typed_syntax_node! {StructArgExpr.expr, Expr}
from_typed_syntax_node! {ExprClause.expr, Expr}
from_typed_syntax_node! {TypeClause.ty, Expr}

syntax_option! {OptionStructArgExpr {StructArgExpr: Expr}}
syntax_option! {OptionReturnTypeClause{ReturnTypeClause: Expr}}
syntax_option! {OptionFixedSizeArraySize {FixedSizeArraySize: Box<Expr>}}
syntax_option! {OptionTypeClause {TypeClause: Expr}}
syntax_option! {OptionReturnTypeClause {ReturnTypeClause: Box<Expr>}}
syntax_option! {OptionExprClause{ExprClause: Expr}}

// name only
syntax_terminal_bool! {Dollar}
syntax_terminal_bool! {NoPanic}

vec_from_element_list! {ExprList, Expr}
vec_from_element_list! {ExprListParenthesized.expressions, Expr}
vec_from_element_list! {ExprPathInner, PathSegment}
vec_from_element_list! {GenericArgs.generic_args, GenericArg}
vec_from_element_list! {StructArgListBraced.arguments, StructArg}
vec_from_element_list! {ClosureParams.params, Param}
vec_from_element_list! {ExprBlock.statements, Statement}
vec_from_element_list! {MatchArms, MatchArm}

from_typed_syntax_node! {FixedSizeArraySize.size, Expr}
typed_syntax_node_to_string_without_trivia! {PathSegmentSimple.ident}
typed_syntax_node_to_string_without_trivia! {ExprFieldInitShorthand.name}
typed_syntax_node_to_string_without_trivia! {ExprPath}

syntax_enum! {PathSegment {
    Simple(String),
    WithGenerics[WithGenericArgs](PathSegmentWithGenerics),
    Missing,
}}

syntax_enum! {
    StructArg{
        Single[StructArgSingle](StructArgSingle),
        Tail[StructArgTail](Box<Expr>),
    }
}

syntax_enum!(
    GenericArg {
        Unnamed(Box<Expr>),
        Named(GenericArgNamed),
    }
);

syntax_enum! {
    BlockOrIf{
        Block(Vec<Statement>),
        If(IfExpr),
    }
}

syntax_terminal_enum! {
    BinaryOp[BinaryOperator]{
        Dot,
        Not,
        Mul,
        MulEq,
        Div,
        DivEq,
        Mod,
        ModEq,
        Plus,
        PlusEq,
        Minus,
        MinusEq,
        EqEq,
        Neq,
        Eq,
        And,
        AndAnd,
        Or,
        OrOr,
        Xor,
        LE,
        GE,
        LT,
        GT,
        DotDot,
        DotDotEq,
    }
}

syntax_terminal_enum! {
    UnaryOp[UnaryOperator]{
        Not,
        BitNot,
        Minus,
        At,
        Desnap[TerminalMul],
        Reference[TerminalAnd],
    }
}
