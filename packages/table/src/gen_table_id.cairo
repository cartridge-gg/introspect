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

impl CharacterTableMeta2 of introspect_table::TableMeta {
    const ID: felt252 = selector!("Character");
    fn name() -> ByteArray {
        "Character"
    }
    fn attributes() -> Span<introspect_types::Attribute> {
        [].span()
    }
}


impl CharacterStructure of introspect_table::TableStructure {
    type Primary = u128;
    type Record = Character;
    fn primary() -> introspect_types::PrimaryDef {
        introspect_types::PrimaryDef {
            name: "cid", attributes: [].span(), type_def: introspect_types::PrimaryTypeDef::U128,
        }
    }
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

pub impl CharacterTable = introspect_table::TableImpl<CharacterStructure, CharacterTableMeta>;

pub impl CharacterTable2 = introspect_table::TableImpl<CharacterStructure, CharacterTableMeta2>;

pub impl Character_name_MemberImpl =
    introspect_table::m_utils::TableMemberImpl<
        CharacterStructure, CharacterColumns::name, ByteArray,
    >;

pub impl Character_something_MemberImpl =
    introspect_table::m_utils::TableMemberImpl<CharacterStructure, CharacterColumns::something, u8>;


impl CharacterRecordValuesSpan of introspect_table::RecordValuesSpanTrait<
    CharacterStructure, Character,
> {
    fn serialize_values(self: @Character, ref data: Array<felt252>) {
        Character_name_MemberImpl::serialize_member(self.name, ref data);
        Character_something_MemberImpl::serialize_member(self.something, ref data);
    }
}

impl CharacterRecordId<> of introspect_table::RecordId<Character, CharacterStructure> {
    fn record_id(self: @Character) -> felt252 {
        introspect_types::PrimaryTrait::to_felt252(self.cid)
    }
}
