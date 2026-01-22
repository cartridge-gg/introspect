use crate::TableStructure;

pub trait RecordKeyValue<impl Struct: TableStructure> {
    type Value;
    type Snapped;
    fn record_key(self: @Struct::Record) -> Self::Snapped;
    fn serialize_key(self: Self::Snapped, ref data: Array<felt252>);
    fn key_value_id(self: Self::Snapped) -> felt252;
}

pub trait RecordKeySerialized<impl Struct: TableStructure> {
    type Value;
    type Snapped;
    fn record_key(self: @Struct::Record) -> Self::Snapped;
    fn serialize_key(self: Self::Snapped, ref data: Array<felt252>);
    fn serialised_key_id(self: Span<felt252>) -> felt252;
}

pub trait RecordKey<impl Struct: TableStructure, T> {
    type Value;
    type Snapped;
    fn key_id(self: Self::Snapped) -> felt252;
    fn record_key(self: @Struct::Record) -> Self::Snapped;
    fn serialize_key_id(self: Self::Snapped, ref data: Array<felt252>) -> felt252;
    fn serialize_key(self: Self::Snapped, ref data: Array<felt252>);
    fn serialize_key_inline(
        self: Self::Snapped,
    ) -> Span<felt252> {
        let mut data = array![];
        Self::serialize_key(self, ref data);
        data.span()
    }
}

pub impl KeySerializedIdEntry<
    impl Struct: TableStructure, impl Key: RecordKeySerialized<Struct>, +Drop<Key::Snapped>,
> of RecordKey<Struct, Struct::Record> {
    type Value = Key::Value;
    type Snapped = Key::Snapped;
    fn serialize_key_id(self: Key::Snapped, ref data: Array<felt252>) -> felt252 {
        let start = data.len();
        Key::serialize_key(self, ref data);
        Key::serialised_key_id(data.span().slice(start, data.len() - start))
    }
    fn key_id(self: Key::Snapped) -> felt252 {
        Key::serialised_key_id(Self::serialize_key_inline(self))
    }
    fn record_key(self: @Struct::Record) -> Key::Snapped {
        Key::record_key(self)
    }
    fn serialize_key(self: Key::Snapped, ref data: Array<felt252>) {
        Key::serialize_key(self, ref data);
    }
}

impl KeyIdEntry<
    impl Struct: TableStructure,
    impl Key: RecordKeyValue<Struct>,
    +Copy<Key::Snapped>,
    +Drop<Key::Snapped>,
> of RecordKey<Struct, Struct::Record> {
    type Value = Key::Value;
    type Snapped = Key::Snapped;
    fn serialize_key_id(self: Key::Snapped, ref data: Array<felt252>) -> felt252 {
        Key::serialize_key(self, ref data);
        Key::key_value_id(self)
    }
    fn key_id(self: Key::Snapped) -> felt252 {
        Key::key_value_id(self)
    }
    fn record_key(self: @Struct::Record) -> Key::Snapped {
        Key::record_key(self)
    }
    fn serialize_key(self: Key::Snapped, ref data: Array<felt252>) {
        Key::serialize_key(self, ref data);
    }
}
