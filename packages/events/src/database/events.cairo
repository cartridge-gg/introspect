use introspect_types::{Attribute, ColumnDef, ISerde, IdData, PrimaryDef, PrimaryTypeDef, TypeDef};
use starknet::Event;
use crate::utils::{DrainSpanTrait, VerifyEventDeserializeTrait};

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
    CreateFieldGroup: CreateFieldGroup,
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
    InsertFieldGroup: InsertFieldGroup,
    InsertFieldGroups: InsertFieldGroups,
    InsertsFieldGroup: InsertsFieldGroup,
    InsertsFieldGroups: InsertsFieldGroups,
    DeleteRecord: DeleteRecord,
    DeleteRecords: DeleteRecords,
    DeleteField: DeleteField,
    DeleteFields: DeleteFields,
    DeletesField: DeletesField,
    DeletesFields: DeletesFields,
    DeleteFieldGroup: DeleteFieldGroup,
    DeleteFieldGroups: DeleteFieldGroups,
    DeletesFieldGroup: DeletesFieldGroup,
    DeletesFieldGroups: DeletesFieldGroups,
}


/// Emitted when a new field group (schema) is created.
/// - id: felt252 - Unique identifier for the field group.
/// - columns: Span<felt252> - List of column IDs included in the field group
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct CreateFieldGroup {
    #[key]
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
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct CreateTable {
    #[key]
    pub id: felt252,
    pub name: ByteArray,
    pub attributes: Span<Attribute>,
    pub primary: PrimaryDef,
}

/// Emitted when a new table is created with specified columns.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct CreateTableWithColumns {
    #[key]
    pub id: felt252,
    pub name: ByteArray,
    pub attributes: Span<Attribute>,
    pub primary: PrimaryDef,
    pub columns: Span<ColumnDef>,
}

#[derive(Drop, Serde, PartialEq, Debug)]
pub struct CreateTableFromClassHash {
    #[key]
    pub id: felt252,
    pub name: ByteArray,
    pub class_hash: felt252,
}

///Emitted when a table is renamed.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct RenameTable {
    #[key]
    pub id: felt252,
    pub name: ByteArray,
}

///Emitted when a table is dropped.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct DropTable {
    #[key]
    pub id: felt252,
}

#[derive(Drop, Serde, PartialEq, Debug)]
pub struct CreateIndex {
    #[key]
    pub table: felt252,
    #[key]
    pub id: felt252,
    pub name: ByteArray,
    pub columns: Span<felt252>,
}


#[derive(Drop, Serde, PartialEq, Debug)]
pub struct DropIndex {
    #[key]
    pub table: felt252,
    #[key]
    pub id: felt252,
}

/// Primary key management events
/// - table: felt252 - Unique identifier for the table.
/// - name: ByteArray - Name of the primary key field.
/// - attributes: Span<Attribute> - Attributes of the primary key field.
/// - type_def: TypeDef - Type definition of the primary key field.

/// Emitted when the primary key field is renamed.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct RenamePrimary {
    #[key]
    pub table: felt252,
    pub name: ByteArray,
}

/// Emitted when the primary key field is retyped.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct RetypePrimary {
    #[key]
    pub table: felt252,
    pub attributes: Span<Attribute>,
    pub type_def: PrimaryTypeDef,
}

/// Column management events
/// - table: felt252 - Unique identifier for the table.
/// - column: felt252 - Unique identifier for the column.
/// - name: ByteArray - Name of the column.
/// - attributes: Span<Attribute> - Attributes of the column.
/// - type_def: TypeDef - Type definition of the column.

#[derive(Drop, Serde, PartialEq, Debug)]
pub struct AddColumn {
    #[key]
    pub table: felt252,
    #[key]
    pub id: felt252,
    pub name: ByteArray,
    pub attributes: Span<Attribute>,
    pub type_def: TypeDef,
}

/// Emitted when multiple new columns are declared for an existing table.
/// - columns: Definitions of the columns being added.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct AddColumns {
    #[key]
    pub table: felt252,
    pub columns: Span<ColumnDef>,
}

// Emitted when a column is renamed in a table.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct RenameColumn {
    #[key]
    pub table: felt252,
    #[key]
    pub id: felt252,
    pub name: ByteArray,
}


// Emitted when a columns is renamed in a table.
/// - columns: List of (column ID, new name) pairs.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct RenameColumns {
    #[key]
    pub table: felt252,
    pub columns: Span<IdName>,
}

/// Remove a single record from a table.
/// Emitted when a column is retyped in a table.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct RetypeColumn {
    #[key]
    pub table: felt252,
    #[key]
    pub id: felt252,
    pub attributes: Span<Attribute>,
    pub type_def: TypeDef,
}

/// Emitted when multiple columns are retyped in a table.
/// - columns: List of (column ID, new TypeDef) pairs.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct RetypeColumns {
    #[key]
    pub table: felt252,
    pub columns: Span<IdTypeAttributes>,
}

/// Emitted when a column is undeclared from a table.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct DropColumn {
    #[key]
    pub table: felt252,
    #[key]
    pub column: felt252,
}

/// Emitted when multiple columns are undeclared from a table.
/// - columns: List of column IDs being dropped.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct DropColumns {
    #[key]
    pub table: felt252,
    pub columns: Span<felt252>,
}


/// Record management events
/// - table - Table ID.
/// - record/records - Record ID.
/// - column/columns - Column ID.
/// - data - Serialised data being set.
/// - records_data - Pairs of Record IDs and their serialised data being set.
/// - group - Field group (schema) ID.

#[derive(Drop, Serde, PartialEq, Debug)]
pub struct InsertRecord {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    pub data: Span<felt252>,
}

/// Remove a record from a table using a schema.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct InsertRecords {
    #[key]
    pub table: felt252,
    pub records_data: Span<IdData>,
}


//// Insert a single field into a record.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct InsertField {
    #[key]
    pub table: felt252,
    #[key]
    pub column: felt252,
    #[key]
    pub record: felt252,
    pub data: Span<felt252>,
}

/// Insert multiple fields into a record.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct InsertFields {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    pub columns: Span<felt252>,
    pub data: Span<felt252>,
}

/// Insert a single field into multiple records.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct InsertsField {
    #[key]
    pub table: felt252,
    #[key]
    pub column: felt252,
    pub records_data: Span<IdData>,
}

/// Insert multiple fields into multiple records.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct InsertsFields {
    #[key]
    pub table: felt252,
    pub columns: Span<felt252>,
    pub records_data: Span<IdData>,
}

/// Insert a schema into a record.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct InsertFieldGroup {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    #[key]
    pub group: felt252,
    pub data: Span<felt252>,
}


/// Insert multiple groups of columns into a record.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct InsertFieldGroups {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    pub groups: Span<felt252>,
    pub data: Span<felt252>,
}

/// Insert multiple records into a table using a schema.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct InsertsFieldGroup {
    #[key]
    pub table: felt252,
    #[key]
    pub group: felt252,
    pub records_data: Span<IdData>,
}


/// Insert multiple schemas into a record.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct InsertsFieldGroups {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    pub groups: Span<felt252>,
    pub records_data: Span<IdData>,
}

/// Remove a single record from a table.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct DeleteRecord {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
}

/// Remove multiple records from a table.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct DeleteRecords {
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
}


/// Remove a single field from a record.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct DeleteField {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    #[key]
    pub column: felt252,
}


/// Remove multiple fields from a record.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct DeleteFields {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    pub columns: Span<felt252>,
}


/// Remove a single field from multiple records.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct DeletesField {
    #[key]
    pub table: felt252,
    #[key]
    pub column: felt252,
    pub records: Span<felt252>,
}

/// Remove multiple fields from multiple records.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct DeletesFields {
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
    pub columns: Span<felt252>,
}


/// Remove a schema from a record.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct DeleteFieldGroup {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    #[key]
    pub group: felt252,
}
/// Remove multiple fields from a record.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct DeleteFieldGroups {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    pub groups: Span<felt252>,
}

/// Remove multiple fields from multiple records.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct DeletesFieldGroup {
    #[key]
    pub table: felt252,
    #[key]
    pub group: felt252,
    pub records: Span<felt252>,
}

/// Remove multiple fields from multiple records.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct DeletesFieldGroups {
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
    pub groups: Span<felt252>,
}


#[derive(Drop, Serde, PartialEq, Debug)]
pub struct IdName {
    pub id: felt252,
    pub name: ByteArray,
}


#[derive(Drop, Serde, PartialEq, Debug)]
pub struct IdTypeAttributes {
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
        let id = *serialized.pop_front()?;
        let name = ISerde::ideserialize(ref serialized)?;
        Some(IdName { id, name })
    }
}

impl IdTypeAttributesISerde of ISerde<IdTypeAttributes> {
    fn iserialize(self: @IdTypeAttributes, ref output: Array<felt252>) {
        output.append(*self.id);
        self.attributes.iserialize(ref output);
        self.type_def.iserialize(ref output);
    }

    fn ideserialize(ref serialized: Span<felt252>) -> Option<IdTypeAttributes> {
        let id = *serialized.pop_front()?;
        let attributes = ISerde::ideserialize(ref serialized)?;
        let type_def = ISerde::ideserialize(ref serialized)?;
        Some(IdTypeAttributes { id, attributes, type_def })
    }
}


impl CreateFieldGroupEvent of Event<CreateFieldGroup> {
    fn append_keys_and_data(
        self: @CreateFieldGroup, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.id);
        data.append_span(*self.columns)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<CreateFieldGroup> {
        CreateFieldGroup { id: *keys.pop_front()?, columns: data.drain() }.verify_keys(ref keys)
    }
}


impl CreateTableEvent of Event<CreateTable> {
    fn append_keys_and_data(
        self: @CreateTable, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.id);
        self.name.iserialize(ref data);
        self.attributes.iserialize(ref data);
        self.primary.iserialize(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<CreateTable> {
        let id = *keys.pop_front()?;
        let name = ISerde::ideserialize(ref data)?;
        let attributes = ISerde::ideserialize(ref data)?;
        let primary = ISerde::ideserialize(ref data)?;
        CreateTable { id, name, attributes, primary }.verify(ref keys, ref data)
    }
}

impl CreateTableWithColumnsEvent of Event<CreateTableWithColumns> {
    fn append_keys_and_data(
        self: @CreateTableWithColumns, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.id);
        self.name.iserialize(ref data);
        self.attributes.iserialize(ref data);
        self.primary.iserialize(ref data);
        self.columns.iserialize(ref data);
    }

    fn deserialize(
        ref keys: Span<felt252>, ref data: Span<felt252>,
    ) -> Option<CreateTableWithColumns> {
        let id = *keys.pop_front()?;
        let name = ISerde::ideserialize(ref data)?;
        let attributes = ISerde::ideserialize(ref data)?;
        let primary = ISerde::ideserialize(ref data)?;
        let columns = ISerde::ideserialize(ref data)?;
        CreateTableWithColumns { id, name, attributes, primary, columns }.verify(ref keys, ref data)
    }
}


impl CreateTableFromClassHashEvent of Event<CreateTableFromClassHash> {
    fn append_keys_and_data(
        self: @CreateTableFromClassHash, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.id);
        self.name.iserialize(ref data);
        data.append(*self.class_hash);
    }

    fn deserialize(
        ref keys: Span<felt252>, ref data: Span<felt252>,
    ) -> Option<CreateTableFromClassHash> {
        let id = *keys.pop_front()?;
        let name = ISerde::ideserialize(ref data)?;
        let class_hash = *data.pop_front()?;
        CreateTableFromClassHash { id, name, class_hash }.verify(ref keys, ref data)
    }
}

impl RenameTableEvent of Event<RenameTable> {
    fn append_keys_and_data(
        self: @RenameTable, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.id);
        self.name.iserialize(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<RenameTable> {
        let id = *keys.pop_front()?;
        let name = ISerde::ideserialize(ref data)?;
        RenameTable { id, name }.verify(ref keys, ref data)
    }
}

impl DropTableEvent of Event<DropTable> {
    fn append_keys_and_data(self: @DropTable, ref keys: Array<felt252>, ref data: Array<felt252>) {
        keys.append(*self.id);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DropTable> {
        let id = *keys.pop_front()?;
        DropTable { id }.verify(ref keys, ref data)
    }
}

impl CreateIndexEvent of Event<CreateIndex> {
    fn append_keys_and_data(
        self: @CreateIndex, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        keys.append(*self.id);
        self.name.iserialize(ref data);
        self.columns.iserialize(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<CreateIndex> {
        let table = *keys.pop_front()?;
        let id = *keys.pop_front()?;
        let name = ISerde::ideserialize(ref data)?;
        let columns = ISerde::ideserialize(ref data)?;
        CreateIndex { table, id, name, columns }.verify(ref keys, ref data)
    }
}

impl DropIndexEvent of Event<DropIndex> {
    fn append_keys_and_data(self: @DropIndex, ref keys: Array<felt252>, ref data: Array<felt252>) {
        keys.append(*self.table);
        keys.append(*self.id);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DropIndex> {
        let table = *keys.pop_front()?;
        let id = *keys.pop_front()?;
        DropIndex { table, id }.verify(ref keys, ref data)
    }
}

impl RenamePrimaryEvent of Event<RenamePrimary> {
    fn append_keys_and_data(
        self: @RenamePrimary, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        self.name.iserialize(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<RenamePrimary> {
        let table = *keys.pop_front()?;
        let name = ISerde::ideserialize(ref data)?;
        RenamePrimary { table, name }.verify(ref keys, ref data)
    }
}

impl RetypePrimaryEvent of Event<RetypePrimary> {
    fn append_keys_and_data(
        self: @RetypePrimary, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        self.attributes.iserialize(ref data);
        self.type_def.iserialize(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<RetypePrimary> {
        let table = *keys.pop_front()?;
        let attributes = ISerde::ideserialize(ref data)?;
        let type_def = ISerde::ideserialize(ref data)?;
        RetypePrimary { table, attributes, type_def }.verify(ref keys, ref data)
    }
}

impl AddColumnEvent of Event<AddColumn> {
    fn append_keys_and_data(self: @AddColumn, ref keys: Array<felt252>, ref data: Array<felt252>) {
        keys.append(*self.table);
        keys.append(*self.id);
        self.name.iserialize(ref data);
        self.attributes.iserialize(ref data);
        self.type_def.iserialize(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<AddColumn> {
        let table = *keys.pop_front()?;
        let id = *keys.pop_front()?;
        let name = ISerde::ideserialize(ref data)?;
        let attributes = ISerde::ideserialize(ref data)?;
        let type_def = ISerde::ideserialize(ref data)?;
        AddColumn { table, id, name, attributes, type_def }.verify(ref keys, ref data)
    }
}

impl AddColumnsEvent of Event<AddColumns> {
    fn append_keys_and_data(self: @AddColumns, ref keys: Array<felt252>, ref data: Array<felt252>) {
        keys.append(*self.table);
        for column in self.columns {
            column.iserialize(ref data);
        }
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<AddColumns> {
        let mut columns: Array<ColumnDef> = Default::default();
        while !data.is_empty() {
            columns.append(ISerde::ideserialize(ref data)?);
        }
        AddColumns { table: *keys.pop_front()?, columns: columns.span() }.verify(ref keys, ref data)
    }
}

impl RenameColumnEvent of Event<RenameColumn> {
    fn append_keys_and_data(
        self: @RenameColumn, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        keys.append(*self.id);
        self.name.iserialize(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<RenameColumn> {
        let table = *keys.pop_front()?;
        let id = *keys.pop_front()?;
        let name = ISerde::ideserialize(ref data)?;
        RenameColumn { table, id, name }.verify(ref keys, ref data)
    }
}

impl RenameColumnsEvent of Event<RenameColumns> {
    fn append_keys_and_data(
        self: @RenameColumns, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        self.columns.iserialize(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<RenameColumns> {
        let table = *keys.pop_front()?;
        let columns = ISerde::ideserialize(ref data)?;
        RenameColumns { table, columns }.verify(ref keys, ref data)
    }
}

impl RetypeColumnEvent of Event<RetypeColumn> {
    fn append_keys_and_data(
        self: @RetypeColumn, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        keys.append(*self.id);
        self.attributes.iserialize(ref data);
        self.type_def.iserialize(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<RetypeColumn> {
        let table = *keys.pop_front()?;
        let id = *keys.pop_front()?;
        let attributes = ISerde::ideserialize(ref data)?;
        let type_def = ISerde::ideserialize(ref data)?;
        RetypeColumn { table, id, attributes, type_def }.verify(ref keys, ref data)
    }
}

impl RetypeColumnsEvent of Event<RetypeColumns> {
    fn append_keys_and_data(
        self: @RetypeColumns, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        self.columns.iserialize(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<RetypeColumns> {
        let table = *keys.pop_front()?;
        let columns = ISerde::ideserialize(ref data)?;
        RetypeColumns { table, columns }.verify(ref keys, ref data)
    }
}

impl DropColumnEvent of Event<DropColumn> {
    fn append_keys_and_data(self: @DropColumn, ref keys: Array<felt252>, ref data: Array<felt252>) {
        keys.append(*self.table);
        keys.append(*self.column);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DropColumn> {
        let table = *keys.pop_front()?;
        let column = *keys.pop_front()?;
        DropColumn { table, column }.verify(ref keys, ref data)
    }
}


impl DropColumnsEvent of Event<DropColumns> {
    fn append_keys_and_data(
        self: @DropColumns, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        data.append_span(*self.columns)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DropColumns> {
        DropColumns { table: *keys.pop_front()?, columns: data.drain() }.verify_keys(ref keys)
    }
}

impl InsertRecordEvent of Event<InsertRecord> {
    fn append_keys_and_data(
        self: @InsertRecord, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        keys.append(*self.record);
        data.append_span(*self.data)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<InsertRecord> {
        InsertRecord { table: *keys.pop_front()?, record: *keys.pop_front()?, data: data.drain() }
            .verify_keys(ref keys)
    }
}

impl InsertRecordsEvent of Event<InsertRecords> {
    fn append_keys_and_data(
        self: @InsertRecords, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        for rd in self.records_data {
            rd.iserialize(ref data);
        }
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<InsertRecords> {
        let mut records_data: Array<IdData> = Default::default();
        while !data.is_empty() {
            records_data.append(ISerde::ideserialize(ref data)?);
        }
        InsertRecords { table: *keys.pop_front()?, records_data: records_data.span() }
            .verify(ref keys, ref data)
    }
}

impl InsertFieldEvent of Event<InsertField> {
    fn append_keys_and_data(
        self: @InsertField, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        keys.append(*self.column);
        keys.append(*self.record);
        data.append_span(*self.data)
    }
    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<InsertField> {
        InsertField {
            table: *keys.pop_front()?,
            column: *keys.pop_front()?,
            record: *keys.pop_front()?,
            data: data.drain(),
        }
            .verify_keys(ref keys)
    }
}

impl InsertFieldsEvent of Event<InsertFields> {
    fn append_keys_and_data(
        self: @InsertFields, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        keys.append(*self.record);
        self.columns.iserialize(ref data);
        data.append_span(*self.data)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<InsertFields> {
        InsertFields {
            table: *keys.pop_front()?,
            record: *keys.pop_front()?,
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
        keys.append(*self.table);
        keys.append(*self.column);
        for rd in self.records_data {
            rd.iserialize(ref data);
        }
    }
    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<InsertsField> {
        let mut records_data: Array<IdData> = Default::default();
        while !data.is_empty() {
            records_data.append(ISerde::ideserialize(ref data)?);
        }
        InsertsField {
            table: *keys.pop_front()?,
            column: *keys.pop_front()?,
            records_data: records_data.span(),
        }
            .verify(ref keys, ref data)
    }
}

impl InsertsFieldsEvent of Event<InsertsFields> {
    fn append_keys_and_data(
        self: @InsertsFields, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        self.columns.iserialize(ref data);
        for rd in self.records_data {
            rd.iserialize(ref data);
        }
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<InsertsFields> {
        let mut records_data: Array<IdData> = Default::default();
        let columns = ISerde::ideserialize(ref data)?;
        while !data.is_empty() {
            records_data.append(ISerde::ideserialize(ref data)?);
        }
        InsertsFields { table: *keys.pop_front()?, columns, records_data: records_data.span() }
            .verify(ref keys, ref data)
    }
}


impl InsertFieldGroupEvent of Event<InsertFieldGroup> {
    fn append_keys_and_data(
        self: @InsertFieldGroup, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        keys.append(*self.record);
        keys.append(*self.group);
        data.append_span(*self.data)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<InsertFieldGroup> {
        InsertFieldGroup {
            table: *keys.pop_front()?,
            record: *keys.pop_front()?,
            group: *keys.pop_front()?,
            data: data.drain(),
        }
            .verify_keys(ref keys)
    }
}

impl InsertFieldGroupsEvent of Event<InsertFieldGroups> {
    fn append_keys_and_data(
        self: @InsertFieldGroups, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        keys.append(*self.record);
        self.groups.iserialize(ref data);
        data.append_span(*self.data)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<InsertFieldGroups> {
        InsertFieldGroups {
            table: *keys.pop_front()?,
            record: *keys.pop_front()?,
            groups: ISerde::ideserialize(ref data)?,
            data: data.drain(),
        }
            .verify_keys(ref keys)
    }
}

impl InsertsFieldGroupEvent of Event<InsertsFieldGroup> {
    fn append_keys_and_data(
        self: @InsertsFieldGroup, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        keys.append(*self.group);
        for rd in self.records_data {
            rd.iserialize(ref data);
        }
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<InsertsFieldGroup> {
        let mut records_data: Array<IdData> = Default::default();
        while !data.is_empty() {
            records_data.append(ISerde::ideserialize(ref data)?);
        }
        InsertsFieldGroup {
            table: *keys.pop_front()?, group: *keys.pop_front()?, records_data: records_data.span(),
        }
            .verify(ref keys, ref data)
    }
}

impl InsertsFieldGroupsEvent of Event<InsertsFieldGroups> {
    fn append_keys_and_data(
        self: @InsertsFieldGroups, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        keys.append(*self.record);
        self.groups.iserialize(ref data);
        for rd in self.records_data {
            rd.iserialize(ref data);
        }
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<InsertsFieldGroups> {
        let groups = ISerde::ideserialize(ref data)?;
        let mut records_data: Array<IdData> = Default::default();
        while !data.is_empty() {
            records_data.append(ISerde::ideserialize(ref data)?);
        }
        InsertsFieldGroups {
            table: *keys.pop_front()?,
            record: *keys.pop_front()?,
            groups,
            records_data: records_data.span(),
        }
            .verify(ref keys, ref data)
    }
}

impl DeleteRecordEvent of Event<DeleteRecord> {
    fn append_keys_and_data(
        self: @DeleteRecord, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        keys.append(*self.record);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DeleteRecord> {
        DeleteRecord { table: *keys.pop_front()?, record: *keys.pop_front()? }
            .verify(ref keys, ref data)
    }
}

impl DeleteRecordsEvent of Event<DeleteRecords> {
    fn append_keys_and_data(
        self: @DeleteRecords, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        data.append_span(*self.records)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DeleteRecords> {
        DeleteRecords { table: *keys.pop_front()?, records: data.drain() }.verify_keys(ref keys)
    }
}

impl DeleteFieldEvent of Event<DeleteField> {
    fn append_keys_and_data(
        self: @DeleteField, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        keys.append(*self.record);
        keys.append(*self.column);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DeleteField> {
        DeleteField {
            table: *keys.pop_front()?, record: *keys.pop_front()?, column: *keys.pop_front()?,
        }
            .verify(ref keys, ref data)
    }
}

impl DeleteFieldsEvent of Event<DeleteFields> {
    fn append_keys_and_data(
        self: @DeleteFields, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        keys.append(*self.record);
        data.append_span(*self.columns)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DeleteFields> {
        DeleteFields {
            table: *keys.pop_front()?, record: *keys.pop_front()?, columns: data.drain(),
        }
            .verify_keys(ref keys)
    }
}

impl DeletesFieldEvent of Event<DeletesField> {
    fn append_keys_and_data(
        self: @DeletesField, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        keys.append(*self.column);
        data.append_span(*self.records)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DeletesField> {
        DeletesField {
            table: *keys.pop_front()?, column: *keys.pop_front()?, records: data.drain(),
        }
            .verify_keys(ref keys)
    }
}

impl DeletesFieldsEvent of Event<DeletesFields> {
    fn append_keys_and_data(
        self: @DeletesFields, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        self.records.iserialize(ref data);
        data.append_span(*self.columns)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DeletesFields> {
        DeletesFields {
            table: *keys.pop_front()?,
            records: ISerde::ideserialize(ref data)?,
            columns: data.drain(),
        }
            .verify(ref keys, ref data)
    }
}

impl DeleteFieldGroupEvent of Event<DeleteFieldGroup> {
    fn append_keys_and_data(
        self: @DeleteFieldGroup, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        keys.append(*self.record);
        keys.append(*self.group);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DeleteFieldGroup> {
        DeleteFieldGroup {
            table: *keys.pop_front()?, record: *keys.pop_front()?, group: *keys.pop_front()?,
        }
            .verify(ref keys, ref data)
    }
}


impl DeleteFieldGroupsEvent of Event<DeleteFieldGroups> {
    fn append_keys_and_data(
        self: @DeleteFieldGroups, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        keys.append(*self.record);
        data.append_span(*self.groups)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DeleteFieldGroups> {
        DeleteFieldGroups {
            table: *keys.pop_front()?, record: *keys.pop_front()?, groups: data.drain(),
        }
            .verify_keys(ref keys)
    }
}

impl DeletesFieldGroupEvent of Event<DeletesFieldGroup> {
    fn append_keys_and_data(
        self: @DeletesFieldGroup, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        keys.append(*self.group);
        data.append_span(*self.records)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DeletesFieldGroup> {
        DeletesFieldGroup {
            table: *keys.pop_front()?, group: *keys.pop_front()?, records: data.drain(),
        }
            .verify_keys(ref keys)
    }
}


impl DeletesFieldGroupsEvent of Event<DeletesFieldGroups> {
    fn append_keys_and_data(
        self: @DeletesFieldGroups, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.table);
        self.records.iserialize(ref data);
        data.append_span(*self.groups)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DeletesFieldGroups> {
        DeletesFieldGroups {
            table: *keys.pop_front()?,
            records: ISerde::ideserialize(ref data)?,
            groups: data.drain(),
        }
            .verify(ref keys, ref data)
    }
}
