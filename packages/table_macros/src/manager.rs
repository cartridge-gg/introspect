use introspect_macros::i_type::extraction::IExtractor;
use introspect_macros::{AttributeCallType, Struct};

use crate::TableInterface;
use crate::structure::TableStructure;

pub struct TableManager<T> {
    pub structure: TableStructure,
    pub interfaces: Vec<TableInterface>,
    pub extractor: T,
}
