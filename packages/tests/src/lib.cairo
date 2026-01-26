pub mod ascii;
pub mod byte_array;
pub mod fuzzable;
pub use byte_array::{
    ByteArrayExt, BytesIntoByteArray, RandomAlphanumeric, RandomCairoDisplayable, RandomCharTrait,
    RandomDigit, RandomLetter, RandomLowercase, RandomUppercase, generate_capaptlised_word,
    generate_lowercase_char_span, generate_lowercase_word, random_byte_array,
    random_digit_char_span, random_pascal_string, random_snake_string,
};

pub use fuzzable::{FuzzableMaxDepth, FuzzableMaxDepthNode, FuzzableMaxDepthNodeImpl, Fuzzy};
pub mod events;
pub mod introspect;
pub mod introspect_fuzzable {
    pub use super::introspect::{AStruct, EnumKey, Foo, Foo2, Foo3, Foo4, FooSchema};
}
