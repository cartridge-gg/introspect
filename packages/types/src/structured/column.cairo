use crate::serialized::TypeDefTrait;
use crate::{Attribute, ISerde, TypeDef};

#[derive(Drop, Serde, PartialEq, Debug)]
pub struct ColumnDef {
    pub id: felt252,
    pub name: ByteArray,
    pub attributes: Span<Attribute>,
    pub type_def: TypeDef,
}

impl ColumnDefISerde of ISerde<ColumnDef> {
    const SIZE_HINT: Option<u32> = None;
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
