use starknet::ContractAddress;

#[derive(Drop, Clone)]
pub struct Character {
    #[key]
    pub cid: u128,
    pub name: ByteArray,
    pub something: u8,
    pub player: ContractAddress,
}

//// GENERATED CODE BELOW

impl CharacterTableMeta of introspect_table::TableMeta {
    const ID: felt252 = selector!("Character");
    fn name() -> ByteArray {
        "Character"
    }
    fn attributes() -> Span<introspect_types::Attribute> {
        [].span()
    }
}


impl CharacterTablePrimary of introspect_table::TablePrimary {
    type Primary = u128;
    fn primary_def() -> introspect_types::PrimaryDef {
        introspect_types::PrimaryDef {
            name: "cid", attributes: [].span(), type_def: introspect_types::PrimaryTypeDef::U128,
        }
    }
}


impl CharacterTableColumns of introspect_table::TableColumns {
    fn columns() -> Span<introspect_types::ColumnDef> {
        [
            introspect_types::ColumnDef {
                id: CharacterColumns::name,
                name: "name",
                attributes: [].span(),
                type_def: introspect_types::Introspect::<ByteArray>::type_def(),
            },
            introspect_types::ColumnDef {
                id: CharacterColumns::something,
                name: "something",
                attributes: [].span(),
                type_def: introspect_types::Introspect::<u8>::type_def(),
            },
            introspect_types::ColumnDef {
                id: CharacterColumns::player,
                name: "player",
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

pub mod CharacterColumns {
    pub const name: felt252 = selector!("name");
    pub const something: felt252 = selector!("something");
    pub const player: felt252 = selector!("player");
}

///// Non Overridable

pub impl CharacterTable =
    introspect_table::TableImpl<
        Character, CharacterTableMeta, CharacterTablePrimary, CharacterTableColumns,
    >;

pub impl ICharacterTable = introspect_table::ITableImpl<CharacterTable>;

pub impl Character_name_MemberImpl =
    introspect_table::table_member::Impl<CharacterTable, CharacterColumns::name, ByteArray>;

pub impl Character_something_MemberImpl =
    introspect_table::table_member::Impl<CharacterTable, CharacterColumns::something, u8>;

impl CharacterRecordId of introspect_table::RecordId<Character, CharacterTable> {
    fn record_id(self: @Character) -> felt252 {
        introspect_types::PrimaryTrait::to_felt252(self.cid)
    }
}

impl CharacterRecordValuesSpan of introspect_table::RecordValuesSpanTrait<
    Character, CharacterTable,
> {
    fn serialize_values(self: @Character, ref data: Array<felt252>) {
        Character_name_MemberImpl::serialize_member(self.name, ref data);
        Character_something_MemberImpl::serialize_member(self.something, ref data);
    }
}
