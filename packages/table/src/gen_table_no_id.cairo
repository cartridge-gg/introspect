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

pub impl PlayerTable = introspect_table::TableImpl<PlayerStructure, PlayerTableMeta>;


pub impl Player_name_MemberImpl =
    introspect_table::m_utils::TableMemberImpl<PlayerStructure, PlayerColumns::name, ByteArray>;
pub impl Player_something_MemberImpl =
    introspect_table::m_utils::TableMemberImpl<PlayerStructure, PlayerColumns::something, u8>;
pub impl Player_address_MemberImpl =
    introspect_table::m_utils::TableMemberImpl<
        PlayerStructure, PlayerColumns::address, ContractAddress,
    >;

// impl PlayerRecordId<
//     K, impl T: introspect_table::TableStructure[Primary: u128, Record: Player], +Snapable<@K,
//     u128>,
// > =
//     introspect_table::table::TablePrimaryIdImpl<K, PlayerTableSchema>;
// impl CharacterRecordId<
//     impl T: introspect_table::TableStructure[Primary: felt252],
// > of introspect_table::RecordId<T::Primary, T> {
//     fn record_id(self: @T::Primary) -> felt252 {
//         introspect_types::PrimaryTrait::to_felt252(self)
//     }
// }

pub impl PlayerRecordValuesSpan of introspect_table::RecordValuesSpanTrait<
    PlayerStructure, Player,
> {
    fn serialize_values(self: @Player, ref data: Array<felt252>) {
        Player_name_MemberImpl::serialize_member(self.name, ref data);
        Player_something_MemberImpl::serialize_member(self.something, ref data);
        Player_address_MemberImpl::serialize_member(self.address, ref data);
    }
}

