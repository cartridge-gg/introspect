use super::fmt::OptionSizeHint;
use super::{CairoWrite, CairoSliceFormat};
use crate::common::{Identifier, Modifier, Param, Visibility};
use crate::fmt::fmt::SizeHint;
use std::fmt::{Result, Write};

impl CairoWrite for Identifier {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.modifiers.cfmt(buf)?;
        self.name.cfmt(buf)
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.modifiers.cfmt_size_hint() + self.name.len()
    }
}

impl CairoWrite for Modifier {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            Modifier::Ref => buf.write_str("ref"),
            Modifier::Mut => buf.write_str("mut"),
        }
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        3
    }
}

impl CairoWrite for Vec<Modifier> {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.cfmt_terminated(buf, ' ')
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.size_hint_slice::<1>()
    }
}

impl CairoWrite for Param {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.modifiers.cfmt(buf)?;
        self.name.cfmt(buf)?;
        if let Some(type_clause) = &self.type_clause {
            type_clause.cfmt_prefixed_str(buf, ": ")?;
        }
        Ok(())
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.modifiers.cfmt_size_hint()
            + self.name.cfmt_size_hint()
            + self.type_clause.size_hint_option::<2, 0>()
    }
}

impl SizeHint for Vec<Param> {
    fn size_hint(&self) -> usize {
        self.size_hint_slice::<2>()
    }
}

impl CairoWrite for Visibility {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            Visibility::Default => Ok(()),
            Visibility::Pub(p) => {
                buf.write_str("pub")?;
                if let Some(arg) = p {
                    arg.cfmt_parenthesized(buf)?;
                }
                buf.write_char(' ')
            }
        }
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        match self {
            Visibility::Default => 0,
            Visibility::Pub(arg) => 4 + arg.size_hint_option::<1, 0>(),
        }
    }
}
