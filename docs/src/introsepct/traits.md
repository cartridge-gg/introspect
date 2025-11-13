# Introspect Trait

THe Introspect trait is used to parse types into a [`TypeDef`](./type_definitions.md) so they can be reconstructed by another system.

```rust
pub trait Introspect<T> {
    fn type_def() -> TypeDef;
    fn child_defs() -> Array<(felt252, TypeDef)> {
        Default::default()
    }
    fn hash() -> felt252 {
        let mut serialized: Array<felt252> = Default::default();
        Serde::<TypeDef>::serialize(@Self::type_def(), ref serialized);
        poseidon_hash_span(serialized.span())
    }
}
```

- `type_def` returns the type definition for the type so when a parent type is being constructed it can include the correct type definition for its members/variants.
- `child_defs` returns any additional type definitions that are needed to fully describe the type or any types down its hierarchy such as Ref types.
- `hash` returns a unique hash for the type definition which can be used to identify the type.

An Implementation of the Introspect trait for a struct could look like this:

```rust
use introspect::Introspect;
use introspect::type_definitions::{TypeDef, StructDef, MemberDef, merge_defs};
use crate::{AStruct, AnEnum};


struct MyStruct {
    field1: AStruct,
    field2: bool,
    filed3: AnEnum,
}

impl MyStructIntrospectImpl of Introspect<MyStruct>{
    fn type_def() -> TypeDef {
        TypeDef::Struct(StructDef {
            name: "MyStruct",
            attributes: [].span(),
            members: Span<MemberDef>::from_array([
                MemberDef {
                    name: "field1",
                    attributes: [].span(),
                    type_def: Introspect::<AnotherStruct>::type_def(),
                },
                MemberDef {
                    name: "field2",
                    attributes: [].span(),
                    type_def: TypeDef::<bool>::type_def(),
                },
                MemberDef {
                    name: "field3",
                    attributes: [].span(),
                    type_def: Introspect::<AnEnum>::type_def(),
                },
            ]),
        })
    }

    fn child_defs() -> Array<(felt252, TypeDef)> {
        merge_defs(array![
            Introspect::<AStruct>::child_defs(),
            Introspect::<AnEnum>::child_defs(),
        ])
    }
}
```
