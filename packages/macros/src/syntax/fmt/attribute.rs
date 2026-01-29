use super::{CairoCollectionFormat, CairoFormat, CodeBuffer};
use crate::syntax::attribute::Attribute;

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
