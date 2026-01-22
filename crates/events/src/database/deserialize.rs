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
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let id = event_data.next()?;
        let columns = event_data.collect();
        CreateColumnSet { id, columns }.verify_keys(event_keys)
    }
}

impl EventTrait for CreateTable {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("CreateTable");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let id = event_data.next()?;
        let name = ideserialize_utf8_string(event_data)?;
        let primary = PrimaryDef::ideserialize(event_data)?;
        let attributes = Attribute::ideserialize_end(event_data)?;
        CreateTable {
            id,
            name,
            attributes,
            primary,
        }
        .verify(event_keys, event_data)
    }
}

impl EventTrait for CreateTableWithColumns {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("CreateTableWithColumns");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let id = event_data.next()?;
        let name = ideserialize_utf8_string(event_data)?;
        let attributes = Vec::<Attribute>::ideserialize(event_data)?;
        let primary = PrimaryDef::ideserialize(event_data)?;
        let columns = ColumnDef::ideserialize_end(event_data)?;
        CreateTableWithColumns {
            id,
            name,
            attributes,
            primary,
            columns,
        }
        .verify(event_keys, event_data)
    }
}

impl EventTrait for CreateTableFromClassHash {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("CreateTableFromClassHash");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let id = event_data.next()?;
        let name = ideserialize_utf8_string(event_data)?;
        let class_hash = event_data.next()?;
        CreateTableFromClassHash {
            id,
            name,
            class_hash,
        }
        .verify(event_keys, event_data)
    }
}

impl EventTrait for RenameTable {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("RenameTable");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let id = event_data.next()?;
        let name = ideserialize_utf8_string(event_data)?;
        RenameTable { id, name }.verify(event_keys, event_data)
    }
}

impl EventTrait for DropTable {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DropTable");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let id = event_data.next()?;
        DropTable { id }.verify(event_keys, event_data)
    }
}

impl EventTrait for RenamePrimary {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("RenamePrimary");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let name = ideserialize_utf8_string(event_data)?;
        RenamePrimary { table, name }.verify(event_keys, event_data)
    }
}

impl EventTrait for RetypePrimary {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("RetypePrimary");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let type_def = PrimaryTypeDef::ideserialize(event_data)?;
        let attributes = Attribute::ideserialize_end(event_data)?;
        RetypePrimary {
            table,
            type_def,
            attributes,
        }
        .verify(event_keys, event_data)
    }
}
impl EventTrait for AddColumn {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("AddColumn");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let id = event_data.next()?;
        let name = ideserialize_utf8_string(event_data)?;
        let type_def = TypeDef::ideserialize(event_data)?;
        let attributes = Attribute::ideserialize_end(event_data)?;
        AddColumn {
            table,
            id,
            name,
            attributes,
            type_def,
        }
        .verify(event_keys, event_data)
    }
}
impl EventTrait for AddColumns {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("AddColumns");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let columns = ColumnDef::ideserialize_end(event_data)?;
        AddColumns { table, columns }.verify(event_keys, event_data)
    }
}
impl EventTrait for RenameColumn {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("RenameColumn");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let id = event_data.next()?;
        let name = ideserialize_utf8_string(event_data)?;
        RenameColumn { table, id, name }.verify(event_keys, event_data)
    }
}
impl EventTrait for RenameColumns {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("RenameColumns");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let columns = IdName::ideserialize_end(event_data)?;
        RenameColumns { table, columns }.verify(event_keys, event_data)
    }
}
impl EventTrait for RetypeColumn {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("RetypeColumn");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let id = event_data.next()?;
        let type_def = TypeDef::ideserialize(event_data)?;
        let attributes = Attribute::ideserialize_end(event_data)?;
        RetypeColumn {
            table,
            id,
            attributes,
            type_def,
        }
        .verify(event_keys, event_data)
    }
}

impl EventTrait for RetypeColumns {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("RetypeColumns");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let columns = IdTypeDef::ideserialize_end(event_data)?;
        RetypeColumns { table, columns }.verify(event_keys, event_data)
    }
}
impl EventTrait for DropColumn {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DropColumn");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let id = event_data.next()?;
        DropColumn { table, id }.verify(event_keys, event_data)
    }
}
impl EventTrait for DropColumns {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DropColumns");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let ids = event_data.collect();
        DropColumns { table, ids }.verify(event_keys, event_data)
    }
}

impl EventTrait for CreateIndex {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("CreateIndex");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let id = event_data.next()?;
        let name = ideserialize_utf8_string(event_data)?;
        let columns = event_data.collect();
        CreateIndex {
            table,
            id,
            name,
            columns,
        }
        .verify(event_keys, event_data)
    }
}

impl EventTrait for DropIndex {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DropIndex");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let id = event_data.next()?;
        DropIndex { table, id }.verify(event_keys, event_data)
    }
}

impl EventTrait for InsertRecord {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("InsertRecord");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let row = event_data.next()?;
        let data = event_data.collect();
        InsertRecord { table, row, data }.verify(event_keys, event_data)
    }
}
impl EventTrait for InsertRecords {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("InsertRecords");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let entries = Entry::ideserialize_end(event_data)?;
        InsertRecords { table, entries }.verify(event_keys, event_data)
    }
}
impl EventTrait for InsertField {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("InsertField");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let row = event_data.next()?;
        let column = event_data.next()?;
        let data = event_data.collect();
        InsertField {
            table,
            row,
            column,
            data,
        }
        .verify(event_keys, event_data)
    }
}
impl EventTrait for InsertFields {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("InsertFields");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let row = event_data.next()?;
        let columns = Vec::<Felt>::ideserialize(event_data)?;
        let data = event_data.collect();
        InsertFields {
            table,
            row,
            columns,
            data,
        }
        .verify(event_keys, event_data)
    }
}
impl EventTrait for InsertsField {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("InsertsField");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let column = event_data.next()?;
        let entries = Entry::ideserialize_end(event_data)?;
        InsertsField {
            table,
            column,
            entries,
        }
        .verify(event_keys, event_data)
    }
}
impl EventTrait for InsertsFields {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("InsertsFields");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let columns = Vec::<Felt>::ideserialize(event_data)?;
        let entries = Entry::ideserialize_end(event_data)?;
        InsertsFields {
            table,
            columns,
            entries,
        }
        .verify(event_keys, event_data)
    }
}
impl EventTrait for InsertFieldSet {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("InsertFieldSet");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let row = event_data.next()?;
        let set = event_data.next()?;
        let data = event_data.collect();
        InsertFieldSet {
            table,
            row,
            set,
            data,
        }
        .verify(event_keys, event_data)
    }
}

impl EventTrait for InsertFieldSets {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("InsertFieldSets");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let row = event_data.next()?;
        let sets = Vec::<Felt>::ideserialize(event_data)?;
        let data = event_data.collect();
        InsertFieldSets {
            table,
            row,
            sets,
            data,
        }
        .verify(event_keys, event_data)
    }
}

impl EventTrait for InsertsFieldSet {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("InsertsFieldSet");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let set = event_data.next()?;
        let entries = Entry::ideserialize_end(event_data)?;
        InsertsFieldSet {
            table,
            set,
            entries,
        }
        .verify(event_keys, event_data)
    }
}

impl EventTrait for InsertsFieldSets {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("InsertsFieldSets");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let sets = Vec::<Felt>::ideserialize(event_data)?;
        let entries = Entry::ideserialize_end(event_data)?;
        InsertsFieldSets {
            table,
            sets,
            entries,
        }
        .verify(event_keys, event_data)
    }
}

impl EventTrait for DeleteRecord {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DeleteRecord");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let row = event_data.next()?;
        DeleteRecord { table, row }.verify(event_keys, event_data)
    }
}
impl EventTrait for DeleteRecords {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DeleteRecords");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let rows = event_data.collect();
        DeleteRecords { table, rows }.verify(event_keys, event_data)
    }
}
impl EventTrait for DeleteField {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DeleteField");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let row = event_data.next()?;
        let column = event_data.next()?;
        DeleteField { table, row, column }.verify(event_keys, event_data)
    }
}
impl EventTrait for DeleteFields {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DeleteFields");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let row = event_data.next()?;
        let columns = event_data.collect();
        DeleteFields {
            table,
            row,
            columns,
        }
        .verify(event_keys, event_data)
    }
}
impl EventTrait for DeletesField {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DeletesField");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let column = event_data.next()?;
        let rows = event_data.collect();
        DeletesField {
            table,
            rows,
            column,
        }
        .verify(event_keys, event_data)
    }
}
impl EventTrait for DeletesFields {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DeletesFields");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let rows = Vec::<Felt>::ideserialize(event_data)?;
        let columns = event_data.collect();
        DeletesFields {
            table,
            rows,
            columns,
        }
        .verify(event_keys, event_data)
    }
}

impl EventTrait for DeleteFieldSet {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DeleteFieldSet");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let row = event_data.next()?;
        let set = event_data.next()?;
        DeleteFieldSet { table, row, set }.verify(event_keys, event_data)
    }
}

impl EventTrait for DeleteFieldSets {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DeleteFieldSets");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let row = event_data.next()?;
        let sets = event_data.collect();
        DeleteFieldSets { table, row, sets }.verify(event_keys, event_data)
    }
}

impl EventTrait for DeletesFieldSet {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DeletesFieldSet");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let set = event_data.next()?;
        let rows = event_data.collect();
        DeletesFieldSet { table, rows, set }.verify(event_keys, event_data)
    }
}

impl EventTrait for DeletesFieldSets {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DeletesFieldSets");
    fn deserialize_event(
        event_keys: &mut FeltIterator,
        event_data: &mut FeltIterator,
    ) -> Option<Self> {
        let table = event_data.next()?;
        let rows = Vec::<Felt>::ideserialize(event_data)?;
        let sets = event_data.collect();
        DeletesFieldSets { table, rows, sets }.verify(event_keys, event_data)
    }
}
