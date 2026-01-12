use introspect_tests::fuzzable;
use introspect_types::TypeDef;
use super::iserde_test;

impl TypeDefFuzzable = fuzzable::TypeDefFuzzableToDepth<10>;


#[test]
#[fuzzer]
fn test_iserde_type_def(type_def: TypeDef) {
    iserde_test(type_def)
}

