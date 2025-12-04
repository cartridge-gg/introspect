pub use core::byte_array::ByteArray;
use core::keccak::compute_keccak_byte_array;
use core::metaprogramming::TypeEqual;
use snforge_std::fuzzable::{Fuzzable, generate_arg};
use crate::ascii;
use crate::fuzzable::Fuzzy;


const SELECTOR_MASK: u256 =
    0x3ff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff;

pub fn selector_from_byte_array(ba: @ByteArray) -> felt252 {
    let hash = compute_keccak_byte_array(ba);
    (hash & SELECTOR_MASK).try_into().unwrap()
}

pub trait RandomCharTrait {
    fn random_char() -> u8;
    fn random_chars(
        length: u32,
    ) -> Span<
        u8,
    > {
        (0..length).into_iter().map(|_a| Self::random_char()).collect::<Array<u8>>().span()
    }
    fn random_chars_lt(
        max_length: u32,
    ) -> Span<u8> {
        let length = generate_arg(0, max_length);
        Self::random_chars(length)
    }
    fn random_byte_array(length: u32) -> ByteArray {
        Self::random_chars(length).into()
    }
    fn random_byte_array_lt(
        max_length: u32,
    ) -> ByteArray {
        Self::random_chars_lt(max_length).into()
    }
}

pub impl RandomChar<
    const SIZE: u32, const CHARACTERS: [u8; SIZE], -TypeEqual<[u8; SIZE], [u8; 0]>,
> of RandomCharTrait {
    fn random_char() -> u8 {
        *CHARACTERS.span().at(generate_arg(0, SIZE - 1))
    }
}

pub impl RandomUppercase = RandomChar<_, ascii::UPPERCASE>;
pub impl RandomLowercase = RandomChar<_, ascii::LOWERCASE>;
pub impl RandomLetter = RandomChar<_, ascii::LETTERS>;
pub impl RandomDigit = RandomChar<_, ascii::DIGITS>;
pub impl RandomAlphanumeric = RandomChar<_, ascii::ALPHANUMERIC>;
pub impl RandomCairoDisplayable = RandomChar<_, ascii::CAIRO_DISPLAYABLE>;

pub impl BytesIntoByteArray of Into<Span<u8>, ByteArray> {
    fn into(self: Span<u8>) -> ByteArray {
        let mut arr: ByteArray = "";
        arr.append_bytes(self);
        arr
    }
}

#[generate_trait]
pub impl ByteArrayExtImpl of ByteArrayExt {
    fn append_bytes(ref self: ByteArray, span: Span<u8>) {
        for byte in span {
            self.append_byte(*byte);
        }
    }
    fn selector(self: @ByteArray) -> felt252 {
        selector_from_byte_array(self)
    }
}


pub fn generate_capaptlised_char_span(max_length: u32) -> Span<u8> {
    let mut word = array![RandomUppercase::random_char()];
    word.append_span(RandomLowercase::random_chars(generate_arg(0, max_length - 1)));
    word.span()
}

pub fn generate_capaptlised_word(max_length: u32) -> ByteArray {
    generate_capaptlised_char_span(max_length).into()
}


pub fn generate_lowercase_char_span(max_length: u32) -> Span<u8> {
    RandomLowercase::random_chars(generate_arg(1, max_length))
}

pub fn generate_lowercase_word(max_length: u32) -> ByteArray {
    generate_lowercase_char_span(max_length).into()
}

pub fn random_digit_char_span(max_length: u32) -> Span<u8> {
    RandomDigit::random_chars(generate_arg(1, max_length))
}

pub fn random_pascal_string(max_length: u32, max_parts: u32) -> ByteArray {
    let parts: u32 = generate_arg(1, max_parts);
    let max_len = (max_length / parts) - 1;
    let mut name: ByteArray = generate_capaptlised_word(max_len);

    for _ in 1..parts {
        let part = match Fuzzable::<bool>::generate() {
            false => generate_capaptlised_char_span(max_len),
            true => random_digit_char_span(max_len),
        };
        name.append_bytes(part);
    }
    name
}


pub fn random_snake_string(max_length: u32, max_parts: u32) -> ByteArray {
    use crate::ascii::UNDERSCORE;
    let parts: u32 = generate_arg(1, max_parts);
    let max_part_len = (max_length / parts) - 1;
    let mut name: ByteArray = generate_lowercase_word(max_part_len);
    for _ in 0..(parts - 1) {
        name.append_byte(UNDERSCORE);
        let part = match Fuzzable::<bool>::generate() {
            false => generate_lowercase_char_span(max_part_len),
            true => random_digit_char_span(max_part_len),
        };
        name.append_bytes(part);
    }
    name
}

pub fn random_byte_array(max_length: u32) -> ByteArray {
    let mut ba: ByteArray = "";
    ba.append_bytes(Fuzzy::<u8>::generate_span_lt(max_length));
    ba
}
