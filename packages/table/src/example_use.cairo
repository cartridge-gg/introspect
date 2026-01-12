use crate::gen_groups::{AColumnGroup, AKeyedColumnGroup, AnIdColumnGroup};
use crate::gen_table_id::{Character, CharacterColumns, CharacterTable, ICharacterTable};
use crate::gen_table_keyed::{Foo, FooColumns, FooTable, IFooTable};
use crate::gen_table_no_id::{IPlayerTable, Player, PlayerColumns, PlayerTable};

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

    IFooTable::insert(foo.clone());
    IFooTable::insert(@foo);
    IFooTable::insert((@key_1.clone(), a_group.clone()));
    IFooTable::insert(@a_keyed_group);
    IFooTable::insert(a_keyed_group.clone());
    IFooTable::insert(@a_id_group);
    IFooTable::insert(a_id_group.clone());
    IFooTable::inserts(@[@foo, @foo_2].span());
    IFooTable::inserts([foo.clone(), foo_2.clone()]);
    IFooTable::inserts([@(12, @a_group)].span());
    IFooTable::inserts([(12, @a_group)]);
    IFooTable::inserts([(@12, a_group.clone())]);
    IFooTable::inserts([a_id_group.clone(), a_id_group.clone()].span());

    IFooTable::insert_field::<{ selector!("name") }>(12, @a_byte_array);
    IFooTable::insert_field::<FooColumns::name>(@12, @a_byte_array);
    IFooTable::insert_field::<FooColumns::name>(12, @a_byte_array);
    IFooTable::insert_field::<FooColumns::name>(key_1.clone(), a_byte_array.clone());
    IFooTable::insert_field::<FooColumns::name>(key_1.clone(), @a_byte_array);
    IFooTable::insert_field::<FooColumns::name>(@key_1, @a_byte_array);
    IFooTable::insert_field::<FooColumns::name>(@key_1, a_byte_array.clone());
    IFooTable::insert_field::<FooColumns::name>(@ss_key_1, a_byte_array.clone());
    IFooTable::insert_field::<FooColumns::name>(ss_key_1, a_byte_array.clone());
    IFooTable::insert_field::<FooColumns::name>(s_key_1.clone(), a_byte_array.clone());
    IFooTable::inserts_field::<FooColumns::name>([(12, @a_byte_array.clone())]);
    IFooTable::inserts_field::<
        FooColumns::name,
    >([(12, a_byte_array.clone()), (12, a_byte_array.clone())]);
    IFooTable::inserts_field::<
        FooColumns::name,
    >([(key_1.clone(), a_byte_array.clone()), (key_2.clone(), a_byte_array.clone())]);

    IFooTable::insert_fields((@key_1.clone(), a_group.clone()));
    IFooTable::insert_fields(@a_keyed_group);
    IFooTable::insert_fields(a_keyed_group.clone());
    IFooTable::insert_fields(@a_id_group);
    IFooTable::insert_fields(a_id_group.clone());
    IFooTable::inserts_fields([@(12, @a_group)].span());
    IFooTable::inserts_fields([(12, @a_group)]);
    IFooTable::inserts_fields([(@12, a_group.clone())]);
    IFooTable::inserts_fields([a_id_group.clone(), a_id_group.clone()].span());

    IFooTable::delete_record(@key_1);
    IFooTable::delete_record(ss_key_1);
    IFooTable::delete_record(ss_key_1);
    IFooTable::delete_record(12);
    IFooTable::delete_records([@1, @2]);
    IFooTable::delete_records([1, 2]);
    IFooTable::delete_records(@[@1, @2]);
    IFooTable::delete_records(@array![@1, @2]);
    IFooTable::delete_records(@array![1, 2]);
    IFooTable::delete_records(array![@1, @2]);
    IFooTable::delete_records(array![1, 2]);
    IFooTable::delete_records([1, 2].span());
    IFooTable::delete_records(@[1, 2].span());
    IFooTable::delete_records([@key_1, @key_2]);
    IFooTable::delete_records([@ss_key_1, @ss_key_2]);
    IFooTable::delete_records([ss_key_1, ss_key_2]);
    IFooTable::delete_field(@key_1, FooColumns::name);
    IFooTable::delete_field(@key_1, FooColumns::name);
    IFooTable::delete_field(@ss_key_1, FooColumns::name);
    IFooTable::delete_field(@key_1, FooColumns::name);
    IFooTable::delete_field(@key_1, FooColumns::name);
    IFooTable::delete_fields(@key_1, [@FooColumns::name]);
    IFooTable::deletes_field([key_1, key_2].span(), @FooColumns::name);
    IFooTable::deletes_fields(
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
    ICharacterTable::insert(character.clone());
    ICharacterTable::insert(@character);
    ICharacterTable::insert((@cid_1, a_group.clone()));
    ICharacterTable::inserts(@[@character, @character].span());
    ICharacterTable::inserts([character.clone(), character.clone()].span());
    ICharacterTable::inserts([(@cid_1, a_group.clone())]);
    ICharacterTable::inserts([(@cid_1, a_group.clone()), (@cid_2, a_group.clone())]);
    ICharacterTable::insert_field::<CharacterColumns::name>(cid_1, a_byte_array.clone());
    ICharacterTable::insert_field::<CharacterColumns::name>(@cid_1, @a_byte_array);

    ICharacterTable::delete_record(1_u128);
    ICharacterTable::delete_record(@cid_1);
    ICharacterTable::delete_record(cid_1);
    ICharacterTable::delete_records([@cid_1, @cid_2]);
    ICharacterTable::delete_records([cid_1, cid_2]);
    ICharacterTable::delete_records(@[@cid_1, @cid_2]);
    ICharacterTable::delete_field(cid_1, CharacterColumns::name);
    ICharacterTable::delete_field(@cid_1, CharacterColumns::name);
    ICharacterTable::delete_field(@cid_1, @CharacterColumns::name);
    ICharacterTable::delete_fields(@cid_1, [@CharacterColumns::name]);
    ICharacterTable::deletes_field([cid_1, cid_2].span(), @CharacterColumns::name);
    ICharacterTable::deletes_fields(
        [cid_1, cid_2].span(), [CharacterColumns::name, CharacterColumns::something],
    );
}

fn test_player() {
    let player = Player { name: "PlayerOne", something: 7, address: 34.try_into().unwrap() };
    let a_byte_array: ByteArray = "NewPlayerName";
    let a_group = AColumnGroup { name: "group_name", something: 42 };
    // IPlayerTable::insert(@(12, player.clone()));
    IPlayerTable::insert((12, player.clone()));
    IPlayerTable::insert((12, @player.clone()));
    IPlayerTable::insert(@(@12, @player));
    IPlayerTable::insert((@12, a_group.clone()));
    IPlayerTable::inserts(@[@(12, @player), @(13, @player)].span());
    IPlayerTable::inserts([(12, player.clone()), (13, player.clone())].span());
    IPlayerTable::inserts([(@12, a_group.clone())]);
    IPlayerTable::inserts([(@12, a_group.clone()), (@13, a_group.clone())]);
    IPlayerTable::insert_field::<PlayerColumns::name>(@12, a_byte_array.clone());
    IPlayerTable::insert_field::<CharacterColumns::name>(12, a_byte_array.clone());
    IPlayerTable::insert_field::<CharacterColumns::name>(@12, @a_byte_array);

    IPlayerTable::delete_record(12);
    IPlayerTable::delete_record(@12);
    IPlayerTable::delete_record(12);
    IPlayerTable::delete_records([@12, @13]);
    IPlayerTable::delete_records([12, 13]);
    IPlayerTable::delete_records(@[@12, @13]);
    IPlayerTable::delete_field(@12, PlayerColumns::name);
    IPlayerTable::delete_field(12, @PlayerColumns::name);
    IPlayerTable::delete_field(@12, PlayerColumns::name);
    IPlayerTable::delete_fields(@12, [@PlayerColumns::name]);
    IPlayerTable::deletes_field([12, 13].span(), @PlayerColumns::name);
    IPlayerTable::deletes_fields([12, 13].span(), [PlayerColumns::name, PlayerColumns::something]);
}
