use crate::IntrospectItem;
use crate::item::ItemTrait;
use std::fmt::{Result as FmtResult, Write};

mod derive;
mod enums;
mod structs;

pub trait CWriteISerde: ItemTrait {
    fn cwrite_iserialize<W: Write>(&self, buf: &mut W, i_path: &str) -> FmtResult;
    fn cwrite_ideserialize<W: Write>(&self, buf: &mut W, i_path: &str) -> FmtResult;
    fn cwrite_iserde_impl<W: Write>(&self, buf: &mut W, i_path: &str) -> FmtResult {
        let name = self.name();
        let instantiated_name = self.instantiated_name();
        write!(buf, "impl {name}ISerde")?;
        self.write_generics_with_traits(buf, &["Drop", &format!("{i_path}::ISerde")])?;
        writeln!(buf, " of {i_path}::ISerde<{instantiated_name}>{{")?;
        writeln!(
            buf,
            "fn iserialize(self: @{instantiated_name}, ref output: Array<felt252>) {{"
        )?;
        self.cwrite_iserialize(buf, i_path)?;
        writeln!(
            buf,
            "}}\nfn ideserialize(ref serialized: Span<felt252>) -> Option<{instantiated_name}> {{"
        )?;
        self.cwrite_ideserialize(buf, i_path)?;
        buf.write_str("\n}\n}")
    }
    fn iserde_impl_to_string(&self, i_path: &str) -> String {
        let mut buf = String::new();
        self.cwrite_iserde_impl(&mut buf, i_path).unwrap();
        buf
    }
}

impl CWriteISerde for IntrospectItem {
    fn cwrite_iserialize<W: Write>(&self, buf: &mut W, i_path: &str) -> FmtResult {
        match self {
            IntrospectItem::Struct(s) => s.cwrite_iserialize(buf, i_path),
            IntrospectItem::Enum(e) => e.cwrite_iserialize(buf, i_path),
        }
    }
    fn cwrite_ideserialize<W: Write>(&self, buf: &mut W, i_path: &str) -> FmtResult {
        match self {
            IntrospectItem::Struct(s) => s.cwrite_ideserialize(buf, i_path),
            IntrospectItem::Enum(e) => e.cwrite_ideserialize(buf, i_path),
        }
    }
}
