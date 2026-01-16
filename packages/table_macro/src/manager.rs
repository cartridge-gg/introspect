use introspect_macros::i_type::extraction::IExtractor;
use introspect_macros::{AttributeCallType, Struct};

use crate::TableInterface;
use crate::structure::TableStructure;

pub struct TableManager<T> {
    pub macro_mode: AttributeCallType,
    pub structure: TableStructure,
    pub interfaces: Vec<TableInterface>,
    pub extractor: T,
}

impl<T> TableManager<T> {
    fn from_struct(item: Struct) -> Self {}
}
