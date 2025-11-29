use introspect_types::FeltIterator;
use starknet_types_core::felt::Felt;

pub trait EventTrait
where
    Self: Sized,
{
    const SELECTOR_RAW: [u64; 4];
    const SELECTOR: Felt = Felt::from_raw(Self::SELECTOR_RAW);
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self>;

    fn verify(self, keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        match (keys.next(), data.next()) {
            (None, None) => Some(self),
            _ => None,
        }
    }

    fn verify_keys(self, keys: &mut FeltIterator) -> Option<Self> {
        match keys.next() {
            None => Some(self),
            _ => None,
        }
    }
}
