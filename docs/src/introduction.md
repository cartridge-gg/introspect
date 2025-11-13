# Introspection

One of the challenges with building on starknet is making chain data easily accessible and queryable to client side applications. Most apps will have to make there own systems to fetch, decode and make available the data stored on chain. This leads to a lot of duplicated effort, high barrier to entry for new developers in both time and skill.

Some standards such as ERC20 and ERC721 have helped with this by providing a common interface for certain data structures, but there is no general purpose standard for describing arbitrary data structures stored on chain.

Introspection provides a standardised way to reconstruct and interpret generic onchain data without requiring the ABI.
The two main types of data are variables and database-like structures.

Variables are single values stored that can be updated individually, such as configuration settings or global state.
Database-like structures are collections of records with fields organised into tables with defined schemas, similar to traditional databases.

The two main parts to reconstructing data from the chain is the structure and the data itself. The structure is defined using type definitions and schemas, while the data is serialised in a defined way to allow decoding.

The aim of this library is to provide a easy way for a standard user to make data on the chain indexable and queryable easily with very little effort and knowledge required but also provide the ability to modify and extend the system for developers who need more control and wish to provide application level optimisation.

## Introspect and TypeDef - defining structures

The [`TypeDef`](./introsepct/type_definitions.md) is used to represent and encode a type on chain. The [`Introspect`](./introsepct/traits.md) trait is used to allow easy creation of type definitions for custom types and parent types.
