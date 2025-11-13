use introspect::{
    Attribute, ColumnDef, ISerde, PrimaryDef, PrimaryTypeDef, RecordPrimary, Schema, TypeDef,
};
use crate::table::ITable;

// #[derive(Table)]
#[derive(Drop)]
pub struct Foo {
    #[key]
    pub id: felt252,
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
        PrimaryDef { name: "__id", attributes: [].span(), type_def: PrimaryTypeDef::Felt252 }
    }

    fn record_id(self: @Foo) -> felt252 {
        *self.id
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

#[derive(Drop, Schema)]
pub struct Bar {
    pub name: ByteArray,
    pub something: ByteArray,
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
    fn columns() -> Span<ColumnDef> {
        Schema::<Bar>::columns()
    }
    fn child_defs() -> Array<(felt252, TypeDef)> {
        Schema::<Bar>::child_defs()
    }
}

