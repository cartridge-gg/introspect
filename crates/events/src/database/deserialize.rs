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
use introspect_types::deserialize_def::TypeDefDeserializer;
use introspect_types::schema::{PrimaryDef, PrimaryTypeDef};
use introspect_types::{
    Attribute, CairoDeserialize, CairoDeserializer, CairoEvent, CairoEventInfo, DecodeResult,
    FeltSource, TypeDef,
};
use starknet_types_raw::Felt;

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

impl<D: FeltSource + TypeDefDeserializer + CairoDeserializeRemaining> CairoEvent<D>
    for CreateTable
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

impl<D: FeltSource + TypeDefDeserializer + CairoDeserializeRemaining> CairoEvent<D>
    for RetypePrimary
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

impl<D: FeltSource + TypeDefDeserializer + CairoDeserializeRemaining> CairoEvent<D> for AddColumn {
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
impl<D: FeltSource + TypeDefDeserializer + CairoDeserializeRemaining> CairoEvent<D> for AddColumns {
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
impl<D: FeltSource + TypeDefDeserializer + CairoDeserializeRemaining> CairoEvent<D>
    for RetypeColumn
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

impl<D: FeltSource + TypeDefDeserializer + CairoDeserializeRemaining> CairoEvent<D>
    for RetypeColumns
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

impl CairoEventInfo for CreateColumnSet {
    const NAME: &'static str = "CreateColumnSet";
}
impl CairoEventInfo for CreateTable {
    const NAME: &'static str = "CreateTable";
}
impl CairoEventInfo for CreateTableFromClass {
    const NAME: &'static str = "CreateTableFromClass";
}
impl CairoEventInfo for CreateTableFromContract {
    const NAME: &'static str = "CreateTableFromContract";
}
impl CairoEventInfo for RenameTable {
    const NAME: &'static str = "RenameTable";
}
impl CairoEventInfo for DropTable {
    const NAME: &'static str = "DropTable";
}
impl CairoEventInfo for RenamePrimary {
    const NAME: &'static str = "RenamePrimary";
}
impl CairoEventInfo for RetypePrimary {
    const NAME: &'static str = "RetypePrimary";
}
impl CairoEventInfo for AddColumn {
    const NAME: &'static str = "AddColumn";
}
impl CairoEventInfo for AddColumns {
    const NAME: &'static str = "AddColumns";
}
impl CairoEventInfo for RenameColumn {
    const NAME: &'static str = "RenameColumn";
}
impl CairoEventInfo for RenameColumns {
    const NAME: &'static str = "RenameColumns";
}
impl CairoEventInfo for RetypeColumn {
    const NAME: &'static str = "RetypeColumn";
}
impl CairoEventInfo for RetypeColumns {
    const NAME: &'static str = "RetypeColumns";
}
impl CairoEventInfo for DropColumn {
    const NAME: &'static str = "DropColumn";
}
impl CairoEventInfo for DropColumns {
    const NAME: &'static str = "DropColumns";
}
impl CairoEventInfo for CreateIndex {
    const NAME: &'static str = "CreateIndex";
}
impl CairoEventInfo for DropIndex {
    const NAME: &'static str = "DropIndex";
}
impl CairoEventInfo for InsertRecord {
    const NAME: &'static str = "InsertRecord";
}
impl CairoEventInfo for InsertRecords {
    const NAME: &'static str = "InsertRecords";
}
impl CairoEventInfo for InsertField {
    const NAME: &'static str = "InsertField";
}
impl CairoEventInfo for InsertFields {
    const NAME: &'static str = "InsertFields";
}
impl CairoEventInfo for InsertsField {
    const NAME: &'static str = "InsertsField";
}
impl CairoEventInfo for InsertsFields {
    const NAME: &'static str = "InsertsFields";
}
impl CairoEventInfo for InsertFieldSet {
    const NAME: &'static str = "InsertFieldSet";
}
impl CairoEventInfo for InsertFieldSets {
    const NAME: &'static str = "InsertFieldSets";
}
impl CairoEventInfo for InsertsFieldSet {
    const NAME: &'static str = "InsertsFieldSet";
}
impl CairoEventInfo for InsertsFieldSets {
    const NAME: &'static str = "InsertsFieldSets";
}
impl CairoEventInfo for DeleteRecord {
    const NAME: &'static str = "DeleteRecord";
}
impl CairoEventInfo for DeleteRecords {
    const NAME: &'static str = "DeleteRecords";
}
impl CairoEventInfo for DeleteField {
    const NAME: &'static str = "DeleteField";
}
impl CairoEventInfo for DeleteFields {
    const NAME: &'static str = "DeleteFields";
}
impl CairoEventInfo for DeletesField {
    const NAME: &'static str = "DeletesField";
}
impl CairoEventInfo for DeletesFields {
    const NAME: &'static str = "DeletesFields";
}
impl CairoEventInfo for DeleteFieldSet {
    const NAME: &'static str = "DeleteFieldSet";
}
impl CairoEventInfo for DeleteFieldSets {
    const NAME: &'static str = "DeleteFieldSets";
}
impl CairoEventInfo for DeletesFieldSet {
    const NAME: &'static str = "DeletesFieldSet";
}
impl CairoEventInfo for DeletesFieldSets {
    const NAME: &'static str = "DeletesFieldSets";
}
