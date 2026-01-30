use super::{CairoWrite, CairoWriteSlice};
use crate::common::{Identifier, Modifier, Param, Visibility};
use std::fmt::{Result, Write};

impl CairoWrite for Identifier {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.modifiers.cwrite(buf)?;
        self.name.cwrite(buf)
    }
}

impl CairoWrite for Modifier {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            Modifier::Ref => buf.write_str("ref"),
            Modifier::Mut => buf.write_str("mut"),
        }
    }
}

impl CairoWrite for Vec<Modifier> {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.cwrite_terminated(buf, ' ')
    }
}

impl CairoWrite for Param {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.modifiers.cwrite(buf)?;
        self.name.cwrite(buf)?;
        if let Some(type_clause) = &self.type_clause {
            type_clause.cwrite_prefixed_str(buf, ": ")?;
        }
        Ok(())
    }
}

impl CairoWrite for Visibility {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            Visibility::Default => Ok(()),
            Visibility::Pub(p) => {
                buf.write_str("pub")?;
                if let Some(arg) = p {
                    arg.cwrite_parenthesized(buf)?;
                }
                buf.write_char(' ')
            }
        }
    }
}
