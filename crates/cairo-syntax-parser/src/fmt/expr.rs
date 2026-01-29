use super::fmt::{OptionSizeHint, SizeHint};
use super::{CairoWrite, CairoSliceFormat};
use crate::expr::{
    BinaryExpr, BinaryOp, Closure, ElseIfBlock, Expr, ExprPath, FixedSizeArray, ForExpr,
    FunctionCall, GenericArg, GenericArgNamed, IfExpr, IndexExpr, InlineMacroExpr, LoopExpr,
    MatchArm, MatchExpr, PathSegment, PathSegmentWithGenerics, StructArg, StructArgSingle,
    StructConstructorCall, UnaryExpr, UnaryOp, WhileExpr,
};
use std::fmt::{Result, Write};

impl CairoWrite for Expr {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            Expr::Path(e) => e.cfmt(buf),
            Expr::Literal(e) | Expr::ShortString(e) | Expr::String(e) => e.cfmt(buf),
            Expr::False => buf.write_str("false"),
            Expr::True => buf.write_str("true"),
            Expr::Parenthesized(e) => e.cfmt_parenthesized(buf),
            Expr::Unary(e) => e.cfmt(buf),
            Expr::Binary(e) => e.cfmt(buf),
            Expr::Tuple(e) => e.cfmt_tuple(buf),
            Expr::FunctionCall(e) => e.cfmt(buf),
            Expr::StructConstructorCall(e) => e.cfmt(buf),
            Expr::Block(e) => e.cfmt(buf),
            Expr::Match(e) => e.cfmt(buf),
            Expr::If(e) => e.cfmt(buf),
            Expr::Loop(e) => e.cfmt(buf),
            Expr::While(e) => e.cfmt(buf),
            Expr::For(e) => e.cfmt(buf),
            Expr::Closure(e) => e.cfmt(buf),
            Expr::ErrorPropagate(e) => e.cfmt_suffixed(buf, '?'),
            Expr::FieldInitShorthand(e) => e.cfmt(buf),
            Expr::Indexed(e) => e.cfmt(buf),
            Expr::InlineMacro(e) => e.cfmt(buf),
            Expr::FixedSizeArray(e) => e.cfmt(buf),
            Expr::Underscore => buf.write_char('_'),
            Expr::Missing => Ok(()),
        }
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        match self {
            Expr::Path(e) => e.size_hint(),
            Expr::Literal(e) | Expr::ShortString(e) | Expr::String(e) => e.len(),
            Expr::False => 5,
            Expr::True => 4,
            Expr::Parenthesized(e) => 2 + e.cfmt_size_hint(),
            Expr::Unary(e) => e.cfmt_size_hint(),
            Expr::Binary(e) => e.cfmt_size_hint(),
            Expr::Tuple(e) => 3 + e.size_hint_slice::<2>(),
            Expr::FunctionCall(e) => e.cfmt_size_hint(),
            Expr::StructConstructorCall(e) => e.cfmt_size_hint(),
            Expr::Block(e) => e.cfmt_size_hint(),
            Expr::Match(e) => e.cfmt_size_hint(),
            Expr::If(e) => e.cfmt_size_hint(),
            Expr::Loop(e) => e.cfmt_size_hint(),
            Expr::While(e) => e.cfmt_size_hint(),
            Expr::For(e) => e.cfmt_size_hint(),
            Expr::Closure(e) => e.cfmt_size_hint(),
            Expr::ErrorPropagate(e) => 1 + e.cfmt_size_hint(),
            Expr::FieldInitShorthand(e) => e.cfmt_size_hint(),
            Expr::Indexed(e) => e.cfmt_size_hint(),
            Expr::InlineMacro(e) => e.cfmt_size_hint(),
            Expr::FixedSizeArray(e) => e.cfmt_size_hint(),
            Expr::Underscore => 1,
            Expr::Missing => 0,
        }
    }
}

impl CairoWrite for ExprPath {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        if self.dollar {
            buf.write_char('$')?;
        }
        self.path.cfmt_join(buf, "::")
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        (if self.dollar { 1 } else { 0 }) + self.path.size_hint_slice::<2>()
    }
}

impl SizeHint for Vec<ExprPath> {
    fn size_hint(&self) -> usize {
        self.size_hint_slice::<2>()
    }
}

impl CairoWrite for PathSegment {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            PathSegment::Simple(s) => s.cfmt(buf),
            PathSegment::WithGenerics(s) => s.cfmt(buf),
            PathSegment::Missing => Ok(()),
        }
    }
}

impl CairoWrite for PathSegmentWithGenerics {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.ident.cfmt(buf)?;
        if self.separator {
            buf.write_str("::")?;
        }
        self.args.cfmt_csv_angled(buf)
    }
}

impl CairoWrite for GenericArg {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            GenericArg::Unnamed(ty) => ty.cfmt(buf),
            GenericArg::Named(expr) => expr.cfmt(buf),
        }
    }
}

impl CairoWrite for GenericArgNamed {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.name.cfmt(buf)?;
        buf.write_str(": ")?;
        self.value.cfmt(buf)
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.name.size_hint() + 2 + self.value.size_hint()
    }
}

impl CairoWrite for BinaryOp {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            BinaryOp::Dot => buf.write_char('.'),
            BinaryOp::DotDot => buf.write_str(".."),
            BinaryOp::DotDotEq => buf.write_str("..="),
            BinaryOp::Not => buf.write_str(" ! "),
            BinaryOp::Mul => buf.write_str(" * "),
            BinaryOp::Div => buf.write_str(" / "),
            BinaryOp::Mod => buf.write_str(" % "),
            BinaryOp::Plus => buf.write_str(" + "),
            BinaryOp::Minus => buf.write_str(" - "),
            BinaryOp::And => buf.write_str(" & "),
            BinaryOp::Or => buf.write_str(" | "),
            BinaryOp::Xor => buf.write_str(" ^ "),
            BinaryOp::LT => buf.write_str(" < "),
            BinaryOp::GT => buf.write_str(" > "),
            BinaryOp::MulEq => buf.write_str(" *= "),
            BinaryOp::DivEq => buf.write_str(" /= "),
            BinaryOp::ModEq => buf.write_str(" %= "),
            BinaryOp::PlusEq => buf.write_str(" += "),
            BinaryOp::MinusEq => buf.write_str(" -= "),
            BinaryOp::EqEq => buf.write_str(" == "),
            BinaryOp::Neq => buf.write_str(" != "),
            BinaryOp::Eq => buf.write_str(" = "),
            BinaryOp::AndAnd => buf.write_str(" && "),
            BinaryOp::OrOr => buf.write_str(" || "),
            BinaryOp::LE => buf.write_str(" <= "),
            BinaryOp::GE => buf.write_str(" >= "),
        }
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        match self {
            BinaryOp::Dot => 1,
            BinaryOp::DotDot => 2,
            BinaryOp::DotDotEq
            | BinaryOp::Not
            | BinaryOp::Mul
            | BinaryOp::Div
            | BinaryOp::Mod
            | BinaryOp::Plus
            | BinaryOp::Minus
            | BinaryOp::And
            | BinaryOp::Or
            | BinaryOp::Xor
            | BinaryOp::LT
            | BinaryOp::GT => 3,
            BinaryOp::MulEq
            | BinaryOp::DivEq
            | BinaryOp::ModEq
            | BinaryOp::PlusEq
            | BinaryOp::MinusEq
            | BinaryOp::EqEq
            | BinaryOp::Neq
            | BinaryOp::Eq
            | BinaryOp::AndAnd
            | BinaryOp::OrOr
            | BinaryOp::LE
            | BinaryOp::GE => 4,
        }
    }
}

impl CairoWrite for BinaryExpr {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.lhs.cfmt(buf)?;
        self.op.cfmt(buf)?;
        self.rhs.cfmt(buf)
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.lhs.cfmt_size_hint() + self.op.cfmt_size_hint() + self.rhs.cfmt_size_hint()
    }
}

impl CairoWrite for UnaryOp {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        let val = match self {
            UnaryOp::Not => '!',
            UnaryOp::BitNot => '~',
            UnaryOp::Minus => '-',
            UnaryOp::At => '@',
            UnaryOp::Desnap => '*',
            UnaryOp::Reference => '&',
        };
        buf.write_char(val)
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        1
    }
}

impl CairoWrite for UnaryExpr {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.op.cfmt(buf)?;
        self.expr.cfmt(buf)
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.op.cfmt_size_hint() + self.expr.cfmt_size_hint()
    }
}

impl CairoWrite for FunctionCall {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.path.cfmt(buf)?;
        self.args.cfmt_csv_parenthesized(buf)
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.path.size_hint() + 2 + self.args.size_hint_slice::<2>()
    }
}

impl CairoWrite for StructConstructorCall {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.path.cfmt(buf)?;
        self.args.cfmt_csv_braced(buf)
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.path.size_hint() + 2 + self.args.size_hint_slice::<2>()
    }
}

impl CairoWrite for StructArg {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            StructArg::Single(arg) => arg.cfmt(buf),
            StructArg::Tail(expr) => expr.cfmt_prefixed_str(buf, ".."),
        }
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        match self {
            StructArg::Single(arg) => arg.cfmt_size_hint(),
            StructArg::Tail(expr) => 2 + expr.cfmt_size_hint(),
        }
    }
}

impl CairoWrite for StructArgSingle {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.identifier.cfmt(buf)?;
        if let Some(expr) = &self.arg_expr {
            expr.cfmt_prefixed_str(buf, ": ")?;
        }
        Ok(())
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.identifier.size_hint() + self.arg_expr.size_hint_option::<2, 0>()
    }
}

impl CairoWrite for MatchExpr {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_str("match ")?;
        self.expr.cfmt(buf)?;
        self.arms.cfmt_fields_braced(buf)
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        10 + self.expr.cfmt_size_hint() + self.arms.size_hint_slice::<2>()
    }
}
impl CairoWrite for MatchArm {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.patterns.cfmt(buf)?;
        buf.write_str(" => ")?;
        self.expr.cfmt(buf)
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.patterns.size_hint_slice::<2>() + 4 + self.expr.cfmt_size_hint()
    }
}

impl CairoWrite for IfExpr {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_str("if ")?;
        self.conditions.cfmt(buf)?;
        self.if_block.cfmt(buf)?;
        self.else_if_clauses.cfmt_concatenated(buf)?;
        if let Some(else_block) = &self.else_clause {
            buf.write_str(" else ")?;
            else_block.cfmt(buf)?;
        }
        Ok(())
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        4 + self.conditions.cfmt_size_hint()
            + self.if_block.cfmt_size_hint()
            + self.else_if_clauses.size_hint_slice::<0>()
            + self.else_clause.size_hint_option::<6, 0>()
    }
}

impl CairoWrite for ElseIfBlock {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_str(" else ")?;
        buf.write_str("if ")?;
        self.conditions.cfmt(buf)?;
        self.body.cfmt(buf)
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        9 + self.conditions.cfmt_size_hint() + self.body.cfmt_size_hint()
    }
}

impl CairoWrite for LoopExpr {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_str("loop ")?;
        self.body.cfmt(buf)
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        5 + self.body.cfmt_size_hint()
    }
}

impl CairoWrite for WhileExpr {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_str("while ")?;
        self.conditions.cfmt(buf)?;
        self.body.cfmt(buf)
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        6 + self.conditions.cfmt_size_hint() + self.body.cfmt_size_hint()
    }
}

impl CairoWrite for ForExpr {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_str("for ")?;
        self.pattern.cfmt_suffixed(buf, ' ')?;
        self.identifier.cfmt_suffixed(buf, ' ')?;
        self.expr.cfmt(buf)?;
        self.body.cfmt(buf)
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        6 + self.pattern.size_hint()
            + self.identifier.size_hint()
            + self.expr.size_hint()
            + self.body.size_hint()
    }
}

impl CairoWrite for Closure {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.params.cfmt_csv_barred(buf)?;
        if let Some(ret_ty) = &self.ret_ty {
            ret_ty.cfmt_prefixed_str(buf, " -> ")?;
        }
        if self.no_panic {
            buf.write_str(" nopanic")?;
        }
        self.expr.cfmt_prefixed(buf, ' ')
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        2 + self.params.size_hint_slice::<2>()
            + self.ret_ty.size_hint_option::<4, 0>()
            + if self.no_panic { 8 } else { 0 }
            + 1
            + self.expr.cfmt_size_hint()
    }
}

impl CairoWrite for IndexExpr {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.expr.cfmt(buf)?;
        self.index_expr.cfmt_bracketed(buf)
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.expr.cfmt_size_hint() + 2 + self.index_expr.cfmt_size_hint()
    }
}

impl CairoWrite for InlineMacroExpr {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.path.cfmt_suffixed(buf, '!')?;
        self.arguments.cfmt(buf)
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.path.size_hint() + 1 + self.arguments.size_hint()
    }
}

impl CairoWrite for FixedSizeArray {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_char('[')?;
        self.exprs.cfmt_csv(buf)?;
        if let Some(size) = &self.size {
            size.cfmt_prefixed_str(buf, "; ")?;
        }
        buf.write_str("]")
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        2 + self.exprs.size_hint_slice::<2>() + self.size.size_hint_option::<2, 0>()
    }
}
