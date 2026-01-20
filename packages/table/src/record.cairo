use introspect_types::{IdData, PrimaryTrait};
use crate::keyed::RecordKey;
use crate::{Snapable, Spannable, TableStructure, TupleSnappable};

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

// pub trait EntryRecordTrait<impl Struct: TableStructure> {
//     fn serialize_id_record(self: @Struct::Record, ref data: Array<felt252>) -> felt252;
//     fn record_tuple(
//         self: @Struct::Record,
//     ) -> (
//         felt252, Span<felt252>,
//     ) {
//         let mut data = array![];
//         let id = Self::serialize_id_record(self, ref data);
//         (id, data.span())
//     }
//     fn record_id_data(self: @Struct::Record) -> IdData {
//         Self::record_tuple(self).into()
//     }
// }

pub trait RecordValues<impl Struct: TableStructure, T> {
    fn serialize_values(self: @Struct::Record, ref data: Array<felt252>);
}

pub trait RecordId<impl Struct: TableStructure, ToId> {
    fn record_id(self: @ToId) -> felt252;
}

pub trait RecordIds<impl Struct: TableStructure, ToIds> {
    fn record_ids(self: ToIds) -> Span<felt252>;
}

impl RecordIdPrimary<
    T,
    impl Struct: TableStructure,
    impl S: Snapable<@T, Struct::Primary>,
    +PrimaryTrait<Struct::Primary>,
> of RecordId<Struct, T> {
    fn record_id(self: @T) -> felt252 {
        S::snapshot(self).to_felt252()
    }
}

pub impl KeyRecordId<
    T,
    impl Struct: TableStructure,
    impl Key: RecordKey<Struct>,
    impl SK: TupleSnappable<T, Key::Snapped>,
> of RecordId<Struct, T> {
    fn record_id(self: @T) -> felt252 {
        let snapped_key = SK::snap_tuple(self);
        Key::key_id(snapped_key)
    }
}

pub impl KeyedRecordIdRecordImpl<
    T,
    impl Struct: TableStructure,
    impl Key: RecordKey<Struct>,
    impl SK: Snapable<@T, Struct::Record>,
    +Drop<Key::Snapped>,
> of RecordId<Struct, T> {
    fn record_id(self: @T) -> felt252 {
        let snapped_key = SK::snapshot(self);
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
    T,
    impl Struct: TableStructure,
    impl Key: RecordKey<Struct>,
    impl Values: RecordValues<Struct, Struct::Record>,
    impl SR: Snapable<@T, Struct::Record>,
    +Drop<Key::Snapped>,
> of RecordTrait<Struct, T> {
    fn serialize_record(self: @T, ref data: Array<felt252>) -> felt252 {
        let record = SR::snapshot(self);
        let id = Key::serialize_key_id(Key::record_key(record), ref data);
        Values::serialize_values(record, ref data);
        id
    }
}

impl ToIdRecord<
    T,
    impl Struct: TableStructure,
    impl Id: RecordId<Struct, Struct::Record>,
    -RecordKey<Struct>,
    impl Values: RecordValues<Struct, Struct::Record>,
    impl SR: Snapable<@T, Struct::Record>,
> of RecordTrait<Struct, T> {
    fn serialize_record(self: @T, ref data: Array<felt252>) -> felt252 {
        let record = SR::snapshot(self);
        Values::serialize_values(record, ref data);
        Id::record_id(record)
    }
}

pub impl TupleRecordTrait<
    Entry,
    impl Struct: TableStructure,
    impl TS: TupleSnappable<Entry, (@Struct::Primary, @Struct::Record)>,
    -RecordId<Struct, Struct::Record>,
    impl Primary: PrimaryTrait<Struct::Primary>,
    impl Values: RecordValues<Struct, Struct::Record>,
> of RecordTrait<Struct, Entry> {
    fn serialize_record(self: @Entry, ref data: Array<felt252>) -> felt252 {
        let (key, record) = TS::snap_tuple(self);
        Values::serialize_values(record, ref data);
        Primary::to_felt252(key)
    }
}


impl RecordsTraitImpl<
    Entries,
    Entry,
    impl Struct: TableStructure,
    impl IM: RecordTrait<Struct, Entry>,
    +Spannable<Entries, Entry>,
> of RecordsTrait<Struct, Entries> {
    fn serialize_records(self: Entries) -> Span<IdData> {
        self.to_span().into_iter().map(|M| IM::record_id_data(M)).collect::<Array<_>>().span()
    }
}
