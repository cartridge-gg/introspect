use core::poseidon;
use poseidon::poseidon_hash_span;
use crate::{Attribute, ISerde, Introspect, PrimaryDef, TypeDef};

#[derive(Drop, Serde, PartialEq, Debug)]
pub struct ColumnDef {
    pub id: felt252,
    pub name: ByteArray,
    pub attributes: Span<Attribute>,
    pub type_def: TypeDef,
}


pub trait Schema<T> {
    fn columns() -> Span<ColumnDef>;
    fn child_defs() -> Array<(felt252, TypeDef)>;
    fn hash() -> felt252 {
        let mut data: Array<felt252> = Default::default();
        let columns = Self::columns();
        columns.serialize(ref data);
        poseidon_hash_span(data.span())
    }
    fn column_ids() -> Span<
        felt252,
    > {
        Self::columns().into_iter().map(|col| *col.id).collect::<Array<felt252>>().span()
    }
    fn group_id() -> felt252 {
        poseidon_hash_span(Self::column_ids())
    }
    fn record_data(self: @T) -> Span<felt252>;
    // > {
//     let mut data: Array<felt252> = Default::default();
//     for id in Self::column_ids() {
//         Self::_field_data(self, *id, ref data);
//     }
//     data.span()
// }
// fn field_data<const FIELD: felt252>(
//     self: @T,
// ) -> Span<
//     felt252,
// > {
//     let mut data: Array<felt252> = Default::default();
//     Self::_field_data(self, FIELD, ref data);
//     data.span()
// }
// fn fields_data<const FIELDS: [felt252; N], const N: usize>(
//     self: @T,
// ) -> Span<
//     felt252,
// > {
//     let mut data: Array<felt252> = Default::default();
//     for field in BoxTrait::new(@FIELDS).span() {
//         Self::_field_data(self, *field, ref data);
//     }
//     data.span()
// }
// fn _field_data(self: @T, field: felt252, ref data: Array<felt252>);
}

#[generate_trait]
pub impl ColumnDefImpl of ColumnDefTrait {
    fn new<T, +Introspect<T>>(
        id: felt252, name: ByteArray, attributes: Span<Attribute>,
    ) -> ColumnDef {
        ColumnDef { id, name, attributes, type_def: Introspect::<T>::type_def() }
    }
}


pub trait RecordPrimary<T> {
    fn primary_def() -> PrimaryDef;
    fn record_id(self: @T) -> felt252;
}


impl ColumnDefISerde of ISerde<ColumnDef> {
    fn iserialize(self: @ColumnDef, ref output: Array<felt252>) {
        output.append(*self.id);
        self.name.iserialize(ref output);
        self.attributes.iserialize(ref output);
        self.type_def.iserialize(ref output);
    }
    fn ideserialize(ref serialized: Span<felt252>) -> Option<ColumnDef> {
        let id = *serialized.pop_front()?;
        let name = ISerde::ideserialize(ref serialized)?;
        let attributes = ISerde::ideserialize(ref serialized)?;
        let type_def = ISerde::ideserialize(ref serialized)?;
        Some(ColumnDef { id, name, attributes, type_def })
    }
}


pub fn column_def(
    id: felt252, name: ByteArray, attributes: Span<Attribute>, type_def: TypeDef,
) -> ColumnDef {
    ColumnDef {
        id,
        name,
        attributes,
        type_def,
    }
} 