use super::{CairoFormat, CodeBuffer};
use crate::syntax::arg::{Arg, ArgClause, NamedArg};

impl<T: CodeBuffer> CairoFormat<T> for Arg {
    fn cfmt(&self, buf: &mut T) {
        self.modifiers.cfmt(buf);
        self.clause.cfmt(buf);
    }
}

impl<T: CodeBuffer> CairoFormat<T> for ArgClause {
    fn cfmt(&self, buf: &mut T) {
        match self {
            ArgClause::Unnamed(expr) => expr.cfmt(buf),
            ArgClause::Named(a) => a.cfmt(buf),
            ArgClause::Shorthand(s) => s.cfmt_prefixed(buf, ':'),
        }
    }
}

impl<T: CodeBuffer> CairoFormat<T> for NamedArg {
    fn cfmt(&self, buf: &mut T) {
        self.name.cfmt(buf);
        self.value.cfmt_prefixed_str(buf, ": ");
    }
}
