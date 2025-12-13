pub mod deref;
pub mod example_use;
pub mod gen_groups;
pub mod gen_table_id;
pub mod gen_table_keyed;
pub mod gen_table_no_id;
pub mod table;
pub use deref::{Snapable, Spannable};
pub use table::{
    ColumnId, FieldOnlyColumnGroup, ITableImpl, IdColumnGroup, KeySpanToId, KeySpanToPrimary,
    MemberTrait, RecordId, RecordKey, RecordValuesSpanTrait, SerialisedKey, Table, TableColumns,
    TableImpl, TableMeta, TablePrimary, iserde_table_member, table_primary,
};
