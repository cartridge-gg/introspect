#[derive(Drop)]
pub struct Foo {
    #[key]
    pub key_1: u128,
    #[key]
    pub key_2: ByteArray,
    pub name: ByteArray,
    pub something: u8,
}

//// GENERATED CODE BELOW

pub mod FooColumns {
    pub const key_1: felt252 = selector!("key_1");
    pub const key_2: felt252 = selector!("key_2");
    pub const name: felt252 = selector!("name");
    pub const something: felt252 = selector!("something");
}

// impl FooKeyImpl of introspect_table::table::KeyTrait<
//     Foo, (u128, ByteArray), 2, { [FooColumns::key_1, FooColumns::key_2] },
// > {}

impl Foo_key_1_MemberImpl of introspect_table::table::MemberTrait<
    FooTable, u128, FooColumns::key_1,
> {
    fn serialize_member(self: @u128, ref data: Array<felt252>) {
        introspect_types::ISerde::iserialize(self, ref data);
    }
}

impl Foo_key_2_MemberImpl of introspect_table::table::MemberTrait<
    FooTable, ByteArray, FooColumns::key_2,
> {
    fn serialize_member(self: @ByteArray, ref data: Array<felt252>) {
        introspect_types::ISerde::iserialize(self, ref data);
    }
}

pub impl Foo_name_MemberImpl of introspect_table::table::MemberTrait<
    FooTable, ByteArray, FooColumns::name,
> {
    fn serialize_member(self: @ByteArray, ref data: Array<felt252>) {
        introspect_types::ISerde::iserialize(self, ref data);
    }
}

pub impl Foo_something_MemberImpl of introspect_table::table::MemberTrait<
    FooTable, u8, FooColumns::something,
> {
    fn serialize_member(self: @u8, ref data: Array<felt252>) {
        introspect_types::ISerde::iserialize(self, ref data);
    }
}


#[derive(Drop)]
pub enum FooColumn {
    name,
    something,
}

#[derive(Drop)]
pub enum FooField {
    name: @ByteArray,
    something: @u8,
}

#[derive(Drop)]
pub enum FooFields<K> {
    name: Span<(K, @ByteArray)>,
    something: Span<(K, @u8)>,
}

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
    type Field = FooField;
    fn columns() -> Span<introspect_types::ColumnDef> {
        [
            introspect_types::ColumnDef {
                id: FooColumns::key_1,
                name: "key_1",
                attributes: [].span(),
                type_def: introspect_types::TypeDef::U128,
            },
            introspect_types::ColumnDef {
                id: FooColumns::key_2,
                name: "key_2",
                attributes: [].span(),
                type_def: introspect_types::TypeDef::ByteArray,
            },
            introspect_types::ColumnDef {
                id: FooColumns::name,
                name: "name",
                attributes: [].span(),
                type_def: introspect_types::TypeDef::ByteArray,
            },
            introspect_types::ColumnDef {
                id: FooColumns::something,
                name: "something",
                attributes: [].span(),
                type_def: introspect_types::TypeDef::U8,
            },
        ]
            .span()
    }
    fn child_defs() -> Array<(felt252, introspect_types::TypeDef)> {
        Default::default()
    }
}


pub impl FooTable =
    introspect_table::table::TableImpl<Foo, FooTableMeta, FooTablePrimary, FooTableColumns>;

pub impl IFooTable = introspect_table::table::ITableImpl<FooTable>;


impl FooColumnImpl of introspect_table::table::ColumnTrait<FooColumn> {
    fn column_id(self: @FooColumn) -> felt252 {
        match self {
            FooColumn::name => FooColumns::name,
            FooColumn::something => FooColumns::something,
        }
    }
}

pub impl FooKeySpanTrait of introspect_table::table::TableKeySpanTrait<
    (u128, ByteArray), FooTable,
> {
    fn serialize_keys(self: @(u128, ByteArray), ref keys: Array<felt252>) {
        let (key_1, key_2) = self;
        Foo_key_1_MemberImpl::serialize_member(key_1, ref keys);
        Foo_key_2_MemberImpl::serialize_member(key_2, ref keys);
    }
}

pub impl FooKeySpanDataSpan of introspect_table::table::KeySpanDataSpanTrait<Foo, FooTable> {
    fn serialize_keys(self: @Foo, ref keys: Array<felt252>) {
        Foo_key_1_MemberImpl::serialize_member(self.key_1, ref keys);
        Foo_key_2_MemberImpl::serialize_member(self.key_2, ref keys);
    }
    fn serialize_values(self: @Foo, ref values: Array<felt252>) {
        Foo_name_MemberImpl::serialize_member(self.name, ref values);
        Foo_something_MemberImpl::serialize_member(self.something, ref values);
    }
}


pub impl FooKeySpanToPrimaryImpl of introspect_table::table::KeySpanToPrimary<FooTable> {
    fn key_span_to_primary(self: Span<felt252>) -> felt252 {
        core::poseidon::poseidon_hash_span(self)
    }
}

pub mod _Foo_ {
    pub use super::{
        FooKeySpanToPrimaryImpl, FooKeySpanTrait, FooTable, Foo_key_1_MemberImpl,
        Foo_key_2_MemberImpl, Foo_name_MemberImpl, Foo_something_MemberImpl,
    };
}
