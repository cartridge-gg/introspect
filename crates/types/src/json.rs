use crate::utils::felt_to_hex_string;
use crate::{DecodeResult, Enum, Struct, Value};
use primitive_types::{U256, U512};
use serde_json::Value::{
    Array as JsonArray, Bool as JsonBool, Null as JsonNull, Number as JsonNumber,
    Object as JsonObject, String as JsonString,
};
use serde_json::{Map, Value as JsonValue};
use starknet_types_core::felt::Felt;

fn vec_values_to_json_array(values: Vec<Value>) -> JsonValue {
    JsonArray(values.into_iter().map(|v| v.into()).collect())
}

fn to_json_number<T: Into<serde_json::Number>>(value: T) -> JsonValue {
    JsonNumber(value.into())
}

fn to_json_string<T: ToString>(value: T) -> JsonValue {
    JsonString(value.to_string())
}

impl Into<Map<String, JsonValue>> for Struct {
    fn into(self) -> Map<String, JsonValue> {
        let mut map = serde_json::Map::new();
        for member in self.members {
            let key = member.name;
            let value: JsonValue = member.value.into();
            map.insert(key, value);
        }
        map
    }
}

impl Into<JsonValue> for Enum {
    fn into(self) -> JsonValue {
        let mut map = serde_json::Map::new();
        match self.value {
            Value::None => JsonString(self.variant),
            _ => {
                map.insert(self.variant, self.value.into());
                JsonObject(map)
            }
        }
    }
}

impl Into<JsonValue> for Value {
    fn into(self) -> JsonValue {
        match self {
            Value::None => JsonNull,
            Value::Felt252(v)
            | Value::ClassHash(v)
            | Value::ContractAddress(v)
            | Value::EthAddress(v) => JsonString(felt_to_hex_string(&v)),
            Value::Bool(v) => JsonBool(v),
            Value::U8(v) => to_json_number(v),
            Value::U16(v) => to_json_number(v),
            Value::U32(v) => to_json_number(v),
            Value::U64(v) => to_json_number(v),
            Value::U128(v) => to_json_string(v),
            Value::U256(v) => to_json_string(v),
            Value::I8(v) => to_json_number(v),
            Value::I16(v) => to_json_number(v),
            Value::I32(v) => to_json_number(v),
            Value::I64(v) => to_json_string(v),
            Value::I128(v) => to_json_string(v),
            Value::ByteArray(v) => v.into(),
            Value::Utf8String(v) => to_json_string(v),
            Value::Struct(v) => JsonObject(v.into()),
            Value::Tuple(values) | Value::Array(values) | Value::FixedArray(values) => {
                vec_values_to_json_array(values)
            }
            Value::Enum(e) => (*e).into(),
            _ => unimplemented!(),
        }
    }
}
