use introspect_types::ISerde;
use introspect_types::serde::{B31_2, B31_3, SHIFT_30B};

fn test_iserde_byte_array(byte_array: ByteArray, expected: Span<felt252>) {
    let mut serialized = byte_array.iserialize_inline();
    assert(serialized == expected, 'Array does not match');
    assert(ISerde::ideserialize_unwrap(ref serialized) == byte_array, 'Deserialized doesnt match');
}

#[test]
fn test_empty_byte_array() {
    const EXPECTED: [felt252; 1] = [B31_3];
    let byte_array: ByteArray = "";
    test_iserde_byte_array(byte_array, EXPECTED.span());
}

#[test]
fn test_partial_byte_array() {
    const EXPECTED: [felt252; 1] = ['Remember the Cant' + (17 * SHIFT_30B) + B31_3];
    let byte_array: ByteArray = "Remember the Cant";
    test_iserde_byte_array(byte_array, EXPECTED.span());
}

#[test]
fn test_31_bytes_array() {
    const EXPECTED: [felt252; 1] = ['what is this? a center for ants' + B31_2];
    let byte_array: ByteArray = "what is this? a center for ants";
    test_iserde_byte_array(byte_array, EXPECTED.span());
}

#[test]
fn test_longer_than_31_bytes() {
    const EXPECTED: [felt252; 2] = [
        'ubiquitous, mendacious, polyglo', 'ttal' + B31_3 + 4 * SHIFT_30B,
    ];
    let byte_array: ByteArray = "ubiquitous, mendacious, polyglottal";
    test_iserde_byte_array(byte_array, EXPECTED.span());
}

#[test]
fn test_61_bytes_array() {
    const EXPECTED: [felt252; 2] = [
        'Even the smallest person can ch',
        'ange the course of the future.' + B31_3 + (30 * SHIFT_30B),
    ];
    let byte_array: ByteArray = "Even the smallest person can change the course of the future.";
    test_iserde_byte_array(byte_array, EXPECTED.span());
}

#[test]
fn test_62_bytes_array() {
    const EXPECTED: [felt252; 2] = [
        'Even the smallest person can ch', 'ange the course of the future..' + B31_2,
    ];
    let byte_array: ByteArray = "Even the smallest person can change the course of the future..";
    test_iserde_byte_array(byte_array, EXPECTED.span());
}


#[test]
fn test_long_bytes_array() {
    const EXPECTED: [felt252; 4] = [
        'One ring to rule them all, One ', 'ring to find them, One ring to ',
        'bring them all and in the darkn', 'ess bind them.' + B31_3 + (14 * SHIFT_30B),
    ];
    let byte_array: ByteArray =
        "One ring to rule them all, One ring to find them, One ring to bring them all and in the darkness bind them.";
    test_iserde_byte_array(byte_array, EXPECTED.span());
}

#[test]
#[fuzzer]
fn test_many_byte_arrays(byte_array: ByteArray) {
    let mut serialized = byte_array.iserialize_inline();
    assert(ISerde::ideserialize_unwrap(ref serialized) == byte_array, 'Deserialized doesnt match');
}
