use core_ext::poseidon::PoseidonFixedArray;
use core_ext::{SnapForwardTo, ToSnapshotBase, ToSpan, poseidon_hash_fixed_array};
use introspect_events::EmitEvent;
use introspect_events::database::CreateColumnSet;
use introspect_types::Entry;
use crate::{RecordId, TableStructure};


pub trait ColumnSetDefaultIds<T, const SIZE: usize> {
    const COLUMN_IDS: [felt252; SIZE];
}

pub trait ColumnSetDefault<T, const SIZE: usize> {
    const COLUMN_IDS: [felt252; SIZE];
    const GROUP_ID: felt252;
    fn column_ids() -> Span<felt252>;
}

pub impl ColumnSetDefaultImpl<
    T,
    const SIZE: usize,
    impl CS: ColumnSetDefaultIds<T, SIZE>,
    +PoseidonFixedArray<[felt252; SIZE]>,
    impl ToSpan: ToSpanTrait<[felt252; SIZE], felt252>,
> of ColumnSetDefault<T, SIZE> {
    const COLUMN_IDS: [felt252; SIZE] = CS::COLUMN_IDS;
    const GROUP_ID: felt252 = poseidon_hash_fixed_array(Self::COLUMN_IDS);
    fn column_ids() -> Span<felt252> {
        ToSpan::span(@Self::COLUMN_IDS)
    }
}

pub trait ColumnSets {
    fn create<
        Set, const SIZE: usize, impl ColumnSet: ColumnSetDefault<Set, SIZE>,
    >() {
        CreateColumnSet { id: ColumnSet::GROUP_ID, columns: ColumnSet::column_ids() }.emit_event()
    }
}

pub trait ItemColumnSet<impl Table: TableStructure, Item, const SIZE: usize> {
    const COLUMN_IDS: [felt252; SIZE];
    const GROUP_ID: felt252;
    fn serialize_set_id(self: @Item, ref data: Array<felt252>) -> felt252;
    fn serialize_set_value(self: @Item, ref data: Array<felt252>);
    fn column_ids() -> Span<felt252>;
}

pub trait ValueColumnSet<impl Table: TableStructure, Value, const SIZE: usize> {
    const COLUMN_IDS: [felt252; SIZE];
    const GROUP_ID: felt252;
    fn serialize_set_value(self: @Value, ref data: Array<felt252>);
    fn column_ids() -> Span<felt252>;
}

pub trait ColumnSet<impl Table: TableStructure, Item, const SIZE: usize> {
    const GROUP_ID: felt252;
    const COLUMN_IDS: [felt252; SIZE];
    fn serialize_set(self: @Item, ref data: Array<felt252>) -> felt252;
    fn set_tuple(
        self: @Item,
    ) -> (
        felt252, Span<felt252>,
    ) {
        let mut data = array![];
        let record_id = Self::serialize_set(self, ref data);
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
    fn column_ids() -> Span<felt252>;
}

pub impl ItemColumnSetImpl<
    const SIZE: usize,
    AsSet,
    impl TS: ToSnapshotBase<@AsSet>,
    impl Table: TableStructure,
    impl SetTrait: ItemColumnSet<Table, TS::Base, SIZE>,
> of ColumnSet<Table, AsSet, SIZE> {
    const GROUP_ID: felt252 = SetTrait::GROUP_ID;
    const COLUMN_IDS: [felt252; SIZE] = SetTrait::COLUMN_IDS;
    fn serialize_set(self: @AsSet, ref data: Array<felt252>) -> felt252 {
        let set = TS::to_snapshot(self);
        let id = SetTrait::serialize_set_id(set, ref data);
        SetTrait::serialize_set_value(set, ref data);
        id
    }
    fn column_ids() -> Span<felt252> {
        SetTrait::column_ids()
    }
}

pub impl SetRecordId<
    const SIZE: usize,
    AsSet,
    impl TS: ToSnapshotBase<@AsSet>,
    impl Table: TableStructure,
    impl SetTrait: ItemColumnSet<Table, TS::Base, SIZE>,
> of RecordId<Table, AsSet> {
    fn record_id(self: @AsSet) -> felt252 {
        let set = TS::to_snapshot(self);
        let mut data = array![];
        SetTrait::serialize_set_id(set, ref data)
    }
}


impl TupleColumnSetImpl<
    const SIZE: usize,
    Tuple,
    AsId,
    Value,
    impl Table: TableStructure,
    impl Id: RecordId<Table, AsId>,
    impl SetTrait: ValueColumnSet<Table, Value, SIZE>,
    impl TS: SnapForwardTo<Tuple, (@AsId, @Value)>,
> of ColumnSet<Table, Tuple, SIZE> {
    const COLUMN_IDS: [felt252; SIZE] = SetTrait::COLUMN_IDS;
    const GROUP_ID: felt252 = SetTrait::GROUP_ID; // Cant make here due to compiler
    fn serialize_set(self: @Tuple, ref data: Array<felt252>) -> felt252 {
        let (to_id, field) = TS::snap_forward(self);
        SetTrait::serialize_set_value(field, ref data);
        Id::record_id(to_id)
    }
    fn column_ids() -> Span<felt252> {
        SetTrait::column_ids()
    }
}
