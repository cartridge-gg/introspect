use cgg_utils::testing::{FuzzyImpl, RandomCairoDisplayable, random_snake_string};
pub use introspect_types::structured::Attribute;
use snforge_std::fuzzable::{Fuzzable, FuzzableBool};


pub impl FuzzableAttribute of Fuzzable<Attribute> {
    fn blank() -> Attribute {
        Default::default()
    }

    fn generate() -> Attribute {
        Attribute {
            name: random_snake_string(31, 4),
            data: match FuzzableBool::generate() {
                false => Option::None,
                true => Option::Some(RandomCairoDisplayable::random_byte_array_lt(64)),
            },
        }
    }
}
