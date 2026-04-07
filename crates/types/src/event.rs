use crate::{DecodeError, DecodeResult, FeltSource, IntoFeltSource};
pub use introspect_rust_macros::{selector_raw, selector_raw_ident};
use starknet_types_raw::Felt;
use starknet_types_raw::event::EmittedEvent;

pub trait CairoEventInfo {
    const NAME: &'static str;
    const SELECTOR: Felt = Felt::selector(Self::NAME);
}

pub trait CairoEvent<D>
where
    Self: Sized,
    D: FeltSource,
{
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

    fn from_emitted_event<'a>(event: &'a EmittedEvent) -> DecodeResult<Self>
    where
        D: From<&'a Vec<Felt>>,
    {
        let mut keys = event.keys[1..].into_source();
        let mut data = (&event.data).into();
        Self::deserialize_and_verify_event(&mut keys, &mut data)
    }
}
