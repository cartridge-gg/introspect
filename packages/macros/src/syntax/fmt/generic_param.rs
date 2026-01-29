use super::{CairoCollectionFormat, CairoFormat, CodeBuffer};
use crate::syntax::generic_param::{
    AssociatedItemConstraint, ConstGenericParam, GenericParam, ImplAnonymousGenericParam,
    ImplNamedGenericParam,
};

impl<T: CodeBuffer> CairoFormat<T> for GenericParam {
    fn cfmt(&self, buf: &mut T) {
        match self {
            GenericParam::Type(name) => name.cfmt(buf),
            GenericParam::Const(const_param) => const_param.cfmt(buf),
            GenericParam::ImplNamed(named) => named.cfmt(buf),
            GenericParam::ImplAnonymous(anon) => anon.cfmt(buf),
            GenericParam::NegativeImpl(path) => path.cfmt_prefixed(buf, '-'),
        }
    }
}
impl<T: CodeBuffer> CairoFormat<T> for ConstGenericParam {
    fn cfmt(&self, buf: &mut T) {
        self.name.cfmt_prefixed_str(buf, "const ");
        self.ty.cfmt_prefixed_str(buf, ": ");
    }
}

impl<T: CodeBuffer> CairoFormat<T> for ImplNamedGenericParam {
    fn cfmt(&self, buf: &mut T) {
        self.name.cfmt_prefixed_str(buf, "impl ");
        self.trait_path.cfmt_prefixed_str(buf, ": ");
        self.type_constrains.cfmt(buf);
    }
}

impl<T: CodeBuffer> CairoFormat<T> for AssociatedItemConstraint {
    fn cfmt(&self, buf: &mut T) {
        self.item.cfmt(buf);
        self.value.cfmt_prefixed_str(buf, ": ");
    }
}

impl<T: CodeBuffer> CairoFormat<T> for Option<Vec<AssociatedItemConstraint>> {
    fn cfmt(&self, buf: &mut T) {
        if let Some(constraints) = self {
            constraints.cfmt_csv_bracketed(buf);
        }
    }
}

impl<T: CodeBuffer> CairoFormat<T> for ImplAnonymousGenericParam {
    fn cfmt(&self, buf: &mut T) {
        self.trait_path.cfmt_prefixed(buf, '+');
        self.type_constrains.cfmt(buf);
    }
}

impl<T: CodeBuffer> CairoFormat<T> for Option<Vec<GenericParam>> {
    fn cfmt(&self, buf: &mut T) {
        if let Some(params) = self {
            params.cfmt_csv_angled(buf);
        }
    }
}
