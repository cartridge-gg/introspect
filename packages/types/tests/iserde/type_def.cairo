use introspect_tests::fuzzable;
use introspect_types::{ISerde, TypeDef};

impl TypeDefFuzzable = fuzzable::TypeDefFuzzableToDepth<10>;

#[test]
#[fuzzer]
fn test_iserde_type_def(type_def: TypeDef) {
    let mut serialized = type_def.iserialize_inline();
    assert_eq!(ISerde::ideserialize(ref serialized).unwrap(), type_def);
}
// const SPAN: [felt252; 5] = [1, 2, 3, 4, 5];
// const EMPTY_SPAN: [felt252; 0] = [];

// #[test]
// fn test_unboxing() {
//     let span = array![1, 2, 3, 4, 5].span();
//     let span = array![0].span();
//     let out = BoxTrait::new(SPAN).span();

//     let out = BoxTrait::new(EMPTY_SPAN).span();
// }

