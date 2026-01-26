use crate::TableStructure;

pub trait Member<impl Table: TableStructure, T, const ID: felt252> {
    const ID: felt252;
    type Type;

    #[inline(always)]
    fn serialize_member(self: @Self::Type, ref data: Array<felt252>);
    fn serialize_member_inline(
        self: @Self::Type,
    ) -> Span<
        felt252,
    > {
        let mut data = array![];
        Self::serialize_member(self, ref data);
        data.span()
    }
}


pub mod impls {
    use introspect_types::ISerde;
    pub impl Impl<
        impl Table: super::TableStructure, T, const SELECTOR: felt252, +ISerde<T>,
    > of super::Member<Table, Table::Record, SELECTOR> {
        const ID: felt252 = SELECTOR;
        type Type = T;
        #[inline(always)]
        fn serialize_member(self: @Self::Type, ref data: Array<felt252>) {
            ISerde::iserialize(self, ref data);
        }
    }
}
