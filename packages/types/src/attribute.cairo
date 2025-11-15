#[derive(Drop, Serde, PartialEq, Debug)]
pub struct Attribute {
    pub id: felt252,
    pub data: Span<felt252>,
}
