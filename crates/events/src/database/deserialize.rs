use crate::event::CairoDeserializeRemaining;

use super::{
    AddColumn, AddColumns, CreateColumnSet, CreateIndex, CreateTable, CreateTableFromClass,
    CreateTableFromContract, DeleteField, DeleteFieldSet, DeleteFieldSets, DeleteFields,
    DeleteRecord, DeleteRecords, DeletesField, DeletesFieldSet, DeletesFieldSets, DeletesFields,
    DropColumn, DropColumns, DropIndex, DropTable, InsertField, InsertFieldSet, InsertFieldSets,
    InsertFields, InsertRecord, InsertRecords, InsertsField, InsertsFieldSet, InsertsFieldSets,
    InsertsFields, RenameColumn, RenameColumns, RenamePrimary, RenameTable, RetypeColumn,
    RetypeColumns, RetypePrimary,
};
use introspect_types::schema::{PrimaryDef, PrimaryTypeDef};
use introspect_types::{
    Attribute, CairoDeserialize, CairoDeserializer, CairoEvent, DecodeResult, FeltSource, TypeDef,
    cairo_event_name_and_selector,
};
use starknet_types_core::felt::Felt;

impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D>
    for CreateColumnSet
{
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let id = event_data.next_felt()?;
        let columns = event_data.deserialize_remaining()?;
        Ok(CreateColumnSet { id, columns })
    }
}

impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D> for CreateTable
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let id = event_data.next_felt()?;
        let name = event_data.next_string()?;
        let attributes = Vec::<Attribute>::deserialize(event_data)?;
        let primary = PrimaryDef::deserialize(event_data)?;
        let columns = event_data.deserialize_remaining()?;
        Ok(CreateTable {
            id,
            name,
            attributes,
            primary,
            columns,
        })
    }
}

impl<D: FeltSource + CairoDeserializer> CairoEvent<D> for CreateTableFromClass {
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let id = event_data.next_felt()?;
        let name = event_data.next_string()?;
        let class_hash = event_data.next_felt()?;
        Ok(CreateTableFromClass {
            id,
            name,
            class_hash,
        })
    }
}

impl<D: FeltSource + CairoDeserializer> CairoEvent<D> for CreateTableFromContract {
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let id = event_data.next_felt()?;
        let name = event_data.next_string()?;
        let contract_address = event_data.next_felt()?;
        Ok(CreateTableFromContract {
            id,
            name,
            contract_address,
        })
    }
}

impl<D: FeltSource + CairoDeserializer> CairoEvent<D> for RenameTable {
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let id = event_data.next_felt()?;
        let name = event_data.next_string()?;
        Ok(RenameTable { id, name })
    }
}

impl<D: FeltSource + CairoDeserializer> CairoEvent<D> for DropTable {
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let id = event_data.next_felt()?;
        Ok(DropTable { id })
    }
}

impl<D: FeltSource + CairoDeserializer> CairoEvent<D> for RenamePrimary {
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let name = event_data.next_string()?;
        Ok(RenamePrimary { table, name })
    }
}

impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D> for RetypePrimary
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let type_def = PrimaryTypeDef::deserialize(event_data)?;
        let attributes = event_data.deserialize_remaining()?;
        Ok(RetypePrimary {
            table,
            type_def,
            attributes,
        })
    }
}

impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D> for AddColumn
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let id = event_data.next_felt()?;
        let name = event_data.next_string()?;
        let type_def = TypeDef::deserialize(event_data)?;
        let attributes = event_data.deserialize_remaining()?;
        Ok(AddColumn {
            table,
            id,
            name,
            attributes,
            type_def,
        })
    }
}
impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D> for AddColumns
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let columns = event_data.deserialize_remaining()?;
        Ok(AddColumns { table, columns })
    }
}
impl<D: FeltSource + CairoDeserializer> CairoEvent<D> for RenameColumn {
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let id = event_data.next_felt()?;
        let name = event_data.next_string()?;
        Ok(RenameColumn { table, id, name })
    }
}
impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D>
    for RenameColumns
{
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let columns = event_data.deserialize_remaining()?;
        Ok(RenameColumns { table, columns })
    }
}
impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D> for RetypeColumn
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let id = event_data.next_felt()?;
        let type_def = TypeDef::deserialize(event_data)?;
        let attributes = event_data.deserialize_remaining()?;
        Ok(RetypeColumn {
            table,
            id,
            attributes,
            type_def,
        })
    }
}

impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D> for RetypeColumns
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let columns = event_data.deserialize_remaining()?;
        Ok(RetypeColumns { table, columns })
    }
}
impl<D: FeltSource + CairoDeserializer> CairoEvent<D> for DropColumn {
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let id = event_data.next_felt()?;
        Ok(DropColumn { table, id })
    }
}
impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D> for DropColumns {
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let ids = event_data.deserialize_remaining()?;
        Ok(DropColumns { table, ids })
    }
}

impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D> for CreateIndex
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let id = event_data.next_felt()?;
        let attributes = Vec::<Attribute>::deserialize(event_data)?;
        let columns = event_data.deserialize_remaining()?;
        Ok(CreateIndex {
            table,
            id,
            attributes,
            columns,
        })
    }
}

impl<D: FeltSource + CairoDeserializer> CairoEvent<D> for DropIndex {
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let id = event_data.next_felt()?;
        Ok(DropIndex { table, id })
    }
}

impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D> for InsertRecord {
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let row = event_data.next_felt()?;
        let data = event_data.deserialize_remaining()?;
        Ok(InsertRecord { table, row, data })
    }
}
impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D>
    for InsertRecords
{
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let entries = event_data.deserialize_remaining()?;
        Ok(InsertRecords { table, entries })
    }
}
impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D> for InsertField {
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let row = event_data.next_felt()?;
        let column = event_data.next_felt()?;
        let data = event_data.deserialize_remaining()?;
        Ok(InsertField {
            table,
            row,
            column,
            data,
        })
    }
}
impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D> for InsertFields {
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let row = event_data.next_felt()?;
        let columns = Vec::<Felt>::deserialize(event_data)?;
        let data = event_data.deserialize_remaining()?;
        Ok(InsertFields {
            table,
            row,
            columns,
            data,
        })
    }
}

impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D> for InsertsField {
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let column = event_data.next_felt()?;
        let entries = event_data.deserialize_remaining()?;
        Ok(InsertsField {
            table,
            column,
            entries,
        })
    }
}

impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D>
    for InsertsFields
{
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let columns = Vec::<Felt>::deserialize(event_data)?;
        let entries = event_data.deserialize_remaining()?;
        Ok(InsertsFields {
            table,
            columns,
            entries,
        })
    }
}
impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D>
    for InsertFieldSet
{
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let row = event_data.next_felt()?;
        let set = event_data.next_felt()?;
        let data = event_data.deserialize_remaining()?;
        Ok(InsertFieldSet {
            table,
            row,
            set,
            data,
        })
    }
}

impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D>
    for InsertFieldSets
{
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let row = event_data.next_felt()?;
        let sets = Vec::<Felt>::deserialize(event_data)?;
        let data = event_data.deserialize_remaining()?;
        Ok(InsertFieldSets {
            table,
            row,
            sets,
            data,
        })
    }
}

impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D>
    for InsertsFieldSet
{
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let set = event_data.next_felt()?;
        let entries = event_data.deserialize_remaining()?;
        Ok(InsertsFieldSet {
            table,
            set,
            entries,
        })
    }
}

impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D>
    for InsertsFieldSets
{
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let sets = Vec::<Felt>::deserialize(event_data)?;
        let entries = event_data.deserialize_remaining()?;
        Ok(InsertsFieldSets {
            table,
            sets,
            entries,
        })
    }
}

impl<D: FeltSource + CairoDeserializer> CairoEvent<D> for DeleteRecord {
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let row = event_data.next_felt()?;
        Ok(DeleteRecord { table, row })
    }
}
impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D>
    for DeleteRecords
{
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let rows = event_data.deserialize_remaining()?;
        Ok(DeleteRecords { table, rows })
    }
}
impl<D: FeltSource + CairoDeserializer> CairoEvent<D> for DeleteField {
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let row = event_data.next_felt()?;
        let column = event_data.next_felt()?;
        Ok(DeleteField { table, row, column })
    }
}
impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D> for DeleteFields {
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let row = event_data.next_felt()?;
        let columns = event_data.deserialize_remaining()?;
        Ok(DeleteFields {
            table,
            row,
            columns,
        })
    }
}
impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D> for DeletesField {
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let column = event_data.next_felt()?;
        let rows = event_data.deserialize_remaining()?;
        Ok(DeletesField {
            table,
            rows,
            column,
        })
    }
}
impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D>
    for DeletesFields
{
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let rows = Vec::<Felt>::deserialize(event_data)?;
        let columns = event_data.deserialize_remaining()?;
        Ok(DeletesFields {
            table,
            rows,
            columns,
        })
    }
}

impl<D: FeltSource + CairoDeserializer> CairoEvent<D> for DeleteFieldSet {
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let row = event_data.next_felt()?;
        let set = event_data.next_felt()?;
        Ok(DeleteFieldSet { table, row, set })
    }
}

impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D>
    for DeleteFieldSets
{
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let row = event_data.next_felt()?;
        let sets = event_data.deserialize_remaining()?;
        Ok(DeleteFieldSets { table, row, sets })
    }
}

impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D>
    for DeletesFieldSet
{
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let set = event_data.next_felt()?;
        let rows = event_data.deserialize_remaining()?;
        Ok(DeletesFieldSet { table, rows, set })
    }
}

impl<D: FeltSource + CairoDeserializer + CairoDeserializeRemaining> CairoEvent<D>
    for DeletesFieldSets
{
    fn deserialize_event<K: FeltSource>(
        _event_keys: &mut K,
        event_data: &mut D,
    ) -> DecodeResult<Self> {
        let table = event_data.next_felt()?;
        let rows = Vec::<Felt>::deserialize(event_data)?;
        let sets = event_data.deserialize_remaining()?;
        Ok(DeletesFieldSets { table, rows, sets })
    }
}

cairo_event_name_and_selector!(CreateColumnSet);
cairo_event_name_and_selector!(CreateTable);
cairo_event_name_and_selector!(CreateTableFromClass);
cairo_event_name_and_selector!(CreateTableFromContract);
cairo_event_name_and_selector!(RenameTable);
cairo_event_name_and_selector!(DropTable);
cairo_event_name_and_selector!(RenamePrimary);
cairo_event_name_and_selector!(RetypePrimary);
cairo_event_name_and_selector!(AddColumn);
cairo_event_name_and_selector!(AddColumns);
cairo_event_name_and_selector!(RenameColumn);
cairo_event_name_and_selector!(RenameColumns);
cairo_event_name_and_selector!(RetypeColumn);
cairo_event_name_and_selector!(RetypeColumns);
cairo_event_name_and_selector!(DropColumn);
cairo_event_name_and_selector!(DropColumns);
cairo_event_name_and_selector!(CreateIndex);
cairo_event_name_and_selector!(DropIndex);
cairo_event_name_and_selector!(InsertRecord);
cairo_event_name_and_selector!(InsertRecords);
cairo_event_name_and_selector!(InsertField);
cairo_event_name_and_selector!(InsertFields);
cairo_event_name_and_selector!(InsertsField);
cairo_event_name_and_selector!(InsertsFields);
cairo_event_name_and_selector!(InsertFieldSet);
cairo_event_name_and_selector!(InsertFieldSets);
cairo_event_name_and_selector!(InsertsFieldSet);
cairo_event_name_and_selector!(InsertsFieldSets);
cairo_event_name_and_selector!(DeleteRecord);
cairo_event_name_and_selector!(DeleteRecords);
cairo_event_name_and_selector!(DeleteField);
cairo_event_name_and_selector!(DeleteFields);
cairo_event_name_and_selector!(DeletesField);
cairo_event_name_and_selector!(DeletesFields);
cairo_event_name_and_selector!(DeleteFieldSet);
cairo_event_name_and_selector!(DeleteFieldSets);
cairo_event_name_and_selector!(DeletesFieldSet);
cairo_event_name_and_selector!(DeletesFieldSets);
