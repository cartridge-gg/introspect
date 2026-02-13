use crate::{DecodeError, DecodeResult, FeltSource};
pub use introspect_rust_macros::selector_raw;
use starknet_types_core::felt::Felt;

#[macro_export]
macro_rules! cairo_event_name_and_selector {
    ($name:literal) => {
        const NAME: &'static str = $name;
        const SELECTOR_RAW: [u64; 4] = $crate::event::selector_raw!($name);
    };
}

pub trait CairoEvent<D>
where
    Self: Sized,
    D: FeltSource,
{
    const NAME: &'static str;
    const SELECTOR_RAW: [u64; 4];
    const SELECTOR: Felt = Felt::from_raw(Self::SELECTOR_RAW);

    fn deserialize_event<K: FeltSource>(keys: &mut K, data: &mut D) -> DecodeResult<Self>;
    fn deserialize_and_verify_event<K: FeltSource>(
        keys: &mut K,
        data: &mut D,
    ) -> DecodeResult<Self> {
        Self::deserialize_event(keys, data)?.verify_eof(keys, data)
    }
    fn verify_eof<K: FeltSource>(self, keys: &mut K, data: &mut D) -> DecodeResult<Self> {
        match (keys.next(), data.next()) {
            (Err(DecodeError::Eof), Err(DecodeError::Eof)) => Ok(self),
            (Ok(_), Err(DecodeError::Eof)) | (Err(DecodeError::Eof), Ok(_)) | (Ok(_), Ok(_)) => {
                Err(DecodeError::NotEof)
            }
            (Err(e), _) | (_, Err(e)) => Err(e),
        }
    }
    fn deserialize_and_verify_event_enum<K: FeltSource, T: From<Self>>(
        keys: &mut K,
        data: &mut D,
    ) -> DecodeResult<T> {
        Self::deserialize_and_verify_event(keys, data).map(Into::into)
    }
}
