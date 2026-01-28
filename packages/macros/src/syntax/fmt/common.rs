use super::{CairoCollectionFormat, CairoFormat};
use crate::syntax::common::{Identifier, Modifier, Param, Visibility};

impl CairoFormat for Identifier {
    fn cfmt(&self, buf: &mut String) {
        self.modifiers.cfmt(buf);
        self.name.cfmt(buf);
    }
}

impl CairoFormat for Modifier {
    fn cfmt(&self, buf: &mut String) {
        match self {
            Modifier::Ref => buf.push_str("ref"),
            Modifier::Mut => buf.push_str("mut"),
        }
    }
}

impl CairoFormat for Vec<Modifier> {
    fn cfmt(&self, buf: &mut String) {
        self.cfmt_terminated(buf, ' ');
    }
}

impl CairoFormat for Param {
    fn cfmt(&self, buf: &mut String) {
        self.modifiers.cfmt(buf);
        self.name.cfmt(buf);
        if let Some(type_clause) = &self.type_clause {
            type_clause.cfmt_prefixed(buf, ':');
        }
    }
}

impl CairoFormat for Visibility {
    fn cfmt(&self, buf: &mut String) {
        match self {
            Visibility::Default => {}
            Visibility::Pub(p) => {
                buf.push_str("pub");
                if let Some(arg) = p {
                    arg.cfmt_parenthesized(buf);
                }
                buf.push(' ')
            }
        }
    }
}
