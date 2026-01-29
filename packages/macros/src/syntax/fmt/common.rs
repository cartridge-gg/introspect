use super::{CairoCollectionFormat, CairoFormat, CodeBuffer};
use crate::syntax::common::{Identifier, Modifier, Param, Visibility};

impl<T: CodeBuffer> CairoFormat<T> for Identifier {
    fn cfmt(&self, buf: &mut T) {
        self.modifiers.cfmt(buf);
        self.name.cfmt(buf);
    }
}

impl<T: CodeBuffer> CairoFormat<T> for Modifier {
    fn cfmt(&self, buf: &mut T) {
        match self {
            Modifier::Ref => buf.push_token_str("ref"),
            Modifier::Mut => buf.push_token_str("mut"),
        }
    }
}

impl<T: CodeBuffer> CairoFormat<T> for Vec<Modifier> {
    fn cfmt(&self, buf: &mut T) {
        self.cfmt_terminated(buf, ' ');
    }
}

impl<T: CodeBuffer> CairoFormat<T> for Param {
    fn cfmt(&self, buf: &mut T) {
        self.modifiers.cfmt(buf);
        self.name.cfmt(buf);
        if let Some(type_clause) = &self.type_clause {
            type_clause.cfmt_prefixed(buf, ':');
        }
    }
}

impl<T: CodeBuffer> CairoFormat<T> for Visibility {
    fn cfmt(&self, buf: &mut T) {
        match self {
            Visibility::Default => {}
            Visibility::Pub(p) => {
                buf.push_token_str("pub");
                if let Some(arg) = p {
                    arg.cfmt_parenthesized(buf);
                }
                buf.push_token_char(' ')
            }
        }
    }
}
