use crate::ISerde;
use crate::utils::SpanDefault;

#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct IdData {
    pub id: felt252,
    pub data: Span<felt252>,
}


impl IdDataIntoTuple of Into<IdData, (felt252, Span<felt252>)> {
    fn into(self: IdData) -> (felt252, Span<felt252>) {
        (self.id, self.data)
    }
}

impl TupleIntoIdData of Into<(felt252, Span<felt252>), IdData> {
    fn into(self: (felt252, Span<felt252>)) -> IdData {
        let (id, data) = self;
        IdData { id, data }
    }
}

impl IdDataISerde of ISerde<IdData> {
    fn iserialize(self: @IdData, ref output: Array<felt252>) {
        output.append(*self.id);
        self.data.iserialize(ref output);
    }

    fn ideserialize(ref serialized: Span<felt252>) -> Option<IdData> {
        Some(IdData { id: *serialized.pop_front()?, data: ISerde::ideserialize(ref serialized)? })
    }
}
