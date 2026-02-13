use introspect_types::{
    CairoDeserialize, CairoDeserializer, CairoISerde, CairoSerde, DecodeResult, FeltSource,
};

pub trait CairoDeserializeRemaining {
    fn deserialize_remaining<T: CairoDeserialize<Self>>(&mut self) -> DecodeResult<Vec<T>>
    where
        Self: Sized;
}

impl<F: FeltSource> CairoDeserializeRemaining for CairoSerde<F> {
    fn deserialize_remaining<T: CairoDeserialize<Self>>(&mut self) -> DecodeResult<Vec<T>> {
        self.next_array()
    }
}

impl<F: FeltSource> CairoDeserializeRemaining for CairoISerde<F> {
    fn deserialize_remaining<T: CairoDeserialize<Self>>(&mut self) -> DecodeResult<Vec<T>> {
        self.drain_values()
    }
}
