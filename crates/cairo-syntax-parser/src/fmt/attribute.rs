use super::{CairoCollectionFormat, CairoFormat, CodeBuffer};
use crate::attribute::{Arg, ArgClause, Attribute, NamedArg};

impl<T: CodeBuffer> CairoFormat<T> for Attribute {
    fn cfmt(&self, buf: &mut T) {
        buf.push_token_str("#[");
        self.path.cfmt(buf);
        if let Some(arguments) = &self.arguments {
            arguments.cfmt_csv_parenthesized(buf);
        }
        buf.push_token_char(']');
    }
}

impl<T: CodeBuffer> CairoFormat<T> for Vec<Attribute> {
    fn cfmt(&self, buf: &mut T) {
        if !self.is_empty() {
            self.cfmt_terminated(buf, '\n');
        }
    }
}

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
