use super::{CairoWrite, CairoWriteSlice};
use crate::generic_param::{
    AssociatedItemConstraint, ConstGenericParam, GenericParam, ImplAnonymousGenericParam,
    ImplNamedGenericParam,
};
use std::fmt::{Result, Write};

impl CairoWrite for GenericParam {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            GenericParam::Type(name) => name.cwrite(buf),
            GenericParam::Const(const_param) => const_param.cwrite(buf),
            GenericParam::ImplNamed(named) => named.cwrite(buf),
            GenericParam::ImplAnonymous(anon) => anon.cwrite(buf),
            GenericParam::NegativeImpl(path) => path.cwrite_prefixed(buf, '-'),
        }
    }
}

impl CairoWrite for ConstGenericParam {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.name.cwrite_prefixed_str(buf, "const ")?;
        self.ty.cwrite_prefixed_str(buf, ": ")
    }
}

impl CairoWrite for ImplNamedGenericParam {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.name.cwrite_prefixed_str(buf, "impl ")?;
        self.trait_path.cwrite_prefixed_str(buf, ": ")?;
        self.type_constrains.cwrite(buf)
    }
}

impl CairoWrite for AssociatedItemConstraint {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.item.cwrite(buf)?;
        self.value.cwrite_prefixed_str(buf, ": ")
    }
}

impl CairoWrite for Option<Vec<AssociatedItemConstraint>> {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        if let Some(constraints) = self {
            constraints.cwrite_csv_bracketed(buf)?;
        }
        Ok(())
    }
}

impl CairoWrite for ImplAnonymousGenericParam {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.trait_path.cwrite_prefixed(buf, '+')?;
        self.type_constrains.cwrite(buf)
    }
}

impl CairoWrite for Option<Vec<GenericParam>> {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        if let Some(params) = self {
            params.cwrite_csv_angled(buf)?;
        }
        Ok(())
    }
}
