use core::metaprogramming::TypeEqual;
use introspect_types::PrimaryTrait;

#[derive(Drop)]
pub struct AColumnGroup {
    pub name: ByteArray,
    pub something: u8,
}

#[derive(Drop)]
pub struct AKeyedColumnGroup {
    #[key]
    pub key_1: u128,
    #[key]
    pub key_2: ByteArray,
    pub name: ByteArray,
}


// #[derive(Drop)]
// struct AMissingKeyedColumnGroup {
//     #[key]
//     key_1: u128,
//     name: ByteArray,
// }

// #[derive(Drop)]
// struct ExtraValuesColumnGroup {
//     name: ByteArray,
//     something: u8,
//     something_else: u16,
// }

//// GENERATED CODE BELOW

mod AColumnGroup_column_selectors {
    pub const name: felt252 = selector!("name");
    pub const something: felt252 = selector!("something");
}

impl AColumnGroupColumnGroupImpl of introspect_table::table::ColumnGroupTrait<AColumnGroup, 2> {
    const GROUP_ID: felt252 = 12;
    const COLUMN_IDS: [felt252; 2] = [
        AColumnGroup_column_selectors::name, AColumnGroup_column_selectors::something,
    ];
}


impl AColumnGroupTableDataImpl<
    impl T: introspect_table::table::Table,
> of introspect_table::table::TableDataTrait<AColumnGroup, T> {
    fn data(self: @AColumnGroup) -> Span<felt252> {
        let mut data: Array<felt252> = Default::default();
        introspect_types::ISerde::iserialize(self.name, ref data);
        introspect_types::ISerde::iserialize(self.something, ref data);
        data.span()
    }
}

impl AColumnGroupTableHasColumns<
    impl T: introspect_table::table::Table,
    +introspect_table::table::MemberTrait<
        T::Record, ByteArray, AColumnGroup_column_selectors::name,
    >,
    +introspect_table::table::MemberTrait<T::Record, u8, AColumnGroup_column_selectors::something>,
> of introspect_table::table::TableHasColumnsTrait<AColumnGroup, T>;


mod AKeyedColumnGroup_column_selectors {
    pub const key_1: felt252 = selector!("key_1");
    pub const key_2: felt252 = selector!("key_2");
    pub const name: felt252 = selector!("name");
}

impl AKeyedColumnGroupColumnGroupImpl of introspect_table::table::ColumnGroupTrait<
    AKeyedColumnGroup, 3,
> {
    const GROUP_ID: felt252 = 13;
    const COLUMN_IDS: [felt252; 3] = [
        AKeyedColumnGroup_column_selectors::key_1, AKeyedColumnGroup_column_selectors::key_2,
        AKeyedColumnGroup_column_selectors::name,
    ];
}

impl AKeyedColumnGroupKeyImpl of introspect_table::table::KeyTrait<
    AKeyedColumnGroup,
    (u128, ByteArray),
    2,
    { [AKeyedColumnGroup_column_selectors::key_1, AKeyedColumnGroup_column_selectors::key_2] },
> {}


impl AKeyedColumnGroupKeySpanDataSpanImpl<
    impl T: introspect_table::table::Table,
> of introspect_table::table::KeySpanDataSpanTrait<AKeyedColumnGroup> {
    fn key_span_data_span(self: @AKeyedColumnGroup) -> Span<felt252> {
        let mut data: Array<felt252> = Default::default();
        introspect_types::ISerde::iserialize(self.key_1, ref data);
        introspect_types::ISerde::iserialize(self.key_2, ref data);
        data.span()
    }
}

impl AKeyedColumnGroupTableHasColumns<
    impl T: introspect_table::table::Table,
    +introspect_table::table::MemberTrait<
        T::Record, ByteArray, AKeyedColumnGroup_column_selectors::name,
    >,
> of introspect_table::table::TableHasColumnsTrait<AKeyedColumnGroup, T>;
// impl AKeyedColumnGroupTableIdDataImpl<
//     impl T: introspect_table::table::Table,
//     +introspect_table::table::KeySpanToPrimary<T>,
//     -introspect_table::table::KeyToPrimary<T>,
//     +Drop<T::Primary>,
//     +PrimaryTrait<T::Primary>,
// > of introspect_table::table::TableIdDataTrait<AKeyedColumnGroup, T> {
//     fn record_tuple(self: @AKeyedColumnGroup) -> (felt252, Span<felt252>) {
//         let id = introspect_types::ISerde::iserialize_inline(self.key_1);
//         let mut data: Array<felt252> = Default::default();
//         introspect_types::ISerde::iserialize(self.key_2, ref data);
//         introspect_types::ISerde::iserialize(self.name, ref data);
//         (id, data.span())
//     }
// }


