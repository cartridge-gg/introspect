use super::{CairoWrite, CairoWriteSlice};
use crate::expr::{
    BinaryExpr, BinaryOp, Closure, ElseIfBlock, Expr, ExprPath, FixedSizeArray, ForExpr,
    FunctionCall, GenericArg, GenericArgNamed, IfExpr, IndexExpr, InlineMacroExpr, LoopExpr,
    MatchArm, MatchExpr, PathSegment, PathSegmentWithGenerics, StructArg, StructArgSingle,
    StructConstructorCall, UnaryExpr, UnaryOp, WhileExpr,
};
use std::fmt::{Result, Write};

impl CairoWrite for Expr {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            Expr::Path(e) => e.cwrite(buf),
            Expr::Literal(e) | Expr::ShortString(e) | Expr::String(e) => e.cwrite(buf),
            Expr::False => buf.write_str("false"),
            Expr::True => buf.write_str("true"),
            Expr::Parenthesized(e) => e.cwrite_parenthesized(buf),
            Expr::Unary(e) => e.cwrite(buf),
            Expr::Binary(e) => e.cwrite(buf),
            Expr::Tuple(e) => e.cwrite_tuple(buf),
            Expr::FunctionCall(e) => e.cwrite(buf),
            Expr::StructConstructorCall(e) => e.cwrite(buf),
            Expr::Block(e) => e.cwrite(buf),
            Expr::Match(e) => e.cwrite(buf),
            Expr::If(e) => e.cwrite(buf),
            Expr::Loop(e) => e.cwrite(buf),
            Expr::While(e) => e.cwrite(buf),
            Expr::For(e) => e.cwrite(buf),
            Expr::Closure(e) => e.cwrite(buf),
            Expr::ErrorPropagate(e) => e.cwrite_suffixed(buf, '?'),
            Expr::FieldInitShorthand(e) => e.cwrite(buf),
            Expr::Indexed(e) => e.cwrite(buf),
            Expr::InlineMacro(e) => e.cwrite(buf),
            Expr::FixedSizeArray(e) => e.cwrite(buf),
            Expr::Underscore => buf.write_char('_'),
            Expr::Missing => Ok(()),
        }
    }
}

impl CairoWrite for ExprPath {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        if self.dollar {
            buf.write_char('$')?;
        }
        self.path.cwrite_join(buf, "::")
    }
}

impl CairoWrite for PathSegment {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            PathSegment::Simple(s) => s.cwrite(buf),
            PathSegment::WithGenerics(s) => s.cwrite(buf),
            PathSegment::Missing => Ok(()),
        }
    }
}

impl CairoWrite for PathSegmentWithGenerics {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.ident.cwrite(buf)?;
        if self.separator {
            buf.write_str("::")?;
        }
        self.args.cwrite_csv_angled(buf)
    }
}

impl CairoWrite for GenericArg {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            GenericArg::Unnamed(ty) => ty.cwrite(buf),
            GenericArg::Named(expr) => expr.cwrite(buf),
        }
    }
}

impl CairoWrite for GenericArgNamed {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.name.cwrite(buf)?;
        buf.write_str(": ")?;
        self.value.cwrite(buf)
    }
}

impl CairoWrite for BinaryOp {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
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

impl CairoWrite for BinaryExpr {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.lhs.cwrite(buf)?;
        self.op.cwrite(buf)?;
        self.rhs.cwrite(buf)
    }
}

impl CairoWrite for UnaryOp {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
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

impl CairoWrite for UnaryExpr {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.op.cwrite(buf)?;
        self.expr.cwrite(buf)
    }
}

impl CairoWrite for FunctionCall {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.path.cwrite(buf)?;
        self.args.cwrite_csv_parenthesized(buf)
    }
}

impl CairoWrite for StructConstructorCall {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.path.cwrite(buf)?;
        self.args.cwrite_csv_braced(buf)
    }
}

impl CairoWrite for StructArg {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            StructArg::Single(arg) => arg.cwrite(buf),
            StructArg::Tail(expr) => expr.cwrite_prefixed_str(buf, ".."),
        }
    }
}

impl CairoWrite for StructArgSingle {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.identifier.cwrite(buf)?;
        if let Some(expr) = &self.arg_expr {
            expr.cwrite_prefixed_str(buf, ": ")?;
        }
        Ok(())
    }
}

impl CairoWrite for MatchExpr {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_str("match ")?;
        self.expr.cwrite(buf)?;
        self.arms.cwrite_fields_braced(buf)
    }
}

impl CairoWrite for MatchArm {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.patterns.cwrite(buf)?;
        buf.write_str(" => ")?;
        self.expr.cwrite(buf)
    }
}

impl CairoWrite for IfExpr {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_str("if ")?;
        self.conditions.cwrite(buf)?;
        self.if_block.cwrite(buf)?;
        self.else_if_clauses.cwrite_concatenated(buf)?;
        if let Some(else_block) = &self.else_clause {
            buf.write_str(" else ")?;
            else_block.cwrite(buf)?;
        }
        Ok(())
    }
}

impl CairoWrite for ElseIfBlock {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_str(" else ")?;
        buf.write_str("if ")?;
        self.conditions.cwrite(buf)?;
        self.body.cwrite(buf)
    }
}

impl CairoWrite for LoopExpr {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_str("loop ")?;
        self.body.cwrite(buf)
    }
}

impl CairoWrite for WhileExpr {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_str("while ")?;
        self.conditions.cwrite(buf)?;
        self.body.cwrite(buf)
    }
}

impl CairoWrite for ForExpr {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_str("for ")?;
        self.pattern.cwrite_suffixed(buf, ' ')?;
        self.identifier.cwrite_suffixed(buf, ' ')?;
        self.expr.cwrite(buf)?;
        self.body.cwrite(buf)
    }
}

impl CairoWrite for Closure {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.params.cwrite_csv_barred(buf)?;
        if let Some(ret_ty) = &self.ret_ty {
            ret_ty.cwrite_prefixed_str(buf, " -> ")?;
        }
        if self.no_panic {
            buf.write_str(" nopanic")?;
        }
        self.expr.cwrite_prefixed(buf, ' ')
    }
}

impl CairoWrite for IndexExpr {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.expr.cwrite(buf)?;
        self.index_expr.cwrite_bracketed(buf)
    }
}

impl CairoWrite for InlineMacroExpr {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.path.cwrite_suffixed(buf, '!')?;
        self.arguments.cwrite(buf)
    }
}

impl CairoWrite for FixedSizeArray {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_char('[')?;
        self.exprs.cwrite_csv(buf)?;
        if let Some(size) = &self.size {
            size.cwrite_prefixed_str(buf, "; ")?;
        }
        buf.write_str("]")
    }
}
