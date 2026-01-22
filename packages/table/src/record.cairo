use introspect_types::{IdData, PrimaryTrait};
use crate::deref::AsSnapshot;
use crate::keyed::RecordKey;
use crate::{RecordPrimary, Spannable, TableStructure, ToSnapshot, TupleSnappable};

pub trait RecordTrait<impl Struct: TableStructure, R> {
    fn serialize_record(self: @R, ref data: Array<felt252>) -> felt252;
    fn record_tuple(
        self: @R,
    ) -> (
        felt252, Span<felt252>,
    ) {
        let mut data = array![];
        let id = Self::serialize_record(self, ref data);
        (id, data.span())
    }
    fn record_id_data(self: @R) -> IdData {
        Self::record_tuple(self).into()
    }
}

pub trait RecordsTrait<impl Struct: TableStructure, Records> {
    fn serialize_records(self: Records) -> Span<IdData>;
}

pub trait RecordValues<impl Struct: TableStructure, T> {
    fn serialize_values(self: @Struct::Record, ref data: Array<felt252>);
}

pub trait RecordId<impl Struct: TableStructure, ToId> {
    fn record_id(self: @ToId) -> felt252;
}

pub trait RecordIds<impl Struct: TableStructure, ToIds> {
    fn record_ids(self: ToIds) -> Span<felt252>;
}

pub trait RecordIdSerialized<impl Struct: TableStructure, ToId> {
    fn record_id_serialized(self: ToId, ref data: Array<felt252>) -> felt252;
}


impl KeyRecordSerializedImpl<
    impl Struct: TableStructure, impl Key: RecordKey<Struct, Struct::Record>,
> of RecordIdSerialized<Struct, Key::Snapped> {
    fn record_id_serialized(self: Key::Snapped, ref data: Array<felt252>) -> felt252 {
        Key::serialize_key_id(self, ref data)
    }
}

impl PrimaryRecordIdSerializedImpl<
    impl Struct: TableStructure, +PrimaryTrait<Struct::Primary>,
> of RecordIdSerialized<Struct, @Struct::Primary> {
    fn record_id_serialized(self: @Struct::Primary, ref data: Array<felt252>) -> felt252 {
        self.to_felt252()
    }
}


impl PrimaryId<
    T,
    impl Struct: TableStructure,
    impl S: ToSnapshot<@T, Struct::Primary>,
    +PrimaryTrait<Struct::Primary>,
> of RecordId<Struct, T> {
    fn record_id(self: @T) -> felt252 {
        S::to_snapshot(self).to_felt252()
    }
}

impl PrimaryRecordId<
    T,
    impl Struct: TableStructure,
    impl Primary: RecordPrimary<Struct, Struct::Record>,
    impl SR: ToSnapshot<@T, Struct::Record>,
    +PrimaryTrait<Struct::Primary>,
> of RecordId<Struct, T> {
    fn record_id(self: @T) -> felt252 {
        let record = SR::to_snapshot(self);
        Primary::record_primary(record).to_felt252()
    }
}

impl KeyId<
    T,
    impl Struct: TableStructure,
    impl Key: RecordKey<Struct, Struct::Record>,
    impl SK: TupleSnappable<T, Key::Snapped>,
> of RecordId<Struct, T> {
    fn record_id(self: @T) -> felt252 {
        let snapped_key = SK::snap_tuple(self);
        Key::key_id(snapped_key)
    }
}

impl SingleKeyId<
    T,
    impl Struct: TableStructure,
    impl Key: RecordKey<Struct, Struct::Record>,
    impl SS: AsSnapshot<@T, Key::Snapped>,
> of RecordId<Struct, T> {
    fn record_id(self: @T) -> felt252 {
        let snapped_key = SS::as_snapshot(self);
        Key::key_id(snapped_key)
    }
}

impl KeyedRecordIdImpl<
    T,
    impl Struct: TableStructure,
    impl Key: RecordKey<Struct, Struct::Record>,
    impl SK: ToSnapshot<@T, Struct::Record>,
    +Drop<Key::Snapped>,
> of RecordId<Struct, T> {
    fn record_id(self: @T) -> felt252 {
        let snapped_key = SK::to_snapshot(self);
        Key::key_id(Key::record_key(snapped_key))
    }
}


impl RecordIdsImpl<
    ToIds,
    ToId,
    impl Struct: TableStructure,
    impl Id: RecordId<Struct, ToId>,
    +Spannable<ToIds, ToId>,
> of RecordIds<Struct, ToIds> {
    fn record_ids(self: ToIds) -> Span<felt252> {
        self.to_span().into_iter().map(|id| Id::record_id(id)).collect::<Array<_>>().span()
    }
}


impl KeyedRecord<
    impl Struct: TableStructure,
    impl Key: RecordKey<Struct, Struct::Record>,
    impl Values: RecordValues<Struct, Struct::Record>,
    T,
    impl SR: ToSnapshot<@T, Struct::Record>,
    +Drop<Key::Snapped>,
> of RecordTrait<Struct, T> {
    fn serialize_record(self: @T, ref data: Array<felt252>) -> felt252 {
        let record = SR::to_snapshot(self);
        let id = Key::serialize_key_id(Key::record_key(record), ref data);
        Values::serialize_values(record, ref data);
        id
    }
}

impl PrimaryRecord<
    impl Struct: TableStructure,
    impl Primary: RecordPrimary<Struct, Struct::Record>,
    impl Values: RecordValues<Struct, Struct::Record>,
    T,
    impl SR: ToSnapshot<@T, Struct::Record>,
    -RecordKey<Struct, Struct::Record>,
    +PrimaryTrait<Struct::Primary>,
> of RecordTrait<Struct, T> {
    fn serialize_record(self: @T, ref data: Array<felt252>) -> felt252 {
        let record = SR::to_snapshot(self);
        Values::serialize_values(record, ref data);
        Primary::record_primary(record).to_felt252()
    }
}

pub impl TupleRecordTrait<
    impl Struct: TableStructure,
    impl Primary: PrimaryTrait<Struct::Primary>,
    impl Values: RecordValues<Struct, Struct::Record>,
    Entry,
    impl TS: TupleSnappable<Entry, (@Struct::Primary, @Struct::Record)>,
    -RecordId<Struct, Struct::Record>,
> of RecordTrait<Struct, Entry> {
    fn serialize_record(self: @Entry, ref data: Array<felt252>) -> felt252 {
        let (key, record) = TS::snap_tuple(self);
        Values::serialize_values(record, ref data);
        Primary::to_felt252(key)
    }
}


pub impl RecordsTraitImpl<
    impl Struct: TableStructure,
    Entries,
    Entry,
    +Spannable<Entries, Entry>,
    impl IM: RecordTrait<Struct, Entry>,
> of RecordsTrait<Struct, Entries> {
    fn serialize_records(self: Entries) -> Span<IdData> {
        self.to_span().into_iter().map(|M| IM::record_id_data(M)).collect::<Array<_>>().span()
    }
}
