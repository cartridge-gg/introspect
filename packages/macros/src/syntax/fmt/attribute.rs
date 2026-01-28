use super::{CairoCollectionFormat, CairoFormat};
use crate::syntax::attribute::Attribute;

impl CairoFormat for Attribute {
    fn cfmt(&self, buf: &mut String) {
        buf.push_str("#[");
        self.path.cfmt(buf);
        if let Some(arguments) = &self.arguments {
            arguments.cfmt_csv_parenthesized(buf);
        }
        buf.push(']');
    }
}

impl CairoFormat for Vec<Attribute> {
    fn cfmt(&self, buf: &mut String) {
        if !self.is_empty() {
            self.cfmt_terminated(buf, '\n');
        }
    }
}
