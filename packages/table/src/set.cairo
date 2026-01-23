use core_ext::{ToSnapshotBase, ToSnapshotOf, ToSpan, TupleSnapForwardTo};
use introspect_types::Entry;
use crate::{RecordId, TableStructure};

pub trait ValueColumnSet<impl Table: TableStructure, const SIZE: usize, Value> {
    const GROUP_ID: felt252;
    const COLUMN_IDS: [felt252; SIZE];
    fn serialize_set_value(self: @Value, ref data: Array<felt252>);
}


pub trait ItemColumnSet<impl Table: TableStructure, const SIZE: usize, Item> {
    const GROUP_ID: felt252;
    const COLUMN_IDS: [felt252; SIZE];
    fn serialize_set_item(self: @Item, ref data: Array<felt252>) -> felt252;
}


pub trait ColumnSet<impl Table: TableStructure, Item, const SIZE: usize> {
    const GROUP_ID: felt252;
    const COLUMN_IDS: [felt252; SIZE];
    fn serialise_set(self: @Item, ref data: Array<felt252>) -> felt252;
    fn set_tuple(
        self: @Item,
    ) -> (
        felt252, Span<felt252>,
    ) {
        let mut data = array![];
        let record_id = Self::serialise_set(self, ref data);
        (record_id, data.span())
    }
    fn set_id_data(self: @Item) -> Entry {
        Self::set_tuple(self).into()
    }
    fn serialise_rows_set<Items, +ToSpan<Items, Item>>(
        self: Items,
    ) -> Span<
        Entry,
    > {
        self.to_span().into_iter().map(|M| Self::set_id_data(M)).collect::<Array<_>>().span()
    }
}

pub impl ItemColumnSetImpl<
    const SIZE: usize,
    AsSet,
    impl TS: ToSnapshotBase<@AsSet>,
    impl Table: TableStructure,
    impl SetTrait: ItemColumnSet<Table, SIZE, TS::Base>,
> of ColumnSet<Table, AsSet, SIZE> {
    const GROUP_ID: felt252 = SetTrait::GROUP_ID;
    const COLUMN_IDS: [felt252; SIZE] = SetTrait::COLUMN_IDS;
    fn serialise_set(self: @AsSet, ref data: Array<felt252>) -> felt252 {
        let set = TS::to_snapshot(self);
        SetTrait::serialize_set_item(set, ref data)
    }
}


impl TupleColumnSetImpl<
    const SIZE: usize,
    Tuple,
    AsId,
    Value,
    impl Table: TableStructure,
    impl Id: RecordId<Table, AsId>,
    impl SetTrait: ValueColumnSet<Table, SIZE, Value>,
    impl TS: TupleSnapForwardTo<Tuple, (@AsId, @Value)>,
> of ColumnSet<Table, Tuple, SIZE> {
    const GROUP_ID: felt252 = SetTrait::GROUP_ID;
    const COLUMN_IDS: [felt252; SIZE] = SetTrait::COLUMN_IDS;
    fn serialise_set(self: @Tuple, ref data: Array<felt252>) -> felt252 {
        let (to_id, field) = TS::snap_forward(self);
        SetTrait::serialize_set_value(field, ref data);
        Id::record_id(to_id)
    }
}
