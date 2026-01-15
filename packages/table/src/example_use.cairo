use crate::gen_groups::{AColumnGroup, AKeyedColumnGroup, AnIdColumnGroup};
use crate::gen_table_id::{Character, CharacterColumns, CharacterTable, CharacterTable2};
use crate::gen_table_keyed::{Foo, FooColumns, FooTable};
use crate::gen_table_no_id::{Player, PlayerColumns, PlayerTable};

fn test_foo() {
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

    FooTable::insert(foo.clone());
    FooTable::insert(@foo);
    FooTable::insert((@key_1.clone(), a_group.clone()));
    FooTable::insert(@a_keyed_group);
    FooTable::insert(a_keyed_group.clone());
    FooTable::insert(@a_id_group);
    FooTable::insert(a_id_group.clone());
    FooTable::inserts(@[@foo, @foo_2].span());
    FooTable::inserts([foo.clone(), foo_2.clone()]);
    FooTable::inserts([@(12, @a_group)].span());
    FooTable::inserts([(12, @a_group)]);
    FooTable::inserts([(@12, a_group.clone())]);
    FooTable::inserts([a_id_group.clone(), a_id_group.clone()].span());

    FooTable::insert_field::<{ selector!("name") }>(12, @a_byte_array);
    FooTable::insert_field::<FooColumns::name>(@12, @a_byte_array);
    FooTable::insert_field::<FooColumns::name>(12, @a_byte_array);
    FooTable::insert_field::<FooColumns::name>(key_1.clone(), a_byte_array.clone());
    FooTable::insert_field::<FooColumns::name>(key_1.clone(), @a_byte_array);
    FooTable::insert_field::<FooColumns::name>(@key_1, @a_byte_array);
    FooTable::insert_field::<FooColumns::name>(@key_1, a_byte_array.clone());
    FooTable::insert_field::<FooColumns::name>(@ss_key_1, a_byte_array.clone());
    FooTable::insert_field::<FooColumns::name>(ss_key_1, a_byte_array.clone());
    FooTable::insert_field::<FooColumns::name>(s_key_1.clone(), a_byte_array.clone());
    FooTable::inserts_field::<FooColumns::name>([(12, @a_byte_array.clone())]);
    FooTable::inserts_field::<
        FooColumns::name,
    >([(12, a_byte_array.clone()), (12, a_byte_array.clone())]);
    FooTable::inserts_field::<
        FooColumns::name,
    >([(key_1.clone(), a_byte_array.clone()), (key_2.clone(), a_byte_array.clone())]);

    FooTable::insert_fields((@key_1.clone(), a_group.clone()));
    FooTable::insert_fields(@a_keyed_group);
    FooTable::insert_fields(a_keyed_group.clone());
    FooTable::insert_fields(@a_id_group);
    FooTable::insert_fields(a_id_group.clone());
    FooTable::inserts_fields([@(12, @a_group)].span());
    FooTable::inserts_fields([(12, @a_group)]);
    FooTable::inserts_fields([(@12, a_group.clone())]);
    FooTable::inserts_fields([a_id_group.clone(), a_id_group.clone()].span());

    FooTable::delete_record(@key_1);
    FooTable::delete_record(ss_key_1);
    FooTable::delete_record(ss_key_1);
    FooTable::delete_record(12);
    FooTable::delete_records([@1, @2]);
    FooTable::delete_records([1, 2]);
    FooTable::delete_records(@[@1, @2]);
    FooTable::delete_records(@array![@1, @2]);
    FooTable::delete_records(@array![1, 2]);
    FooTable::delete_records(array![@1, @2]);
    FooTable::delete_records(array![1, 2]);
    FooTable::delete_records([1, 2].span());
    FooTable::delete_records(@[1, 2].span());
    FooTable::delete_records([@key_1, @key_2]);
    FooTable::delete_records([@ss_key_1, @ss_key_2]);
    FooTable::delete_records([ss_key_1, ss_key_2]);
    FooTable::delete_field(@key_1, FooColumns::name);
    FooTable::delete_field(@key_1, FooColumns::name);
    FooTable::delete_field(@ss_key_1, FooColumns::name);
    FooTable::delete_field(@key_1, FooColumns::name);
    FooTable::delete_field(@key_1, FooColumns::name);
    FooTable::delete_fields(@key_1, [@FooColumns::name]);
    FooTable::deletes_field([key_1, key_2].span(), @FooColumns::name);
    FooTable::deletes_fields(
        [ss_key_1, ss_key_2].span(), [FooColumns::name, FooColumns::something],
    );
}

fn test_character() {
    let character = Character {
        cid: 1, name: "John Doe", something: 42, player: 12.try_into().unwrap(),
    };
    let a_byte_array: ByteArray = "NewHero";
    let a_group = AColumnGroup { name: "group_name", something: 42 };

    let cid_1: u128 = 1;
    let cid_2: u128 = 1;
    CharacterTable::insert(character.clone());
    CharacterTable2::insert(@character);
    CharacterTable::insert((@cid_1, a_group.clone()));
    CharacterTable::inserts(@[@character, @character].span());
    CharacterTable::inserts([character.clone(), character.clone()].span());
    CharacterTable::inserts([(@cid_1, a_group.clone())]);
    CharacterTable::inserts([(@cid_1, a_group.clone()), (@cid_2, a_group.clone())]);
    CharacterTable2::insert_field::<CharacterColumns::name>(cid_1, a_byte_array.clone());
    CharacterTable2::insert_field::<CharacterColumns::name>(@cid_1, @a_byte_array);

    CharacterTable2::delete_record(1_u128);
    CharacterTable2::delete_record(@cid_1);
    CharacterTable2::delete_record(cid_1);
    CharacterTable::delete_records([@cid_1, @cid_2]);
    CharacterTable::delete_records([cid_1, cid_2]);
    CharacterTable::delete_records(@[@cid_1, @cid_2]);
    CharacterTable::delete_field(cid_1, CharacterColumns::name);
    CharacterTable::delete_field(@cid_1, CharacterColumns::name);
    CharacterTable::delete_field(@cid_1, @CharacterColumns::name);
    CharacterTable::delete_fields(@cid_1, [@CharacterColumns::name]);
    CharacterTable::deletes_field([cid_1, cid_2].span(), @CharacterColumns::name);
    CharacterTable::deletes_fields(
        [cid_1, cid_2].span(), [CharacterColumns::name, CharacterColumns::something],
    );
}

fn test_player() {
    let player = Player { name: "PlayerOne", something: 7, address: 34.try_into().unwrap() };
    let a_byte_array: ByteArray = "NewPlayerName";
    let a_group = AColumnGroup { name: "group_name", something: 42 };
    PlayerTable::insert(@(12, player.clone()));
    PlayerTable::insert((12, player.clone()));
    PlayerTable::insert((12, @player.clone()));
    PlayerTable::insert(@(@12, @player));
    PlayerTable::insert((@12, a_group.clone()));
    PlayerTable::inserts(@[@(12, @player), @(13, @player)].span());
    PlayerTable::inserts([(12, player.clone()), (13, player.clone())].span());
    PlayerTable::inserts([(@12, a_group.clone())]);
    PlayerTable::inserts([(@12, a_group.clone()), (@13, a_group.clone())]);
    PlayerTable::insert_field::<PlayerColumns::name>(@12, a_byte_array.clone());
    PlayerTable::insert_field::<CharacterColumns::name>(12, a_byte_array.clone());
    PlayerTable::insert_field::<CharacterColumns::name>(@12, @a_byte_array);

    PlayerTable::delete_record(12);
    PlayerTable::delete_record(@12);
    PlayerTable::delete_record(12);
    PlayerTable::delete_records([@12, @13]);
    PlayerTable::delete_records([12, 13]);
    PlayerTable::delete_records(@[@12, @13]);
    PlayerTable::delete_field(@12, PlayerColumns::name);
    PlayerTable::delete_field(12, @PlayerColumns::name);
    PlayerTable::delete_field(@12, PlayerColumns::name);
    PlayerTable::delete_fields(@12, [@PlayerColumns::name]);
    PlayerTable::deletes_field([12, 13].span(), @PlayerColumns::name);
    PlayerTable::deletes_fields([12, 13].span(), [PlayerColumns::name, PlayerColumns::something]);
}
