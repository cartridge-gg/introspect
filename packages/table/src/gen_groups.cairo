#[derive(Drop, Clone)]
pub struct AColumnGroup {
    pub name: ByteArray,
    pub something: u8,
}

#[derive(Drop, Clone)]
pub struct AKeyedColumnGroup {
    #[key]
    pub key_1: u128,
    #[key]
    pub key_2: ByteArray,
    pub name: ByteArray,
}

#[derive(Drop, Clone)]
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

pub impl AColumnGroupTableDataImpl<
    impl T: introspect_table::Table,
    impl name_Member: introspect_table::MemberTrait<
        T::Record, T, AColumnGroup_column_selectors::name,
    >,
    impl something_Member: introspect_table::MemberTrait<
        T::Record, T, AColumnGroup_column_selectors::something,
    >,
    +core::metaprogramming::TypeEqual<name_Member::Type, ByteArray>,
    +core::metaprogramming::TypeEqual<something_Member::Type, u8>,
> of introspect_table::FieldOnlyColumnGroup<AColumnGroup, 2, T> {
    const GROUP_ID: felt252 = 12;
    const COLUMN_IDS: [felt252; 2] = [
        AColumnGroup_column_selectors::name, AColumnGroup_column_selectors::something,
    ];
    fn group_data(self: @AColumnGroup) -> Span<felt252> {
        let mut data: Array<felt252> = Default::default();
        name_Member::serialize_member(self.name, ref data);
        something_Member::serialize_member(self.something, ref data);
        data.span()
    }
}

mod AKeyedColumnGroup_column_selectors {
    pub const key_1: felt252 = selector!("key_1");
    pub const key_2: felt252 = selector!("key_2");
    pub const name: felt252 = selector!("name");
}

impl AKeyedColumnGroupKeySpanDataSpanImpl<
    impl T: introspect_table::Table,
    impl key_1_Member: introspect_table::MemberTrait<
        T::Record, T, AKeyedColumnGroup_column_selectors::key_1,
    >[Type: u128],
    impl key_2_Member: introspect_table::MemberTrait<
        T::Record, T, AKeyedColumnGroup_column_selectors::key_2,
    >[Type: ByteArray],
    impl name_Member: introspect_table::MemberTrait<
        T::Record, T, AKeyedColumnGroup_column_selectors::name,
    >[Type: ByteArray],
    +introspect_table::RecordId<(u128, ByteArray), T>,
    impl SK: introspect_table::SerialisedKey<T::Record, (@u128, @ByteArray), T>,
    impl KI: introspect_table::KeySpanToId<T::Record, T>,
    impl RK: introspect_table::RecordKey<T::Record, (@u128, @ByteArray), T>,
    +core::metaprogramming::TypeEqual<RK::Key, (u128, ByteArray)>,
> of introspect_table::IdColumnGroup<AKeyedColumnGroup, 3, T> {
    const GROUP_ID: felt252 = 13;
    const COLUMN_IDS: [felt252; 3] = [
        AKeyedColumnGroup_column_selectors::key_1, AKeyedColumnGroup_column_selectors::key_2,
        AKeyedColumnGroup_column_selectors::name,
    ];
    fn group_tuple(self: @AKeyedColumnGroup) -> (felt252, Span<felt252>) {
        let mut data: Array<felt252> = Default::default();
        SK::serialize_key(@(self.key_1, self.key_2), ref data);
        let id = KI::key_span_to_id(data.span());
        name_Member::serialize_member(self.name, ref data);
        (id, data.span())
    }
}

mod AnIdColumnGroup_column_selectors {
    pub const something: felt252 = selector!("something");
}

impl AnIdColumnGroupImpl<
    impl T: introspect_table::Table,
    impl something_Member: introspect_table::MemberTrait<
        T::Record, T, AnIdColumnGroup_column_selectors::something,
    >[Type: u8],
    +core::metaprogramming::TypeEqual<T::Primary, felt252>,
> of introspect_table::IdColumnGroup<AnIdColumnGroup, 1, T> {
    const GROUP_ID: felt252 = 14;
    const COLUMN_IDS: [felt252; 1] = [AnIdColumnGroup_column_selectors::something,];
    fn group_tuple(self: @AnIdColumnGroup) -> (felt252, Span<felt252>) {
        let mut data: Array<felt252> = Default::default();
        something_Member::serialize_member(self.something, ref data);
        (introspect_types::PrimaryTrait::to_felt252(self.id), data.span())
    }
}

