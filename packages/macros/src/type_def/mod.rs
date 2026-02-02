pub mod derive;
pub mod enums;
pub mod structs;

use crate::IntrospectItem;
use std::fmt::{Result as FmtResult, Write};

pub trait CWriteTypeDef {
    fn cwrite_type_def<W: Write>(&self, buf: &mut W, i_path: &str) -> FmtResult;
    fn type_def_impl_to_string(&self, i_path: &str) -> String {
        let mut buf = String::new();
        self.cwrite_type_def(&mut buf, i_path).unwrap();
        buf
    }
}

impl CWriteTypeDef for IntrospectItem {
    fn cwrite_type_def<W: Write>(&self, buf: &mut W, i_path: &str) -> FmtResult {
        match self {
            IntrospectItem::Struct(s) => s.cwrite_type_def(buf, i_path),
            IntrospectItem::Enum(e) => e.cwrite_type_def(buf, i_path),
        }
    }
}

pub trait FieldsTrait {
    fn cwrite_function_calls<W: Write>(
        &self,
        buf: &mut W,
        function: &str,
        function_args: &str,
        field_function: &str,
        field_function_args: &str,
        generics_call: &str,
        prefix: &str,
    ) -> FmtResult;
    fn cwrite_serialize<W: Write>(
        &self,
        buf: &mut W,
        field: &str,
        generics_call: &str,
        prefix: &str,
    ) -> FmtResult {
        self.cwrite_function_calls(
            buf,
            &format!("serialize_{field}s"),
            "ref output: Array<felt252>",
            "serialize",
            "ref output",
            generics_call,
            prefix,
        )
    }
    fn cwrite_collect_children<W: Write>(
        &self,
        buf: &mut W,
        i_path: &str,
        field: &str,
        generics_call: &str,
        prefix: &str,
    ) -> FmtResult {
        self.cwrite_function_calls(
            buf,
            &format!("collect_{field}_children"),
            &format!("ref children: {i_path}::ChildDefs"),
            "collect_children",
            "ref children",
            generics_call,
            prefix,
        )
    }
    fn write_serialize_with_children<W: Write>(
        &self,
        buf: &mut W,
        i_path: &str,
        field: &str,
        generics_call: &str,
        prefix: &str,
    ) -> FmtResult {
        self.cwrite_function_calls(
            buf,
            &format!("serialize_{field}s_with_children"),
            &format!("ref type_def: Array<felt252>, ref children: {i_path}::ChildDefs"),
            "serialize_with_children",
            "ref type_def, ref children",
            generics_call,
            prefix,
        )
    }
}
impl FieldsTrait for [&str] {
    fn cwrite_function_calls<W: Write>(
        &self,
        buf: &mut W,
        function: &str,
        function_args: &str,
        field_function: &str,
        field_function_args: &str,
        generics_call: &str,
        prefix: &str,
    ) -> FmtResult {
        write!(buf, "fn {function}({function_args}) {{",)?;
        self.iter().try_for_each(|field| {
            write!(
                buf,
                "{prefix}::{field}{generics_call}::{field_function}({field_function_args});",
            )
        })?;
        buf.write_char('}')
    }
}
