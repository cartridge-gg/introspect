#[derive(Drop, Clone)]
pub struct Foo {
    #[key]
    pub key_1: u128,
    #[key]
    pub key_2: ByteArray,
    pub name: ByteArray,
    pub something: u8,
}

// #[derive(Drop)]
// pub struct Foo2 {
//     #[key]
//     pub key_1: u128,
//     pub name: ByteArray,
//     pub something: u8,
//     pub value_16: u16,
// }

//// GENERATED CODE BELOW

impl FooTableMeta of introspect_table::TableMeta {
    const ID: felt252 = selector!("Foo");
    fn name() -> ByteArray {
        "Foo"
    }
    fn attributes() -> Span<introspect_types::Attribute> {
        [].span()
    }
}

impl FooStructure of introspect_table::TableStructure {
    type Primary = felt252;
    type Record = Foo;
    fn columns() -> Span<introspect_types::ColumnDef> {
        [
            introspect_types::ColumnDefTrait::new::<u128>(FooColumns::key_1, "key_1", [].span()),
            introspect_types::ColumnDefTrait::new::<
                ByteArray,
            >(FooColumns::key_2, "key_2", [].span()),
            introspect_types::ColumnDefTrait::new::<ByteArray>(FooColumns::name, "name", [].span()),
            introspect_types::ColumnDefTrait::new::<
                u8,
            >(FooColumns::something, "something", [].span()),
        ]
            .span()
    }
    fn child_defs() -> Array<(felt252, introspect_types::TypeDef)> {
        Default::default()
    }
}

pub impl FooKeySpanToPrimary of introspect_table::KeySpanToPrimary<Foo, FooTableSchema> {
    fn key_span_to_primary(self: Span<felt252>) -> felt252 {
        core::poseidon::poseidon_hash_span(self)
    }
}


///// Non Overridable

pub mod FooColumns {
    pub const key_1: felt252 = selector!("key_1");
    pub const key_2: felt252 = selector!("key_2");
    pub const name: felt252 = selector!("name");
    pub const something: felt252 = selector!("something");
}

pub impl FooTableSchema = introspect_table::TableSchemaImpl<FooTableMeta, FooStructure>;

pub impl FooTable = introspect_table::TableImpl<FooTableSchema>;

impl Foo_key_1_MemberImpl<impl T: introspect_table::TableSchema[Record: Foo]> =
    introspect_table::m_utils::TableMemberImpl<T, FooColumns::key_1, u128>;

impl Foo_key_2_MemberImpl<impl T: introspect_table::TableSchema[Record: Foo]> =
    introspect_table::m_utils::TableMemberImpl<T, FooColumns::key_2, ByteArray>;

pub impl Foo_name_MemberImpl<impl T: introspect_table::TableSchema[Record: Foo]> =
    introspect_table::m_utils::TableMemberImpl<T, FooColumns::name, ByteArray>;
pub impl Foo_something_MemberImpl<impl T: introspect_table::TableSchema[Record: Foo]> =
    introspect_table::m_utils::TableMemberImpl<T, FooColumns::something, u8>;

impl FooRecordKey of introspect_table::RecordKey<Foo, (@u128, @ByteArray), FooTableSchema> {
    type Key = (u128, ByteArray);
    fn record_key(self: @Foo) -> (@u128, @ByteArray) {
        (self.key_1, self.key_2)
    }
}

impl FooSerialisedKey<
    impl T: introspect_table::TableSchema[Record: Foo],
    KS,
    K0,
    K1,
    +introspect_table::Snapable<@KS, (K0, K1)>,
    +introspect_table::Snapable<@K0, u128>,
    +introspect_table::Snapable<@K1, ByteArray>,
> of introspect_table::SerialisedKey<T::Record, KS, T> {
    fn serialize_key(self: @KS, ref data: Array<felt252>) {
        let (key_1, key_2) = self.snapshot();
        Foo_key_1_MemberImpl::<T>::serialize_member(key_1, ref data);
        Foo_key_2_MemberImpl::<T>::serialize_member(key_2, ref data);
    }
}

impl FooRecordValuesSpan<
    impl T: introspect_table::TableSchema[Record: Foo],
> of introspect_table::RecordValuesSpanTrait<T, T::Record> {
    fn serialize_values(self: @T::Record, ref data: Array<felt252>) {
        Foo_name_MemberImpl::<T>::serialize_member(self.name, ref data);
        Foo_something_MemberImpl::<T>::serialize_member(self.something, ref data);
    }
}
