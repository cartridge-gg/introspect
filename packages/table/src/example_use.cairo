use crate::example_groups::{
    AColumnGroup, AColumnGroupTableDataImpl, AKeyedColumnGroup, AnIdColumnGroup,
};
use crate::example_tables::{Foo, FooColumn, FooColumns, IFooTable};
// use crate::table::{
//     ColumnGroupRecordable, EmitRecordableRecordImpl, IdDataTupleImpl,
//     KeySpanToPrimaryTableIdDataImpl, TableKeyIdImpl,
// };

#[test]
fn test_fn() {
    let key_1: (u128, ByteArray) = (12, "Key1");
    let key_2: (u128, ByteArray) = (34, "Key2");
    let ss_key_1: (@u128, @ByteArray) = (@12, @"Key1");
    let ss_key_2: (@u128, @ByteArray) = (@34, @"Key2");
    let s_key_1: (@u128, ByteArray) = (@34, "Key2");
    let foo = Foo { key_1: 12, key_2: "Key1", name: "example", something: 8 };
    let foo_2 = Foo { key_1: 34, key_2: "Key2", name: "test", something: 16 };
    let a_group = AColumnGroup { name: "group_name", something: 42 };
    let a_keyed_group = AKeyedColumnGroup {
        key_1: 56, key_2: "keyed_group_key", name: "keyed_group_name",
    };
    let a_id_group = AnIdColumnGroup { id: 78, something: 99 };
    let a_byte_array: ByteArray = "example";
    IFooTable::insert(@foo);
    // IFooTable::insert((@key_1, @a_group));
    IFooTable::insert(a_keyed_group);
    IFooTable::insert(a_id_group);
    // IFooTable::inserts([(12, @a_group)].span());
    // IFooTable::inserts([(12, @a_group)]);
    // IFooTable::inserts([(@12, a_group)]);
    IFooTable::inserts([foo, foo_2].span());
    IFooTable::insert_field::<{ selector!("name") }>(12, @a_byte_array);
    IFooTable::insert_field::<FooColumns::name>(@12, @a_byte_array);
    IFooTable::insert_field::<FooColumns::name>(12, @a_byte_array);
    IFooTable::insert_field::<FooColumns::name>(key_1.clone(), a_byte_array.clone());
    IFooTable::insert_field::<FooColumns::name>(@key_1, @a_byte_array);
    IFooTable::insert_field::<FooColumns::name>(@key_1, a_byte_array.clone());
    IFooTable::insert_field::<FooColumns::name>(@ss_key_1, a_byte_array.clone());
    IFooTable::insert_field::<FooColumns::name>(ss_key_1.clone(), a_byte_array.clone());
    IFooTable::insert_field::<FooColumns::name>(s_key_1.clone(), a_byte_array.clone());
    // IFooTable::inserts_field::<
    //     FooColumns::name,
    // >([(@key_1, @a_byte_array), (@key_2, @a_byte_array)]);
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
    IFooTable::delete_record(@key_1);
    IFooTable::delete_record(ss_key_1);
    IFooTable::delete_record(12);
    IFooTable::delete_records([@1, @2]);
    IFooTable::delete_records([1, 2].span());
    // IFooTable::delete_field(@key_1, FooColumn::name);
    IFooTable::delete_fields(@key_1, [FooColumns::name]);
    IFooTable::deletes_field([key_1, key_2].span(), FooColumn::name);
    IFooTable::deletes_fields([ss_key_1, ss_key_2].span(), [FooColumn::name, FooColumn::something]);
}
