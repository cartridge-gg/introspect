use cairo_syntax_parser::{CairoWriteSlice, GenericParamsTrait, NameTrait};
use std::fmt::{Result as FmtResult, Write};
pub trait ItemTrait: GenericParamsTrait + NameTrait {
    fn type_selector(&self) -> &'static str;
    fn write_name<W: Write>(&self, buf: &mut W) -> FmtResult {
        buf.write_str(self.name())
    }
    fn instantiated_name(&self) -> String {
        let mut name = String::new();
        self.write_instantiated_name(&mut name).unwrap();
        name
    }
    fn write_instantiated_name<W: Write>(&self, buf: &mut W) -> FmtResult {
        self.write_name(buf)?;
        self.cwrite_generic_types(buf)
    }
    fn write_name_call<W: Write>(&self, buf: &mut W) -> FmtResult {
        self.write_name(buf)?;
        self.cwrite_generic_types_call(buf)
    }
    fn write_generics_with_traits<W: Write>(&self, buf: &mut W, traits: &[&str]) -> FmtResult {
        let generic_types = self.generic_types();
        if let Some(generic_types) = generic_types {
            buf.write_char('<')?;
            generic_types.cwrite_csv(buf)?;
            for t in traits {
                generic_types
                    .iter()
                    .map(|g| write!(buf, ", +{t}<{g}>"))
                    .collect::<FmtResult>()?;
            }
            buf.write_char('>')?;
        }
        Ok(())
    }
    fn generics_with_traits(&self, traits: &[&str]) -> String {
        let mut buf = String::new();
        self.write_generics_with_traits(&mut buf, traits).unwrap();
        buf
    }
    fn generics_call(&self) -> String {
        let mut buf = String::new();
        self.cwrite_generic_types_call(&mut buf).unwrap();
        buf
    }
}
