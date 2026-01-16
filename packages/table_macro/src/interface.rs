use crate::id::IdVariantTrait;
use crate::templates::{table_impl_tpl, table_meta_tpl};
use crate::{IdVariant, TableError, TableResult};
use introspect_macros::i_type::{AttributeParser, DefaultIExtractor, IExtract};
use introspect_macros::{CollectionsAsCairo, IAttribute, Struct};
use introspect_rust_macros::macro_attributes;

pub struct TableInterface {
    pub id: String,
    pub name: String,
    pub attributes: Vec<IAttribute>,
    pub impl_name: String,
    pub meta_impl_name: String,
}

#[derive(Default)]
#[macro_attributes]
pub struct TableAttributes {
    name: String,
    id: IdVariant,
}

impl IExtract<TableInterface> for DefaultIExtractor {
    type SyntaxType = Struct;
    type Error = TableError;
    fn iextract(&self, item: &mut Self::SyntaxType) -> TableResult<TableInterface> {
        let name = name.unwrap_or_else(|| item.name.clone());
        Ok(TableInterface {
            id: id.to_id_string(&name),
            attributes,
            impl_name: format!("{}Table", name),
            meta_impl_name: format!("{}Metadata", name),
            name: name,
        })
    }
}

impl TableInterface {
    pub fn table_impl(&self, struct_impl_name: &str) -> String {
        table_impl_tpl(&self.impl_name, struct_impl_name, &self.meta_impl_name)
    }

    pub fn meta_impl(&self) -> String {
        let attributes = self.attributes.as_cairo_span();
        table_meta_tpl(&self.meta_impl_name, &self.id, &self.impl_name, &attributes)
    }
}
