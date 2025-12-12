use crate::Snapable;

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

impl FooTableMeta of introspect_table::table::TableMeta {
    const ID: felt252 = selector!("Foo");
    fn name() -> ByteArray {
        "Foo"
    }
    fn attributes() -> Span<introspect_types::Attribute> {
        [].span()
    }
}


impl FooTablePrimary of introspect_table::table::TablePrimary {
    type Primary = felt252;
    fn primary_def() -> introspect_types::PrimaryDef {
        introspect_table::table::multi_key_primary_def()
    }
}


impl FooTableColumns of introspect_table::table::TableColumns {
    type Column = FooColumn;
    fn columns() -> Span<introspect_types::ColumnDef> {
        [
            introspect_types::ColumnDef {
                id: FooColumns::key_1,
                name: "key_1",
                attributes: [].span(),
                type_def: introspect_types::Introspect::<u128>::type_def(),
            },
            introspect_types::ColumnDef {
                id: FooColumns::key_2,
                name: "key_2",
                attributes: [].span(),
                type_def: introspect_types::Introspect::<ByteArray>::type_def(),
            },
            introspect_types::ColumnDef {
                id: FooColumns::name,
                name: "name",
                attributes: [].span(),
                type_def: introspect_types::Introspect::<ByteArray>::type_def(),
            },
            introspect_types::ColumnDef {
                id: FooColumns::something,
                name: "something",
                attributes: [].span(),
                type_def: introspect_types::Introspect::<u8>::type_def(),
            },
        ]
            .span()
    }
    fn child_defs() -> Array<(felt252, introspect_types::TypeDef)> {
        Default::default()
    }
}

pub impl FooKeySpanToPrimary of introspect_table::table::KeySpanToPrimary<Foo, FooTable> {
    fn key_span_to_primary(self: Span<felt252>) -> felt252 {
        core::poseidon::poseidon_hash_span(self)
    }
}

///// Non Overridable

#[derive(Drop)]
pub enum FooColumn {
    name,
    something,
}

pub mod FooColumns {
    pub const key_1: felt252 = selector!("key_1");
    pub const key_2: felt252 = selector!("key_2");
    pub const name: felt252 = selector!("name");
    pub const something: felt252 = selector!("something");
}

pub impl FooTable =
    introspect_table::table::TableImpl<Foo, FooTableMeta, FooTablePrimary, FooTableColumns>;

pub impl IFooTable = introspect_table::table::ITableImpl<FooTable>;

impl Foo_key_1_MemberImpl =
    introspect_table::table::iserde_table_member::Impl<FooTable, FooColumns::key_1, u128>;

impl Foo_key_2_MemberImpl =
    introspect_table::table::iserde_table_member::Impl<FooTable, FooColumns::key_2, ByteArray>;

pub impl Foo_name_MemberImpl =
    introspect_table::table::iserde_table_member::Impl<FooTable, FooColumns::name, ByteArray>;

pub impl Foo_something_MemberImpl =
    introspect_table::table::iserde_table_member::Impl<FooTable, FooColumns::something, u8>;

impl FooColumnImpl<
    C, impl SS: Snapable<@C, FooColumn>,
> of introspect_table::table::ColumnId<C, FooTable> {
    const fn column_id(self: @C) -> felt252 {
        match SS::snapshot(self) {
            FooColumn::name => FooColumns::name,
            FooColumn::something => FooColumns::something,
        }
    }
}


impl FooRecordKey of introspect_table::table::RecordKey<Foo, (@u128, @ByteArray), FooTable> {
    type Key = (u128, ByteArray);
    fn record_key(self: @Foo) -> (@u128, @ByteArray) {
        (self.key_1, self.key_2)
    }
}

impl FooSerialisedKey<
    KS, K0, K1, +Snapable<@KS, (K0, K1)>, +Snapable<@K0, u128>, +Snapable<@K1, ByteArray>,
> of introspect_table::table::SerialisedKey<FooTable::Record, KS, FooTable> {
    fn serialize_key(self: @KS, ref data: Array<felt252>) {
        let (key_1, key_2) = self.snapshot();
        Foo_key_1_MemberImpl::serialize_member(key_1.snapshot(), ref data);
        Foo_key_2_MemberImpl::serialize_member(key_2.snapshot(), ref data);
    }
}
fn test_fn() {
    let key_1: (u128, ByteArray) = (12, "Key1");
    let mut data: Array<felt252> = Default::default();
    FooSerialisedKey::serialize_key(@key_1, ref data);
}
// impl FooKeySpanTrait<
//     KS,
//     K1,
//     K2,
//     +Snapable<@KS, (K1, K2)>,
//     +Snapable<@K1, u128>,
//     +Snapable<@K2, ByteArray>,
//     +Drop<K1>,
//     +Drop<K2>,
// > of introspect_table::table::TableKeySpanTrait<Foo, KS, FooTable> {
//     fn serialize_keys(self: @KS, ref keys: Array<felt252>) {
//         let (key_1, key_2) = self.snapshot();
//         Foo_key_1_MemberImpl::serialize_member(key_1.snapshot(), ref keys);
//         Foo_key_2_MemberImpl::serialize_member(key_2.snapshot(), ref keys);
//     }
// }

impl FooKeySpanDataSpan of introspect_table::table::RecordValuesSpanTrait<Foo, FooTable> {
    fn serialize_values(self: @Foo, ref data: Array<felt252>) {
        Foo_name_MemberImpl::serialize_member(self.name, ref data);
        Foo_something_MemberImpl::serialize_member(self.something, ref data);
    }
}
