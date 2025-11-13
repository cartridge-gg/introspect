# Example Framework

Although the tools in the base repository can be used directly to build applications, we provide an example framework that builds on top of them to provide a higher-level interface for defining and managing tables.

This framework works by providing imps for each table which can then be used to declare, and perform operations on the tables.

Tables can either be defined directly using the `Table` macro with one or multiple `#[key]` fields. If a single` #[key]` of primary key type (base type that can be serialised to a single felt) is used then this is treated as the primary key for the table. If multiple `#[key]` fields are used then these are combined to form a composite primary key with the id `__id`.

```rust
#[derive(Drop, Table)]
pub struct Foo {
    #[key]
    pub id: felt252,
    pub name: ByteArray,
    pub something: ByteArray,
}

/// GENERATED CODE
impl FooSchema of Schema<Foo> {
    fn columns() -> Span<ColumnDef> {
        [
            ColumnDef {
                id: selector!("name"),
                name: "name",
                attributes: [].span(),
                type_def: TypeDef::ByteArray,
            },
            ColumnDef {
                id: selector!("something"),
                name: "something",
                attributes: [].span(),
                type_def: TypeDef::ByteArray,
            },
        ]
            .span()
    }

    fn child_defs() -> Array<(felt252, TypeDef)> {
        Default::default()
    }

    fn record_data(self: @Foo) -> Span<felt252> {
        let mut data: Array<felt252> = Default::default();
        self.name.iserialize(ref data);
        self.something.iserialize(ref data);
        data.span()
    }
}

impl FooSchemaWithPrimary of RecordPrimary<Foo> {
    fn primary_def() -> PrimaryDef {
        PrimaryDef { name: "a_key", attributes: [].span(), type_def: PrimaryTypeDef::U128 }
    }

    fn record_id(self: @Foo) -> felt252 {
        self.a_key.to_felt252()
    }
}

pub impl FooTable of ITable<Foo> {
    const SELECTOR: felt252 = selector!("Foo");
    fn name() -> ByteArray {
        "Foo"
    }
    fn attributes() -> Span<Attribute> {
        [].span()
    }
    fn primary() -> PrimaryDef {
        FooSchemaWithPrimary::primary_def()
    }
    fn columns() -> Span<ColumnDef> {
        Schema::<Foo>::columns()
    }
    fn child_defs() -> Array<(felt252, TypeDef)> {
        Schema::<Foo>::child_defs()
    }
}

///-----------------------------
/// Another Table with Composite Key
///
#[derive(Drop, Table)]
pub struct Foo2 {
    #[key]
    pub key1: felt252,
    #[key]
    pub key2: ByteArray,
    pub name: ByteArray,
    pub something: ByteArray,
}

/// GENERATED CODE

impl Foo2Schema of Schema<Foo2> {
    fn columns() -> Span<ColumnDef> {
        [
            ColumnDef {
                id: selector!("key1"),
                name: "key1",
                attributes: [].span(),
                type_def: TypeDef::Felt252,
            },
            ColumnDef {
                id: selector!("key2"),
                name: "key2",
                attributes: [].span(),
                type_def: TypeDef::ByteArray,
            },
            ColumnDef {
                id: selector!("name"),
                name: "name",
                attributes: [].span(),
                type_def: TypeDef::ByteArray,
            },
            ColumnDef {
                id: selector!("something"),
                name: "something",
                attributes: [].span(),
                type_def: TypeDef::ByteArray,
            },
        ]
            .span()
    }

    fn child_defs() -> Array<(felt252, TypeDef)> {
        Default::default()
    }

    fn record_data(self: @Foo2) -> Span<felt252> {
        let mut data: Array<felt252> = Default::default();
        self.key1.iserialize(ref data);
        self.key2.iserialize(ref data);
        self.name.iserialize(ref data);
        self.something.iserialize(ref data);
        data.span()
    }
}

impl Foo2SchemaWithPrimary of RecordPrimary<Foo2> {
    fn primary_def() -> PrimaryDef {
        PrimaryDef { name: "__id", attributes: [].span(), type_def: PrimaryTypeDef::Felt252 }
    }

    fn record_id(self: @Foo2) -> felt252 {
        let mut data: Array<felt252> = Default::default();
        self.key1.iserialize(ref data);
        self.key2.iserialize(ref data);
        poseidon_hash_span(data.span())
    }
}

pub impl Foo2Table of ITable<Foo2> {
    const SELECTOR: felt252 = selector!("Foo2");
    fn name() -> ByteArray {
        "Foo2"
    }
    fn attributes() -> Span<Attribute> {
        [].span()
    }
    fn primary() -> PrimaryDef {
        Foo2SchemaWithPrimary::primary_def()
    }
    fn columns() -> Span<ColumnDef> {
        Schema::<Foo2>::columns()
    }
    fn child_defs() -> Array<(felt252, TypeDef)> {
        Schema::<Foo2>::child_defs()
    }
}
```

A table can also be defined from a schema and a primary key using an inline `table!` macro.

```rust
#[derive(Drop, Schema)]
pub struct Bar {
    pub name: ByteArray,
    pub something: ByteArray,
}
table!(Bar, {primary_type: u128, primary_name: "id", impl_name: BarTable, table_name: "Bar"});

/// GENERATED CODE
pub impl BarTable of ITable<(felt252, @Bar)> {
    const SELECTOR: felt252 = selector!("Bar");
    fn name() -> ByteArray {
        "Bar"
    }
    fn attributes() -> Span<Attribute> {
        [].span()
    }
    fn primary() -> PrimaryDef {
        PrimaryDef { name: "id", attributes: [].span(), type_def: PrimaryTypeDef::Felt252 }
    }
    fn columns() -> Span<ColumnDef> {
        Schema::<Bar>::columns()
    }
    fn child_defs() -> Array<(felt252, TypeDef)> {
        Schema::<Bar>::child_defs()
    }
}
```

These table can then be used like in the example contract below:

```rust
#[starknet::contract]
mod example_contract {
    use crate::example_gen::{Bar, BarTable, Foo, Foo2Table, FooTable};
    #[storage]
    struct Storage {}


    #[constructor]
    fn constructor(ref self: ContractState) {
        FooTable::register_table();
        Foo2Table::register_table();
        BarTable::register_table();
    }

    #[external(v0)]
    fn update_foo(ref self: ContractState, a_key: u128, name: ByteArray, something: ByteArray) {
        let foo = Foo { a_key, name, something };
        FooTable::insert_record(@foo);
    }

    #[external(v0)]
    fn update_bar(ref self: ContractState, id: felt252, name: ByteArray, something: ByteArray) {
        let bar = Bar { name, something };
        BarTable::insert_record(@(id, @bar));
    }
}
```
