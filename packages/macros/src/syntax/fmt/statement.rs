use super::{CairoCollectionFormat, CairoFormat};
use crate::syntax::statement::{
    BreakStatement, Condition, ContinueStatement, ExprStatement, LetCondition, LetStatement,
    ParamWithPatten, Pattern, PatternEnum, PatternStruct, PatternStructParam, ReturnStatement,
    Statement,
};

impl CairoFormat for Statement {
    fn cfmt(&self, buf: &mut String) {
        match self {
            Statement::Let(stmt) => stmt.cfmt(buf),
            Statement::Expr(stmt) => stmt.cfmt(buf),
            Statement::Continue(stmt) => stmt.cfmt(buf),
            Statement::Return(stmt) => stmt.cfmt(buf),
            Statement::Break(stmt) => stmt.cfmt(buf),
            Statement::Item(item) => item.cfmt(buf),
            Statement::Missing => {}
        }
    }
}
impl CairoFormat for Vec<Statement> {
    fn cfmt(&self, buf: &mut String) {
        self.cfmt_block_braced(buf);
    }
}

impl CairoFormat for Pattern {
    fn cfmt(&self, buf: &mut String) {
        match self {
            Pattern::Underscore => buf.push('_'),
            Pattern::Literal(s) | Pattern::ShortString(s) | Pattern::String(s) => s.cfmt(buf),
            Pattern::False => buf.push_str("false"),
            Pattern::True => buf.push_str("true"),
            Pattern::Identifier(ident) => ident.cfmt(buf),
            Pattern::Struct(pat_struct) => pat_struct.cfmt(buf),
            Pattern::Tuple(patterns) => patterns.cfmt_tuple(buf),
            Pattern::Enum(pat_enum) => pat_enum.cfmt(buf),
            Pattern::FixedSizeArray(patterns) => patterns.cfmt_csv_bracketed(buf),
            Pattern::Path(expr_path) => expr_path.cfmt(buf),
        }
    }
}

impl CairoFormat for Vec<Pattern> {
    fn cfmt(&self, buf: &mut String) {
        self.cfmt_join(buf, " | ");
    }
}

impl CairoFormat for Condition {
    fn cfmt(&self, buf: &mut String) {
        match self {
            Condition::Let(let_cond) => let_cond.cfmt(buf),
            Condition::Expr(expr) => expr.cfmt(buf),
        }
    }
}

impl CairoFormat for Vec<Condition> {
    fn cfmt(&self, buf: &mut String) {
        self.cfmt_join(buf, " && ");
    }
}

impl CairoFormat for LetStatement {
    fn cfmt(&self, buf: &mut String) {
        self.attributes.cfmt(buf);
        buf.push_str("let ");
        self.pattern.cfmt(buf);
        if let Some(type_clause) = &self.type_clause {
            type_clause.cfmt_prefixed_str(buf, ": ");
        }
        buf.push_str(" = ");
        self.rhs.cfmt(buf);
        if let Some(let_else_clause) = &self.let_else_clause {
            buf.push_str(" else ");
            let_else_clause.cfmt_block_braced(buf);
        }
        buf.push(';');
    }
}

impl CairoFormat for ExprStatement {
    fn cfmt(&self, buf: &mut String) {
        self.attributes.cfmt(buf);
        self.expr.cfmt(buf);
        if self.semicolon {
            buf.push(';');
        }
    }
}

impl CairoFormat for ContinueStatement {
    fn cfmt(&self, buf: &mut String) {
        self.attributes.cfmt(buf);
        buf.push_str("continue;");
    }
}

impl CairoFormat for ReturnStatement {
    fn cfmt(&self, buf: &mut String) {
        self.attributes.cfmt(buf);
        buf.push_str("return");
        if let Some(expr) = &self.expr {
            expr.cfmt_prefixed(buf, ' ');
        }
        buf.push(';');
    }
}

impl CairoFormat for BreakStatement {
    fn cfmt(&self, buf: &mut String) {
        self.attributes.cfmt(buf);
        buf.push_str("break");
        if let Some(expr) = &self.expr {
            expr.cfmt_prefixed(buf, ' ');
        }
        buf.push(';');
    }
}

impl CairoFormat for PatternStruct {
    fn cfmt(&self, buf: &mut String) {
        self.path.cfmt(buf);
        self.params.cfmt_csv_braced(buf);
    }
}

impl CairoFormat for PatternStructParam {
    fn cfmt(&self, buf: &mut String) {
        match self {
            PatternStructParam::Single(param) => param.cfmt(buf),
            PatternStructParam::WithExpr(param) => param.cfmt(buf),
            PatternStructParam::Tail => buf.push_str(".."),
        }
    }
}

impl CairoFormat for ParamWithPatten {
    fn cfmt(&self, buf: &mut String) {
        self.modifiers.cfmt(buf);
        self.name.cfmt(buf);
        self.pattern.cfmt_prefixed_str(buf, ": ");
    }
}

impl CairoFormat for PatternEnum {
    fn cfmt(&self, buf: &mut String) {
        self.path.cfmt(buf);
        if let Some(pattern) = &self.pattern {
            pattern.cfmt_parenthesized(buf);
        }
    }
}

impl CairoFormat for LetCondition {
    fn cfmt(&self, buf: &mut String) {
        buf.push_str("let ");
        self.patterns.cfmt(buf);
        buf.push_str(" = ");
        self.expr.cfmt(buf);
    }
}
