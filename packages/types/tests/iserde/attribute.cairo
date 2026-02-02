use introspect_test_utils::types::{Attribute, FuzzableAttribute};
use introspect_types::structured::attribute::{
    full_terminator_with_data, partial_terminator_with_data,
};
use introspect_types::{ISerde, ISerdeByteArray};

fn test_iserde_attribute(attribute: Attribute, expected: Span<felt252>) {
    let mut serialized = attribute.iserialize_inline();
    assert_eq!(serialized, expected, "Array does not match");
    assert_eq!(
        ISerde::ideserialize(ref serialized).unwrap(), attribute, "Deserialized doesnt match",
    );
}

#[test]
fn test_attribute_short_key_only() {
    const EXPECTED: [felt252; 1] = ['key'.partial_terminator(3)];
    let attribute = Attribute { name: "key", data: None };
    test_iserde_attribute(attribute, EXPECTED.span());
}

#[test]
fn test_attribute_31_bytes_key_only() {
    const EXPECTED: [felt252; 1] = ['attribute_name_31_bytes_longggg'.full_terminator()];
    let attribute = Attribute { name: "attribute_name_31_bytes_longggg", data: None };
    test_iserde_attribute(attribute, EXPECTED.span());
}

#[test]
fn test_attribute_32_bytes_key_only() {
    const EXPECTED: [felt252; 2] = ['attribute_name_32_bytes_longggg', 'g'.partial_terminator(1)];
    let attribute = Attribute { name: "attribute_name_32_bytes_longgggg", data: None };
    test_iserde_attribute(attribute, EXPECTED.span());
}

#[test]
fn test_short_key_with_short_data() {
    const EXPECTED: [felt252; 2] = [
        partial_terminator_with_data('key', 3), 'data'.partial_terminator(4),
    ];
    let attribute = Attribute { name: "key", data: Some("data") };
    test_iserde_attribute(attribute, EXPECTED.span());
}

#[test]
fn test_attribute_31_bytes_key_with_data() {
    const EXPECTED: [felt252; 2] = [
        full_terminator_with_data('attribute_name_31_bytes_longggg'), 'data'.partial_terminator(4),
    ];
    let attribute = Attribute { name: "attribute_name_31_bytes_longggg", data: Some("data") };
    test_iserde_attribute(attribute, EXPECTED.span());
}

#[test]
fn test_attribute_32_bytes_key_with_data() {
    const EXPECTED: [felt252; 3] = [
        'attribute_name_32_bytes_longggg', partial_terminator_with_data('g', 1),
        'data'.partial_terminator(4),
    ];
    let attribute = Attribute { name: "attribute_name_32_bytes_longgggg", data: Some("data") };
    test_iserde_attribute(attribute, EXPECTED.span());
}

#[test]
fn test_short_key_with_short_31_bytes_data() {
    const EXPECTED: [felt252; 2] = [
        partial_terminator_with_data('key', 3), 'data that is 31 bytes longggggg'.full_terminator(),
    ];
    let attribute = Attribute { name: "key", data: Some("data that is 31 bytes longggggg") };
    test_iserde_attribute(attribute, EXPECTED.span());
}

#[test]
fn test_attribute_31_bytes_key_with_31_bytes_data() {
    const EXPECTED: [felt252; 2] = [
        full_terminator_with_data('attribute_name_31_bytes_longggg'),
        'data that is 31 bytes longggggg'.full_terminator(),
    ];
    let attribute = Attribute {
        name: "attribute_name_31_bytes_longggg", data: Some("data that is 31 bytes longggggg"),
    };
    test_iserde_attribute(attribute, EXPECTED.span());
}

#[test]
fn test_attribute_32_bytes_key_with_31_bytes_data() {
    const EXPECTED: [felt252; 3] = [
        'attribute_name_32_bytes_longggg', partial_terminator_with_data('g', 1),
        'data that is 31 bytes longggggg'.full_terminator(),
    ];
    let attribute = Attribute {
        name: "attribute_name_32_bytes_longgggg", data: Some("data that is 31 bytes longggggg"),
    };
    test_iserde_attribute(attribute, EXPECTED.span());
}

#[test]
fn test_short_key_with_short_long_data() {
    const EXPECTED: [felt252; 3] = [
        partial_terminator_with_data('key', 3), 'data that is longer than 31 byt',
        'es'.partial_terminator(2),
    ];
    let attribute = Attribute { name: "key", data: Some("data that is longer than 31 bytes") };
    test_iserde_attribute(attribute, EXPECTED.span());
}

#[test]
fn test_attribute_31_bytes_key_with_long_data() {
    const EXPECTED: [felt252; 3] = [
        full_terminator_with_data('attribute_name_31_bytes_longggg'),
        'data that is longer than 31 byt', 'es'.partial_terminator(2),
    ];
    let attribute = Attribute {
        name: "attribute_name_31_bytes_longggg", data: Some("data that is longer than 31 bytes"),
    };
    test_iserde_attribute(attribute, EXPECTED.span());
}

#[test]
fn test_attribute_32_bytes_key_with_long_data() {
    const EXPECTED: [felt252; 4] = [
        'attribute_name_32_bytes_longggg', partial_terminator_with_data('g', 1),
        'data that is longer than 31 byt', 'es'.partial_terminator(2),
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
    assert_eq!(
        ISerde::ideserialize(ref serialized).unwrap(), attribute, "Deserialized doesnt match",
    );
}
