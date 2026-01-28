use super::{CairoCollectionFormat, CairoFormat};
use crate::syntax::generic_param::{
    AssociatedItemConstraint, ConstGenericParam, GenericParam, ImplAnonymousGenericParam,
    ImplNamedGenericParam,
};

impl CairoFormat for GenericParam {
    fn cfmt(&self, buf: &mut String) {
        match self {
            GenericParam::Type(name) => name.cfmt(buf),
            GenericParam::Const(const_param) => const_param.cfmt(buf),
            GenericParam::ImplNamed(named) => named.cfmt(buf),
            GenericParam::ImplAnonymous(anon) => anon.cfmt(buf),
            GenericParam::NegativeImpl(path) => path.cfmt_prefixed(buf, '-'),
        }
    }
}
impl CairoFormat for ConstGenericParam {
    fn cfmt(&self, buf: &mut String) {
        self.name.cfmt_prefixed_str(buf, "const ");
        self.ty.cfmt_prefixed_str(buf, ": ");
    }
}

impl CairoFormat for ImplNamedGenericParam {
    fn cfmt(&self, buf: &mut String) {
        self.name.cfmt_prefixed_str(buf, "impl ");
        self.trait_path.cfmt_prefixed_str(buf, ": ");
        self.type_constrains.cfmt(buf);
    }
}

impl CairoFormat for AssociatedItemConstraint {
    fn cfmt(&self, buf: &mut String) {
        self.item.cfmt(buf);
        self.value.cfmt_prefixed_str(buf, ": ");
    }
}

impl CairoFormat for Option<Vec<AssociatedItemConstraint>> {
    fn cfmt(&self, buf: &mut String) {
        if let Some(constraints) = self {
            constraints.cfmt_csv_bracketed(buf);
        }
    }
}

impl CairoFormat for ImplAnonymousGenericParam {
    fn cfmt(&self, buf: &mut String) {
        self.trait_path.cfmt_prefixed(buf, '+');
        self.type_constrains.cfmt(buf);
    }
}

impl CairoFormat for Option<Vec<GenericParam>> {
    fn cfmt(&self, buf: &mut String) {
        if let Some(params) = self {
            params.cfmt_csv_angled(buf);
        }
    }
}
