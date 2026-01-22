use crate::TableStructure;

pub trait RecordPrimary<impl Table: TableStructure, Record> {
    fn record_primary(self: @Table::Record) -> @Table::Primary;
}
