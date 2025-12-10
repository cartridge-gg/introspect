use introspect_types::ISerde;
use crate::table::fields_to_id_datas;


#[derive(Drop, Table)]
struct Foo {
    #[key]
    key_1: u128,
    #[key]
    key_2: ByteArray,
    name: ByteArray,
    something: u8,
}


#[derive(Drop, Table)]
struct Foo2 {
    #[key]
    key_1: u128,
    name: ByteArray,
    something: u8,
}


struct TokenData {
    #[key]
    token_address: ContractAddress,
    #[key]
    token_id: u256,
    traits: Something,
    points: u32,
    games: u16,
}

//// GENERATED CODE BELOW

mod Foo_column_selectors {
    pub const key_1: felt252 = selector!("key_1");
    pub const key_2: felt252 = selector!("key_2");
    pub const name: felt252 = selector!("name");
    pub const something: felt252 = selector!("something");
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


impl FooTableColumns of introspect_table::table::TableColumns<Foo> {
    type Column = FooColumn;
    type Field = FooField;
    fn columns() -> Span<introspect_types::ColumnDef> {
        [
            introspect_types::ColumnDef {
                id: Foo_column_selectors::key_1,
                name: "key_1",
                attributes: [].span(),
                type_def: introspect_types::TypeDef::U128,
            },
            introspect_types::ColumnDef {
                id: Foo_column_selectors::key_2,
                name: "key_2",
                attributes: [].span(),
                type_def: introspect_types::TypeDef::ByteArray,
            },
            introspect_types::ColumnDef {
                id: Foo_column_selectors::name,
                name: "name",
                attributes: [].span(),
                type_def: introspect_types::TypeDef::ByteArray,
            },
            introspect_types::ColumnDef {
                id: Foo_column_selectors::something,
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
    fn record_data(self: @Foo) -> Span<felt252> {
        let mut data: Array<felt252> = Default::default();
        introspect_types::ISerde::iserialize(self.key_1, ref data);
        introspect_types::ISerde::iserialize(self.key_2, ref data);
        introspect_types::ISerde::iserialize(self.name, ref data);
        introspect_types::ISerde::iserialize(self.something, ref data);
        data.span()
    }
}

impl FooTablePrimary of introspect_table::table::TablePrimary<Foo> {
    type Primary = felt252;
    type Key = (u128, ByteArray);
    fn record_id(self: @Foo) -> felt252 {
        let mut data: Array<felt252> = Default::default();
        self.key_1.iserialize(ref data);
        self.key_2.iserialize(ref data);
        core::poseidon::poseidon_hash_span(data.span())
    }
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


pub impl FooTable =
    introspect_table::table::TableImpl<Foo, FooTableMeta, FooTablePrimary, FooTableColumns>;

pub impl IFooTable = introspect_table::table::ITableImpl<FooTable>;


impl FooTablePrimaryOrKey of introspect_table::table::TablePrimaryOrKey<
    (@u128, @ByteArray), FooTable,
> {
    fn to_id(self: @(@u128, @ByteArray)) -> felt252 {
        let (key_1, key_2) = self;
        let mut data: Array<felt252> = Default::default();
        introspect_types::ISerde::iserialize(*key_1, ref data);
        introspect_types::ISerde::iserialize(*key_2, ref data);
        core::poseidon::poseidon_hash_span(data.span())
    }
}

impl FooTableSSPrimaryOrKey of introspect_table::table::TablePrimaryOrKey<
    (u128, ByteArray), FooTable,
> {
    fn to_id(self: @(u128, ByteArray)) -> felt252 {
        let (key_1, key_2) = self;
        let mut data: Array<felt252> = Default::default();
        introspect_types::ISerde::iserialize(key_1, ref data);
        introspect_types::ISerde::iserialize(key_2, ref data);
        core::poseidon::poseidon_hash_span(data.span())
    }
}


impl FooColumnImpl of introspect_table::table::ColumnTrait<FooColumn> {
    fn column_id(self: @FooColumn) -> felt252 {
        match self {
            FooColumn::name => Foo_column_selectors::name,
            FooColumn::something => Foo_column_selectors::something,
        }
    }
}

impl FooFieldImpl of introspect_table::table::FieldTrait<FooField> {
    fn column_id(self: @FooField) -> felt252 {
        match self {
            FooField::name(_) => Foo_column_selectors::name,
            FooField::something(_) => Foo_column_selectors::something,
        }
    }

    fn data(self: @FooField) -> Span<felt252> {
        match self {
            FooField::name(value) => value.iserialize_inline(),
            FooField::something(value) => value.iserialize_inline(),
        }
    }
}

impl FooKeyedFieldsTrait<
    K, +introspect_table::table::TablePrimaryOrKey<K, FooTable>,
> of introspect_table::table::FieldsTrait<FooFields<K>, FooTable> {
    fn column_id(self: @FooFields<K>) -> felt252 {
        match self {
            FooFields::name(_) => Foo_column_selectors::name,
            FooFields::something(_) => Foo_column_selectors::something,
        }
    }

    fn id_datas(self: @FooFields<K>) -> Span<introspect_types::IdData> {
        match self {
            FooFields::name(span) => introspect_table::table::fields_to_id_datas(span),
            FooFields::something(span) => introspect_table::table::fields_to_id_datas(span),
        }
    }
}


fn test_fn() {
    let key_1: (u128, ByteArray) = (12, "Key1");
    let key_2: (u128, ByteArray) = (34, "Key2");
    let ss_key_1: (@u128, @ByteArray) = (@12, @"Key1");
    let ss_key_2: (@u128, @ByteArray) = (@34, @"Key2");
    let foo = Foo { key_1: 12, key_2: "Key1", name: "example", something: 8 };
    let foo_2 = Foo { key_1: 34, key_2: "Key2", name: "test", something: 16 };
    IFooTable::insert_record(@foo);
    IFooTable::insert_records([@foo, @foo_2].span());
    IFooTable::insert_field(@key_1, FooField::name(@"example"));
    IFooTable::insert_field(ss_key_1, FooField::name(@"example"));
    IFooTable::insert_field(12, FooTable::Field::name(@"example"));
    IFooTable::inserts_field(FooFields::name([(@key_1, @"example"), (@key_2, @"test")].span()));
    IFooTable::inserts_field(FooFields::name([(ss_key_1, @"example"), (ss_key_2, @"test")].span()));
    IFooTable::inserts_field(FooFields::name([(12, @"example"), (34, @"test")].span()));
    IFooTable::insert_fields(
        @key_1, [FooTable::Field::name(@"example"), FooTable::Field::something(@8)].span(),
    );
    IFooTable::delete_record(@key_1);
    IFooTable::delete_record(ss_key_1);
    IFooTable::delete_record(12);
    IFooTable::delete_field(@key_1, FooColumn::name);
    IFooTable::delete_fields(@key_1, [FooColumn::name, FooColumn::something]);
    IFooTable::deletes_field([key_1, key_2].span(), FooColumn::name);
    IFooTable::deletes_fields([ss_key_1, ss_key_2].span(), [FooColumn::name, FooColumn::something]);
    // IFooTable::insert_field(12_u8, FooTable::Field::name(@"example"));
// IFooTable::inserts_field(FooFields::name([(12_u8, @"example"), (34_u8, @"test")].span()));
}
