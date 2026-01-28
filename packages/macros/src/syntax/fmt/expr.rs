use super::{CairoCollectionFormat, CairoFormat};
use crate::syntax::expr::{
    BinaryExpr, BinaryOp, Closure, ElseIfBlock, Expr, ExprPath, FixedSizeArray, ForExpr,
    FunctionCall, GenericArg, GenericArgNamed, IfExpr, IndexExpr, InlineMacroExpr, LoopExpr,
    MatchArm, MatchExpr, PathSegment, PathSegmentWithGenerics, StructArg, StructArgSingle,
    StructConstructorCall, UnaryExpr, UnaryOp, WhileExpr,
};

impl CairoFormat for Expr {
    fn cfmt(&self, buf: &mut String) {
        match self {
            Expr::Path(e) => e.cfmt(buf),
            Expr::Literal(e) | Expr::ShortString(e) | Expr::String(e) => e.cfmt(buf),
            Expr::False => buf.push_str("false"),
            Expr::True => buf.push_str("true"),
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
            Expr::Underscore => buf.push('_'),
            Expr::Missing => {}
        };
    }
}

impl CairoFormat for ExprPath {
    fn cfmt(&self, buf: &mut String) {
        if self.dollar {
            buf.push('$');
        }
        self.path.cfmt_join(buf, "::");
    }
}

impl CairoFormat for PathSegment {
    fn cfmt(&self, buf: &mut String) {
        match self {
            PathSegment::Simple(s) => s.cfmt(buf),
            PathSegment::WithGenerics(s) => s.cfmt(buf),
            PathSegment::Missing => {}
        }
    }
}

impl CairoFormat for PathSegmentWithGenerics {
    fn cfmt(&self, buf: &mut String) {
        self.ident.cfmt(buf);
        if self.separator {
            buf.push_str("::");
        }
        self.args.cfmt_csv_angled(buf);
    }
}

impl CairoFormat for GenericArg {
    fn cfmt(&self, buf: &mut String) {
        match self {
            GenericArg::Unnamed(ty) => ty.cfmt(buf),
            GenericArg::Named(expr) => expr.cfmt(buf),
        }
    }
}

impl CairoFormat for GenericArgNamed {
    fn cfmt(&self, buf: &mut String) {
        self.name.cfmt(buf);
        buf.push_str(": ");
        self.value.cfmt(buf);
    }
}

impl CairoFormat for BinaryOp {
    fn cfmt(&self, buf: &mut String) {
        match self {
            BinaryOp::Dot => buf.push('.'),
            BinaryOp::DotDot => buf.push_str(".."),
            BinaryOp::DotDotEq => buf.push_str("..="),
            BinaryOp::Not => buf.push_str(" ! "),
            BinaryOp::Mul => buf.push_str(" * "),
            BinaryOp::MulEq => buf.push_str(" *= "),
            BinaryOp::Div => buf.push_str(" / "),
            BinaryOp::DivEq => buf.push_str(" /= "),
            BinaryOp::Mod => buf.push_str(" % "),
            BinaryOp::ModEq => buf.push_str(" %= "),
            BinaryOp::Plus => buf.push_str(" + "),
            BinaryOp::PlusEq => buf.push_str(" += "),
            BinaryOp::Minus => buf.push_str(" - "),
            BinaryOp::MinusEq => buf.push_str(" -= "),
            BinaryOp::EqEq => buf.push_str(" == "),
            BinaryOp::Neq => buf.push_str(" != "),
            BinaryOp::Eq => buf.push_str(" = "),
            BinaryOp::And => buf.push_str(" & "),
            BinaryOp::AndAnd => buf.push_str(" && "),
            BinaryOp::Or => buf.push_str(" | "),
            BinaryOp::OrOr => buf.push_str(" || "),
            BinaryOp::Xor => buf.push_str(" ^ "),
            BinaryOp::LE => buf.push_str(" <= "),
            BinaryOp::GE => buf.push_str(" >= "),
            BinaryOp::LT => buf.push_str(" < "),
            BinaryOp::GT => buf.push_str(" > "),
        };
    }
}

impl CairoFormat for BinaryExpr {
    fn cfmt(&self, buf: &mut String) {
        self.lhs.cfmt(buf);
        self.op.cfmt(buf);
        self.rhs.cfmt(buf);
    }
}

impl CairoFormat for UnaryOp {
    fn cfmt(&self, buf: &mut String) {
        let val = match self {
            UnaryOp::Not => '!',
            UnaryOp::BitNot => '~',
            UnaryOp::Minus => '-',
            UnaryOp::At => '@',
            UnaryOp::Desnap => '*',
            UnaryOp::Reference => '&',
        };
        buf.push(val);
    }
}

impl CairoFormat for UnaryExpr {
    fn cfmt(&self, buf: &mut String) {
        self.op.cfmt(buf);
        self.expr.cfmt(buf);
    }
}

impl CairoFormat for FunctionCall {
    fn cfmt(&self, buf: &mut String) {
        self.path.cfmt(buf);
        self.args.cfmt_csv_parenthesized(buf);
    }
}

impl CairoFormat for StructConstructorCall {
    fn cfmt(&self, buf: &mut String) {
        self.path.cfmt(buf);
        self.args.cfmt_csv_braced(buf);
    }
}

impl CairoFormat for StructArg {
    fn cfmt(&self, buf: &mut String) {
        match self {
            StructArg::Single(arg) => arg.cfmt(buf),
            StructArg::Tail(expr) => expr.cfmt_prefixed_str(buf, ".."),
        }
    }
}

impl CairoFormat for StructArgSingle {
    fn cfmt(&self, buf: &mut String) {
        self.identifier.cfmt(buf);
        if let Some(expr) = &self.arg_expr {
            expr.cfmt_prefixed(buf, ':');
        }
    }
}

impl CairoFormat for MatchExpr {
    fn cfmt(&self, buf: &mut String) {
        buf.push_str("match ");
        self.expr.cfmt(buf);
        self.arms.cfmt_fields_braced(buf);
    }
}

impl CairoFormat for MatchArm {
    fn cfmt(&self, buf: &mut String) {
        self.patterns.cfmt(buf);
        buf.push_str(" => ");
        self.expr.cfmt(buf);
    }
}

impl CairoFormat for IfExpr {
    fn cfmt(&self, buf: &mut String) {
        buf.push_str("if ");
        self.conditions.cfmt(buf);
        self.if_block.cfmt(buf);
        for elif in &self.else_if_clauses {
            elif.cfmt(buf);
        }
        if let Some(else_block) = &self.else_clause {
            buf.push_str(" else ");
            else_block.cfmt(buf);
        }
    }
}

impl CairoFormat for ElseIfBlock {
    fn cfmt(&self, buf: &mut String) {
        buf.push_str(" else if ");
        self.conditions.cfmt(buf);
        self.body.cfmt(buf);
    }
}

impl CairoFormat for LoopExpr {
    fn cfmt(&self, buf: &mut String) {
        buf.push_str("loop ");
        self.body.cfmt(buf);
    }
}

impl CairoFormat for WhileExpr {
    fn cfmt(&self, buf: &mut String) {
        buf.push_str("while ");
        self.conditions.cfmt(buf);
        self.body.cfmt(buf);
    }
}

impl CairoFormat for ForExpr {
    fn cfmt(&self, buf: &mut String) {
        buf.push_str("for ");
        self.pattern.cfmt_suffixed(buf, ' ');
        self.identifier.cfmt_suffixed(buf, ' ');
        self.expr.cfmt(buf);
        self.body.cfmt(buf);
    }
}

impl CairoFormat for Closure {
    fn cfmt(&self, buf: &mut String) {
        self.params.cfmt_csv_barred(buf);
        if let Some(ret_ty) = &self.ret_ty {
            ret_ty.cfmt_prefixed_str(buf, " -> ");
        }
        if self.no_panic {
            buf.push_str(" nopanic");
        }
        self.expr.cfmt_prefixed(buf, ' ');
    }
}

impl CairoFormat for IndexExpr {
    fn cfmt(&self, buf: &mut String) {
        self.expr.cfmt(buf);
        self.index_expr.cfmt_bracketed(buf);
    }
}

impl CairoFormat for InlineMacroExpr {
    fn cfmt(&self, buf: &mut String) {
        self.path.cfmt_suffixed(buf, '!');
        self.arguments.cfmt(buf);
    }
}

impl CairoFormat for FixedSizeArray {
    fn cfmt(&self, buf: &mut String) {
        buf.push('[');
        self.exprs.cfmt_csv(buf);
        if let Some(size) = &self.size {
            size.cfmt_prefixed_str(buf, "; ");
        }
        buf.push_str("]");
    }
}
