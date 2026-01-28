mod arg;
mod attribute;
mod common;
mod expr;
mod generic_param;
mod item;
mod statement;
pub trait CairoFormat {
    fn cfmt(&self, buf: &mut String);
    fn cfmt_suffixed_str(&self, buf: &mut String, suffix: &str) {
        self.cfmt(buf);
        buf.push_str(suffix);
    }
    fn cfmt_prefixed(&self, buf: &mut String, prefix: char) {
        buf.push(prefix);
        self.cfmt(buf);
    }
    fn cfmt_prefixed_str(&self, buf: &mut String, prefix: &str) {
        buf.push_str(prefix);
        self.cfmt(buf);
    }
    fn cfmt_suffixed(&self, buf: &mut String, suffix: char) {
        self.cfmt(buf);
        buf.push(suffix);
    }
    fn cfmt_wrapped(&self, buf: &mut String, prefix: char, suffix: char) {
        buf.push(prefix);
        self.cfmt(buf);
        buf.push(suffix);
    }
    fn cfmt_wrapped_str(&self, buf: &mut String, prefix: &str, suffix: &str) {
        buf.push_str(prefix);
        self.cfmt(buf);
        buf.push_str(suffix);
    }
    fn cfmt_parenthesized(&self, buf: &mut String) {
        self.cfmt_wrapped(buf, '(', ')');
    }
    fn cfmt_braced(&self, buf: &mut String) {
        self.cfmt_wrapped(buf, '{', '}');
    }
    fn cfmt_bracketed(&self, buf: &mut String) {
        self.cfmt_wrapped(buf, '[', ']');
    }

    fn to_cairo(&self) -> String {
        let mut buf = String::new();
        self.cfmt(&mut buf);
        buf
    }
}

pub trait CairoCollectionFormat {
    type Element: CairoFormat;
    fn cfmt_elements(&self) -> &[Self::Element];

    fn cfmt_join(&self, buf: &mut String, delimiter: &str) {
        let elements = self.cfmt_elements();
        if let Some((first, rest)) = elements.split_first() {
            first.cfmt(buf);
            rest.iter()
                .for_each(|e| e.cfmt_prefixed_str(buf, delimiter));
        }
    }
    fn cfmt_delimited(&self, buf: &mut String, delimiter: char) {
        let elements = self.cfmt_elements();
        if let Some((first, rest)) = elements.split_first() {
            first.cfmt(buf);
            rest.iter().for_each(|e| e.cfmt_prefixed(buf, delimiter));
        }
    }
    fn cfmt_terminated(&self, buf: &mut String, terminator: char) {
        self.cfmt_elements()
            .iter()
            .for_each(|e| e.cfmt_suffixed(buf, terminator));
    }
    fn cfmt_terminated_str(&self, buf: &mut String, terminator: &str) {
        self.cfmt_elements()
            .iter()
            .for_each(|e| e.cfmt_suffixed_str(buf, terminator));
    }

    fn cfmt_concatenated(&self, buf: &mut String) {
        self.cfmt_elements().iter().for_each(|e| e.cfmt(buf));
    }

    fn cfmt_csv(&self, buf: &mut String) {
        self.cfmt_join(buf, ", ");
    }
    fn cfmt_block(&self, buf: &mut String) {
        let elements = self.cfmt_elements();
        if !elements.is_empty() {
            buf.push('\n');
            elements.cfmt_terminated(buf, '\n');
        }
    }
    fn cfmt_block_braced(&self, buf: &mut String) {
        buf.push('{');
        self.cfmt_block(buf);
        buf.push('}');
    }
    fn cfmt_tuple(&self, buf: &mut String) {
        buf.push('(');
        let elements = self.cfmt_elements();
        match elements.len() {
            0 => {}
            1 => {
                elements[0].cfmt_suffixed_str(buf, ", ");
            }
            _ => self.cfmt_join(buf, ", "),
        }
        buf.push(')');
    }
    fn cfmt_csv_braced(&self, buf: &mut String) {
        buf.push('{');
        self.cfmt_csv(buf);
        buf.push('}');
    }
    fn cfmt_csv_parenthesized(&self, buf: &mut String) {
        buf.push('(');
        self.cfmt_csv(buf);
        buf.push(')');
    }
    fn cfmt_csv_bracketed(&self, buf: &mut String) {
        buf.push('[');
        self.cfmt_csv(buf);
        buf.push(']');
    }
    fn cfmt_csv_angled(&self, buf: &mut String) {
        buf.push('<');
        self.cfmt_csv(buf);
        buf.push('>');
    }
    fn cfmt_csv_barred(&self, buf: &mut String) {
        buf.push('|');
        self.cfmt_csv(buf);
        buf.push('|');
    }
    fn cfmt_array(&self, buf: &mut String) {
        buf.push_str("array![");
        self.cfmt_csv(buf);
        buf.push(']');
    }
    fn cfmt_span(&self, buf: &mut String) {
        buf.push('[');
        self.cfmt_csv(buf);
        buf.push_str("].span()");
    }
    fn cfmt_fields(&self, buf: &mut String) {
        let elements = self.cfmt_elements();
        if !elements.is_empty() {
            buf.push('\n');
            elements.cfmt_terminated_str(buf, ",\n");
        }
    }
    fn cfmt_fields_braced(&self, buf: &mut String) {
        buf.push('{');
        self.cfmt_fields(buf);
        buf.push('}');
    }

    fn to_cairo_block(&self) -> String {
        let mut buf = String::new();
        self.cfmt_block(&mut buf);
        buf
    }
}

impl CairoFormat for String {
    fn cfmt(&self, buf: &mut String) {
        buf.push_str(self);
    }
}

impl<T: CairoFormat> CairoCollectionFormat for Vec<T> {
    type Element = T;
    fn cfmt_elements(&self) -> &[Self::Element] {
        self
    }
}

impl<T: CairoFormat> CairoCollectionFormat for [T] {
    type Element = T;
    fn cfmt_elements(&self) -> &[Self::Element] {
        self
    }
}
