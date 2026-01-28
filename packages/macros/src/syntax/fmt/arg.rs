use super::CairoFormat;
use crate::syntax::arg::{Arg, ArgClause, NamedArg};

impl CairoFormat for Arg {
    fn cfmt(&self, buf: &mut String) {
        self.modifiers.cfmt(buf);
        self.clause.cfmt(buf);
    }
}

impl CairoFormat for ArgClause {
    fn cfmt(&self, buf: &mut String) {
        match self {
            ArgClause::Unnamed(expr) => expr.cfmt(buf),
            ArgClause::Named(a) => a.cfmt(buf),
            ArgClause::Shorthand(s) => s.cfmt_prefixed(buf, ':'),
        }
    }
}

impl CairoFormat for NamedArg {
    fn cfmt(&self, buf: &mut String) {
        self.name.cfmt(buf);
        self.value.cfmt_prefixed_str(buf, ": ");
    }
}
