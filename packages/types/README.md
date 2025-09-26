# Introspect

This package provides a way to describe the structure of data coming off the chain so that an indexer can decode the data.

The Ty enum describes the simple and complex types that can be used in structs and enums. The types are:

- Felt252: The `felt252` type to be read as a felt`
- Bool: The `bool` type to be read as a boolean.
- Uint8: The `u8` type to be read as an unsigned 8-bit integer.
- Uint16: The `u16` type to be read as an unsigned 16-bit integer.
- Uint32: The `u32` type to be read as an unsigned 32-bit integer.
- Uint64: The `u64` type to be read as an unsigned 64-bit integer.
- Uint128: The `u128` type to be read as an unsigned 128-bit integer.
- Uint256: The `u256` type to be read as an unsigned 256-bit integer.
- Int8: The `i8` type to be read as a signed 8-bit integer.
- Int16: The `i16` type to be read as a signed 16-bit integer.
- Int32: The `i32` type to be read as a signed 32-bit integer.
- Int64: The `i64` type to be read as a signed 64-bit integer.
- Int128: The `i128` type to be read as a signed 128-bit integer.
- USize: The `usize` type to be read as an unsigned size integer (system dependent).
- ShortString: A felt252 that should be interpreted as a short string (up to 31 bytes)
- ClassHash: A `ClassHash` representing a StarkNet class hash.
- ContractAddress: A `ContractAddress` representing a StarkNet contract address.
- EthAddress: A `EthAddress` representing an Ethereum address.
- ByteArray: A `ByteArray` that should be interpreted as a string.
- Tuple: A tuple of types.
- Array: A dynamic array of a type.
- FixedArray: A fixed-size array of a type and length.
- Custom: A custom type identified by a felt252 hash.
- Schema: A schema identified by a felt252 hash.
- Struct: A struct type defined by a name and fields.
- Enum: An enum type defined by a name and variants.
