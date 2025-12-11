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

#[derive(Drop)]
pub struct AnIdColumnGroup {
    #[key]
    pub id: felt252,
    pub something: u8,
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


pub impl AColumnGroupTableDataImpl<
    impl T: introspect_table::table::Table,
    impl name_Member: introspect_table::table::MemberTrait<
        T, ByteArray, AColumnGroup_column_selectors::name,
    >,
    impl something_Member: introspect_table::table::MemberTrait<
        T, u8, AColumnGroup_column_selectors::something,
    >,
> of introspect_table::table::TableData<AColumnGroup, T> {
    fn data(self: @AColumnGroup) -> Span<felt252> {
        let mut data: Array<felt252> = Default::default();
        name_Member::serialize_member(self.name, ref data);
        something_Member::serialize_member(self.something, ref data);
        data.span()
    }
}

// impl AColumnGroupTableHasColumns<
//     impl T: introspect_table::table::Table,
//     +introspect_table::table::MemberTrait<T, ByteArray, AColumnGroup_column_selectors::name>,
//     +introspect_table::table::MemberTrait<T, u8, AColumnGroup_column_selectors::something>,
// > of introspect_table::table::TableHasColumnsTrait<AColumnGroup, T>;

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


impl AKeyedColumnGroupKeySpanDataSpanImpl<
    impl T: introspect_table::table::Table,
    impl key_1_Member: introspect_table::table::MemberTrait<
        T, u128, AKeyedColumnGroup_column_selectors::key_1,
    >,
    impl key_2_Member: introspect_table::table::MemberTrait<
        T, ByteArray, AKeyedColumnGroup_column_selectors::key_2,
    >,
    impl name_Member: introspect_table::table::MemberTrait<
        T, ByteArray, AKeyedColumnGroup_column_selectors::name,
    >,
    +introspect_table::table::TableId<(u128, ByteArray), T>,
> of introspect_table::table::KeySpanDataSpanTrait<AKeyedColumnGroup, T> {
    fn serialize_keys(self: @AKeyedColumnGroup, ref keys: Array<felt252>) {
        key_1_Member::serialize_member(self.key_1, ref keys);
        key_2_Member::serialize_member(self.key_2, ref keys);
    }
    fn serialize_values(self: @AKeyedColumnGroup, ref values: Array<felt252>) {
        name_Member::serialize_member(self.name, ref values);
    }
}


mod AnIdColumnGroup_column_selectors {
    pub const something: felt252 = selector!("something");
}

impl AnIdColumnGroupColumnGroupImpl of introspect_table::table::ColumnGroupTrait<
    AnIdColumnGroup, 1,
> {
    const GROUP_ID: felt252 = 14;
    const COLUMN_IDS: [felt252; 1] = [AnIdColumnGroup_column_selectors::something,];
}


impl AnIdColumnGroupImpl<
    impl T: introspect_table::table::Table,
    impl something_Member: introspect_table::table::MemberTrait<
        T, u8, AnIdColumnGroup_column_selectors::something,
    >,
    +introspect_table::table::TableId<felt252, T>,
> of introspect_table::table::TableIdData<AnIdColumnGroup, T> {
    fn record_tuple(self: @AnIdColumnGroup) -> (felt252, Span<felt252>) {
        let mut data: Array<felt252> = Default::default();
        something_Member::serialize_member(self.something, ref data);
        (self.id.to_felt252(), data.span())
    }
}
// impl AKeyedColumnGroupTableHasColumns<
//     impl T: introspect_table::table::Table,
//     +introspect_table::table::MemberTrait<T, ByteArray,
//     AKeyedColumnGroup_column_selectors::name>,
// > of introspect_table::table::TableHasColumnsTrait<AKeyedColumnGroup, T>;
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


