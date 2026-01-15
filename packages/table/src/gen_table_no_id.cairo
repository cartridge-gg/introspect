use starknet::ContractAddress;

#[derive(Drop, Clone)]
pub struct Player {
    pub name: ByteArray,
    pub something: u8,
    pub address: ContractAddress,
}
//// GENERATED CODE BELOW

impl PlayerTableMeta of introspect_table::TableMeta {
    const ID: felt252 = selector!("Player");
    fn name() -> ByteArray {
        "Player"
    }
    fn attributes() -> Span<introspect_types::Attribute> {
        [].span()
    }
}

impl PlayerStructure of introspect_table::TableStructure {
    type Primary = felt252;
    type Record = Player;
    fn columns() -> Span<introspect_types::ColumnDef> {
        [
            introspect_types::ColumnDef {
                id: PlayerColumns::name,
                name: "name",
                attributes: [].span(),
                type_def: introspect_types::Introspect::<ByteArray>::type_def(),
            },
            introspect_types::ColumnDef {
                id: PlayerColumns::something,
                name: "something",
                attributes: [].span(),
                type_def: introspect_types::Introspect::<u8>::type_def(),
            },
            introspect_types::ColumnDef {
                id: PlayerColumns::address,
                name: "address",
                attributes: [].span(),
                type_def: introspect_types::Introspect::<ContractAddress>::type_def(),
            },
        ]
            .span()
    }
    fn child_defs() -> Array<(felt252, introspect_types::TypeDef)> {
        Default::default()
    }
}


// ///// Non Overridable

pub mod PlayerColumns {
    pub const name: felt252 = selector!("name");
    pub const something: felt252 = selector!("something");
    pub const address: felt252 = selector!("address");
}

pub impl PlayerTableSchema = introspect_table::TableSchemaImpl<PlayerTableMeta, PlayerStructure>;
pub impl PlayerTable = introspect_table::TableImpl<PlayerTableSchema>;


pub impl Player_name_MemberImpl<impl T: introspect_table::TableSchema[Record: Player]> =
    introspect_table::m_utils::TableMemberImpl<T, PlayerColumns::name, ByteArray>;
pub impl Player_something_MemberImpl<impl T: introspect_table::TableSchema[Record: Player]> =
    introspect_table::m_utils::TableMemberImpl<T, PlayerColumns::something, u8>;
pub impl Player_address_MemberImpl<impl T: introspect_table::TableSchema[Record: Player]> =
    introspect_table::m_utils::TableMemberImpl<T, PlayerColumns::address, ContractAddress>;

// impl PlayerRecordId<
//     K, impl T: introspect_table::TableSchema[Primary: u128, Record: Player], +Snapable<@K, u128>,
// > =
//     introspect_table::table::TablePrimaryIdImpl<K, PlayerTableSchema>;
// impl CharacterRecordId<
//     impl T: introspect_table::TableSchema[Primary: felt252],
// > of introspect_table::RecordId<T::Primary, T> {
//     fn record_id(self: @T::Primary) -> felt252 {
//         introspect_types::PrimaryTrait::to_felt252(self)
//     }
// }

pub impl PlayerRecordValuesSpan<
    impl T: introspect_table::TableSchema[Record: Player],
> of introspect_table::RecordValuesSpanTrait<T, T::Record> {
    fn serialize_values(self: @T::Record, ref data: Array<felt252>) {
        Player_name_MemberImpl::<T>::serialize_member(self.name, ref data);
        Player_something_MemberImpl::<T>::serialize_member(self.something, ref data);
        Player_address_MemberImpl::<T>::serialize_member(self.address, ref data);
    }
}

