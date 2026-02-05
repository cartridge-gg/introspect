use core::fmt::Debug;
use introspect_types::ISerde;

pub fn iserde_test<T, impl I: ISerde<T>, +PartialEq<T>, +Debug<T>, +Drop<T>>(value: T) {
    let mut serialized = value.iserialize_inline();
    assert_eq!(ISerde::ideserialize(ref serialized).unwrap(), value);
    // if let Some(size) = I::SIZE_HINT {
//     assert_eq!(serialized.len(), size);
// }
}

macro run_iserde_test_fuzzable {
    ($item:expr) => {
        #[fuzzable]
        fn $(item)_iserde_test(value: $item){

        }
    };
}
