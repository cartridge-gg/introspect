# Basic use

The basic flow of this library is to attach the derive `Introspect` macro to your structs or enums you wish to be indexed (variables and any member types of schemas). This will automatically generate the Introspect and ISerde traits for your types.

```rust
#[derive(Drop, Introspect)]
enum ArmourEnchantment {
    Fire,
    Ice,
    Lightning,
}

#[derive(Drop, Introspect)]
struct ArmourPiece {
    experience: u32,
    wear: u8,
    enchantments: Array<ArmourEnchantment>,
}

#[derive(Drop, Introspect)]
enum Armour {
    None,
    Cloth,
    Hide: ArmourPiece,
    Metal: ArmourPiece,
}

#[derive(Drop, Introspect)]
struct ArmourSet {
    head: Armour,
    chest: Armour,
    legs: Armour,
    gloves: Armour,
    boots: Armour,
}

#[derive(Drop, Introspect)]
struct Weather {
    wind: u8,
    temperature: i8,
}

#[derive(Drop, Introspect)]
struct Water {
    depth: u8,
    current_speed: u8,
    fish: bool,
}

#[derive(Drop, Introspect)]
struct Mountain {
    height: u32,
    trolls: u8,
}

#[derive(Drop, Introspect)]
enum Terrain {
    Grass,
    Water: Water,
    Mountain: Mountain,
    Desert,
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
    warrior: Opti\on<felt252>,
}
```
