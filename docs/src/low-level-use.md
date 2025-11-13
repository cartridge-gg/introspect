# Low level use

The basic flow of this library is to attach the derive `Introspect` macro to your structs or enums you wish to be indexed (variables and any member types of schemas). This will automatically generate the Introspect and ISerde traits for your types.

> It is expected that wrappers around this will be made to make it easier to use in specific contexts such as databases and events.

```rust
#[derive(Drop, Introspect, starknet::Store)]
enum Element {
    #[default]
    None,
    Air: u8,
    Fire: u8,
    Earth: u8,
    Water: u8,
}

#[derive(Drop, Introspect, starknet::Store)]
enum Material {
    #[default]
    Cloth,
    Leather,
    Iron,
    Steel,
    Mythril,
    Elemental: Element,
}

#[derive(Drop, Introspect, starknet::Store)]
struct ArmourPiece {
    experience: u32,
    wear: u8,
    material: Material,
}


#[derive(Drop, Introspect)]
enum WeaponType {
    Sword,
    Axe,
    Bow,
}


#[derive(Drop, Schema)]
struct Weapon {
    name: ByteArray,
    level: u16,
    material: Material,
    weapon_type: WeaponType,
}


#[derive(Drop, Introspect)]
struct ArmourSet {
    head: ArmourPiece,
    chest: ArmourPiece,
    legs: ArmourPiece,
    gloves: ArmourPiece,
    boots: ArmourPiece,
}
```

There is also the `IntrospectRef` macro for types that are large or are used in multiple places to avoid duplication of the type definitions. This also helps with the limitation of event data size of 300 felts. You can split large types into smaller referenced types which can each be emitted as there own event. In the below example the `Element`, `Material` and `ArmourPiece` types are defined with `IntrospectRef` so that they can be referenced in other types without duplicating their type definitions. This means when another type uses them e.g. `ArmourSet` the type definition for `ArmourPiece` is not duplicated but referenced instead using only 2 felts instead of the full type definition.

```rust
#[derive(Drop, IntrospectRef, starknet::Store)]
enum Element {
    #[default]
    None,
    Air: u8,
    Fire: u8,
    Earth: u8,
    Water: u8,
}

#[derive(Drop, IntrospectRef, starknet::Store)]
enum Material {
    #[default]
    Cloth,
    Leather,
    Iron,
    Steel,
    Mythril,
    Elemental: Element,
}

#[derive(Drop, IntrospectRef, starknet::Store)]
struct ArmourPiece {
    experience: u32,
    wear: u8,
    material: Material,
}


#[derive(Drop, IntrospectRef)]
enum WeaponType {
    Sword,
    Axe,
    Bow,
}


#[derive(Drop, Schema)]
struct Weapon {
    name: ByteArray,
    level: u16,
    material: Material,
    weapon_type: WeaponType,
}


#[derive(Drop, Introspect)]
struct ArmourSet {
    head: ArmourPiece,
    chest: ArmourPiece,
    legs: ArmourPiece,
    gloves: ArmourPiece,
    boots: ArmourPiece,
}
```

For types that are to be used as schemas for databases the `Schema` macro is used:

```rust
#[derive(Drop, Schema)]
struct Warrior {
    name: ByteArray,
    level: u8,
    health: u16,
    armour: ArmourSet,
    gold: u128,
    alive: bool,
}

#[derive(Drop, Schema)]
struct MapPosition {
    x: u16,
    y: u16,
    terrain: Terrain,
    warrior: Option<felt252>,
}
```

To create a database the emitter can be used:

```rust
use introspect::events::emit_create_table_with_columns;

#[constructor]
fn constructor(){
    emit_create_table_with_columns(
        selector!("Warrior"),
        "Warrior",
        [].span(),
        PrimaryDef{
            name: "id",
            attributes: [].span(),
            type_def: TypeDef::Felt252,
        }
        Schema::<Warrior>::columns(),
    );
    emit_declare_variable(selector!("Weather"), "Weather", Schema::<Weather>::type_def());
}
...
    fn new_warrior(self: ContractState, id: felt252, name: ByteArray, level: u8, health: u16, armour: ArmourSet, gold: u128, alive: bool){
        emit_insert_row(
            selector!("Warrior"),
            id,
            Warrior {
                name,
                level,
                health,
                armour,
                gold,
                alive,
            },
        );
    }


    fn update_weather(self: ContractState, wind: u8, temperature: i8){
        emit_set_variable(
            selector!("Weather"),
            Weather {
                wind,
                temperature,
            },
        );
    }

    fn buy_health_potion(self: ContractState, warrior_id: felt252){
        let mut gold = self.get_warrior_gold(warrior_id);
        gold -= self.health_potion_price.read();
        self.set_warrior_gold(warrior_id, gold);
        let mut health = self.get_warrior_health(warrior_id);
        health += 50;
        self.set_warrior_health(warrior_id, health);
        let (id, data) = (warrior_id, (gold, health)).id_data_tuple();
        emit_inset_fields(
            selector!("Warrior"), [selector!("gold") selector!("health")].span() id, data);
    }
...

```

Different frameworks can make these nicer to work with e.g. a Dojo style engine could implement macros to automatically generate the events and impls according to their specifications.
