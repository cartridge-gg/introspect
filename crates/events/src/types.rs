use introspect_types::deserialize_def::TypeDefDeserializer;
use introspect_types::{
    CairoDeserialize, CairoEvent, CairoEventInfo, DecodeResult, FeltSource, TypeDef,
};
use starknet_types_raw::Felt;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeclareType {
    pub id: Felt,
    pub type_def: TypeDef,
}

impl<D: FeltSource + TypeDefDeserializer> CairoEvent<D> for DeclareType {
    fn deserialize_event<K: FeltSource>(_keys: &mut K, data: &mut D) -> DecodeResult<Self> {
        let id = data.next()?;
        let type_def = TypeDef::deserialize(data)?;
        Ok(DeclareType { id, type_def })
    }
}

impl CairoEventInfo for DeclareType {
    const NAME: &'static str = "DeclareType";
}
