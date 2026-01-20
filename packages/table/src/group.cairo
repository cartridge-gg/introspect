use crate::TableStructure;

pub trait FieldOnlyColumnGroup<C, const SIZE: usize, impl Struct: TableStructure> {
    const GROUP_ID: felt252;
    const COLUMN_IDS: [felt252; SIZE];
    fn group_data(self: @C) -> Span<felt252>;
}


pub trait IdColumnGroup<C, const SIZE: usize, impl Struct: TableStructure> {
    const GROUP_ID: felt252;
    const COLUMN_IDS: [felt252; SIZE];
    fn group_tuple(self: @C) -> (felt252, Span<felt252>);
}
