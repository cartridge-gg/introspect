use crate::templates::{append_table_attribute_tpl, interface_impl_name_tpl, table_impl_tpl};
use crate::{IdVariant, TableError, TableResult};
use introspect_macros::i_type::{AttributeParser, AttributeVariant, IExtract};
use introspect_macros::utils::string_to_keccak_hex;
use introspect_macros::{AsCairoBytes, CairoElementDef, IAttribute, IntrospectError, Struct};
use introspect_rust_macros::macro_attributes;
use itertools::Itertools;

pub struct TableInterface {
    pub id: String,
    pub name: String,
    pub attributes: Vec<IAttribute>,
    pub impl_name: String,
}

#[derive(Default)]
#[macro_attributes]
pub struct TableAttributes {
    name: String,
    id: IdVariant,
}

impl IExtract for TableInterface {
    type SyntaxType = Struct;
    type Error = TableError;
    fn iextract(item: &mut Self::SyntaxType) -> TableResult<TableInterface> {
        Ok(TableInterface {
            id: string_to_keccak_hex(&item.name),
            attributes: vec![],
            impl_name: interface_impl_name_tpl(&item.name),
            name: item.name.clone(),
        })
    }
}

impl AttributeParser<Struct> for TableAttributes {
    type Error = TableError;
    fn parse_attribute(
        &mut self,
        _module: &mut Struct,
        attribute: introspect_macros::Attribute,
    ) -> TableResult<Vec<AttributeVariant>> {
        match attribute.path_str() {
            "name" => self.set_name_return_empty(attribute.single_unnamed_arg()?),
            "id" => self.set_id_return_empty(attribute.single_unnamed_arg()?.try_into()?),
            _ => attribute.into(),
        }
    }
}

impl TableInterface {
    pub fn table_impl(&self, i_path: &str, i_table_path: &str, struct_impl_name: &str) -> String {
        let attributes = self
            .attributes
            .iter()
            .map(|attr| append_table_attribute_tpl(&attr.as_element_def(i_path)))
            .join("\n");

        table_impl_tpl(
            i_path,
            i_table_path,
            &self.impl_name,
            struct_impl_name,
            &self.id,
            &self.name.as_cairo_byte_array(),
            &attributes,
        )
    }
}
