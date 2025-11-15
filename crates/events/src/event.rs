use introspect_types::FeltIterator;
use starknet_types_core::felt::Felt;

pub trait EventTrait
where
    Self: Sized,
{
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self>;

    fn verify(self, keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        match (keys.next(), data.next()) {
            (None, None) => Some(self),
            _ => None,
        }
    }
}
