use super::{CairoWrite, CairoWriteSlice};
use crate::statement::{
    BreakStatement, Condition, ContinueStatement, ExprStatement, LetCondition, LetStatement,
    ParamWithPatten, Pattern, PatternEnum, PatternStruct, PatternStructParam, ReturnStatement,
    Statement,
};
use std::fmt::{Result, Write};

impl CairoWrite for Statement {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            Statement::Let(stmt) => stmt.cwrite(buf),
            Statement::Expr(stmt) => stmt.cwrite(buf),
            Statement::Continue(stmt) => stmt.cwrite(buf),
            Statement::Return(stmt) => stmt.cwrite(buf),
            Statement::Break(stmt) => stmt.cwrite(buf),
            Statement::Item(item) => item.cwrite(buf),
            Statement::Missing => Ok(()),
        }
    }
}

impl CairoWrite for Vec<Statement> {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.cwrite_block_braced(buf)
    }
}

impl CairoWrite for Pattern {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            Pattern::Underscore => buf.write_char('_'),
            Pattern::Literal(s) | Pattern::ShortString(s) | Pattern::String(s) => s.cwrite(buf),
            Pattern::False => buf.write_str("false"),
            Pattern::True => buf.write_str("true"),
            Pattern::Identifier(ident) => ident.cwrite(buf),
            Pattern::Struct(pat_struct) => pat_struct.cwrite(buf),
            Pattern::Tuple(patterns) => patterns.cwrite_tuple(buf),
            Pattern::Enum(pat_enum) => pat_enum.cwrite(buf),
            Pattern::FixedSizeArray(patterns) => patterns.cwrite_csv_bracketed(buf),
            Pattern::Path(expr_path) => expr_path.cwrite(buf),
        }
    }
}

impl CairoWrite for Vec<Pattern> {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.cwrite_join(buf, " | ")
    }
}

impl CairoWrite for Condition {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            Condition::Let(let_cond) => let_cond.cwrite(buf),
            Condition::Expr(expr) => expr.cwrite(buf),
        }
    }
}

impl CairoWrite for Vec<Condition> {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.cwrite_join(buf, " && ")
    }
}

impl CairoWrite for LetStatement {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cwrite(buf)?;
        buf.write_str("let ")?;
        self.pattern.cwrite(buf)?;
        if let Some(type_clause) = &self.type_clause {
            type_clause.cwrite_prefixed_str(buf, ": ")?;
        }
        buf.write_str(" = ")?;
        self.rhs.cwrite(buf)?;
        if let Some(let_else_clause) = &self.let_else_clause {
            buf.write_str(" else ")?;
            let_else_clause.cwrite(buf)?;
        }
        buf.write_char(';')
    }
}

impl CairoWrite for ExprStatement {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cwrite(buf)?;
        self.expr.cwrite(buf)?;
        if self.semicolon {
            buf.write_char(';')?;
        }
        Ok(())
    }
}

impl CairoWrite for ContinueStatement {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cwrite(buf)?;
        buf.write_str("continue")?;
        buf.write_char(';')
    }
}

impl CairoWrite for ReturnStatement {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cwrite(buf)?;
        buf.write_str("return")?;
        if let Some(expr) = &self.expr {
            expr.cwrite_prefixed(buf, ' ')?;
        }
        buf.write_char(';')
    }
}

impl CairoWrite for BreakStatement {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cwrite(buf)?;
        buf.write_str("break")?;
        if let Some(expr) = &self.expr {
            expr.cwrite_prefixed(buf, ' ')?;
        }
        buf.write_char(';')
    }
}

impl CairoWrite for PatternStruct {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.path.cwrite(buf)?;
        self.params.cwrite_csv_braced(buf)
    }
}

impl CairoWrite for PatternStructParam {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            PatternStructParam::Single(param) => param.cwrite(buf),
            PatternStructParam::WithExpr(param) => param.cwrite(buf),
            PatternStructParam::Tail => buf.write_str(".."),
        }
    }
}

impl CairoWrite for ParamWithPatten {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.modifiers.cwrite(buf)?;
        self.name.cwrite(buf)?;
        self.pattern.cwrite_prefixed_str(buf, ": ")
    }
}

impl CairoWrite for PatternEnum {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.path.cwrite(buf)?;
        if let Some(pattern) = &self.pattern {
            pattern.cwrite_parenthesized(buf)?;
        }
        Ok(())
    }
}

impl CairoWrite for LetCondition {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_str("let ")?;
        self.patterns.cwrite(buf)?;
        buf.write_str(" = ")?;
        self.expr.cwrite(buf)
    }
}
