use introspect_types::{
    Attribute, CairoDeserialize, CairoDeserializer, CairoEvent, DecodeResult, FeltSource, TypeDef,
    cairo_event_name_and_selector,
};
use starknet_types_core::felt::Felt;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeclareType {
    pub id: Felt,
    pub type_def: TypeDef,
}

impl<D: FeltSource + CairoDeserializer> CairoEvent<D> for DeclareType
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize_event<K: FeltSource>(_keys: &mut K, data: &mut D) -> DecodeResult<Self> {
        let id = data.next()?;
        let type_def = TypeDef::deserialize(data)?;
        Ok(DeclareType { id, type_def })
    }
}

cairo_event_name_and_selector!(DeclareType);
