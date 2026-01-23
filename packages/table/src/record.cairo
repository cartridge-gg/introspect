use core_ext::snapshots::ToSnapshotOf;
use core_ext::{AsSnapshot, ToSpan, TupleSnapForwardTo};
use introspect_types::{Entry, PrimaryTrait};
use crate::keyed::RecordKey;
use crate::{RecordPrimary, TableStructure};

pub trait RecordTrait<impl Table: TableStructure, AsRecord> {
    fn serialize_record(self: @AsRecord, ref data: Array<felt252>) -> felt252;
    #[inline(always)]
    fn record_tuple(
        self: @AsRecord,
    ) -> (
        felt252, Span<felt252>,
    ) {
        let mut data = array![];
        let id = Self::serialize_record(self, ref data);
        (id, data.span())
    }
    #[inline(always)]
    fn record_entry(self: @AsRecord) -> Entry {
        Self::record_tuple(self).into()
    }
}

pub trait RecordsTrait<impl Table: TableStructure, AsRecords> {
    fn serialize_records(self: AsRecords) -> Span<Entry>;
}

pub trait RecordValues<impl Table: TableStructure, Record> {
    fn serialize_values(self: @Table::Record, ref data: Array<felt252>);
}

pub trait RecordId<impl Table: TableStructure, AsId> {
    fn record_id(self: @AsId) -> felt252;
}

pub trait RecordIds<impl Table: TableStructure, AsIds> {
    fn record_ids(self: AsIds) -> Span<felt252>;
}

pub trait RecordIdSerialized<impl Table: TableStructure, AsId> {
    fn record_id_serialized(self: AsId, ref data: Array<felt252>) -> felt252;
}


impl KeyRecordSerializedImpl<
    impl Table: TableStructure, impl Key: RecordKey<Table, Table::Record>,
> of RecordIdSerialized<Table, Key::Snapped> {
    fn record_id_serialized(self: Key::Snapped, ref data: Array<felt252>) -> felt252 {
        Key::serialize_key_id(self, ref data)
    }
}

impl PrimaryRecordIdSerializedImpl<
    impl Table: TableStructure, +PrimaryTrait<Table::Primary>,
> of RecordIdSerialized<Table, @Table::Primary> {
    fn record_id_serialized(self: @Table::Primary, ref data: Array<felt252>) -> felt252 {
        self.to_felt252()
    }
}


impl PrimaryId<
    AsId,
    impl Table: TableStructure,
    impl S: ToSnapshotOf<@AsId, Table::Primary>,
    +PrimaryTrait<Table::Primary>,
> of RecordId<Table, AsId> {
    fn record_id(self: @AsId) -> felt252 {
        S::to_snapshot(self).to_felt252()
    }
}

impl PrimaryRecordId<
    AsRecord,
    impl Table: TableStructure,
    impl Primary: RecordPrimary<Table, Table::Record>,
    impl TR: ToSnapshotOf<@AsRecord, Table::Record>,
    +PrimaryTrait<Table::Primary>,
> of RecordId<Table, AsRecord> {
    fn record_id(self: @AsRecord) -> felt252 {
        let record = TR::to_snapshot(self);
        Primary::record_primary(record).to_felt252()
    }
}

impl KeyId<
    AsId,
    impl Table: TableStructure,
    impl Key: RecordKey<Table, Table::Record>,
    impl SK: TupleSnapForwardTo<AsId, Key::Snapped>,
> of RecordId<Table, AsId> {
    fn record_id(self: @AsId) -> felt252 {
        let snapped_key = SK::snap_forward(self);
        Key::key_id(snapped_key)
    }
}

impl SingleKeyId<
    AsId,
    impl Table: TableStructure,
    impl Key: RecordKey<Table, Table::Record>,
    impl SS: AsSnapshot<@AsId, Key::Snapped>,
> of RecordId<Table, AsId> {
    fn record_id(self: @AsId) -> felt252 {
        let snapped_key = SS::as_snapshot(self);
        Key::key_id(snapped_key)
    }
}

impl KeyedRecordIdImpl<
    AsId,
    impl Table: TableStructure,
    impl Key: RecordKey<Table, Table::Record>,
    impl SK: ToSnapshotOf<@AsId, Table::Record>,
    +Drop<Key::Snapped>,
> of RecordId<Table, AsId> {
    fn record_id(self: @AsId) -> felt252 {
        let snapped_key = SK::to_snapshot(self);
        Key::key_id(Key::record_key(snapped_key))
    }
}


impl RecordIdsImpl<
    ToIds, ToId, impl Table: TableStructure, impl Id: RecordId<Table, ToId>, +ToSpan<ToIds, ToId>,
> of RecordIds<Table, ToIds> {
    fn record_ids(self: ToIds) -> Span<felt252> {
        self.to_span().into_iter().map(|id| Id::record_id(id)).collect::<Array<_>>().span()
    }
}


impl KeyedRecord<
    impl Table: TableStructure,
    impl Key: RecordKey<Table, Table::Record>,
    impl Values: RecordValues<Table, Table::Record>,
    AsRecord,
    impl SR: ToSnapshotOf<@AsRecord, Table::Record>,
    +Drop<Key::Snapped>,
> of RecordTrait<Table, AsRecord> {
    fn serialize_record(self: @AsRecord, ref data: Array<felt252>) -> felt252 {
        let record = SR::to_snapshot(self);
        let id = Key::serialize_key_id(Key::record_key(record), ref data);
        Values::serialize_values(record, ref data);
        id
    }
}

impl PrimaryRecord<
    impl Table: TableStructure,
    impl Primary: RecordPrimary<Table, Table::Record>,
    impl Values: RecordValues<Table, Table::Record>,
    AsRecord,
    +PrimaryTrait<Table::Primary>,
    impl SR: ToSnapshotOf<@AsRecord, Table::Record>,
> of RecordTrait<Table, AsRecord> {
    fn serialize_record(self: @AsRecord, ref data: Array<felt252>) -> felt252 {
        let record = SR::to_snapshot(self);
        Values::serialize_values(record, ref data);
        Primary::record_primary(record).to_felt252()
    }
}

pub impl TupleRecordTrait<
    Tuple,
    impl Table: TableStructure,
    // -RecordId<Table, Table::Record>,
    impl Primary: PrimaryTrait<Table::Primary>,
    impl Values: RecordValues<Table, Table::Record>,
    impl TS: TupleSnapForwardTo<Tuple, (@Table::Primary, @Table::Record)>,
> of RecordTrait<Table, Tuple> {
    fn serialize_record(self: @Tuple, ref data: Array<felt252>) -> felt252 {
        let (key, record) = TS::snap_forward(self);
        Values::serialize_values(record, ref data);
        Primary::to_felt252(key)
    }
}


pub impl RecordsTraitImpl<
    impl Table: TableStructure,
    ToRecords,
    ToRecord,
    impl TS: ToSpan<ToRecords, ToRecord>,
    impl IM: RecordTrait<Table, ToRecord>,
> of RecordsTrait<Table, ToRecords> {
    fn serialize_records(self: ToRecords) -> Span<Entry> {
        self.to_span().into_iter().map(|M| IM::record_entry(M)).collect::<Array<_>>().span()
    }
}
