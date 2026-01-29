use super::{CairoCollectionFormat, CairoFormat, CodeBuffer};
use crate::expr::{
    BinaryExpr, BinaryOp, Closure, ElseIfBlock, Expr, ExprPath, FixedSizeArray, ForExpr,
    FunctionCall, GenericArg, GenericArgNamed, IfExpr, IndexExpr, InlineMacroExpr, LoopExpr,
    MatchArm, MatchExpr, PathSegment, PathSegmentWithGenerics, StructArg, StructArgSingle,
    StructConstructorCall, UnaryExpr, UnaryOp, WhileExpr,
};

impl<T: CodeBuffer> CairoFormat<T> for Expr {
    fn cfmt(&self, buf: &mut T) {
        match self {
            Expr::Path(e) => e.cfmt(buf),
            Expr::Literal(e) | Expr::ShortString(e) | Expr::String(e) => e.cfmt(buf),
            Expr::False => buf.push_token_str("false"),
            Expr::True => buf.push_token_str("true"),
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
            Expr::Underscore => buf.push_token_char('_'),
            Expr::Missing => {}
        };
    }
}

impl<T: CodeBuffer> CairoFormat<T> for ExprPath {
    fn cfmt(&self, buf: &mut T) {
        if self.dollar {
            buf.push_token_char('$');
        }
        self.path.cfmt_join(buf, "::");
    }
}

impl<T: CodeBuffer> CairoFormat<T> for PathSegment {
    fn cfmt(&self, buf: &mut T) {
        match self {
            PathSegment::Simple(s) => s.cfmt(buf),
            PathSegment::WithGenerics(s) => s.cfmt(buf),
            PathSegment::Missing => {}
        }
    }
}

impl<T: CodeBuffer> CairoFormat<T> for PathSegmentWithGenerics {
    fn cfmt(&self, buf: &mut T) {
        self.ident.cfmt(buf);
        if self.separator {
            buf.push_token_str("::");
        }
        self.args.cfmt_csv_angled(buf);
    }
}

impl<T: CodeBuffer> CairoFormat<T> for GenericArg {
    fn cfmt(&self, buf: &mut T) {
        match self {
            GenericArg::Unnamed(ty) => ty.cfmt(buf),
            GenericArg::Named(expr) => expr.cfmt(buf),
        }
    }
}

impl<T: CodeBuffer> CairoFormat<T> for GenericArgNamed {
    fn cfmt(&self, buf: &mut T) {
        self.name.cfmt(buf);
        buf.push_token_str(": ");
        self.value.cfmt(buf);
    }
}

impl<T: CodeBuffer> CairoFormat<T> for BinaryOp {
    fn cfmt(&self, buf: &mut T) {
        match self {
            BinaryOp::Dot => buf.push_token_char('.'),
            BinaryOp::DotDot => buf.push_token_str(".."),
            BinaryOp::DotDotEq => buf.push_token_str("..="),
            BinaryOp::Not => buf.push_token_str(" ! "),
            BinaryOp::Mul => buf.push_token_str(" * "),
            BinaryOp::MulEq => buf.push_token_str(" *= "),
            BinaryOp::Div => buf.push_token_str(" / "),
            BinaryOp::DivEq => buf.push_token_str(" /= "),
            BinaryOp::Mod => buf.push_token_str(" % "),
            BinaryOp::ModEq => buf.push_token_str(" %= "),
            BinaryOp::Plus => buf.push_token_str(" + "),
            BinaryOp::PlusEq => buf.push_token_str(" += "),
            BinaryOp::Minus => buf.push_token_str(" - "),
            BinaryOp::MinusEq => buf.push_token_str(" -= "),
            BinaryOp::EqEq => buf.push_token_str(" == "),
            BinaryOp::Neq => buf.push_token_str(" != "),
            BinaryOp::Eq => buf.push_token_str(" = "),
            BinaryOp::And => buf.push_token_str(" & "),
            BinaryOp::AndAnd => buf.push_token_str(" && "),
            BinaryOp::Or => buf.push_token_str(" | "),
            BinaryOp::OrOr => buf.push_token_str(" || "),
            BinaryOp::Xor => buf.push_token_str(" ^ "),
            BinaryOp::LE => buf.push_token_str(" <= "),
            BinaryOp::GE => buf.push_token_str(" >= "),
            BinaryOp::LT => buf.push_token_str(" < "),
            BinaryOp::GT => buf.push_token_str(" > "),
        };
    }
}

impl<T: CodeBuffer> CairoFormat<T> for BinaryExpr {
    fn cfmt(&self, buf: &mut T) {
        self.lhs.cfmt(buf);
        self.op.cfmt(buf);
        self.rhs.cfmt(buf);
    }
}

impl<T: CodeBuffer> CairoFormat<T> for UnaryOp {
    fn cfmt(&self, buf: &mut T) {
        let val = match self {
            UnaryOp::Not => '!',
            UnaryOp::BitNot => '~',
            UnaryOp::Minus => '-',
            UnaryOp::At => '@',
            UnaryOp::Desnap => '*',
            UnaryOp::Reference => '&',
        };
        buf.push_token_char(val);
    }
}

impl<T: CodeBuffer> CairoFormat<T> for UnaryExpr {
    fn cfmt(&self, buf: &mut T) {
        self.op.cfmt(buf);
        self.expr.cfmt(buf);
    }
}

impl<T: CodeBuffer> CairoFormat<T> for FunctionCall {
    fn cfmt(&self, buf: &mut T) {
        self.path.cfmt(buf);
        self.args.cfmt_csv_parenthesized(buf);
    }
}

impl<T: CodeBuffer> CairoFormat<T> for StructConstructorCall {
    fn cfmt(&self, buf: &mut T) {
        self.path.cfmt(buf);
        self.args.cfmt_csv_braced(buf);
    }
}

impl<T: CodeBuffer> CairoFormat<T> for StructArg {
    fn cfmt(&self, buf: &mut T) {
        match self {
            StructArg::Single(arg) => arg.cfmt(buf),
            StructArg::Tail(expr) => expr.cfmt_prefixed_str(buf, ".."),
        }
    }
}

impl<T: CodeBuffer> CairoFormat<T> for StructArgSingle {
    fn cfmt(&self, buf: &mut T) {
        self.identifier.cfmt(buf);
        if let Some(expr) = &self.arg_expr {
            expr.cfmt_prefixed_str(buf, ": ");
        }
    }
}

impl<T: CodeBuffer> CairoFormat<T> for MatchExpr {
    fn cfmt(&self, buf: &mut T) {
        buf.push_token_str("match ");
        self.expr.cfmt(buf);
        self.arms.cfmt_fields_braced(buf);
    }
}

impl<T: CodeBuffer> CairoFormat<T> for MatchArm {
    fn cfmt(&self, buf: &mut T) {
        self.patterns.cfmt(buf);
        buf.push_token_str(" => ");
        self.expr.cfmt(buf);
    }
}

impl<T: CodeBuffer> CairoFormat<T> for IfExpr {
    fn cfmt(&self, buf: &mut T) {
        buf.push_token_str("if ");
        self.conditions.cfmt(buf);
        self.if_block.cfmt(buf);
        for elif in &self.else_if_clauses {
            elif.cfmt(buf);
        }
        if let Some(else_block) = &self.else_clause {
            buf.push_token_str(" else ");
            else_block.cfmt(buf);
        }
    }
}

impl<T: CodeBuffer> CairoFormat<T> for ElseIfBlock {
    fn cfmt(&self, buf: &mut T) {
        buf.push_token_str(" else ");
        buf.push_token_str("if ");
        self.conditions.cfmt(buf);
        self.body.cfmt(buf);
    }
}

impl<T: CodeBuffer> CairoFormat<T> for LoopExpr {
    fn cfmt(&self, buf: &mut T) {
        buf.push_token_str("loop ");
        self.body.cfmt(buf);
    }
}

impl<T: CodeBuffer> CairoFormat<T> for WhileExpr {
    fn cfmt(&self, buf: &mut T) {
        buf.push_token_str("while ");
        self.conditions.cfmt(buf);
        self.body.cfmt(buf);
    }
}

impl<T: CodeBuffer> CairoFormat<T> for ForExpr {
    fn cfmt(&self, buf: &mut T) {
        buf.push_token_str("for ");
        self.pattern.cfmt_suffixed(buf, ' ');
        self.identifier.cfmt_suffixed(buf, ' ');
        self.expr.cfmt(buf);
        self.body.cfmt(buf);
    }
}

impl<T: CodeBuffer> CairoFormat<T> for Closure {
    fn cfmt(&self, buf: &mut T) {
        self.params.cfmt_csv_barred(buf);
        if let Some(ret_ty) = &self.ret_ty {
            ret_ty.cfmt_prefixed_str(buf, " -> ");
        }
        if self.no_panic {
            buf.push_token_str(" nopanic");
        }
        self.expr.cfmt_prefixed(buf, ' ');
    }
}

impl<T: CodeBuffer> CairoFormat<T> for IndexExpr {
    fn cfmt(&self, buf: &mut T) {
        self.expr.cfmt(buf);
        self.index_expr.cfmt_bracketed(buf);
    }
}

impl<T: CodeBuffer> CairoFormat<T> for InlineMacroExpr {
    fn cfmt(&self, buf: &mut T) {
        self.path.cfmt_suffixed(buf, '!');
        self.arguments.cfmt(buf);
    }
}

impl<T: CodeBuffer> CairoFormat<T> for FixedSizeArray {
    fn cfmt(&self, buf: &mut T) {
        buf.push_token_char('[');
        self.exprs.cfmt_csv(buf);
        if let Some(size) = &self.size {
            size.cfmt_prefixed_str(buf, "; ");
        }
        buf.push_token_str("]");
    }
}
