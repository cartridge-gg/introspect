#[starknet::contract]
mod gen_{{name}}_type_contract {
    #[storage]
    struct Storage {}

    #[abi(embed_v0)]
    impl ITypeImpl = introspect::types::interfaces::TypeContractImpl<ContractState, {{full_name}}>;
}