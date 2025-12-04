use introspect_tests::fuzzable::attribute::FuzzableAttribute;
use introspect_types::attribute::B31_4;
use introspect_types::serde::{B31_2, B31_3, SHIFT_30B};
use introspect_types::{Attribute, ISerde};

fn test_iserde_attribute(attribute: Attribute, expected: Span<felt252>) {
    let mut serialized = attribute.iserialize_inline();
    assert(serialized == expected, 'Array does not match');
    assert(ISerde::ideserialize_unwrap(ref serialized) == attribute, 'Deserialized doesnt match');
}


#[test]
fn test_attribute_short_key_only() {
    const EXPECTED: [felt252; 1] = ['key' + 3 * SHIFT_30B + B31_3];
    let attribute = Attribute { name: "key", data: None };
    test_iserde_attribute(attribute, EXPECTED.span());
}

#[test]
fn test_attribute_31_bytes_key_only() {
    const EXPECTED: [felt252; 1] = ['attribute_name_31_bytes_longggg' + B31_2];
    let attribute = Attribute { name: "attribute_name_31_bytes_longggg", data: None };
    test_iserde_attribute(attribute, EXPECTED.span());
}

#[test]
fn test_attribute_32_bytes_key_only() {
    const EXPECTED: [felt252; 2] = ['attribute_name_32_bytes_longggg', 'g' + B31_3 + SHIFT_30B];
    let attribute = Attribute { name: "attribute_name_32_bytes_longgggg", data: None };
    test_iserde_attribute(attribute, EXPECTED.span());
}

#[test]
fn test_short_key_with_short_data() {
    const EXPECTED: [felt252; 2] = [
        'key' + 3 * SHIFT_30B + B31_3 + B31_4, 'data' + 4 * SHIFT_30B + B31_3,
    ];
    let attribute = Attribute { name: "key", data: Some("data") };
    test_iserde_attribute(attribute, EXPECTED.span());
}

#[test]
fn test_attribute_31_bytes_key_with_data() {
    const EXPECTED: [felt252; 2] = [
        'attribute_name_31_bytes_longggg' + B31_2 + B31_4, 'data' + 4 * SHIFT_30B + B31_3,
    ];
    let attribute = Attribute { name: "attribute_name_31_bytes_longggg", data: Some("data") };
    test_iserde_attribute(attribute, EXPECTED.span());
}

#[test]
fn test_attribute_32_bytes_key_with_data() {
    const EXPECTED: [felt252; 3] = [
        'attribute_name_32_bytes_longggg', 'g' + B31_3 + SHIFT_30B + B31_4,
        'data' + 4 * SHIFT_30B + B31_3,
    ];
    let attribute = Attribute { name: "attribute_name_32_bytes_longgggg", data: Some("data") };
    test_iserde_attribute(attribute, EXPECTED.span());
}

#[test]
fn test_short_key_with_short_31_bytes_data() {
    const EXPECTED: [felt252; 2] = [
        'key' + 3 * SHIFT_30B + B31_3 + B31_4, 'data that is 31 bytes longggggg' + B31_2,
    ];
    let attribute = Attribute { name: "key", data: Some("data that is 31 bytes longggggg") };
    test_iserde_attribute(attribute, EXPECTED.span());
}

#[test]
fn test_attribute_31_bytes_key_with_31_bytes_data() {
    const EXPECTED: [felt252; 2] = [
        'attribute_name_31_bytes_longggg' + B31_2 + B31_4,
        'data that is 31 bytes longggggg' + B31_2,
    ];
    let attribute = Attribute {
        name: "attribute_name_31_bytes_longggg", data: Some("data that is 31 bytes longggggg"),
    };
    test_iserde_attribute(attribute, EXPECTED.span());
}

#[test]
fn test_attribute_32_bytes_key_with_31_bytes_data() {
    const EXPECTED: [felt252; 3] = [
        'attribute_name_32_bytes_longggg', 'g' + B31_3 + SHIFT_30B + B31_4,
        'data that is 31 bytes longggggg' + B31_2,
    ];
    let attribute = Attribute {
        name: "attribute_name_32_bytes_longgggg", data: Some("data that is 31 bytes longggggg"),
    };
    test_iserde_attribute(attribute, EXPECTED.span());
}

#[test]
fn test_short_key_with_short_long_data() {
    const EXPECTED: [felt252; 3] = [
        'key' + 3 * SHIFT_30B + B31_3 + B31_4, 'data that is longer than 31 byt',
        'es' + 2 * SHIFT_30B + B31_3,
    ];
    let attribute = Attribute { name: "key", data: Some("data that is longer than 31 bytes") };
    test_iserde_attribute(attribute, EXPECTED.span());
}

#[test]
fn test_attribute_31_bytes_key_with_long_data() {
    const EXPECTED: [felt252; 3] = [
        'attribute_name_31_bytes_longggg' + B31_2 + B31_4, 'data that is longer than 31 byt',
        'es' + 2 * SHIFT_30B + B31_3,
    ];
    let attribute = Attribute {
        name: "attribute_name_31_bytes_longggg", data: Some("data that is longer than 31 bytes"),
    };
    test_iserde_attribute(attribute, EXPECTED.span());
}

#[test]
fn test_attribute_32_bytes_key_with_long_data() {
    const EXPECTED: [felt252; 4] = [
        'attribute_name_32_bytes_longggg', 'g' + B31_3 + SHIFT_30B + B31_4,
        'data that is longer than 31 byt', 'es' + 2 * SHIFT_30B + B31_3,
    ];
    let attribute = Attribute {
        name: "attribute_name_32_bytes_longgggg", data: Some("data that is longer than 31 bytes"),
    };
    test_iserde_attribute(attribute, EXPECTED.span());
}


#[test]
#[fuzzer]
fn test_many_attributes(attribute: Attribute) {
    let mut serialized = attribute.iserialize_inline();
    assert(ISerde::ideserialize_unwrap(ref serialized) == attribute, 'Deserialized doesnt match');
}
