use crate::{ChildDefs, PrimaryDef};

pub trait TableSchema<T, const NAME_SIZE: u32, const ATTRIBUTES_SIZE: u32> {
    const ID: felt252;
    const NAME: [felt252; NAME_SIZE];
    const ATTRIBUTES_COUNT: u32;
    const ATTRIBUTES: [felt252; ATTRIBUTES_SIZE];
    const COLUMNS_COUNT: u32;
    impl Primary: PrimaryDef<_>;
    fn serialize_columns(ref output: Array<felt252>);
    fn collect_columns_children(ref children: ChildDefs) {}
    fn serialize_columns_with_children(ref type_def: Array<felt252>, ref children: ChildDefs);
}



