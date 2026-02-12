use crate::felt::IntoFeltSource;
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
{
    const NAME: &'static str;
    const SELECTOR_RAW: [u64; 4];
    const SELECTOR: Felt = Felt::from_raw(Self::SELECTOR_RAW);

    fn deserialize_event<K: FeltSource, E: FeltSource>(
        keys: &mut K,
        data: &mut E,
    ) -> DecodeResult<Self>;
    fn deserialize_and_verify_event<K: FeltSource, E: FeltSource>(
        keys: &mut K,
        data: &mut E,
    ) -> DecodeResult<Self> {
        let mut keys = keys.into_source();
        let mut data = data.into_source();
        Self::deserialize_event(&mut keys, &mut data)?.verify_eof(&mut keys, &mut data)
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
    fn deserialize_and_verify_event_enum<K: FeltSource, E: FeltSource, T: From<Self>>(
        keys: &mut K,
        data: &mut E,
    ) -> DecodeResult<T> {
        Self::deserialize_and_verify_event(keys, data).map(Into::into)
    }
}
