use super::{
    AddColumn, AddColumns, CreateColumnSet, CreateIndex, CreateTable, CreateTableFromClassHash,
    CreateTableWithColumns, DeleteField, DeleteFieldSet, DeleteFieldSets, DeleteFields,
    DeleteRecord, DeleteRecords, DeletesField, DeletesFieldSet, DeletesFieldSets, DeletesFields,
    DropColumn, DropColumns, DropIndex, DropTable, Entry, IdName, IdTypeDef, InsertField,
    InsertFieldSet, InsertFieldSets, InsertFields, InsertRecord, InsertRecords, InsertsField,
    InsertsFieldSet, InsertsFieldSets, InsertsFields, RenameColumn, RenameColumns, RenamePrimary,
    RenameTable, RetypeColumn, RetypeColumns, RetypePrimary,
};
use crate::event::EventTrait;
use introspect_types::schema::{PrimaryDef, PrimaryTypeDef};
use introspect_types::utils::ideserialize_utf8_string;
use introspect_types::{
    Attribute, ColumnDef, FeltIterator, ISerde, ISerdeEnd, TypeDef, ascii_str_to_limbs,
};
use starknet_types_core::felt::Felt;

impl EventTrait for CreateColumnSet {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("CreateFieldSet");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        CreateColumnSet {
            id: data.next()?,
            columns: data.collect(),
        }
        .verify_keys(keys)
    }
}

impl EventTrait for CreateTable {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("CreateTable");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        CreateTable {
            id: data.next()?,
            name: ideserialize_utf8_string(data)?,
            primary: PrimaryDef::ideserialize(data)?,
            attributes: Attribute::ideserialize_end(data)?,
        }
        .verify(keys, data)
    }
}

impl EventTrait for CreateTableWithColumns {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("CreateTableWithColumns");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        CreateTableWithColumns {
            id: data.next()?,
            name: ideserialize_utf8_string(data)?,
            attributes: Vec::<Attribute>::ideserialize(data)?,
            primary: PrimaryDef::ideserialize(data)?,
            columns: ColumnDef::ideserialize_end(data)?,
        }
        .verify(keys, data)
    }
}

impl EventTrait for CreateTableFromClassHash {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("CreateTableFromClassHash");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        CreateTableFromClassHash {
            id: data.next()?,
            name: ideserialize_utf8_string(data)?,
            class_hash: data.next()?,
        }
        .verify(keys, data)
    }
}

impl EventTrait for RenameTable {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("RenameTable");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        RenameTable {
            id: data.next()?,
            name: ideserialize_utf8_string(data)?,
        }
        .verify(keys, data)
    }
}

impl EventTrait for DropTable {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DropTable");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        DropTable { id: data.next()? }.verify(keys, data)
    }
}

impl EventTrait for RenamePrimary {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("RenamePrimary");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        RenamePrimary {
            table: data.next()?,
            name: ideserialize_utf8_string(data)?,
        }
        .verify(keys, data)
    }
}

impl EventTrait for RetypePrimary {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("RetypePrimary");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        RetypePrimary {
            table: data.next()?,
            type_def: PrimaryTypeDef::ideserialize(data)?,
            attributes: Attribute::ideserialize_end(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for AddColumn {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("AddColumn");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        AddColumn {
            table: data.next()?,
            id: data.next()?,
            name: ideserialize_utf8_string(data)?,
            attributes: Vec::<Attribute>::ideserialize(data)?,
            type_def: TypeDef::ideserialize(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for AddColumns {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("AddColumns");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        AddColumns {
            table: data.next()?,
            columns: ColumnDef::ideserialize_end(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for RenameColumn {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("RenameColumn");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        RenameColumn {
            table: data.next()?,
            id: data.next()?,
            name: ideserialize_utf8_string(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for RenameColumns {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("RenameColumns");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        RenameColumns {
            table: data.next()?,
            columns: IdName::ideserialize_end(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for RetypeColumn {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("RetypeColumn");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        RetypeColumn {
            table: data.next()?,
            id: data.next()?,
            attributes: Vec::<Attribute>::ideserialize(data)?,
            type_def: TypeDef::ideserialize(data)?,
        }
        .verify(keys, data)
    }
}

impl EventTrait for RetypeColumns {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("RetypeColumns");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        RetypeColumns {
            table: data.next()?,
            columns: IdTypeDef::ideserialize_end(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for DropColumn {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DropColumn");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        DropColumn {
            table: data.next()?,
            id: data.next()?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for DropColumns {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DropColumns");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        DropColumns {
            table: data.next()?,
            ids: data.collect(),
        }
        .verify(keys, data)
    }
}

impl EventTrait for CreateIndex {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("CreateIndex");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        CreateIndex {
            table: data.next()?,
            id: data.next()?,
            name: data.next()?,
            columns: data.collect(),
        }
        .verify(keys, data)
    }
}

impl EventTrait for DropIndex {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DropIndex");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        DropIndex {
            table: data.next()?,
            id: data.next()?,
        }
        .verify(keys, data)
    }
}

impl EventTrait for InsertRecord {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("InsertRecord");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        InsertRecord {
            table: data.next()?,
            record: data.next()?,
            data: data.collect(),
        }
        .verify(keys, data)
    }
}
impl EventTrait for InsertRecords {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("InsertRecords");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        InsertRecords {
            table: data.next()?,
            records_data: Entry::ideserialize_end(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for InsertField {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("InsertField");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        InsertField {
            table: data.next()?,
            column: data.next()?,
            record: data.next()?,
            data: data.collect(),
        }
        .verify(keys, data)
    }
}
impl EventTrait for InsertFields {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("InsertFields");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        InsertFields {
            table: data.next()?,
            record: data.next()?,
            columns: Vec::<Felt>::ideserialize(data)?,
            data: data.collect(),
        }
        .verify(keys, data)
    }
}
impl EventTrait for InsertsField {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("InsertsField");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        InsertsField {
            table: data.next()?,
            column: data.next()?,
            records_data: Entry::ideserialize_end(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for InsertsFields {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("InsertsFields");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        InsertsFields {
            table: data.next()?,
            columns: Vec::<Felt>::ideserialize(data)?,
            records_data: Entry::ideserialize_end(data)?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for InsertFieldSet {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("InsertFieldSet");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        InsertFieldSet {
            table: data.next()?,
            record: data.next()?,
            group: data.next()?,
            data: data.collect(),
        }
        .verify(keys, data)
    }
}

impl EventTrait for InsertFieldSets {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("InsertFieldSets");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        InsertFieldSets {
            table: data.next()?,
            record: data.next()?,
            groups: Vec::<Felt>::ideserialize(data)?,
            data: data.collect(),
        }
        .verify(keys, data)
    }
}

impl EventTrait for InsertsFieldSet {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("InsertsFieldSet");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        InsertsFieldSet {
            table: data.next()?,
            group: data.next()?,
            records_data: Entry::ideserialize_end(data)?,
        }
        .verify(keys, data)
    }
}

impl EventTrait for InsertsFieldSets {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("InsertsFieldSets");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        InsertsFieldSets {
            table: data.next()?,
            record: data.next()?,
            groups: Vec::<Felt>::ideserialize(data)?,
            records_data: Entry::ideserialize_end(data)?,
        }
        .verify(keys, data)
    }
}

impl EventTrait for DeleteRecord {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DeleteRecord");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        DeleteRecord {
            table: data.next()?,
            record: data.next()?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for DeleteRecords {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DeleteRecords");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        DeleteRecords {
            table: data.next()?,
            records: data.collect(),
        }
        .verify(keys, data)
    }
}
impl EventTrait for DeleteField {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DeleteField");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        DeleteField {
            table: data.next()?,
            record: data.next()?,
            column: data.next()?,
        }
        .verify(keys, data)
    }
}
impl EventTrait for DeleteFields {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DeleteFields");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        DeleteFields {
            table: data.next()?,
            record: data.next()?,
            columns: data.collect(),
        }
        .verify(keys, data)
    }
}
impl EventTrait for DeletesField {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DeletesField");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        DeletesField {
            table: data.next()?,
            column: data.next()?,
            records: data.collect(),
        }
        .verify(keys, data)
    }
}
impl EventTrait for DeletesFields {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DeletesFields");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        DeletesFields {
            table: data.next()?,
            records: Vec::<Felt>::ideserialize(data)?,
            columns: data.collect(),
        }
        .verify(keys, data)
    }
}

impl EventTrait for DeleteFieldSet {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DeleteFieldSet");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        DeleteFieldSet {
            table: data.next()?,
            record: data.next()?,
            group: data.next()?,
        }
        .verify(keys, data)
    }
}

impl EventTrait for DeleteFieldSets {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DeleteFieldSets");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        DeleteFieldSets {
            table: data.next()?,
            record: data.next()?,
            groups: data.collect(),
        }
        .verify(keys, data)
    }
}

impl EventTrait for DeletesFieldSet {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DeletesFieldSet");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        DeletesFieldSet {
            table: data.next()?,
            group: data.next()?,
            records: data.collect(),
        }
        .verify(keys, data)
    }
}

impl EventTrait for DeletesFieldSets {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DeletesFieldSets");
    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        DeletesFieldSets {
            table: data.next()?,
            records: Vec::<Felt>::ideserialize(data)?,
            groups: data.collect(),
        }
        .verify(keys, data)
    }
}
