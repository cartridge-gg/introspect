use super::{CairoWrite, CairoSliceFormat};
use crate::fmt::fmt::{OptionSizeHint, SizeHint};
use crate::statement::{
    BreakStatement, Condition, ContinueStatement, ExprStatement, LetCondition, LetStatement,
    ParamWithPatten, Pattern, PatternEnum, PatternStruct, PatternStructParam, ReturnStatement,
    Statement,
};
use std::fmt::{Result, Write};

impl CairoWrite for Statement {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            Statement::Let(stmt) => stmt.cfmt(buf),
            Statement::Expr(stmt) => stmt.cfmt(buf),
            Statement::Continue(stmt) => stmt.cfmt(buf),
            Statement::Return(stmt) => stmt.cfmt(buf),
            Statement::Break(stmt) => stmt.cfmt(buf),
            Statement::Item(item) => item.cfmt(buf),
            Statement::Missing => Ok(()),
        }
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        match self {
            Statement::Let(stmt) => stmt.size_hint(),
            Statement::Expr(stmt) => stmt.size_hint(),
            Statement::Continue(stmt) => stmt.size_hint(),
            Statement::Return(stmt) => stmt.size_hint(),
            Statement::Break(stmt) => stmt.size_hint(),
            Statement::Item(item) => item.size_hint(),
            Statement::Missing => 0,
        }
    }
}
impl CairoWrite for Vec<Statement> {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.cfmt_block_braced(buf)
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.size_hint_block::<2>() + 2
    }
}

impl CairoWrite for Pattern {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            Pattern::Underscore => buf.write_char('_'),
            Pattern::Literal(s) | Pattern::ShortString(s) | Pattern::String(s) => s.cfmt(buf),
            Pattern::False => buf.write_str("false"),
            Pattern::True => buf.write_str("true"),
            Pattern::Identifier(ident) => ident.cfmt(buf),
            Pattern::Struct(pat_struct) => pat_struct.cfmt(buf),
            Pattern::Tuple(patterns) => patterns.cfmt_tuple(buf),
            Pattern::Enum(pat_enum) => pat_enum.cfmt(buf),
            Pattern::FixedSizeArray(patterns) => patterns.cfmt_csv_bracketed(buf),
            Pattern::Path(expr_path) => expr_path.cfmt(buf),
        }
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        match self {
            Pattern::Underscore => 1,
            Pattern::Literal(s) | Pattern::ShortString(s) | Pattern::String(s) => s.len(),
            Pattern::False => 5,
            Pattern::True => 4,
            Pattern::Identifier(ident) => ident.size_hint(),
            Pattern::Struct(pat_struct) => pat_struct.size_hint(),
            Pattern::Tuple(patterns) => 2 + patterns.size_hint_slice::<3>(),
            Pattern::Enum(pat_enum) => pat_enum.size_hint(),
            Pattern::FixedSizeArray(patterns) => 2 + patterns.size_hint_slice::<2>(),
            Pattern::Path(expr_path) => expr_path.size_hint(),
        }
    }
}

impl CairoWrite for Vec<Pattern> {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.cfmt_join(buf, " | ")
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.size_hint_slice::<3>()
    }
}

impl CairoWrite for Condition {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            Condition::Let(let_cond) => let_cond.cfmt(buf),
            Condition::Expr(expr) => expr.cfmt(buf),
        }
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        match self {
            Condition::Let(let_cond) => let_cond.size_hint(),
            Condition::Expr(expr) => expr.size_hint(),
        }
    }
}

impl CairoWrite for Vec<Condition> {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.cfmt_join(buf, " && ")
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.size_hint_slice::<4>()
    }
}

impl CairoWrite for LetStatement {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cfmt(buf)?;
        buf.write_str("let ")?;
        self.pattern.cfmt(buf)?;
        if let Some(type_clause) = &self.type_clause {
            type_clause.cfmt_prefixed_str(buf, ": ")?;
        }
        buf.write_str(" = ")?;
        self.rhs.cfmt(buf)?;
        if let Some(let_else_clause) = &self.let_else_clause {
            buf.write_str(" else ")?;
            let_else_clause.cfmt(buf)?;
        }
        buf.write_char(';')
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.attributes.size_hint()
            + 8
            + self.pattern.size_hint()
            + self.type_clause.size_hint_option::<2, 0>()
            + self.rhs.size_hint()
            + self.let_else_clause.size_hint_option::<6, 0>()
    }
}

impl CairoWrite for ExprStatement {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cfmt(buf)?;
        self.expr.cfmt(buf)?;
        if self.semicolon {
            buf.write_char(';')?;
        }
        Ok(())
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.attributes.size_hint() + self.expr.size_hint() + if self.semicolon { 1 } else { 0 }
    }
}

impl CairoWrite for ContinueStatement {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cfmt(buf)?;
        buf.write_str("continue")?;
        buf.write_char(';')
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.attributes.size_hint() + 9
    }
}

impl CairoWrite for ReturnStatement {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cfmt(buf)?;
        buf.write_str("return")?;
        if let Some(expr) = &self.expr {
            expr.cfmt_prefixed(buf, ' ')?;
        }
        buf.write_char(';')
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.attributes.size_hint() + 7 + self.expr.size_hint_option::<1, 0>()
    }
}

impl CairoWrite for BreakStatement {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cfmt(buf)?;
        buf.write_str("break")?;
        if let Some(expr) = &self.expr {
            expr.cfmt_prefixed(buf, ' ')?;
        }
        buf.write_char(';')
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.attributes.size_hint() + 6 + self.expr.size_hint_option::<1, 0>()
    }
}

impl CairoWrite for PatternStruct {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.path.cfmt(buf)?;
        self.params.cfmt_csv_braced(buf)
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.path.size_hint() + 2 + self.params.size_hint_slice::<2>()
    }
}

impl CairoWrite for PatternStructParam {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            PatternStructParam::Single(param) => param.cfmt(buf),
            PatternStructParam::WithExpr(param) => param.cfmt(buf),
            PatternStructParam::Tail => buf.write_str(".."),
        }
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        match self {
            PatternStructParam::Single(param) => param.size_hint(),
            PatternStructParam::WithExpr(param) => param.size_hint(),
            PatternStructParam::Tail => 2,
        }
    }
}

impl CairoWrite for ParamWithPatten {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.modifiers.cfmt(buf)?;
        self.name.cfmt(buf)?;
        self.pattern.cfmt_prefixed_str(buf, ": ")
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.modifiers.size_hint() + self.name.size_hint() + 2 + self.pattern.size_hint()
    }
}

impl CairoWrite for PatternEnum {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.path.cfmt(buf)?;
        if let Some(pattern) = &self.pattern {
            pattern.cfmt_parenthesized(buf)?;
        }
        Ok(())
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.path.size_hint() + self.pattern.size_hint_option::<2, 0>()
    }
}

impl CairoWrite for LetCondition {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_str("let ")?;
        self.patterns.cfmt(buf)?;
        buf.write_str(" = ")?;
        self.expr.cfmt(buf)
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        7 + self.patterns.size_hint() + self.expr.size_hint()
    }
}
