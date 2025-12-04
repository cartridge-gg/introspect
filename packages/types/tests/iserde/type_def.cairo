use introspect_types::{ISerde, TypeDef};
use crate::fuzzable;

impl TypeDefFuzzable = fuzzable::TypeDefFuzzableToDepth<10>;

#[test]
#[fuzzer]
fn test_iserde_type_def(type_def: TypeDef) {
    let mut serialized = type_def.iserialize_inline();
    assert_eq!(ISerde::ideserialize_unwrap(ref serialized), type_def);
}
