use introspect_types::serde::EMPTY_TERMINATOR;
use introspect_types::{ISerde, ISerdeByteArray};


fn test_iserde_byte_array(byte_array: ByteArray, expected: Span<felt252>) {
    let mut serialized = byte_array.iserialize_inline();
    assert(serialized == expected, 'Array does not match');
    assert(
        ISerde::ideserialize(ref serialized).unwrap() == byte_array, 'Deserialized doesnt match',
    );
}

#[test]
fn test_empty_byte_array() {
    const EXPECTED: [felt252; 1] = [EMPTY_TERMINATOR];
    let byte_array: ByteArray = "";
    test_iserde_byte_array(byte_array, EXPECTED.span());
}

#[test]
fn test_partial_byte_array() {
    const EXPECTED: [felt252; 1] = ['Remember the Cant'.partial_terminator(17)];
    let byte_array: ByteArray = "Remember the Cant";
    test_iserde_byte_array(byte_array, EXPECTED.span());
}

#[test]
fn test_31_bytes_array() {
    const EXPECTED: [felt252; 1] = ['what is this? a center for ants'.full_terminator()];
    let byte_array: ByteArray = "what is this? a center for ants";
    test_iserde_byte_array(byte_array, EXPECTED.span());
}

#[test]
fn test_longer_than_31_bytes() {
    const EXPECTED: [felt252; 2] = [
        'ubiquitous, mendacious, polyglo', 'ttal'.partial_terminator(4),
    ];
    let byte_array: ByteArray = "ubiquitous, mendacious, polyglottal";
    test_iserde_byte_array(byte_array, EXPECTED.span());
}

#[test]
fn test_61_bytes_array() {
    const EXPECTED: [felt252; 2] = [
        'Even the smallest person can ch', 'ange the course of the future.'.partial_terminator(30),
    ];
    let byte_array: ByteArray = "Even the smallest person can change the course of the future.";
    test_iserde_byte_array(byte_array, EXPECTED.span());
}

#[test]
fn test_62_bytes_array() {
    const EXPECTED: [felt252; 2] = [
        'Even the smallest person can ch', 'ange the course of the future..'.full_terminator(),
    ];
    let byte_array: ByteArray = "Even the smallest person can change the course of the future..";
    test_iserde_byte_array(byte_array, EXPECTED.span());
}


#[test]
fn test_long_bytes_array() {
    const EXPECTED: [felt252; 4] = [
        'One ring to rule them all, One ', 'ring to find them, One ring to ',
        'bring them all and in the darkn', 'ess bind them.'.partial_terminator(14),
    ];
    let byte_array: ByteArray =
        "One ring to rule them all, One ring to find them, One ring to bring them all and in the darkness bind them.";
    test_iserde_byte_array(byte_array, EXPECTED.span());
}

#[test]
#[fuzzer]
fn test_many_byte_arrays(byte_array: ByteArray) {
    let mut serialized = byte_array.iserialize_inline();
    assert(
        ISerde::ideserialize(ref serialized).unwrap() == byte_array, 'Deserialized doesnt match',
    );
}
