use core::poseidon;
use poseidon::poseidon_hash_span;
use crate::{Attribute, ISerde, Introspect, PrimaryDef, TypeDef};


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
