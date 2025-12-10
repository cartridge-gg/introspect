use core::poseidon;
use introspect::{
    Attribute, ColumnDef, ISerde, PrimaryDef, PrimaryTrait, PrimaryTypeDef, RecordPrimary, Schema,
    TypeDef,
};
use poseidon::poseidon_hash_span;
use crate::table::ITable;

// #[derive(Table)]
#[derive(Drop)]
pub struct Foo {
    #[key]
    pub a_key: u128,
    pub name: ByteArray,
    pub something: ByteArray,
}

#[derive(Drop, Schema)]
pub struct Bar {
    pub name: ByteArray,
    pub something: ByteArray,
}


#[derive(Drop, Table)]
pub struct Foo2 {
    #[key]
    pub key1: felt252,
    #[key]
    pub key2: ByteArray,
    pub name: ByteArray,
    pub something: ByteArray,
}

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
}


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
}


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
}

