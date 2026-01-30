use crate::introspect::TypeDefTrait;
use crate::{Attribute, ISerde, TypeDef};

#[derive(Drop, Serde, PartialEq, Debug)]
pub struct ColumnDef {
    pub id: felt252,
    pub name: ByteArray,
    pub attributes: Span<Attribute>,
    pub type_def: TypeDef,
}


pub trait Column {
    impl TypeDef: TypeDefTrait;
    const ID: felt252;
    const NAME_SIZE: u32;
    const NAME: [felt252; Self::NAME_SIZE];
    const ATTRIBUTES_SIZE: u32;
    const ATTRIBUTES_COUNT: u32;
    const ATTRIBUTES: [felt252; Self::ATTRIBUTES_SIZE];
    fn serialize_column_def(
        ref output: Array<felt252>,
    ) {
        Self::serialize_column_id(ref output);
        Self::serialize_column_name(ref output);
        Self::serialize_column_attributes(ref output);
        Self::serialize_column_type_def(ref output);
    }
    fn serialize_column_id(ref output: Array<felt252>) {
        output.append(Self::ID);
    }
    fn serialize_column_name<impl NameToSpan: ToSpanTrait<[felt252; Self::NAME_SIZE], felt252>>(
        ref output: Array<felt252>,
    ) {
        output.append_span(NameToSpan::span(@Self::NAME));
    }
    fn serialize_column_attributes<
        impl AttrsToSpan: ToSpanTrait<[felt252; Self::ATTRIBUTES_SIZE], felt252>,
    >(
        ref output: Array<felt252>,
    ) {
        output.append(Self::ATTRIBUTES_COUNT.into());
        output.append_span(AttrsToSpan::span(@Self::ATTRIBUTES));
    }
    fn serialize_column_type_def(
        ref output: Array<felt252>,
    ) {
        Self::TypeDef::serialize_type_def(ref output);
    }
}

impl ColumnDefISerde of ISerde<ColumnDef> {
    // const SIZE_HINT: Option<u32> = None;
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
