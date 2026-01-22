use crate::TableStructure;

pub trait RecordPrimary<impl Struct: TableStructure, T> {
    fn record_primary(self: @Struct::Record) -> @Struct::Primary;
}
