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


impl PlayerTablePrimary = introspect_table::table_primary::Default;


impl PlayerTableColumns of introspect_table::TableColumns {
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

pub mod PlayerColumns {
    pub const name: felt252 = selector!("name");
    pub const something: felt252 = selector!("something");
    pub const address: felt252 = selector!("address");
}

///// Non Overridable

pub impl PlayerTable =
    introspect_table::TableImpl<Player, PlayerTableMeta, PlayerTablePrimary, PlayerTableColumns>;

pub impl IPlayerTable = introspect_table::ITableImpl<PlayerTable>;

pub impl Player_name_MemberImpl =
    introspect_table::m_utils::TableMemberImpl<PlayerTable, PlayerColumns::name, ByteArray>;

pub impl Player_something_MemberImpl =
    introspect_table::m_utils::TableMemberImpl<PlayerTable, PlayerColumns::something, u8>;
pub impl Player_address_MemberImpl =
    introspect_table::m_utils::TableMemberImpl<
        PlayerTable, PlayerColumns::address, ContractAddress,
    >;

impl PlayerRecordId = introspect_table::m_utils::RecordIdFelt252Impl<PlayerTable>;


impl PlayerRecordValuesSpan of introspect_table::RecordValuesSpanTrait<Player, PlayerTable> {
    fn serialize_values(self: @Player, ref data: Array<felt252>) {
        Player_name_MemberImpl::serialize_member(self.name, ref data);
        Player_something_MemberImpl::serialize_member(self.something, ref data);
        Player_address_MemberImpl::serialize_member(self.address, ref data);
    }
}
