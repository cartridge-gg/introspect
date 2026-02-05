use crate::IVariant;
use crate::type_def::TypeDefField;
use cairo_syntax_parser::CairoWrite;

impl TypeDefField for IVariant {
    fn cwrite_pre_meta_data<W: std::fmt::Write>(&self, buf: &mut W) -> std::fmt::Result {
        self.selector.cwrite_suffixed(buf, ',')
    }
}
