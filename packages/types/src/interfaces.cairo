use crate::{IntrospectT, TypeDef};


#[starknet::interface]
pub trait ITypeDefClassContract<TState> {
    fn type_def(self: @TState) -> TypeDef;
}

#[starknet::embeddable]
impl TypeDefContractImpl<
    TContractState, T, +IntrospectT<T>,
> of ITypeDefClassContract<TContractState> {
    fn type_def(self: @TContractState) -> TypeDef {
        IntrospectT::<T>::type_def()
    }
}
// #[starknet::interface]
// pub trait ISchemaContract<TState> {
//     fn columns(self: @TState) -> Span<ColumnDef>;
// }

// #[starknet::embeddable]
// impl SchemaContractImpl<TContractState, T, +Schema<T>> of ISchemaContract<TContractState> {
//     fn columns(self: @TContractState) -> Span<ColumnDef> {
//         Schema::<T>::columns()
//     }
// }


