use super::{CairoCollectionFormat, CairoFormat, CodeBuffer};
use crate::statement::{
    BreakStatement, Condition, ContinueStatement, ExprStatement, LetCondition, LetStatement,
    ParamWithPatten, Pattern, PatternEnum, PatternStruct, PatternStructParam, ReturnStatement,
    Statement,
};

impl<T: CodeBuffer> CairoFormat<T> for Statement {
    fn cfmt(&self, buf: &mut T) {
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
impl<T: CodeBuffer> CairoFormat<T> for Vec<Statement> {
    fn cfmt(&self, buf: &mut T) {
        self.cfmt_block_braced(buf);
    }
}

impl<T: CodeBuffer> CairoFormat<T> for Pattern {
    fn cfmt(&self, buf: &mut T) {
        match self {
            Pattern::Underscore => buf.push_token_char('_'),
            Pattern::Literal(s) | Pattern::ShortString(s) | Pattern::String(s) => s.cfmt(buf),
            Pattern::False => buf.push_token_str("false"),
            Pattern::True => buf.push_token_str("true"),
            Pattern::Identifier(ident) => ident.cfmt(buf),
            Pattern::Struct(pat_struct) => pat_struct.cfmt(buf),
            Pattern::Tuple(patterns) => patterns.cfmt_tuple(buf),
            Pattern::Enum(pat_enum) => pat_enum.cfmt(buf),
            Pattern::FixedSizeArray(patterns) => patterns.cfmt_csv_bracketed(buf),
            Pattern::Path(expr_path) => expr_path.cfmt(buf),
        }
    }
}

impl<T: CodeBuffer> CairoFormat<T> for Vec<Pattern> {
    fn cfmt(&self, buf: &mut T) {
        self.cfmt_join(buf, " | ");
    }
}

impl<T: CodeBuffer> CairoFormat<T> for Condition {
    fn cfmt(&self, buf: &mut T) {
        match self {
            Condition::Let(let_cond) => let_cond.cfmt(buf),
            Condition::Expr(expr) => expr.cfmt(buf),
        }
    }
}

impl<T: CodeBuffer> CairoFormat<T> for Vec<Condition> {
    fn cfmt(&self, buf: &mut T) {
        self.cfmt_join(buf, " && ");
    }
}

impl<T: CodeBuffer> CairoFormat<T> for LetStatement {
    fn cfmt(&self, buf: &mut T) {
        self.attributes.cfmt(buf);
        buf.push_token_str("let ");
        self.pattern.cfmt(buf);
        if let Some(type_clause) = &self.type_clause {
            type_clause.cfmt_prefixed_str(buf, ": ");
        }
        buf.push_token_str(" = ");
        self.rhs.cfmt(buf);
        if let Some(let_else_clause) = &self.let_else_clause {
            buf.push_token_str(" else ");
            let_else_clause.cfmt_block_braced(buf);
        }
        buf.push_token_char(';');
    }
}

impl<T: CodeBuffer> CairoFormat<T> for ExprStatement {
    fn cfmt(&self, buf: &mut T) {
        self.attributes.cfmt(buf);
        self.expr.cfmt(buf);
        if self.semicolon {
            buf.push_token_char(';');
        }
    }
}

impl<T: CodeBuffer> CairoFormat<T> for ContinueStatement {
    fn cfmt(&self, buf: &mut T) {
        self.attributes.cfmt(buf);
        buf.push_token_str("continue");
        buf.push_token_char(';');
    }
}

impl<T: CodeBuffer> CairoFormat<T> for ReturnStatement {
    fn cfmt(&self, buf: &mut T) {
        self.attributes.cfmt(buf);
        buf.push_token_str("return");
        if let Some(expr) = &self.expr {
            expr.cfmt_prefixed(buf, ' ');
        }
        buf.push_token_char(';');
    }
}

impl<T: CodeBuffer> CairoFormat<T> for BreakStatement {
    fn cfmt(&self, buf: &mut T) {
        self.attributes.cfmt(buf);
        buf.push_token_str("break");
        if let Some(expr) = &self.expr {
            expr.cfmt_prefixed(buf, ' ');
        }
        buf.push_token_char(';');
    }
}

impl<T: CodeBuffer> CairoFormat<T> for PatternStruct {
    fn cfmt(&self, buf: &mut T) {
        self.path.cfmt(buf);
        self.params.cfmt_csv_braced(buf);
    }
}

impl<T: CodeBuffer> CairoFormat<T> for PatternStructParam {
    fn cfmt(&self, buf: &mut T) {
        match self {
            PatternStructParam::Single(param) => param.cfmt(buf),
            PatternStructParam::WithExpr(param) => param.cfmt(buf),
            PatternStructParam::Tail => buf.push_token_str(".."),
        }
    }
}

impl<T: CodeBuffer> CairoFormat<T> for ParamWithPatten {
    fn cfmt(&self, buf: &mut T) {
        self.modifiers.cfmt(buf);
        self.name.cfmt(buf);
        self.pattern.cfmt_prefixed_str(buf, ": ");
    }
}

impl<T: CodeBuffer> CairoFormat<T> for PatternEnum {
    fn cfmt(&self, buf: &mut T) {
        self.path.cfmt(buf);
        if let Some(pattern) = &self.pattern {
            pattern.cfmt_parenthesized(buf);
        }
    }
}

impl<T: CodeBuffer> CairoFormat<T> for LetCondition {
    fn cfmt(&self, buf: &mut T) {
        buf.push_token_str("let ");
        self.patterns.cfmt(buf);
        buf.push_token_str(" = ");
        self.expr.cfmt(buf);
    }
}
