use super::{CairoWrite, CairoWriteSlice};
use crate::attribute::{Arg, ArgClause, Attribute, NamedArg};
use std::fmt::{Result, Write};

impl CairoWrite for Attribute {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_str("#[")?;
        self.path.cwrite(buf)?;
        if let Some(arguments) = &self.arguments {
            arguments.cwrite_csv_parenthesized(buf)?;
        }
        buf.write_char(']')
    }
}

impl CairoWrite for Vec<Attribute> {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        if !self.is_empty() {
            self.cwrite_terminated(buf, '\n')?;
        }
        Ok(())
    }
}

impl CairoWrite for Arg {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.modifiers.cwrite(buf)?;
        self.clause.cwrite(buf)?;
        Ok(())
    }
}

impl CairoWrite for ArgClause {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            ArgClause::Unnamed(expr) => expr.cwrite(buf),
            ArgClause::Named(a) => a.cwrite(buf),
            ArgClause::Shorthand(s) => s.cwrite_prefixed(buf, ':'),
        }
    }
}

impl CairoWrite for NamedArg {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.name.cwrite(buf)?;
        self.value.cwrite_prefixed_str(buf, ": ")
    }
}
