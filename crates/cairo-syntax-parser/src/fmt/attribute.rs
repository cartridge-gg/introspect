use super::fmt::{OptionSizeHint, SizeHint};
use super::{CairoWrite, CairoSliceFormat};
use crate::attribute::{Arg, ArgClause, Attribute, NamedArg};
use std::fmt::{Result, Write};

impl CairoWrite for Attribute {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_str("#[")?;
        self.path.cfmt(buf)?;
        if let Some(arguments) = &self.arguments {
            arguments.cfmt_csv_parenthesized(buf)?;
        }
        buf.write_char(']')
    }
}
impl SizeHint for Attribute {
    fn size_hint(&self) -> usize {
        3 + self.path.size_hint() + self.arguments.size_hint_option::<2, 0>()
    }
}

impl CairoWrite for Vec<Attribute> {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        if !self.is_empty() {
            self.cfmt_terminated(buf, '\n')?;
        }
        Ok(())
    }
}
impl SizeHint for Vec<Attribute> {
    fn size_hint(&self) -> usize {
        self.size_hint_slice::<1>()
    }
}

impl CairoWrite for Arg {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.modifiers.cfmt(buf)?;
        self.clause.cfmt(buf)?;
        Ok(())
    }
}
impl SizeHint for Arg {
    fn size_hint(&self) -> usize {
        self.modifiers.size_hint() + self.clause.size_hint()
    }
}

impl SizeHint for Vec<Arg> {
    fn size_hint(&self) -> usize {
        self.size_hint_slice::<2>()
    }
}

impl CairoWrite for ArgClause {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            ArgClause::Unnamed(expr) => expr.cfmt(buf),
            ArgClause::Named(a) => a.cfmt(buf),
            ArgClause::Shorthand(s) => s.cfmt_prefixed(buf, ':'),
        }
    }
}
impl SizeHint for ArgClause {
    fn size_hint(&self) -> usize {
        match self {
            ArgClause::Unnamed(expr) => expr.size_hint(),
            ArgClause::Named(a) => a.size_hint(),
            ArgClause::Shorthand(s) => 1 + s.size_hint(),
        }
    }
}

impl CairoWrite for NamedArg {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.name.cfmt(buf)?;
        self.value.cfmt_prefixed_str(buf, ": ")
    }
}
impl SizeHint for NamedArg {
    fn size_hint(&self) -> usize {
        self.name.size_hint() + 2 + self.value.size_hint()
    }
}
