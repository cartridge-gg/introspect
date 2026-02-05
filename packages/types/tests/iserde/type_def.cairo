use introspect_types::structured::TypeDef;
use super::iserde_test;

impl TypeDefFuzzable = introspect_test_utils::types::TypeDefFuzzableToDepth<10>;


#[test]
#[fuzzer]
fn test_iserde_type_def(type_def: TypeDef) {
    iserde_test(type_def)
}

