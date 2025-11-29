use crate::{ISerde, PrimaryTrait, RecordPrimary, Schema};

pub trait IdDataTrait<T> {
    fn id_data(
        self: @T,
    ) -> IdData {
        let (id, data) = Self::id_data_tuple(self);
        IdData { id, data }
    }
    fn id_data_tuple(self: @T) -> (felt252, Span<felt252>);
}

impl KeyedSchemaRecordImpl<T, +RecordPrimary<T>, +Schema<T>> of IdDataTrait<T> {
    fn id_data_tuple(self: @T) -> (felt252, Span<felt252>) {
        (self.record_id(), self.record_data())
    }
}

pub impl PrimaryDataTupleRecordImpl<P, S, +PrimaryTrait<P>, +ISerde<S>> of IdDataTrait<(P, S)> {
    fn id_data_tuple(self: @(P, S)) -> (felt252, Span<felt252>) {
        let (primary, schema) = self;
        (primary.to_felt252(), schema.iserialize_inline())
    }
}

pub impl SPrimarySDataTupleRecordImpl<P, S, +PrimaryTrait<P>, +ISerde<S>> of IdDataTrait<(@P, @S)> {
    fn id_data_tuple(self: @(@P, @S)) -> (felt252, Span<felt252>) {
        let (primary, schema) = self;
        (primary.to_felt252(), schema.iserialize_inline())
    }
}

pub impl PrimarySDataTupleRecordImpl<P, S, +PrimaryTrait<P>, +ISerde<S>> of IdDataTrait<(P, @S)> {
    fn id_data_tuple(self: @(P, @S)) -> (felt252, Span<felt252>) {
        let (primary, schema) = self;
        (primary.to_felt252(), schema.iserialize_inline())
    }
}

pub impl SPrimaryDataTupleRecordImpl<P, S, +PrimaryTrait<P>, +ISerde<S>> of IdDataTrait<(@P, S)> {
    fn id_data_tuple(self: @(@P, S)) -> (felt252, Span<felt252>) {
        let (primary, schema) = self;
        (primary.to_felt252(), schema.iserialize_inline())
    }
}

#[derive(Drop, Serde, PartialEq, Debug)]
pub struct IdData {
    pub id: felt252,
    pub data: Span<felt252>,
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
