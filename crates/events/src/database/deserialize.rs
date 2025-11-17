use super::{
    AddColumn, AddColumns, CreateColumnGroup, CreateTable, CreateTableFromClassHash,
    CreateTableWithColumns, DeleteField, DeleteFieldGroup, DeleteFieldGroups, DeleteFields,
    DeleteRecord, DeleteRecords, DeletesField, DeletesFieldGroup, DeletesFieldGroups,
    DeletesFields, DropColumn, DropColumns, DropTable, IdData, IdName, IdTypeAttributes,
    InsertField, InsertFieldGroup, InsertFieldGroups, InsertFields, InsertRecord, InsertRecords,
    InsertsField, InsertsFieldGroup, InsertsFieldGroups, InsertsFields, RenameColumn,
    RenameColumns, RenamePrimary, RenameTable, RetypeColumn, RetypeColumns, RetypePrimary,
};
use crate::event::EventTrait;
use introspect_types::schema::{PrimaryDef, PrimaryTypeDef};
use introspect_types::{
    Attribute, CairoDeserialize, ColumnDef, FeltIterator, TypeDef, deserialize_byte_array_string,
};
use starknet::macros::selector;
use starknet_types_core::felt::Felt;

impl EventTrait for CreateColumnGroup {
    const SELECTOR: Felt = selector!("CreateColumnGroup");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        CreateColumnGroup {
            id: keys.next()?,
            columns: Vec::<Felt>::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}

impl EventTrait for CreateTable {
    const SELECTOR: Felt = selector!("CreateTable");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        CreateTable {
            id: keys.next()?,
            name: deserialize_byte_array_string(data)?,
            attributes: Vec::<Attribute>::c_deserialize(data)?,
            primary: PrimaryDef::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}

impl EventTrait for CreateTableWithColumns {
    const SELECTOR: Felt = selector!("CreateTableWithColumns");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        CreateTableWithColumns {
            id: keys.next()?,
            name: deserialize_byte_array_string(data)?,
            attributes: Vec::<Attribute>::c_deserialize(data)?,
            primary: PrimaryDef::c_deserialize(data)?,
            columns: Vec::<ColumnDef>::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}

impl EventTrait for CreateTableFromClassHash {
    const SELECTOR: Felt = selector!("CreateTableFromClassHash");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        CreateTableFromClassHash {
            id: keys.next()?,
            name: deserialize_byte_array_string(data)?,
            class_hash: data.next()?,
        }
        .verify(keys, data)
    }
}

impl EventTrait for RenameTable {
    const SELECTOR: Felt = selector!("RenameTable");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        RenameTable {
            id: keys.next()?,
            name: deserialize_byte_array_string(data)?,
        }
        .verify(keys, data)
    }
}

impl EventTrait for DropTable {
    const SELECTOR: Felt = selector!("DropTable");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        let id = keys.next()?;
        DropTable { id }.verify(keys, data)
    }
}
impl EventTrait for RenamePrimary {
    const SELECTOR: Felt = selector!("RenamePrimary");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        RenamePrimary {
            table: keys.next()?,
            name: deserialize_byte_array_string(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for RetypePrimary {
    const SELECTOR: Felt = selector!("RetypePrimary");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        RetypePrimary {
            table: keys.next()?,
            attributes: Vec::<Attribute>::c_deserialize(data)?,
            type_def: PrimaryTypeDef::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for AddColumn {
    const SELECTOR: Felt = selector!("AddColumn");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        AddColumn {
            table: keys.next()?,
            id: keys.next()?,
            name: deserialize_byte_array_string(data)?,
            attributes: Vec::<Attribute>::c_deserialize(data)?,
            type_def: TypeDef::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for AddColumns {
    const SELECTOR: Felt = selector!("AddColumns");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        AddColumns {
            table: keys.next()?,
            columns: Vec::<ColumnDef>::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for RenameColumn {
    const SELECTOR: Felt = selector!("RenameColumn");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        RenameColumn {
            table: keys.next()?,
            id: keys.next()?,
            name: deserialize_byte_array_string(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for RenameColumns {
    const SELECTOR: Felt = selector!("RenameColumns");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        RenameColumns {
            table: keys.next()?,
            columns: Vec::<IdName>::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for RetypeColumn {
    const SELECTOR: Felt = selector!("RetypeColumn");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        RetypeColumn {
            table: keys.next()?,
            id: keys.next()?,
            attributes: Vec::<Attribute>::c_deserialize(data)?,
            type_def: TypeDef::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for RetypeColumns {
    const SELECTOR: Felt = selector!("RetypeColumns");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        RetypeColumns {
            table: keys.next()?,
            columns: Vec::<IdTypeAttributes>::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for DropColumn {
    const SELECTOR: Felt = selector!("DropColumn");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        DropColumn {
            table: keys.next()?,
            id: keys.next()?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for DropColumns {
    const SELECTOR: Felt = selector!("DropColumns");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        DropColumns {
            table: keys.next()?,
            ids: Vec::<Felt>::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for InsertRecord {
    const SELECTOR: Felt = selector!("InsertRecord");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        InsertRecord {
            table: keys.next()?,
            record: keys.next()?,
            data: Vec::<Felt>::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for InsertRecords {
    const SELECTOR: Felt = selector!("InsertRecords");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        InsertRecords {
            table: keys.next()?,
            records_data: Vec::<IdData>::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for InsertField {
    const SELECTOR: Felt = selector!("InsertField");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        InsertField {
            table: keys.next()?,
            column: keys.next()?,
            record: keys.next()?,
            data: Vec::<Felt>::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for InsertFields {
    const SELECTOR: Felt = selector!("InsertFields");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        InsertFields {
            table: keys.next()?,
            record: keys.next()?,
            columns: Vec::<Felt>::c_deserialize(data)?,
            data: Vec::<Felt>::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for InsertsField {
    const SELECTOR: Felt = selector!("InsertsField");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        InsertsField {
            table: keys.next()?,
            column: keys.next()?,
            records_data: Vec::<IdData>::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for InsertsFields {
    const SELECTOR: Felt = selector!("InsertsFields");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        InsertsFields {
            table: keys.next()?,
            columns: Vec::<Felt>::c_deserialize(data)?,
            records_data: Vec::<IdData>::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for InsertFieldGroup {
    const SELECTOR: Felt = selector!("InsertFieldGroup");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        InsertFieldGroup {
            table: keys.next()?,
            record: keys.next()?,
            group: keys.next()?,
            data: Vec::<Felt>::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}

impl EventTrait for InsertFieldGroups {
    const SELECTOR: Felt = selector!("InsertFieldGroups");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        InsertFieldGroups {
            table: keys.next()?,
            record: keys.next()?,
            groups: Vec::<Felt>::c_deserialize(data)?,
            data: Vec::<Felt>::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}

impl EventTrait for InsertsFieldGroup {
    const SELECTOR: Felt = selector!("InsertsFieldGroup");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        InsertsFieldGroup {
            table: keys.next()?,
            group: keys.next()?,
            records_data: Vec::<IdData>::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}

impl EventTrait for InsertsFieldGroups {
    const SELECTOR: Felt = selector!("InsertsFieldGroups");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        InsertsFieldGroups {
            table: keys.next()?,
            record: keys.next()?,
            groups: Vec::<Felt>::c_deserialize(data)?,
            records_data: Vec::<IdData>::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}

impl EventTrait for DeleteRecord {
    const SELECTOR: Felt = selector!("DeleteRecord");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        DeleteRecord {
            table: keys.next()?,
            record: keys.next()?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for DeleteRecords {
    const SELECTOR: Felt = selector!("DeleteRecords");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        DeleteRecords {
            table: keys.next()?,
            records: Vec::<Felt>::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for DeleteField {
    const SELECTOR: Felt = selector!("DeleteField");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        DeleteField {
            table: keys.next()?,
            record: keys.next()?,
            column: keys.next()?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for DeleteFields {
    const SELECTOR: Felt = selector!("DeleteFields");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        DeleteFields {
            table: keys.next()?,
            record: keys.next()?,
            columns: Vec::<Felt>::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for DeletesField {
    const SELECTOR: Felt = selector!("DeletesField");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        DeletesField {
            table: keys.next()?,
            column: keys.next()?,
            records: Vec::<Felt>::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for DeletesFields {
    const SELECTOR: Felt = selector!("DeletesFields");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        DeletesFields {
            table: keys.next()?,
            records: Vec::<Felt>::c_deserialize(data)?,
            columns: Vec::<Felt>::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}

impl EventTrait for DeleteFieldGroup {
    const SELECTOR: Felt = selector!("DeleteFieldGroup");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        DeleteFieldGroup {
            table: keys.next()?,
            record: keys.next()?,
            group: keys.next()?,
        }
        .verify(keys, data)
    }
}

impl EventTrait for DeleteFieldGroups {
    const SELECTOR: Felt = selector!("DeleteFieldGroups");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        DeleteFieldGroups {
            table: keys.next()?,
            record: keys.next()?,
            groups: Vec::<Felt>::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}

impl EventTrait for DeletesFieldGroup {
    const SELECTOR: Felt = selector!("DeletesFieldGroup");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        DeletesFieldGroup {
            table: keys.next()?,
            group: keys.next()?,
            records: Vec::<Felt>::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}

impl EventTrait for DeletesFieldGroups {
    const SELECTOR: Felt = selector!("DeletesFieldGroups");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        DeletesFieldGroups {
            table: keys.next()?,
            records: Vec::<Felt>::c_deserialize(data)?,
            groups: Vec::<Felt>::c_deserialize(data)?,
        }
        .verify(keys, data)
    }
}

impl CairoDeserialize for IdData {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        let id = data.next()?;
        let data = Vec::<Felt>::c_deserialize(data)?;
        Some(IdData { id, data })
    }
}

impl CairoDeserialize for IdName {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        let id = data.next()?;
        let name = deserialize_byte_array_string(data)?;
        Some(IdName { id, name })
    }
}

impl CairoDeserialize for IdTypeAttributes {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        let id = data.next()?;
        let attributes = Vec::<Attribute>::c_deserialize(data)?;
        let type_def = TypeDef::c_deserialize(data)?;
        Some(IdTypeAttributes {
            id,
            attributes,
            type_def,
        })
    }
}
