use crate::TableStructure;

pub trait RecordPrimary<impl Table: TableStructure, T> {
    fn record_primary(self: @Table::Record) -> @Table::Primary;
}
