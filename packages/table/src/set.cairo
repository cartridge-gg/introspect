use introspect_types::IdData;
use crate::{RecordId, Spannable, TableStructure, ToSnapshot, TupleSnappable};

pub trait ValueColumnSet<impl Struct: TableStructure, const SIZE: usize, Value> {
    const GROUP_ID: felt252;
    const COLUMN_IDS: [felt252; SIZE];
    fn serialize_set_value(self: @Value, ref data: Array<felt252>);
}


pub trait EntryColumnSet<impl Struct: TableStructure, const SIZE: usize, Entry> {
    const GROUP_ID: felt252;
    const COLUMN_IDS: [felt252; SIZE];
    fn serialize_set_entry(self: @Entry, ref data: Array<felt252>) -> felt252;
}


pub trait ColumnSet<impl Struct: TableStructure, Entry, const SIZE: usize> {
    const GROUP_ID: felt252;
    const COLUMN_IDS: [felt252; SIZE];
    fn serialise_set(self: @Entry, ref data: Array<felt252>) -> felt252;
    fn set_tuple(
        self: @Entry,
    ) -> (
        felt252, Span<felt252>,
    ) {
        let mut data = array![];
        let record_id = Self::serialise_set(self, ref data);
        (record_id, data.span())
    }
    fn set_id_data(self: @Entry) -> IdData {
        Self::set_tuple(self).into()
    }
    fn serialise_rows_set<Entries, +Spannable<Entries, Entry>>(
        self: Entries,
    ) -> Span<
        IdData,
    > {
        self.to_span().into_iter().map(|M| Self::set_id_data(M)).collect::<Array<_>>().span()
    }
}

pub impl EntryColumnSetImpl<
    const SIZE: usize,
    Entry,
    impl Struct: TableStructure,
    impl Set: EntryColumnSet<Struct, SIZE, Entry>,
    T,
    impl SS: ToSnapshot<@T, Entry>,
> of ColumnSet<Struct, T, SIZE> {
    const GROUP_ID: felt252 = Set::GROUP_ID;
    const COLUMN_IDS: [felt252; SIZE] = Set::COLUMN_IDS;
    fn serialise_set(self: @T, ref data: Array<felt252>) -> felt252 {
        let set = SS::to_snapshot(self);
        Set::serialize_set_entry(set, ref data)
    }
}


impl TupleColumnSetImpl<
    const SIZE: usize,
    Value,
    impl Struct: TableStructure,
    impl Set: ValueColumnSet<Struct, SIZE, Value>,
    Entry,
    ToId,
    impl Id: RecordId<Struct, ToId>,
    impl TS: TupleSnappable<Entry, (@ToId, @Value)>,
> of ColumnSet<Struct, Entry, SIZE> {
    const GROUP_ID: felt252 = Set::GROUP_ID;
    const COLUMN_IDS: [felt252; SIZE] = Set::COLUMN_IDS;
    fn serialise_set(self: @Entry, ref data: Array<felt252>) -> felt252 {
        let (to_id, field) = TS::snap_tuple(self);
        Set::serialize_set_value(field, ref data);
        Id::record_id(to_id)
    }
}
