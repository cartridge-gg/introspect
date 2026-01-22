use introspect_types::utils::SpanDefault;
use introspect_types::{Attribute, ColumnDef, Entry, ISerde, PrimaryDef, PrimaryTypeDef, TypeDef};
use starknet::Event;
use crate::utils::{DrainSpanTrait, ISerdeEnd, VerifyEventDeserializeTrait};
use super::emit_event_impl;
pub mod selectors {
    pub const CreateFieldGroup: felt252 = selector!("CreateFieldGroup");
    pub const CreateTable: felt252 = selector!("CreateTable");
    pub const CreateTableWithColumns: felt252 = selector!("CreateTableWithColumns");
    pub const CreateTableFromClassHash: felt252 = selector!("CreateTableFromClassHash");
    pub const RenameTable: felt252 = selector!("RenameTable");
    pub const DropTable: felt252 = selector!("DropTable");
    pub const CreateIndex: felt252 = selector!("CreateIndex");
    pub const DropIndex: felt252 = selector!("DropIndex");
    pub const RenamePrimary: felt252 = selector!("RenamePrimary");
    pub const RetypePrimary: felt252 = selector!("RetypePrimary");
    pub const AddColumn: felt252 = selector!("AddColumn");
    pub const AddColumns: felt252 = selector!("AddColumns");
    pub const RenameColumn: felt252 = selector!("RenameColumn");
    pub const RenameColumns: felt252 = selector!("RenameColumns");
    pub const RetypeColumn: felt252 = selector!("RetypeColumn");
    pub const RetypeColumns: felt252 = selector!("RetypeColumns");
    pub const DropColumn: felt252 = selector!("DropColumn");
    pub const DropColumns: felt252 = selector!("DropColumns");
    pub const InsertRecord: felt252 = selector!("InsertRecord");
    pub const InsertRecords: felt252 = selector!("InsertRecords");
    pub const InsertField: felt252 = selector!("InsertField");
    pub const InsertFields: felt252 = selector!("InsertFields");
    pub const InsertsField: felt252 = selector!("InsertsField");
    pub const InsertsFields: felt252 = selector!("InsertsFields");
    pub const InsertFieldGroup: felt252 = selector!("InsertFieldGroup");
    pub const InsertFieldGroups: felt252 = selector!("InsertFieldGroups");
    pub const InsertsFieldGroup: felt252 = selector!("InsertsFieldGroup");
    pub const InsertsFieldGroups: felt252 = selector!("InsertsFieldGroups");
    pub const DeleteRecord: felt252 = selector!("DeleteRecord");
    pub const DeleteRecords: felt252 = selector!("DeleteRecords");
    pub const DeleteField: felt252 = selector!("DeleteField");
    pub const DeleteFields: felt252 = selector!("DeleteFields");
    pub const DeletesField: felt252 = selector!("DeletesField");
    pub const DeletesFields: felt252 = selector!("DeletesFields");
    pub const DeleteFieldGroup: felt252 = selector!("DeleteFieldGroup");
    pub const DeleteFieldGroups: felt252 = selector!("DeleteFieldGroups");
    pub const DeletesFieldGroup: felt252 = selector!("DeletesFieldGroup");
    pub const DeletesFieldGroups: felt252 = selector!("DeletesFieldGroups");
}


#[derive(Drop, starknet::Event, PartialEq, Debug)]
pub enum DatabaseEvents {
    CreateFieldGroup: CreateColumnSet,
    CreateTable: CreateTable,
    CreateTableWithColumns: CreateTableWithColumns,
    CreateTableFromClassHash: CreateTableFromClassHash,
    RenameTable: RenameTable,
    DropTable: DropTable,
    CreateIndex: CreateIndex,
    DropIndex: DropIndex,
    RenamePrimary: RenamePrimary,
    RetypePrimary: RetypePrimary,
    AddColumn: AddColumn,
    AddColumns: AddColumns,
    RenameColumn: RenameColumn,
    RenameColumns: RenameColumns,
    RetypeColumn: RetypeColumn,
    RetypeColumns: RetypeColumns,
    DropColumn: DropColumn,
    DropColumns: DropColumns,
    InsertRecord: InsertRecord,
    InsertRecords: InsertRecords,
    InsertField: InsertField,
    InsertFields: InsertFields,
    InsertsField: InsertsField,
    InsertsFields: InsertsFields,
    InsertFieldGroup: InsertFieldSet,
    InsertFieldGroups: InsertFieldSets,
    InsertsFieldGroup: InsertsFieldSet,
    InsertsFieldGroups: InsertsFieldSets,
    DeleteRecord: DeleteRecord,
    DeleteRecords: DeleteRecords,
    DeleteField: DeleteField,
    DeleteFields: DeleteFields,
    DeletesField: DeletesField,
    DeletesFields: DeletesFields,
    DeleteFieldGroup: DeleteFieldSet,
    DeleteFieldGroups: DeleteFieldSets,
    DeletesFieldGroup: DeletesFieldSet,
    DeletesFieldGroups: DeletesFieldSets,
}


/// Emitted when a new column set (schema) is created.
/// - id: felt252 - Unique identifier for the column set.
/// - columns: Span<felt252> - List of column IDs included in the column set
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct CreateColumnSet {
    pub id: felt252,
    pub columns: Span<felt252>,
}


/// Table management events
/// - id: felt252 - Unique identifier for the table.
/// - name: ByteArray - Name of the table.
/// - attributes: Span<Attribute> - Attributes of the table.
/// - columns: Span<ColumnDef> - Definitions of the columns in the table.
/// - class_hash: ClassHash - Class hash to derive schema from.

/// Emitted when a new table is created.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct CreateTable {
    pub id: felt252,
    pub name: ByteArray,
    pub attributes: Span<Attribute>,
    pub primary: PrimaryDef,
}

/// Emitted when a new table is created with specified columns.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct CreateTableWithColumns {
    pub id: felt252,
    pub name: ByteArray,
    pub attributes: Span<Attribute>,
    pub primary: PrimaryDef,
    pub columns: Span<ColumnDef>,
}

/// Emitted when a new table is created from a class hash.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct CreateTableFromClassHash {
    pub id: felt252,
    pub name: ByteArray,
    pub class_hash: felt252,
}

///Emitted when a table is renamed.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct RenameTable {
    pub id: felt252,
    pub name: ByteArray,
}

///Emitted when a table is dropped.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct DropTable {
    pub id: felt252,
}

#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct CreateIndex {
    pub table: felt252,
    pub id: felt252,
    pub name: ByteArray,
    pub columns: Span<felt252>,
}


#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct DropIndex {
    pub table: felt252,
    pub id: felt252,
}

/// Primary key management events
/// - table: felt252 - Unique identifier for the table.
/// - name: ByteArray - Name of the primary key field.
/// - attributes: Span<Attribute> - Attributes of the primary key field.
/// - type_def: TypeDef - Type definition of the primary key field.

/// Emitted when the primary key field is renamed.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct RenamePrimary {
    pub table: felt252,
    pub name: ByteArray,
}

/// Emitted when the primary key field is retyped.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct RetypePrimary {
    pub table: felt252,
    pub type_def: PrimaryTypeDef,
    pub attributes: Span<Attribute>,
}

/// Column management events
/// - table: felt252 - Unique identifier for the table.
/// - id: felt252 - Unique identifier for the column.
/// - name: ByteArray - Name of the column.
/// - attributes: Span<Attribute> - Attributes of the column.
/// - type_def: TypeDef - Type definition of the column.

#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct AddColumn {
    pub table: felt252,
    pub id: felt252,
    pub name: ByteArray,
    pub attributes: Span<Attribute>,
    pub type_def: TypeDef,
}

/// Emitted when multiple new columns are declared for an existing table.
/// - columns: Definitions of the columns being added.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct AddColumns {
    pub table: felt252,
    pub columns: Span<ColumnDef>,
}

// Emitted when a column is renamed in a table.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct RenameColumn {
    pub table: felt252,
    pub id: felt252,
    pub name: ByteArray,
}


/// Emitted when columns are renamed in a table.
/// - columns: List of (column ID, new name) pairs.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct RenameColumns {
    pub table: felt252,
    pub columns: Span<IdName>,
}

/// Emitted when a column is retyped in a table.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct RetypeColumn {
    pub table: felt252,
    pub id: felt252,
    pub attributes: Span<Attribute>,
    pub type_def: TypeDef,
}

/// Emitted when multiple columns are retyped in a table.
/// - columns: List of (column ID, new TypeDef) pairs.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct RetypeColumns {
    pub table: felt252,
    pub columns: Span<IdTypeDef>,
}

/// Emitted when a column is undeclared from a table.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct DropColumn {
    pub table: felt252,
    pub id: felt252,
}

/// Emitted when multiple columns are undeclared from a table.
/// - ids: List of column IDs being dropped.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct DropColumns {
    pub table: felt252,
    pub ids: Span<felt252>,
}


/// Record management events
/// - table - Table ID.
/// - row/rows - Record ID.
/// - column/columns - Column ID.
/// - data - Serialised data being set.
/// - entries - Pairs of Record IDs and their serialised data being set.
/// - group - Field group (schema) ID.

#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct InsertRecord {
    pub table: felt252,
    pub row: felt252,
    pub data: Span<felt252>,
}

/// Insert multiple records into a table.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct InsertRecords {
    pub table: felt252,
    pub entries: Span<Entry>,
}


/// Insert a single field into a row.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct InsertField {
    pub table: felt252,
    pub row: felt252,
    pub column: felt252,
    pub data: Span<felt252>,
}

/// Insert multiple fields into a row.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct InsertFields {
    pub table: felt252,
    pub row: felt252,
    pub columns: Span<felt252>,
    pub data: Span<felt252>,
}

/// Insert a single field into multiple rows.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct InsertsField {
    pub table: felt252,
    pub column: felt252,
    pub entries: Span<Entry>,
}

/// Insert multiple fields into multiple rows.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct InsertsFields {
    pub table: felt252,
    pub columns: Span<felt252>,
    pub entries: Span<Entry>,
}

/// Insert a schema into a row.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct InsertFieldSet {
    pub table: felt252,
    pub row: felt252,
    pub set: felt252,
    pub data: Span<felt252>,
}


/// Insert multiple groups of columns into a row.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct InsertFieldSets {
    pub table: felt252,
    pub row: felt252,
    pub sets: Span<felt252>,
    pub data: Span<felt252>,
}

/// Insert multiple rows into a table using a schema.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct InsertsFieldSet {
    pub table: felt252,
    pub set: felt252,
    pub entries: Span<Entry>,
}


/// Insert multiple groups of fields into multiple rows.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct InsertsFieldSets {
    pub table: felt252,
    pub sets: Span<felt252>,
    pub entries: Span<Entry>,
}

/// Remove a single record from a table.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct DeleteRecord {
    pub table: felt252,
    pub row: felt252,
}

/// Remove multiple records from a table.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct DeleteRecords {
    pub table: felt252,
    pub rows: Span<felt252>,
}


/// Remove a single field from a row.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct DeleteField {
    pub table: felt252,
    pub row: felt252,
    pub column: felt252,
}


/// Remove multiple fields from a row.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct DeleteFields {
    pub table: felt252,
    pub row: felt252,
    pub columns: Span<felt252>,
}


/// Remove a single field from multiple rows.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct DeletesField {
    pub table: felt252,
    pub rows: Span<felt252>,
    pub column: felt252,
}

/// Remove multiple fields from multiple rows.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct DeletesFields {
    pub table: felt252,
    pub rows: Span<felt252>,
    pub columns: Span<felt252>,
}


/// Remove a schema from a row.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct DeleteFieldSet {
    pub table: felt252,
    pub row: felt252,
    pub set: felt252,
}
/// Remove multiple groups from a row.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct DeleteFieldSets {
    pub table: felt252,
    pub row: felt252,
    pub sets: Span<felt252>,
}

/// Remove a group from multiple rows.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct DeletesFieldSet {
    pub table: felt252,
    pub rows: Span<felt252>,
    pub set: felt252,
}

/// Remove multiple groups from multiple rows.
#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct DeletesFieldSets {
    pub table: felt252,
    pub rows: Span<felt252>,
    pub sets: Span<felt252>,
}


#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct IdName {
    pub id: felt252,
    pub name: ByteArray,
}


#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct IdTypeDef {
    pub id: felt252,
    pub attributes: Span<Attribute>,
    pub type_def: TypeDef,
}

impl IdNameISerde of ISerde<IdName> {
    fn iserialize(self: @IdName, ref output: Array<felt252>) {
        output.append(*self.id);
        self.name.iserialize(ref output);
    }

    fn ideserialize(ref serialized: Span<felt252>) -> Option<IdName> {
        Some(IdName { id: *serialized.pop_front()?, name: ISerde::ideserialize(ref serialized)? })
    }
}

impl IdTypeAttributesISerde of ISerde<IdTypeDef> {
    fn iserialize(self: @IdTypeDef, ref output: Array<felt252>) {
        output.append(*self.id);
        self.attributes.iserialize(ref output);
        self.type_def.iserialize(ref output);
    }

    fn ideserialize(ref serialized: Span<felt252>) -> Option<IdTypeDef> {
        Some(
            IdTypeDef {
                id: *serialized.pop_front()?,
                attributes: ISerde::ideserialize(ref serialized)?,
                type_def: ISerde::ideserialize(ref serialized)?,
            },
        )
    }
}


impl CreateFieldGroupEvent of Event<CreateColumnSet> {
    fn append_keys_and_data(
        self: @CreateColumnSet, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.id);
        data.append_span(*self.columns)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<CreateColumnSet> {
        CreateColumnSet { id: *data.pop_front()?, columns: data.drain() }.verify_keys(ref keys)
    }
}

impl CreateTableEvent of Event<CreateTable> {
    fn append_keys_and_data(
        self: @CreateTable, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.id);
        self.name.iserialize(ref data);
        self.primary.iserialize(ref data);
        self.attributes.iserialize_end(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<CreateTable> {
        CreateTable {
            id: *data.pop_front()?,
            name: ISerde::ideserialize(ref data)?,
            primary: ISerde::ideserialize(ref data)?,
            attributes: ISerdeEnd::ideserialize_end(ref data)?,
        }
            .verify_keys(ref keys)
    }
}

impl CreateTableWithColumnsEvent of Event<CreateTableWithColumns> {
    fn append_keys_and_data(
        self: @CreateTableWithColumns, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.id);
        self.name.iserialize(ref data);
        self.attributes.iserialize(ref data);
        self.primary.iserialize(ref data);
        self.columns.iserialize_end(ref data);
    }

    fn deserialize(
        ref keys: Span<felt252>, ref data: Span<felt252>,
    ) -> Option<CreateTableWithColumns> {
        CreateTableWithColumns {
            id: *data.pop_front()?,
            name: ISerde::ideserialize(ref data)?,
            attributes: ISerde::ideserialize(ref data)?,
            primary: ISerde::ideserialize(ref data)?,
            columns: ISerdeEnd::ideserialize_end(ref data)?,
        }
            .verify_keys(ref keys)
    }
}


impl CreateTableFromClassHashEvent of Event<CreateTableFromClassHash> {
    fn append_keys_and_data(
        self: @CreateTableFromClassHash, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.id);
        self.name.iserialize(ref data);
        data.append(*self.class_hash);
    }

    fn deserialize(
        ref keys: Span<felt252>, ref data: Span<felt252>,
    ) -> Option<CreateTableFromClassHash> {
        CreateTableFromClassHash {
            id: *data.pop_front()?,
            name: ISerde::ideserialize(ref data)?,
            class_hash: *data.pop_front()?,
        }
            .verify(ref keys, ref data)
    }
}

impl RenameTableEvent of Event<RenameTable> {
    fn append_keys_and_data(
        self: @RenameTable, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.id);
        self.name.iserialize(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<RenameTable> {
        RenameTable { id: *data.pop_front()?, name: ISerde::ideserialize(ref data)? }
            .verify(ref keys, ref data)
    }
}

impl DropTableEvent of Event<DropTable> {
    fn append_keys_and_data(self: @DropTable, ref keys: Array<felt252>, ref data: Array<felt252>) {
        data.append(*self.id);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DropTable> {
        DropTable { id: *data.pop_front()? }.verify(ref keys, ref data)
    }
}

impl CreateIndexEvent of Event<CreateIndex> {
    fn append_keys_and_data(
        self: @CreateIndex, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        data.append(*self.id);
        self.name.iserialize(ref data);
        self.columns.iserialize_end(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<CreateIndex> {
        CreateIndex {
            table: *data.pop_front()?,
            id: *data.pop_front()?,
            name: ISerde::ideserialize(ref data)?,
            columns: ISerdeEnd::ideserialize_end(ref data)?,
        }
            .verify_keys(ref keys)
    }
}

impl DropIndexEvent of Event<DropIndex> {
    fn append_keys_and_data(self: @DropIndex, ref keys: Array<felt252>, ref data: Array<felt252>) {
        data.append(*self.table);
        data.append(*self.id);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DropIndex> {
        DropIndex { table: *data.pop_front()?, id: *data.pop_front()? }.verify(ref keys, ref data)
    }
}

impl RenamePrimaryEvent of Event<RenamePrimary> {
    fn append_keys_and_data(
        self: @RenamePrimary, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        self.name.iserialize(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<RenamePrimary> {
        RenamePrimary { table: *data.pop_front()?, name: ISerde::ideserialize(ref data)? }
            .verify(ref keys, ref data)
    }
}

impl RetypePrimaryEvent of Event<RetypePrimary> {
    fn append_keys_and_data(
        self: @RetypePrimary, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        self.type_def.iserialize(ref data);
        self.attributes.iserialize_end(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<RetypePrimary> {
        RetypePrimary {
            table: *data.pop_front()?,
            type_def: ISerde::ideserialize(ref data)?,
            attributes: ISerdeEnd::ideserialize_end(ref data)?,
        }
            .verify_keys(ref keys)
    }
}

impl AddColumnEvent of Event<AddColumn> {
    fn append_keys_and_data(self: @AddColumn, ref keys: Array<felt252>, ref data: Array<felt252>) {
        data.append(*self.table);
        data.append(*self.id);
        self.name.iserialize(ref data);
        self.type_def.iserialize(ref data);
        self.attributes.iserialize_end(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<AddColumn> {
        AddColumn {
            table: *data.pop_front()?,
            id: *data.pop_front()?,
            name: ISerde::ideserialize(ref data)?,
            type_def: ISerde::ideserialize(ref data)?,
            attributes: ISerdeEnd::ideserialize_end(ref data)?,
        }
            .verify_keys(ref keys)
    }
}


impl AddColumnsEvent of Event<AddColumns> {
    fn append_keys_and_data(self: @AddColumns, ref keys: Array<felt252>, ref data: Array<felt252>) {
        data.append(*self.table);
        self.columns.iserialize_end(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<AddColumns> {
        AddColumns { table: *data.pop_front()?, columns: ISerdeEnd::ideserialize_end(ref data)? }
            .verify_keys(ref keys)
    }
}

impl RenameColumnEvent of Event<RenameColumn> {
    fn append_keys_and_data(
        self: @RenameColumn, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        data.append(*self.id);
        self.name.iserialize(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<RenameColumn> {
        RenameColumn {
            table: *data.pop_front()?,
            id: *data.pop_front()?,
            name: ISerde::ideserialize(ref data)?,
        }
            .verify(ref keys, ref data)
    }
}

impl RenameColumnsEvent of Event<RenameColumns> {
    fn append_keys_and_data(
        self: @RenameColumns, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        self.columns.iserialize_end(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<RenameColumns> {
        RenameColumns { table: *data.pop_front()?, columns: ISerdeEnd::ideserialize_end(ref data)? }
            .verify_keys(ref keys)
    }
}

impl RetypeColumnEvent of Event<RetypeColumn> {
    fn append_keys_and_data(
        self: @RetypeColumn, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        data.append(*self.id);
        self.type_def.iserialize(ref data);
        self.attributes.iserialize_end(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<RetypeColumn> {
        RetypeColumn {
            table: *data.pop_front()?,
            id: *data.pop_front()?,
            type_def: ISerde::ideserialize(ref data)?,
            attributes: ISerdeEnd::ideserialize_end(ref data)?,
        }
            .verify_keys(ref keys)
    }
}


impl RetypeColumnsEvent of Event<RetypeColumns> {
    fn append_keys_and_data(
        self: @RetypeColumns, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        self.columns.iserialize_end(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<RetypeColumns> {
        RetypeColumns { table: *data.pop_front()?, columns: ISerdeEnd::ideserialize_end(ref data)? }
            .verify_keys(ref keys)
    }
}

impl DropColumnEvent of Event<DropColumn> {
    fn append_keys_and_data(self: @DropColumn, ref keys: Array<felt252>, ref data: Array<felt252>) {
        data.append(*self.table);
        data.append(*self.id);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DropColumn> {
        DropColumn { table: *data.pop_front()?, id: *data.pop_front()? }.verify(ref keys, ref data)
    }
}


impl DropColumnsEvent of Event<DropColumns> {
    fn append_keys_and_data(
        self: @DropColumns, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        data.append_span(*self.ids)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DropColumns> {
        DropColumns { table: *data.pop_front()?, ids: data.drain() }.verify_keys(ref keys)
    }
}

impl InsertRecordEvent of Event<InsertRecord> {
    fn append_keys_and_data(
        self: @InsertRecord, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        data.append(*self.row);
        data.append_span(*self.data)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<InsertRecord> {
        InsertRecord { table: *data.pop_front()?, row: *data.pop_front()?, data: data.drain() }
            .verify_keys(ref keys)
    }
}

impl InsertRecordsEvent of Event<InsertRecords> {
    fn append_keys_and_data(
        self: @InsertRecords, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        self.entries.iserialize_end(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<InsertRecords> {
        InsertRecords { table: *data.pop_front()?, entries: ISerdeEnd::ideserialize_end(ref data)? }
            .verify_keys(ref keys)
    }
}

impl InsertFieldEvent of Event<InsertField> {
    fn append_keys_and_data(
        self: @InsertField, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        data.append(*self.row);
        data.append(*self.column);
        data.append_span(*self.data)
    }
    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<InsertField> {
        InsertField {
            table: *data.pop_front()?,
            row: *data.pop_front()?,
            column: *data.pop_front()?,
            data: data.drain(),
        }
            .verify_keys(ref keys)
    }
}

impl InsertFieldsEvent of Event<InsertFields> {
    fn append_keys_and_data(
        self: @InsertFields, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        data.append(*self.row);
        self.columns.iserialize(ref data);
        data.append_span(*self.data)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<InsertFields> {
        InsertFields {
            table: *data.pop_front()?,
            row: *data.pop_front()?,
            columns: ISerde::ideserialize(ref data)?,
            data: data.drain(),
        }
            .verify_keys(ref keys)
    }
}

impl InsertsFieldEvent of Event<InsertsField> {
    fn append_keys_and_data(
        self: @InsertsField, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        data.append(*self.column);
        self.entries.iserialize_end(ref data);
    }
    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<InsertsField> {
        InsertsField {
            table: *data.pop_front()?,
            column: *data.pop_front()?,
            entries: ISerdeEnd::ideserialize_end(ref data)?,
        }
            .verify_keys(ref keys)
    }
}

impl InsertsFieldsEvent of Event<InsertsFields> {
    fn append_keys_and_data(
        self: @InsertsFields, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        self.columns.iserialize(ref data);
        self.entries.iserialize_end(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<InsertsFields> {
        InsertsFields {
            table: *data.pop_front()?,
            columns: ISerde::ideserialize(ref data)?,
            entries: ISerdeEnd::ideserialize_end(ref data)?,
        }
            .verify_keys(ref keys)
    }
}


impl InsertFieldGroupEvent of Event<InsertFieldSet> {
    fn append_keys_and_data(
        self: @InsertFieldSet, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        data.append(*self.row);
        data.append(*self.set);
        data.append_span(*self.data)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<InsertFieldSet> {
        InsertFieldSet {
            table: *data.pop_front()?,
            row: *data.pop_front()?,
            set: *data.pop_front()?,
            data: data.drain(),
        }
            .verify_keys(ref keys)
    }
}

impl InsertFieldGroupsEvent of Event<InsertFieldSets> {
    fn append_keys_and_data(
        self: @InsertFieldSets, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        data.append(*self.row);
        self.sets.iserialize(ref data);
        data.append_span(*self.data)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<InsertFieldSets> {
        InsertFieldSets {
            table: *data.pop_front()?,
            row: *data.pop_front()?,
            sets: ISerde::ideserialize(ref data)?,
            data: data.drain(),
        }
            .verify_keys(ref keys)
    }
}

impl InsertsFieldGroupEvent of Event<InsertsFieldSet> {
    fn append_keys_and_data(
        self: @InsertsFieldSet, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        data.append(*self.set);
        self.entries.iserialize_end(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<InsertsFieldSet> {
        InsertsFieldSet {
            table: *data.pop_front()?,
            set: *data.pop_front()?,
            entries: ISerdeEnd::ideserialize_end(ref data)?,
        }
            .verify_keys(ref keys)
    }
}

impl InsertsFieldGroupsEvent of Event<InsertsFieldSets> {
    fn append_keys_and_data(
        self: @InsertsFieldSets, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        self.sets.iserialize(ref data);
        self.entries.iserialize_end(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<InsertsFieldSets> {
        InsertsFieldSets {
            table: *data.pop_front()?,
            sets: ISerde::ideserialize(ref data)?,
            entries: ISerdeEnd::ideserialize_end(ref data)?,
        }
            .verify_keys(ref keys)
    }
}

impl DeleteRecordEvent of Event<DeleteRecord> {
    fn append_keys_and_data(
        self: @DeleteRecord, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        data.append(*self.row);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DeleteRecord> {
        DeleteRecord { table: *data.pop_front()?, row: *data.pop_front()? }
            .verify(ref keys, ref data)
    }
}

impl DeleteRecordsEvent of Event<DeleteRecords> {
    fn append_keys_and_data(
        self: @DeleteRecords, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        data.append_span(*self.rows)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DeleteRecords> {
        DeleteRecords { table: *data.pop_front()?, rows: data.drain() }.verify_keys(ref keys)
    }
}

impl DeleteFieldEvent of Event<DeleteField> {
    fn append_keys_and_data(
        self: @DeleteField, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        data.append(*self.row);
        data.append(*self.column);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DeleteField> {
        DeleteField {
            table: *data.pop_front()?, row: *data.pop_front()?, column: *data.pop_front()?,
        }
            .verify(ref keys, ref data)
    }
}

impl DeleteFieldsEvent of Event<DeleteFields> {
    fn append_keys_and_data(
        self: @DeleteFields, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        data.append(*self.row);
        data.append_span(*self.columns)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DeleteFields> {
        DeleteFields { table: *data.pop_front()?, row: *data.pop_front()?, columns: data.drain() }
            .verify_keys(ref keys)
    }
}

impl DeletesFieldEvent of Event<DeletesField> {
    fn append_keys_and_data(
        self: @DeletesField, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        data.append(*self.column);
        data.append_span(*self.rows)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DeletesField> {
        DeletesField { table: *data.pop_front()?, column: *data.pop_front()?, rows: data.drain() }
            .verify_keys(ref keys)
    }
}

impl DeletesFieldsEvent of Event<DeletesFields> {
    fn append_keys_and_data(
        self: @DeletesFields, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        self.rows.iserialize(ref data);
        data.append_span(*self.columns)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DeletesFields> {
        DeletesFields {
            table: *data.pop_front()?, rows: ISerde::ideserialize(ref data)?, columns: data.drain(),
        }
            .verify(ref keys, ref data)
    }
}

impl DeleteFieldGroupEvent of Event<DeleteFieldSet> {
    fn append_keys_and_data(
        self: @DeleteFieldSet, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        data.append(*self.row);
        data.append(*self.set);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DeleteFieldSet> {
        DeleteFieldSet {
            table: *data.pop_front()?, row: *data.pop_front()?, set: *data.pop_front()?,
        }
            .verify(ref keys, ref data)
    }
}


impl DeleteFieldGroupsEvent of Event<DeleteFieldSets> {
    fn append_keys_and_data(
        self: @DeleteFieldSets, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        data.append(*self.row);
        data.append_span(*self.sets)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DeleteFieldSets> {
        DeleteFieldSets { table: *data.pop_front()?, row: *data.pop_front()?, sets: data.drain() }
            .verify_keys(ref keys)
    }
}

impl DeletesFieldGroupEvent of Event<DeletesFieldSet> {
    fn append_keys_and_data(
        self: @DeletesFieldSet, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        data.append(*self.set);
        data.append_span(*self.rows)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DeletesFieldSet> {
        DeletesFieldSet { table: *data.pop_front()?, set: *data.pop_front()?, rows: data.drain() }
            .verify_keys(ref keys)
    }
}


impl DeletesFieldGroupsEvent of Event<DeletesFieldSets> {
    fn append_keys_and_data(
        self: @DeletesFieldSets, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.table);
        self.rows.iserialize(ref data);
        data.append_span(*self.sets)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DeletesFieldSets> {
        DeletesFieldSets {
            table: *data.pop_front()?, rows: ISerde::ideserialize(ref data)?, sets: data.drain(),
        }
            .verify(ref keys, ref data)
    }
}


impl EmitCreateFieldGroup =
    emit_event_impl::EmitEventImpl<CreateColumnSet, selectors::CreateFieldGroup>;
impl EmitCreateTable = emit_event_impl::EmitEventImpl<CreateTable, selectors::CreateTable>;
impl EmitCreateTableWithColumns =
    emit_event_impl::EmitEventImpl<CreateTableWithColumns, selectors::CreateTableWithColumns>;
impl EmitCreateTableFromClassHash =
    emit_event_impl::EmitEventImpl<CreateTableFromClassHash, selectors::CreateTableFromClassHash>;
impl EmitRenameTable = emit_event_impl::EmitEventImpl<RenameTable, selectors::RenameTable>;
impl EmitDropTable = emit_event_impl::EmitEventImpl<DropTable, selectors::DropTable>;
impl EmitCreateIndex = emit_event_impl::EmitEventImpl<CreateIndex, selectors::CreateIndex>;
impl EmitDropIndex = emit_event_impl::EmitEventImpl<DropIndex, selectors::DropIndex>;
impl EmitRenamePrimary = emit_event_impl::EmitEventImpl<RenamePrimary, selectors::RenamePrimary>;
impl EmitRetypePrimary = emit_event_impl::EmitEventImpl<RetypePrimary, selectors::RetypePrimary>;
impl EmitAddColumn = emit_event_impl::EmitEventImpl<AddColumn, selectors::AddColumn>;
impl EmitAddColumns = emit_event_impl::EmitEventImpl<AddColumns, selectors::AddColumns>;
impl EmitRenameColumn = emit_event_impl::EmitEventImpl<RenameColumn, selectors::RenameColumn>;
impl EmitRenameColumns = emit_event_impl::EmitEventImpl<RenameColumns, selectors::RenameColumns>;
impl EmitRetypeColumn = emit_event_impl::EmitEventImpl<RetypeColumn, selectors::RetypeColumn>;
impl EmitRetypeColumns = emit_event_impl::EmitEventImpl<RetypeColumns, selectors::RetypeColumns>;
impl EmitDropColumn = emit_event_impl::EmitEventImpl<DropColumn, selectors::DropColumn>;
impl EmitDropColumns = emit_event_impl::EmitEventImpl<DropColumns, selectors::DropColumns>;
impl EmitInsertRecord = emit_event_impl::EmitEventImpl<InsertRecord, selectors::InsertRecord>;
impl EmitInsertRecords = emit_event_impl::EmitEventImpl<InsertRecords, selectors::InsertRecords>;
impl EmitInsertField = emit_event_impl::EmitEventImpl<InsertField, selectors::InsertField>;
impl EmitInsertFields = emit_event_impl::EmitEventImpl<InsertFields, selectors::InsertFields>;
impl EmitInsertsField = emit_event_impl::EmitEventImpl<InsertsField, selectors::InsertsField>;
impl EmitInsertsFields = emit_event_impl::EmitEventImpl<InsertsFields, selectors::InsertsFields>;
impl EmitInsertFieldGroup =
    emit_event_impl::EmitEventImpl<InsertFieldSet, selectors::InsertFieldGroup>;
impl EmitInsertFieldGroups =
    emit_event_impl::EmitEventImpl<InsertFieldSets, selectors::InsertFieldGroups>;
impl EmitInsertsFieldGroup =
    emit_event_impl::EmitEventImpl<InsertsFieldSet, selectors::InsertsFieldGroup>;
impl EmitInsertsFieldGroups =
    emit_event_impl::EmitEventImpl<InsertsFieldSets, selectors::InsertsFieldGroups>;
impl EmitDeleteRecord = emit_event_impl::EmitEventImpl<DeleteRecord, selectors::DeleteRecord>;
impl EmitDeleteRecords = emit_event_impl::EmitEventImpl<DeleteRecords, selectors::DeleteRecords>;
impl EmitDeleteField = emit_event_impl::EmitEventImpl<DeleteField, selectors::DeleteField>;
impl EmitDeleteFields = emit_event_impl::EmitEventImpl<DeleteFields, selectors::DeleteFields>;
impl EmitDeletesField = emit_event_impl::EmitEventImpl<DeletesField, selectors::DeletesField>;
impl EmitDeletesFields = emit_event_impl::EmitEventImpl<DeletesFields, selectors::DeletesFields>;
impl EmitDeleteFieldGroup =
    emit_event_impl::EmitEventImpl<DeleteFieldSet, selectors::DeleteFieldGroup>;
impl EmitDeleteFieldGroups =
    emit_event_impl::EmitEventImpl<DeleteFieldSets, selectors::DeleteFieldGroups>;
impl EmitDeletesFieldGroup =
    emit_event_impl::EmitEventImpl<DeletesFieldSet, selectors::DeletesFieldGroup>;
impl EmitDeletesFieldGroups =
    emit_event_impl::EmitEventImpl<DeletesFieldSets, selectors::DeletesFieldGroups>;
