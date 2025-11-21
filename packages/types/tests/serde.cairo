use core::num::traits::Zero;
use introspect_types::serde::{B31_1, B31_2, B31_3, B31_4, ISerdeByteArray, SHIFT_30B};
use introspect_types::{Attribute, ISerde};


#[test]
fn test_empty_byte_array() {
    let mut output: Array<felt252> = Default::default();
    let byte_array: ByteArray = "";
    let last = byte_array.iserialize_and_last(ref output);
    assert(output.len().is_zero(), ' Array should be empty ');
    assert(last == B31_3, 'Wrong felt returned')
}

#[test]
fn test_partial_byte_array() {
    const EXPECTED: felt252 = 'Remember the Cant' + (17 * SHIFT_30B) + B31_3;
    let mut output: Array<felt252> = Default::default();
    let byte_array: ByteArray = "Remember the Cant";
    let last = byte_array.iserialize_and_last(ref output);
    assert(output.len().is_zero(), ' Array should be empty ');
    assert(last == EXPECTED, 'Wrong felt returned')
}

#[test]
fn test_31_bytes_array() {
    const EXPECTED: felt252 = 'what is this? a center for ants' + B31_1;
    let mut output: Array<felt252> = Default::default();
    let byte_array: ByteArray = "what is this? a center for ants";
    let last = byte_array.iserialize_and_last(ref output);
    assert(output.len().is_zero(), ' Array should be empty ');
    assert(last == EXPECTED, 'Wrong felt returned')
}

#[test]
fn test_longer_than_31_bytes() {
    const EXPECTED_SPAN: [felt252; 1] = ['ubiquitous, mendacious, polyglo'];
    const EXPECTED_LAST: felt252 = 'ttal' + B31_3 + (4 * SHIFT_30B);
    let mut output: Array<felt252> = Default::default();
    let byte_array: ByteArray = "ubiquitous, mendacious, polyglottal";
    let last = byte_array.iserialize_and_last(ref output);
    assert(output.span() == EXPECTED_SPAN.span(), ' Array does not match');
    assert(last == EXPECTED_LAST, 'Wrong felt returned')
}

#[test]
fn test_61_bytes_array() {
    const EXPECTED_SPAN: [felt252; 1] = ['Even the smallest person can ch'];
    const EXPECTED_LAST: felt252 = 'ange the course of the future.' + B31_3 + (30 * SHIFT_30B);
    let mut output: Array<felt252> = Default::default();
    let byte_array: ByteArray = "Even the smallest person can change the course of the future.";
    let last = byte_array.iserialize_and_last(ref output);
    assert(output.span() == EXPECTED_SPAN.span(), ' Array does not match');
    assert(last == EXPECTED_LAST, 'Wrong felt returned')
}

#[test]
fn test_62_bytes_array() {
    const EXPECTED_SPAN: [felt252; 1] = ['Even the smallest person can ch'];
    const EXPECTED_LAST: felt252 = 'ange the course of the future..' + B31_1;
    let mut output: Array<felt252> = Default::default();
    let byte_array: ByteArray = "Even the smallest person can change the course of the future..";
    let last = byte_array.iserialize_and_last(ref output);
    assert(output.span() == EXPECTED_SPAN.span(), ' Array does not match');
    assert(last == EXPECTED_LAST, 'Wrong felt returned')
}

#[test]
fn test_lots_bytes_array() {
    const EXPECTED_SPAN: [felt252; 3] = [
        'One ring to rule them all, One ', 'ring to find them, One ring to ',
        'bring them all and in the darkn',
    ];
    const EXPECTED_LAST: felt252 = 'ess bind them.' + B31_3 + (14 * SHIFT_30B);
    let mut output: Array<felt252> = Default::default();
    let byte_array: ByteArray =
        "One ring to rule them all, One ring to find them, One ring to bring them all and in the darkness bind them.";
    let last = byte_array.iserialize_and_last(ref output);
    assert(output.span() == EXPECTED_SPAN.span(), ' Array does not match');
    assert(last == EXPECTED_LAST, 'Wrong felt returned')
}

#[test]
fn test_attribute_short_key_only() {
    const EXPECTED_SPAN: [felt252; 1] = ['key' + 3 * SHIFT_30B + B31_3];
    let mut output: Array<felt252> = Default::default();
    let attribute = Attribute { name: "key", data: None };
    attribute.iserialize(ref output);
    assert(output.span() == EXPECTED_SPAN.span(), ' Array does not match');
}

#[test]
fn test_attribute_31_bytes_key_only() {
    const EXPECTED_SPAN: [felt252; 1] = ['attribute_name_31_bytes_longggg' + B31_1];
    let mut output: Array<felt252> = Default::default();
    let attribute = Attribute { name: "attribute_name_31_bytes_longggg", data: None };
    attribute.iserialize(ref output);
    assert(output.span() == EXPECTED_SPAN.span(), ' Array does not match');
}

#[test]
fn test_attribute_32_bytes_key_only() {
    const EXPECTED_SPAN: [felt252; 2] = [
        'attribute_name_32_bytes_longggg', 'g' + B31_3 + SHIFT_30B,
    ];
    let mut output: Array<felt252> = Default::default();
    let attribute = Attribute { name: "attribute_name_32_bytes_longgggg", data: None };
    attribute.iserialize(ref output);
    assert(output.span() == EXPECTED_SPAN.span(), ' Array does not match');
}

#[test]
fn test_short_key_with_short_data() {
    const EXPECTED_SPAN: [felt252; 2] = [
        'key' + 3 * SHIFT_30B + B31_3 + B31_4, 'data' + 4 * SHIFT_30B + B31_3,
    ];
    let mut output: Array<felt252> = Default::default();
    let attribute = Attribute { name: "key", data: Some("data") };
    attribute.iserialize(ref output);
    assert(output.span() == EXPECTED_SPAN.span(), ' Array does not match');
}

#[test]
fn test_attribute_31_bytes_key_with_data() {
    const EXPECTED_SPAN: [felt252; 2] = [
        'attribute_name_31_bytes_longggg' + B31_1 + B31_4, 'data' + 4 * SHIFT_30B + B31_3,
    ];
    let mut output: Array<felt252> = Default::default();
    let attribute = Attribute { name: "attribute_name_31_bytes_longggg", data: Some("data") };
    attribute.iserialize(ref output);
    assert(output.span() == EXPECTED_SPAN.span(), ' Array does not match');
}

#[test]
fn test_attribute_32_bytes_key_with_data() {
    const EXPECTED_SPAN: [felt252; 3] = [
        'attribute_name_32_bytes_longggg', 'g' + B31_3 + SHIFT_30B + B31_4,
        'data' + 4 * SHIFT_30B + B31_3,
    ];
    let mut output: Array<felt252> = Default::default();
    let attribute = Attribute { name: "attribute_name_32_bytes_longgggg", data: Some("data") };
    attribute.iserialize(ref output);
    assert(output.span() == EXPECTED_SPAN.span(), ' Array does not match');
}

#[test]
fn test_short_key_with_short_31_bytes_data() {
    const EXPECTED_SPAN: [felt252; 2] = [
        'key' + 3 * SHIFT_30B + B31_3 + B31_4, 'data that is 31 bytes longggggg' + B31_1,
    ];
    let mut output: Array<felt252> = Default::default();
    let attribute = Attribute { name: "key", data: Some("data that is 31 bytes longggggg") };
    attribute.iserialize(ref output);
    assert(output.span() == EXPECTED_SPAN.span(), ' Array does not match');
}

#[test]
fn test_attribute_31_bytes_key_with_31_bytes_data() {
    const EXPECTED_SPAN: [felt252; 2] = [
        'attribute_name_31_bytes_longggg' + B31_1 + B31_4,
        'data that is 31 bytes longggggg' + B31_1,
    ];
    let mut output: Array<felt252> = Default::default();
    let attribute = Attribute {
        name: "attribute_name_31_bytes_longggg", data: Some("data that is 31 bytes longggggg"),
    };
    attribute.iserialize(ref output);
    assert(output.span() == EXPECTED_SPAN.span(), ' Array does not match');
}

#[test]
fn test_attribute_32_bytes_key_with_31_bytes_data() {
    const EXPECTED_SPAN: [felt252; 3] = [
        'attribute_name_32_bytes_longggg', 'g' + B31_3 + SHIFT_30B + B31_4,
        'data that is 31 bytes longggggg' + B31_1,
    ];
    let mut output: Array<felt252> = Default::default();
    let attribute = Attribute {
        name: "attribute_name_32_bytes_longgggg", data: Some("data that is 31 bytes longggggg"),
    };
    attribute.iserialize(ref output);
    assert(output.span() == EXPECTED_SPAN.span(), ' Array does not match');
}


#[test]
fn test_short_key_with_short_long_data() {
    const EXPECTED_SPAN: [felt252; 3] = [
        'key' + 3 * SHIFT_30B + B31_3 + B31_4, 'data that is longer than 31 byt',
        'es' + 2 * SHIFT_30B + B31_3,
    ];
    let mut output: Array<felt252> = Default::default();
    let attribute = Attribute { name: "key", data: Some("data that is longer than 31 bytes") };
    attribute.iserialize(ref output);
    assert(output.span() == EXPECTED_SPAN.span(), ' Array does not match');
}

#[test]
fn test_attribute_31_bytes_key_with_long_data() {
    const EXPECTED_SPAN: [felt252; 3] = [
        'attribute_name_31_bytes_longggg' + B31_1 + B31_4, 'data that is longer than 31 byt',
        'es' + 2 * SHIFT_30B + B31_3,
    ];
    let mut output: Array<felt252> = Default::default();
    let attribute = Attribute {
        name: "attribute_name_31_bytes_longggg", data: Some("data that is longer than 31 bytes"),
    };
    attribute.iserialize(ref output);
    assert(output.span() == EXPECTED_SPAN.span(), ' Array does not match');
}

#[test]
fn test_attribute_32_bytes_key_with_long_data() {
    const EXPECTED_SPAN: [felt252; 4] = [
        'attribute_name_32_bytes_longggg', 'g' + B31_3 + SHIFT_30B + B31_4,
        'data that is longer than 31 byt', 'es' + 2 * SHIFT_30B + B31_3,
    ];
    let mut output: Array<felt252> = Default::default();
    let attribute = Attribute {
        name: "attribute_name_32_bytes_longgggg", data: Some("data that is longer than 31 bytes"),
    };
    attribute.iserialize(ref output);
    assert(output.span() == EXPECTED_SPAN.span(), ' Array does not match');
}
