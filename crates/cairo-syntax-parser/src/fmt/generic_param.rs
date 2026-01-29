use super::fmt::OptionSizeHint;
use super::{CairoWrite, CairoSliceFormat};
use std::fmt::{Result, Write};

use crate::fmt::fmt::SizeHint;
use crate::generic_param::{
    AssociatedItemConstraint, ConstGenericParam, GenericParam, ImplAnonymousGenericParam,
    ImplNamedGenericParam,
};

impl CairoWrite for GenericParam {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            GenericParam::Type(name) => name.cfmt(buf),
            GenericParam::Const(const_param) => const_param.cfmt(buf),
            GenericParam::ImplNamed(named) => named.cfmt(buf),
            GenericParam::ImplAnonymous(anon) => anon.cfmt(buf),
            GenericParam::NegativeImpl(path) => path.cfmt_prefixed(buf, '-'),
        }
    }
}
impl SizeHint for GenericParam {
    fn size_hint(&self) -> usize {
        match self {
            GenericParam::Type(name) => name.size_hint(),
            GenericParam::Const(const_param) => const_param.size_hint(),
            GenericParam::ImplNamed(named) => named.size_hint(),
            GenericParam::ImplAnonymous(anon) => anon.size_hint(),
            GenericParam::NegativeImpl(path) => 1 + path.size_hint(),
        }
    }
}
impl CairoWrite for ConstGenericParam {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.name.cfmt_prefixed_str(buf, "const ")?;
        self.ty.cfmt_prefixed_str(buf, ": ")
    }
}
impl SizeHint for ConstGenericParam {
    fn size_hint(&self) -> usize {
        8 + self.name.size_hint() + self.ty.size_hint()
    }
}

impl CairoWrite for ImplNamedGenericParam {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.name.cfmt_prefixed_str(buf, "impl ")?;
        self.trait_path.cfmt_prefixed_str(buf, ": ")?;
        self.type_constrains.cfmt(buf)
    }
}
impl SizeHint for ImplNamedGenericParam {
    fn size_hint(&self) -> usize {
        7 + self.name.size_hint() + self.trait_path.size_hint() + self.type_constrains.size_hint()
    }
}

impl CairoWrite for AssociatedItemConstraint {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.item.cfmt(buf)?;
        self.value.cfmt_prefixed_str(buf, ": ")
    }
}
impl SizeHint for AssociatedItemConstraint {
    fn size_hint(&self) -> usize {
        self.item.size_hint() + 2 + self.value.size_hint()
    }
}

impl CairoWrite for Option<Vec<AssociatedItemConstraint>> {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        if let Some(constraints) = self {
            constraints.cfmt_csv_bracketed(buf)?;
        }
        Ok(())
    }
}
impl SizeHint for Option<Vec<AssociatedItemConstraint>> {
    fn size_hint(&self) -> usize {
        self.size_hint_option::<2, 0>()
    }
}

impl SizeHint for Vec<AssociatedItemConstraint> {
    fn size_hint(&self) -> usize {
        self.size_hint_slice::<2>()
    }
}

impl CairoWrite for ImplAnonymousGenericParam {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.trait_path.cfmt_prefixed(buf, '+')?;
        self.type_constrains.cfmt(buf)
    }
}
impl SizeHint for ImplAnonymousGenericParam {
    fn size_hint(&self) -> usize {
        1 + self.trait_path.size_hint() + self.type_constrains.size_hint()
    }
}

impl CairoWrite for Option<Vec<GenericParam>> {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        if let Some(params) = self {
            params.cfmt_csv_angled(buf)?;
        }
        Ok(())
    }
}
impl SizeHint for Option<Vec<GenericParam>> {
    fn size_hint(&self) -> usize {
        self.size_hint_option::<2, 0>()
    }
}

impl SizeHint for Vec<GenericParam> {
    fn size_hint(&self) -> usize {
        self.size_hint_slice::<2>()
    }
}
