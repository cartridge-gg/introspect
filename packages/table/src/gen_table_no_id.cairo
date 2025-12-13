use starknet::ContractAddress;

#[derive(Drop, Clone)]
pub struct Player {
    pub name: ByteArray,
    pub something: u8,
    pub address: ContractAddress,
}

//// GENERATED CODE BELOW

impl PlayerTableMeta of introspect_table::table::TableMeta {
    const ID: felt252 = selector!("Player");
    fn name() -> ByteArray {
        "Player"
    }
    fn attributes() -> Span<introspect_types::Attribute> {
        [].span()
    }
}


impl PlayerTablePrimary = introspect_table::table::table_primary::Default;


impl PlayerTableColumns of introspect_table::table::TableColumns {
    type Column = PlayerColumn;
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

///// Non Overridable

#[derive(Drop)]
pub enum PlayerColumn {
    name,
    something,
    address,
}

pub mod PlayerColumns {
    pub const name: felt252 = selector!("name");
    pub const something: felt252 = selector!("something");
    pub const address: felt252 = selector!("address");
}

pub impl PlayerTable =
    introspect_table::table::TableImpl<
        Player, PlayerTableMeta, PlayerTablePrimary, PlayerTableColumns,
    >;

pub impl IPlayerTable = introspect_table::table::ITableImpl<PlayerTable>;

pub impl Player_name_MemberImpl =
    introspect_table::table::iserde_table_member::Impl<PlayerTable, PlayerColumns::name, ByteArray>;

pub impl Player_something_MemberImpl =
    introspect_table::table::iserde_table_member::Impl<PlayerTable, PlayerColumns::something, u8>;
pub impl Player_address_MemberImpl =
    introspect_table::table::iserde_table_member::Impl<
        PlayerTable, PlayerColumns::address, ContractAddress,
    >;

impl PlayerColumnImpl<
    C, impl SS: introspect_table::Snapable<@C, PlayerColumn>,
> of introspect_table::table::ColumnId<C, PlayerTable> {
    const fn column_id(self: @C) -> felt252 {
        match SS::snapshot(self) {
            PlayerColumn::name => PlayerColumns::name,
            PlayerColumn::something => PlayerColumns::something,
            PlayerColumn::address => PlayerColumns::address,
        }
    }
}

impl PlayerRecordId of introspect_table::table::RecordId<felt252, PlayerTable> {
    fn record_id(self: @felt252) -> felt252 {
        *self
    }
}

impl PlayerRecordValuesSpan of introspect_table::table::RecordValuesSpanTrait<Player, PlayerTable> {
    fn serialize_values(self: @Player, ref data: Array<felt252>) {
        Player_name_MemberImpl::serialize_member(self.name, ref data);
        Player_something_MemberImpl::serialize_member(self.something, ref data);
        Player_address_MemberImpl::serialize_member(self.address, ref data);
    }
}
