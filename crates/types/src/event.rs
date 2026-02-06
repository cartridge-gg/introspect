use crate::deserialize::CairoDeserializer;
use crate::felt::IntoFeltSource;
use crate::{DecodeError, DecodeResult, FeltSource, ascii_str_to_limbs};
use starknet_types_core::felt::Felt;

macro_rules! impl_event_name {
    ($name:literal) => {
        const NAME: &'static str = $name;
        const SELECTOR_RAW: [u64; 4] = introspect_rust_macros::selector_raw!($name).to_raw();
    };
}

pub trait CairoEvent<D: CairoDeserializer>
where
    Self: Sized,
{
    const NAME: &'static str;
    const SELECTOR_RAW: [u64; 4];
    const SELECTOR: Felt = Felt::from_raw(Self::SELECTOR_RAW);

    fn deserialize<K: FeltSource, E: FeltSource>(keys: &mut K, data: &mut E) -> DecodeResult<Self>;
    fn deserialize_complete<K: IntoFeltSource, E: IntoFeltSource>(
        keys: K,
        data: E,
    ) -> DecodeResult<Self> {
        let mut keys = keys.into_source();
        let mut data = data.into_source();
        Self::deserialize(&mut keys, &mut data)?.verify_eof(&mut keys, &mut data)
    }
    fn verify_eof<K: FeltSource, E: FeltSource>(
        self,
        keys: &mut K,
        data: &mut E,
    ) -> DecodeResult<Self> {
        match (keys.next(), data.next()) {
            (Err(DecodeError::Eof), Err(DecodeError::Eof)) => Ok(self),
            (Ok(_), Err(DecodeError::Eof)) | (Err(DecodeError::Eof), Ok(_)) | (Ok(_), Ok(_)) => {
                Err(DecodeError::NotEof)
            }
            (Err(e), _) | (_, Err(e)) => Err(e),
        }
    }
}
