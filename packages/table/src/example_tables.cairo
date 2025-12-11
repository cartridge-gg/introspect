use introspect_types::ISerde;
use crate::example_groups::AKeyedColumnGroup;

#[derive(Drop)]
struct Foo {
    #[key]
    key_1: u128,
    #[key]
    key_2: ByteArray,
    name: ByteArray,
    something: u8,
}

//// GENERATED CODE BELOW

mod FooColumns {
    pub const key_1: felt252 = selector!("key_1");
    pub const key_2: felt252 = selector!("key_2");
    pub const name: felt252 = selector!("name");
    pub const something: felt252 = selector!("something");
}

impl FooKeyImpl of introspect_table::table::KeyTrait<
    Foo, (u128, ByteArray), 2, { [FooColumns::key_1, FooColumns::key_2] },
> {}

impl Foo_name_MemberImpl of introspect_table::table::MemberTrait<
    Foo, ByteArray, FooColumns::name,
> {}

impl Foo_something_MemberImpl of introspect_table::table::MemberTrait<
    Foo, u8, FooColumns::something,
> {}


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


impl FooTablePrimary of introspect_table::table::TablePrimary<Foo> {
    type Primary = felt252;
    type Key = (u128, ByteArray);
    fn primary_def() -> introspect_types::PrimaryDef {
        introspect_table::table::multi_key_primary_def()
    }
}


impl FooTableColumns of introspect_table::table::TableColumns<Foo> {
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


// pub trait KeysToTuple<K>

impl FooTablePrimaryOrKey of introspect_table::table::TableIdTrait<(@u128, @ByteArray), FooTable> {
    fn id(self: @(@u128, @ByteArray)) -> felt252 {
        let (key_1, key_2) = self;
        let mut data: Array<felt252> = Default::default();
        introspect_types::ISerde::iserialize(*key_1, ref data);
        introspect_types::ISerde::iserialize(*key_2, ref data);
        core::poseidon::poseidon_hash_span(data.span())
    }
}

impl FooTableSSPrimaryOrKey of introspect_table::table::TableIdTrait<(u128, ByteArray), FooTable> {
    fn id(self: @(u128, ByteArray)) -> felt252 {
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
            FooColumn::name => FooColumns::name,
            FooColumn::something => FooColumns::something,
        }
    }
}

impl FooFieldImpl of introspect_table::table::FieldTrait<FooField> {
    fn column_id(self: @FooField) -> felt252 {
        match self {
            FooField::name(_) => FooColumns::name,
            FooField::something(_) => FooColumns::something,
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
    K, +introspect_table::table::TableIdTrait<K, FooTable>,
> of introspect_table::table::RecordsFieldTrait<FooFields<K>, FooTable> {
    fn column_id(self: @FooFields<K>) -> felt252 {
        match self {
            FooFields::name(_) => FooColumns::name,
            FooFields::something(_) => FooColumns::something,
        }
    }

    fn id_datas(self: @FooFields<K>) -> Span<introspect_types::IdData> {
        match self {
            FooFields::name(span) => introspect_table::table::fields_to_id_datas(span),
            FooFields::something(span) => introspect_table::table::fields_to_id_datas(span),
        }
    }
}

impl FooTableIdDataImpl of introspect_table::table::TableIdDataTrait<Foo, FooTable> {
    fn record_tuple(self: @Foo) -> (felt252, Span<felt252>) {
        let mut data: Array<felt252> = Default::default();
        self.key_1.iserialize(ref data);
        self.key_2.iserialize(ref data);
        let id = core::poseidon::poseidon_hash_span(data.span());
        self.name.iserialize(ref data);
        self.something.iserialize(ref data);
        (id, data.span())
    }
}
use crate::example_groups::AColumnGroup;

#[test]
fn test_fn() {
    let key_1: (u128, ByteArray) = (12, "Key1");
    let key_2: (u128, ByteArray) = (34, "Key2");
    let ss_key_1: (@u128, @ByteArray) = (@12, @"Key1");
    let ss_key_2: (@u128, @ByteArray) = (@34, @"Key2");
    let foo = Foo { key_1: 12, key_2: "Key1", name: "example", something: 8 };
    let foo_2 = Foo { key_1: 34, key_2: "Key2", name: "test", something: 16 };
    let a_group = AColumnGroup { name: "group_name", something: 42 };
    let a_keyed_group = AKeyedColumnGroup {
        key_1: 56, key_2: "keyed_group_key", name: "keyed_group_name",
    };
    let a_byte_array: ByteArray = "example";
    IFooTable::insert((12, @a_group));
    IFooTable::insert(@foo);
    // IFooTable::insert(@a_keyed_group);
    IFooTable::inserts([(12, @a_group)].span());
    IFooTable::inserts([(12, @a_group)]);
    IFooTable::inserts([(@12, a_group)]);
    IFooTable::inserts([foo, foo_2].span());
    IFooTable::insert_field::<FooColumns::name>(@key_1, a_byte_array);
    IFooTable::insert_field::<FooColumns::name>(@key_1, a_byte_array);
    // IFooTable::insert_field::<FooFields::name>(@key_1, @"example");
// IFooTable::insert_field(@key_1, FooField::name(@"example"));
// IFooTable::insert_field(ss_key_1, FooField::name(@"example"));
// IFooTable::insert_field(12, FooTable::Field::name(@"example"));
// IFooTable::inserts_field(FooFields::name([(@key_1, @"example"), (@key_2, @"test")].span()));
// IFooTable::inserts_field(FooFields::name([(ss_key_1, @"example"), (ss_key_2,
// @"test")].span()));
// IFooTable::inserts_field(FooFields::name([(12, @"example"), (34, @"test")].span()));
// IFooTable::insert_fields(
//     @key_1, [FooTable::Field::name(@"example"), FooTable::Field::something(@8)].span(),
// );
// IFooTable::delete_record(@key_1);
// IFooTable::delete_record(ss_key_1);
// IFooTable::delete_record(12);
// IFooTable::delete_field(@key_1, FooColumn::name);
// IFooTable::delete_fields(@key_1, [FooColumn::name, FooColumn::something]);
// IFooTable::deletes_field([key_1, key_2].span(), FooColumn::name);
// IFooTable::deletes_fields([ss_key_1, ss_key_2].span(), [FooColumn::name,
// FooColumn::something]);
// IFooTable::insert_field(12_u8, FooTable::Field::name(@"example"));
// IFooTable::inserts_field(FooFields::name([(12_u8, @"example"), (34_u8, @"test")].span()));
}
