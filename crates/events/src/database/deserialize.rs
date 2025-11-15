use super::{
    AddColumn, AddColumns, CreateColumnGroup, CreateTable, CreateTableWithColumns, DeleteField,
    DeleteFields, DeleteRecord, DeleteRecords, DeletesField, DeletesFields, DropColumn,
    DropColumns, DropTable, IdData, IdName, IdTypeAttributes, InsertColumnGroup, InsertField,
    InsertFields, InsertRecord, InsertRecords, InsertsColumnGroup, InsertsField, InsertsFields,
    RenameColumn, RenameColumns, RenamePrimary, RenameTable, RetypeColumn, RetypeColumns,
    RetypePrimary,
};
use crate::event::EventTrait;
use introspect_types::schema::{PrimaryDef, PrimaryTypeDef};
use introspect_types::{
    Attribute, CairoDeserialize, ColumnDef, TypeDef, deserialize_byte_array_string,
};
use starknet_types_core::felt::Felt;

impl EventTrait for CreateTable {
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

impl EventTrait for RenameTable {
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
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        let id = keys.next()?;
        DropTable { id }.verify(&mut keys, &mut data)
    }
}
impl EventTrait for RenamePrimary {
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
impl EventTrait for InsertColumnGroup {
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        InsertColumnGroup {
            table: keys.next()?,
            record: keys.next()?,
            group: keys.next()?,
            data: Vec::<Felt>::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}
impl EventTrait for InsertsColumnGroup {
    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys = keys.into_iter();
        let mut data = data.into_iter();
        InsertsColumnGroup {
            table: keys.next()?,
            group: keys.next()?,
            records_data: Vec::<IdData>::c_deserialize(&mut data)?,
        }
        .verify(&mut keys, &mut data)
    }
}
impl EventTrait for DeleteRecord {
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
impl EventTrait for CreateColumnGroup {
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

impl CairoDeserialize for IdData {
    fn c_deserialize(data: &mut impl Iterator<Item = Felt>) -> Option<Self> {
        let id = data.next()?;
        let data = Vec::<Felt>::c_deserialize(data)?;
        Some(IdData { id, data })
    }
}

impl CairoDeserialize for IdName {
    fn c_deserialize(data: &mut impl Iterator<Item = Felt>) -> Option<Self> {
        let id = data.next()?;
        let name = deserialize_byte_array_string(data)?;
        Some(IdName { id, name })
    }
}

impl CairoDeserialize for IdTypeAttributes {
    fn c_deserialize(data: &mut impl Iterator<Item = Felt>) -> Option<Self> {
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
