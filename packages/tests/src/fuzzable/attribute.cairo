use introspect_tests::{RandomCairoDisplayable, random_snake_string};
use introspect_types::Attribute;
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
