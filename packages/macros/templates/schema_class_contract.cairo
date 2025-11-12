#[starknet::contract]
mod gen_{{name}}_schema_contract {
    #[storage]
    struct Storage {}

    #[abi(embed_v0)]
    impl ISchemaImpl = introspect::types::interfaces::SchemaContractImpl<ContractState, {{full_name}}>;
}