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
    fn primary_def() -> introspect_types::PrimaryDef {
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

pub impl CharacterTableSchema =
    introspect_table::TableSchemaImpl<CharacterTableMeta, CharacterStructure>;

pub impl CharacterTable = introspect_table::TableImpl<CharacterTableSchema>;

pub impl CharacterTableSchema2 =
    introspect_table::TableSchemaImpl<CharacterTableMeta2, CharacterStructure>;

pub impl CharacterTable2 = introspect_table::TableImpl<CharacterTableSchema2>;

pub impl Character_name_MemberImpl<impl T: introspect_table::TableSchema[Record: Character]> =
    introspect_table::m_utils::TableMemberImpl<T, CharacterColumns::name, ByteArray>;

pub impl Character_something_MemberImpl<impl T: introspect_table::TableSchema[Record: Character]> =
    introspect_table::m_utils::TableMemberImpl<T, CharacterColumns::something, u8>;

impl CharacterRecordId<
    impl T: introspect_table::TableSchema[Record: Character],
> of introspect_table::RecordId<T::Record, T> {
    fn record_id(self: @T::Record) -> felt252 {
        introspect_types::PrimaryTrait::to_felt252(self.cid)
    }
}

impl CharacterRecordValuesSpan<
    impl T: introspect_table::TableSchema[Record: Character],
> of introspect_table::RecordValuesSpanTrait<T, T::Record> {
    fn serialize_values(self: @T::Record, ref data: Array<felt252>) {
        Character_name_MemberImpl::<T>::serialize_member(self.name, ref data);
        Character_something_MemberImpl::<T>::serialize_member(self.something, ref data);
    }
}
