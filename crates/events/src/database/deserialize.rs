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
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        CreateColumnGroup {
            id: keys.next()?,
            columns: Vec::<Felt>::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}

impl EventTrait for CreateTable {
    const SELECTOR: Felt = selector!("CreateTable");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        CreateTable {
            id: keys.next()?,
            name: deserialize_byte_array_string(&mut data)?,
            attributes: Vec::<Attribute>::c_deserialize(&mut data)?,
            primary: PrimaryDef::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}

impl EventTrait for CreateTableWithColumns {
    const SELECTOR: Felt = selector!("CreateTableWithColumns");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        CreateTableWithColumns {
            id: keys.next()?,
            name: deserialize_byte_array_string(&mut data)?,
            attributes: Vec::<Attribute>::c_deserialize(&mut data)?,
            primary: PrimaryDef::c_deserialize(&mut data)?,
            columns: Vec::<ColumnDef>::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}

impl EventTrait for CreateTableFromClassHash {
    const SELECTOR: Felt = selector!("CreateTableFromClassHash");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        CreateTableFromClassHash {
            id: keys.next()?,
            name: deserialize_byte_array_string(&mut data)?,
            class_hash: data.next()?,
        }
        .verify(&mut keys, &mut data)
    }
}

impl EventTrait for RenameTable {
    const SELECTOR: Felt = selector!("RenameTable");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        RenameTable {
            id: keys.next()?,
            name: deserialize_byte_array_string(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}

impl EventTrait for DropTable {
    const SELECTOR: Felt = selector!("DropTable");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        let id = keys.next()?;
        DropTable { id }.verify(&mut keys, &mut data)
    }
}
impl EventTrait for RenamePrimary {
    const SELECTOR: Felt = selector!("RenamePrimary");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        RenamePrimary {
            table: keys.next()?,
            name: deserialize_byte_array_string(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}
impl EventTrait for RetypePrimary {
    const SELECTOR: Felt = selector!("RetypePrimary");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        RetypePrimary {
            table: keys.next()?,
            attributes: Vec::<Attribute>::c_deserialize(&mut data)?,
            type_def: PrimaryTypeDef::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}
impl EventTrait for AddColumn {
    const SELECTOR: Felt = selector!("AddColumn");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        AddColumn {
            table: keys.next()?,
            id: keys.next()?,
            name: deserialize_byte_array_string(&mut data)?,
            attributes: Vec::<Attribute>::c_deserialize(&mut data)?,
            type_def: TypeDef::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}
impl EventTrait for AddColumns {
    const SELECTOR: Felt = selector!("AddColumns");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        AddColumns {
            table: keys.next()?,
            columns: Vec::<ColumnDef>::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}
impl EventTrait for RenameColumn {
    const SELECTOR: Felt = selector!("RenameColumn");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        RenameColumn {
            table: keys.next()?,
            id: keys.next()?,
            name: deserialize_byte_array_string(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}
impl EventTrait for RenameColumns {
    const SELECTOR: Felt = selector!("RenameColumns");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        RenameColumns {
            table: keys.next()?,
            columns: Vec::<IdName>::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}
impl EventTrait for RetypeColumn {
    const SELECTOR: Felt = selector!("RetypeColumn");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        RetypeColumn {
            table: keys.next()?,
            id: keys.next()?,
            attributes: Vec::<Attribute>::c_deserialize(&mut data)?,
            type_def: TypeDef::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}
impl EventTrait for RetypeColumns {
    const SELECTOR: Felt = selector!("RetypeColumns");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        RetypeColumns {
            table: keys.next()?,
            columns: Vec::<IdTypeAttributes>::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}
impl EventTrait for DropColumn {
    const SELECTOR: Felt = selector!("DropColumn");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        DropColumn {
            table: keys.next()?,
            id: keys.next()?,
        }
        .verify(&mut keys, &mut data)
    }
}
impl EventTrait for DropColumns {
    const SELECTOR: Felt = selector!("DropColumns");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        DropColumns {
            table: keys.next()?,
            ids: Vec::<Felt>::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}
impl EventTrait for InsertRecord {
    const SELECTOR: Felt = selector!("InsertRecord");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        InsertRecord {
            table: keys.next()?,
            record: keys.next()?,
            data: Vec::<Felt>::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}
impl EventTrait for InsertRecords {
    const SELECTOR: Felt = selector!("InsertRecords");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        InsertRecords {
            table: keys.next()?,
            records_data: Vec::<IdData>::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}
impl EventTrait for InsertField {
    const SELECTOR: Felt = selector!("InsertField");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        InsertField {
            table: keys.next()?,
            column: keys.next()?,
            record: keys.next()?,
            data: Vec::<Felt>::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}
impl EventTrait for InsertFields {
    const SELECTOR: Felt = selector!("InsertFields");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        InsertFields {
            table: keys.next()?,
            record: keys.next()?,
            columns: Vec::<Felt>::c_deserialize(&mut data)?,
            data: Vec::<Felt>::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}
impl EventTrait for InsertsField {
    const SELECTOR: Felt = selector!("InsertsField");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        InsertsField {
            table: keys.next()?,
            column: keys.next()?,
            records_data: Vec::<IdData>::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}
impl EventTrait for InsertsFields {
    const SELECTOR: Felt = selector!("InsertsFields");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        InsertsFields {
            table: keys.next()?,
            columns: Vec::<Felt>::c_deserialize(&mut data)?,
            records_data: Vec::<IdData>::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}
impl EventTrait for InsertFieldGroup {
    const SELECTOR: Felt = selector!("InsertFieldGroup");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        InsertFieldGroup {
            table: keys.next()?,
            record: keys.next()?,
            group: keys.next()?,
            data: Vec::<Felt>::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}

impl EventTrait for InsertFieldGroups {
    const SELECTOR: Felt = selector!("InsertFieldGroups");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        InsertFieldGroups {
            table: keys.next()?,
            record: keys.next()?,
            groups: Vec::<Felt>::c_deserialize(&mut data)?,
            data: Vec::<Felt>::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}

impl EventTrait for InsertsFieldGroup {
    const SELECTOR: Felt = selector!("InsertsFieldGroup");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        InsertsFieldGroup {
            table: keys.next()?,
            group: keys.next()?,
            records_data: Vec::<IdData>::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}

impl EventTrait for InsertsFieldGroups {
    const SELECTOR: Felt = selector!("InsertsFieldGroups");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        InsertsFieldGroups {
            table: keys.next()?,
            record: keys.next()?,
            groups: Vec::<Felt>::c_deserialize(&mut data)?,
            records_data: Vec::<IdData>::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}

impl EventTrait for DeleteRecord {
    const SELECTOR: Felt = selector!("DeleteRecord");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        DeleteRecord {
            table: keys.next()?,
            record: keys.next()?,
        }
        .verify(&mut keys, &mut data)
    }
}
impl EventTrait for DeleteRecords {
    const SELECTOR: Felt = selector!("DeleteRecords");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        DeleteRecords {
            table: keys.next()?,
            records: Vec::<Felt>::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}
impl EventTrait for DeleteField {
    const SELECTOR: Felt = selector!("DeleteField");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        DeleteField {
            table: keys.next()?,
            record: keys.next()?,
            column: keys.next()?,
        }
        .verify(&mut keys, &mut data)
    }
}
impl EventTrait for DeleteFields {
    const SELECTOR: Felt = selector!("DeleteFields");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        DeleteFields {
            table: keys.next()?,
            record: keys.next()?,
            columns: Vec::<Felt>::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}
impl EventTrait for DeletesField {
    const SELECTOR: Felt = selector!("DeletesField");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        DeletesField {
            table: keys.next()?,
            column: keys.next()?,
            records: Vec::<Felt>::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}
impl EventTrait for DeletesFields {
    const SELECTOR: Felt = selector!("DeletesFields");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        DeletesFields {
            table: keys.next()?,
            records: Vec::<Felt>::c_deserialize(&mut data)?,
            columns: Vec::<Felt>::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}

impl EventTrait for DeleteFieldGroup {
    const SELECTOR: Felt = selector!("DeleteFieldGroup");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        DeleteFieldGroup {
            table: keys.next()?,
            record: keys.next()?,
            group: keys.next()?,
        }
        .verify(&mut keys, &mut data)
    }
}

impl EventTrait for DeleteFieldGroups {
    const SELECTOR: Felt = selector!("DeleteFieldGroups");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        DeleteFieldGroups {
            table: keys.next()?,
            record: keys.next()?,
            groups: Vec::<Felt>::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}

impl EventTrait for DeletesFieldGroup {
    const SELECTOR: Felt = selector!("DeletesFieldGroup");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        DeletesFieldGroup {
            table: keys.next()?,
            group: keys.next()?,
            records: Vec::<Felt>::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}

impl EventTrait for DeletesFieldGroups {
    const SELECTOR: Felt = selector!("DeletesFieldGroups");
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        DeletesFieldGroups {
            table: keys.next()?,
            records: Vec::<Felt>::c_deserialize(&mut data)?,
            groups: Vec::<Felt>::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
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
