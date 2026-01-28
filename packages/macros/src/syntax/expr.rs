use cairo_lang_syntax::node::ast;
use salsa::Database;

use super::{Condition, Param, Pattern, Statement};
use crate::{
    Arg, AsCairo, AstInto, CollectionsAsCairo, FromAst, from_typed_syntax_node, syntax_enum,
    syntax_option, syntax_terminal_bool, syntax_terminal_enum, syntax_type,
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
    conditions: Vec<Condition>,
    if_block: Vec<Statement>,
    else_if_clauses: Vec<ElseIfBlock>,
    else_clause: Option<Vec<Statement>>,
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

impl AsCairo for Expr {
    fn as_cairo(&self) -> String {
        match self {
            Expr::Path(e) => e.as_cairo(),
            Expr::Literal(e) => e.clone(),
            Expr::ShortString(e) => format!("'{e}'"),
            Expr::String(e) => format!("\"{e}\""),
            Expr::False => "false".to_string(),
            Expr::True => "true".to_string(),
            Expr::Parenthesized(e) => format!("({})", e.as_cairo()),
            Expr::Unary(e) => e.as_cairo(),
            Expr::Binary(e) => e.as_cairo(),
            Expr::Tuple(e) => e.as_cairo_tuple(),
            Expr::FunctionCall(e) => e.as_cairo(),
            Expr::StructConstructorCall(e) => e.as_cairo(),
            Expr::Block(e) => e.as_cairo_block_braced(),
            Expr::Match(e) => e.as_cairo(),
            Expr::If(e) => e.as_cairo(),
            Expr::Loop(e) => e.as_cairo(),
            Expr::While(e) => e.as_cairo(),
            Expr::For(e) => e.as_cairo(),
            Expr::Closure(e) => e.as_cairo(),
            Expr::ErrorPropagate(e) => format!("{}?", e.as_cairo()),
            Expr::FieldInitShorthand(e) => e.as_cairo(),
            Expr::Indexed(e) => e.as_cairo(),
            Expr::InlineMacro(e) => e.as_cairo(),
            Expr::FixedSizeArray(e) => e.as_cairo(),
            Expr::Underscore => "_".to_string(),
            Expr::Missing => "".to_string(),
        }
    }
}

impl AsCairo for ExprPath {
    fn as_cairo(&self) -> String {
        let dollar = if self.dollar { "$" } else { "" };
        let segments = self.path.as_cairo_delimited("::");
        format!("{dollar}{segments}")
    }
}

impl AsCairo for PathSegment {
    fn as_cairo(&self) -> String {
        match self {
            PathSegment::Simple(e) => e.as_cairo(),
            PathSegment::WithGenerics(e) => e.as_cairo(),
            PathSegment::Missing => "".to_string(),
        }
    }
}

impl AsCairo for PathSegmentWithGenerics {
    fn as_cairo(&self) -> String {
        let separator = if self.separator { "::" } else { "" };
        let args = self.args.as_cairo_csv();
        format!("{}{separator}<{args}>", self.ident)
    }
}

impl AsCairo for GenericArg {
    fn as_cairo(&self) -> String {
        match self {
            GenericArg::Unnamed(e) => e.as_cairo(),
            GenericArg::Named(e) => e.as_cairo(),
        }
    }
}

impl AsCairo for GenericArgNamed {
    fn as_cairo(&self) -> String {
        format!("{}:{}", self.name, self.value.as_cairo())
    }
}

impl AsCairo for BinaryOp {
    fn as_cairo(&self) -> String {
        match self {
            BinaryOp::Dot => ".".to_string(),
            BinaryOp::Not => "!".to_string(),
            BinaryOp::Mul => "*".to_string(),
            BinaryOp::MulEq => "*=".to_string(),
            BinaryOp::Div => "/".to_string(),
            BinaryOp::DivEq => "/=".to_string(),
            BinaryOp::Mod => "%".to_string(),
            BinaryOp::ModEq => "%=".to_string(),
            BinaryOp::Plus => "+".to_string(),
            BinaryOp::PlusEq => "+=".to_string(),
            BinaryOp::Minus => "-".to_string(),
            BinaryOp::MinusEq => "-=".to_string(),
            BinaryOp::EqEq => "==".to_string(),
            BinaryOp::Neq => "!=".to_string(),
            BinaryOp::Eq => "=".to_string(),
            BinaryOp::And => "&".to_string(),
            BinaryOp::AndAnd => "&&".to_string(),
            BinaryOp::Or => "|".to_string(),
            BinaryOp::OrOr => "||".to_string(),
            BinaryOp::Xor => "^".to_string(),
            BinaryOp::LE => "<=".to_string(),
            BinaryOp::GE => ">=".to_string(),
            BinaryOp::LT => "<".to_string(),
            BinaryOp::GT => ">".to_string(),
            BinaryOp::DotDot => "..".to_string(),
            BinaryOp::DotDotEq => "..=".to_string(),
        }
    }
}

impl AsCairo for BinaryExpr {
    fn as_cairo(&self) -> String {
        format!(
            "{}{}{}",
            self.lhs.as_cairo(),
            self.op.as_cairo(),
            self.rhs.as_cairo()
        )
    }
}

impl AsCairo for UnaryOp {
    fn as_cairo(&self) -> String {
        match self {
            UnaryOp::Not => "!".to_string(),
            UnaryOp::BitNot => "~".to_string(),
            UnaryOp::Minus => "-".to_string(),
            UnaryOp::At => "@".to_string(),
            UnaryOp::Desnap => "*".to_string(),
            UnaryOp::Reference => "&".to_string(),
        }
    }
}

impl AsCairo for UnaryExpr {
    fn as_cairo(&self) -> String {
        format!("{}{}", self.op.as_cairo(), self.expr.as_cairo())
    }
}

impl AsCairo for FunctionCall {
    fn as_cairo(&self) -> String {
        format!("{}({})", self.path.as_cairo(), self.args.as_cairo_csv())
    }
}

impl AsCairo for StructConstructorCall {
    fn as_cairo(&self) -> String {
        format!("{}{{{}}}", self.path.as_cairo(), self.args.as_cairo_csv())
    }
}

impl AsCairo for StructArg {
    fn as_cairo(&self) -> String {
        match self {
            StructArg::Single(e) => e.as_cairo(),
            StructArg::Tail(e) => format!("..{}", e.as_cairo()),
        }
    }
}

impl AsCairo for StructArgSingle {
    fn as_cairo(&self) -> String {
        match &self.arg_expr {
            Some(expr) => format!("{}:{}", self.identifier, expr.as_cairo()),
            None => self.identifier.clone(),
        }
    }
}

impl AsCairo for MatchExpr {
    fn as_cairo(&self) -> String {
        let arms = self.arms.as_cairo_block();
        format!("match {}{{{arms}}}", self.expr.as_cairo())
    }
}

impl AsCairo for MatchArm {
    fn as_cairo(&self) -> String {
        let patterns = self.patterns.as_cairo_delimited("|");
        format!("{} => {},", patterns, self.expr.as_cairo())
    }
}

impl AsCairo for IfExpr {
    fn as_cairo(&self) -> String {
        let conditions = self.conditions.as_cairo_delimited(" && ");
        let block = self.if_block.as_cairo_block();
        let else_ifs = self.else_if_clauses.as_cairo_concatenated();
        let else_clause = match &self.else_clause {
            Some(else_block) => format!("else {{{}}}", else_block.as_cairo_block()),
            None => "".to_string(),
        };
        format!("if {conditions} {{{block}}}{else_ifs}{else_clause}",)
    }
}

impl AsCairo for ElseIfBlock {
    fn as_cairo(&self) -> String {
        format!(
            "else if {} {{{}}}",
            self.conditions.as_cairo(),
            self.body.as_cairo_block(),
        )
    }
}

impl AsCairo for LoopExpr {
    fn as_cairo(&self) -> String {
        let body = self.body.as_cairo_block();
        format!("loop {{{body}}}")
    }
}

impl AsCairo for WhileExpr {
    fn as_cairo(&self) -> String {
        let conditions = self.conditions.as_cairo();
        let body = self.body.as_cairo_block();
        format!("while {conditions} {{{body}}}")
    }
}

impl AsCairo for ForExpr {
    fn as_cairo(&self) -> String {
        let body = self.body.as_cairo_block();
        format!(
            "for {} {} in {} {{{}}}",
            self.pattern.as_cairo(),
            self.identifier,
            self.expr.as_cairo(),
            body
        )
    }
}

impl AsCairo for FixedSizeArray {
    fn as_cairo(&self) -> String {
        match &self.size {
            Some(size_expr) => format!("[{}; {}]", self.exprs.as_cairo_csv(), size_expr.as_cairo()),
            None => format!("[{}]", self.exprs.as_cairo_csv()),
        }
    }
}

impl AsCairo for IndexExpr {
    fn as_cairo(&self) -> String {
        format!("{}[{}]", self.expr.as_cairo(), self.index_expr.as_cairo())
    }
}

impl AsCairo for Closure {
    fn as_cairo(&self) -> String {
        let no_panic = if self.no_panic { "no_panic " } else { "" };
        let ret_ty = match &self.ret_ty {
            Some(ty) => format!("-> {} ", ty.as_cairo()),
            None => "".to_string(),
        };
        format!(
            "|{}| {}{}{}",
            self.params.as_cairo_csv(),
            no_panic,
            ret_ty,
            self.expr.as_cairo()
        )
    }
}

impl AsCairo for InlineMacroExpr {
    fn as_cairo(&self) -> String {
        format!("{}!({})", self.path.as_cairo(), self.arguments)
    }
}
