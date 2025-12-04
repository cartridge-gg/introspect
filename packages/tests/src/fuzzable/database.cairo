use introspect_events::database::{
    AddColumn, AddColumns, CreateFieldGroup, CreateIndex, CreateTable, CreateTableFromClassHash,
    CreateTableWithColumns, DeleteField, DeleteFieldGroup, DeleteFieldGroups, DeleteFields,
    DeleteRecord, DeleteRecords, DeletesField, DeletesFieldGroup, DeletesFieldGroups, DeletesFields,
    DropColumn, DropColumns, DropIndex, DropTable, IdName, IdTypeAttributes, InsertField,
    InsertFieldGroup, InsertFieldGroups, InsertFields, InsertRecord, InsertRecords, InsertsField,
    InsertsFieldGroup, InsertsFieldGroups, InsertsFields, RenameColumn, RenameColumns,
    RenamePrimary, RenameTable, RetypeColumn, RetypeColumns, RetypePrimary,
};
use crate::{ByteArrayExt, random_pascal_string, random_snake_string};
use super::schema::generate_column_attributes;
use super::{Fuzzable, FuzzableExtColumnDef, Fuzzy, TypeDefFuzzable};


pub impl IdNameFuzzable of Fuzzable<IdName> {
    fn blank() -> IdName {
        Default::default()
    }

    fn generate() -> IdName {
        let name = random_snake_string(31, 4);
        IdName { id: name.selector(), name }
    }
}

pub impl IdTypeAttributesFuzzy<const MAX_DEPTH: u32> of Fuzzy<IdTypeAttributes> {
    fn generate() -> IdTypeAttributes {
        IdTypeAttributes {
            id: Fuzzable::generate(),
            attributes: generate_column_attributes(),
            type_def: TypeDefFuzzable::generate(MAX_DEPTH),
        }
    }
}

pub impl CreateFieldGroupFuzzable of Fuzzable<CreateFieldGroup> {
    fn blank() -> CreateFieldGroup {
        Default::default()
    }

    fn generate() -> CreateFieldGroup {
        CreateFieldGroup { id: Fuzzable::generate(), columns: Fuzzy::generate_span(300) }
    }
}

pub impl CreateTableFuzzable of Fuzzable<CreateTable> {
    fn blank() -> CreateTable {
        Default::default()
    }

    fn generate() -> CreateTable {
        let name = random_pascal_string(64, 4);
        CreateTable {
            id: name.selector(),
            name,
            attributes: Fuzzy::generate_span_lt(10),
            primary: Fuzzy::generate(),
        }
    }
}

pub impl CreateTableWithColumnsFuzzable<
    const MAX_COLUMNS: u32, const MAX_DEPTH: u32,
> of Fuzzable<CreateTableWithColumns> {
    fn blank() -> CreateTableWithColumns {
        Default::default()
    }

    fn generate() -> CreateTableWithColumns {
        let name = random_pascal_string(64, 4);
        CreateTableWithColumns {
            id: name.selector(),
            name,
            attributes: Fuzzy::generate_span_lt(10),
            primary: Fuzzy::generate(),
            columns: FuzzableExtColumnDef::<MAX_DEPTH>::generate_span(MAX_COLUMNS),
        }
    }
}

pub impl CreateTableFromClassHashFuzzable of Fuzzable<CreateTableFromClassHash> {
    fn blank() -> CreateTableFromClassHash {
        Default::default()
    }

    fn generate() -> CreateTableFromClassHash {
        let name = random_pascal_string(64, 4);
        CreateTableFromClassHash { class_hash: Fuzzable::generate(), id: name.selector(), name }
    }
}


pub impl RenameTableFuzzable of Fuzzable<RenameTable> {
    fn blank() -> RenameTable {
        Default::default()
    }

    fn generate() -> RenameTable {
        let name = random_pascal_string(64, 4);
        RenameTable { id: Fuzzable::generate(), name: name }
    }
}

pub impl DropTableFuzzable of Fuzzable<DropTable> {
    fn blank() -> DropTable {
        Default::default()
    }

    fn generate() -> DropTable {
        DropTable { id: Fuzzable::generate() }
    }
}

pub impl CreateIndexFuzzable of Fuzzable<CreateIndex> {
    fn blank() -> CreateIndex {
        Default::default()
    }

    fn generate() -> CreateIndex {
        let name = random_pascal_string(64, 4);
        CreateIndex {
            table: Fuzzable::generate(),
            id: name.selector(),
            name,
            columns: Fuzzy::generate_span(297),
        }
    }
}

pub impl DropIndexFuzzable of Fuzzable<DropIndex> {
    fn blank() -> DropIndex {
        Default::default()
    }

    fn generate() -> DropIndex {
        DropIndex { table: Fuzzable::generate(), id: Fuzzable::generate() }
    }
}

pub impl RenamePrimaryFuzzable of Fuzzable<RenamePrimary> {
    fn blank() -> RenamePrimary {
        Default::default()
    }

    fn generate() -> RenamePrimary {
        RenamePrimary { table: Fuzzable::generate(), name: random_pascal_string(31, 4) }
    }
}

pub impl RetypePrimaryFuzzable of Fuzzable<RetypePrimary> {
    fn blank() -> RetypePrimary {
        Default::default()
    }

    fn generate() -> RetypePrimary {
        RetypePrimary {
            table: Fuzzable::generate(),
            attributes: Fuzzy::generate_span_lt(10),
            type_def: Fuzzy::generate(),
        }
    }
}

pub impl AddColumnFuzzable of Fuzzable<AddColumn> {
    fn blank() -> AddColumn {
        Default::default()
    }

    fn generate() -> AddColumn {
        let name = random_snake_string(31, 4);
        AddColumn {
            table: Fuzzable::generate(),
            id: name.selector(),
            name,
            type_def: TypeDefFuzzable::generate(10),
            attributes: generate_column_attributes(),
        }
    }
}

pub impl AddColumnsFuzzable of Fuzzable<AddColumns> {
    fn blank() -> AddColumns {
        Default::default()
    }

    fn generate() -> AddColumns {
        AddColumns {
            table: Fuzzable::generate(), columns: FuzzableExtColumnDef::<10>::generate_span_lt(10),
        }
    }
}

pub impl RenameColumnFuzzable of Fuzzable<RenameColumn> {
    fn blank() -> RenameColumn {
        Default::default()
    }

    fn generate() -> RenameColumn {
        RenameColumn {
            table: Fuzzable::generate(), id: Fuzzable::generate(), name: random_snake_string(31, 4),
        }
    }
}

pub impl RenameColumnsFuzzable of Fuzzable<RenameColumns> {
    fn blank() -> RenameColumns {
        Default::default()
    }

    fn generate() -> RenameColumns {
        RenameColumns { table: Fuzzable::generate(), columns: Fuzzy::generate_span_lt(10) }
    }
}

pub impl RetypeColumnFuzzable of Fuzzable<RetypeColumn> {
    fn blank() -> RetypeColumn {
        Default::default()
    }

    fn generate() -> RetypeColumn {
        RetypeColumn {
            table: Fuzzable::generate(),
            id: Fuzzable::generate(),
            attributes: Fuzzy::generate_span_lt(10),
            type_def: TypeDefFuzzable::generate(3),
        }
    }
}


pub impl RetypeColumnsFuzzable<
    const MAX_DEPTH: u32, const COLUMNS: u32,
> of Fuzzable<RetypeColumns> {
    fn blank() -> RetypeColumns {
        Default::default()
    }

    fn generate() -> RetypeColumns {
        RetypeColumns {
            table: Fuzzable::generate(),
            columns: IdTypeAttributesFuzzy::<MAX_DEPTH>::generate_span_lt(COLUMNS),
        }
    }
}

pub impl DropColumnFuzzable of Fuzzable<DropColumn> {
    fn blank() -> DropColumn {
        Default::default()
    }

    fn generate() -> DropColumn {
        DropColumn { table: Fuzzable::generate(), id: Fuzzable::generate() }
    }
}

pub impl DropColumnsFuzzable of Fuzzable<DropColumns> {
    fn blank() -> DropColumns {
        Default::default()
    }

    fn generate() -> DropColumns {
        DropColumns { table: Fuzzable::generate(), ids: Fuzzy::generate_span_lt(10) }
    }
}


pub impl InsertRecordFuzzable of Fuzzable<InsertRecord> {
    fn blank() -> InsertRecord {
        Default::default()
    }
    fn generate() -> InsertRecord {
        InsertRecord {
            table: Fuzzable::generate(),
            record: Fuzzable::generate(),
            data: Fuzzy::generate_span_lt(300),
        }
    }
}
pub impl InsertRecordsFuzzable of Fuzzable<InsertRecords> {
    fn blank() -> InsertRecords {
        Default::default()
    }
    fn generate() -> InsertRecords {
        InsertRecords { table: Fuzzable::generate(), records_data: Fuzzy::generate_span_lt(10) }
    }
}
pub impl InsertFieldFuzzable of Fuzzable<InsertField> {
    fn blank() -> InsertField {
        Default::default()
    }
    fn generate() -> InsertField {
        InsertField {
            table: Fuzzable::generate(),
            record: Fuzzable::generate(),
            column: Fuzzable::generate(),
            data: Fuzzy::generate_span_lt(300),
        }
    }
}
pub impl InsertFieldsFuzzable of Fuzzable<InsertFields> {
    fn blank() -> InsertFields {
        Default::default()
    }
    fn generate() -> InsertFields {
        InsertFields {
            table: Fuzzable::generate(),
            record: Fuzzable::generate(),
            columns: Fuzzy::generate_span_lt(200),
            data: Fuzzy::generate_span_lt(200),
        }
    }
}
pub impl InsertsFieldFuzzable of Fuzzable<InsertsField> {
    fn blank() -> InsertsField {
        Default::default()
    }
    fn generate() -> InsertsField {
        InsertsField {
            table: Fuzzable::generate(),
            column: Fuzzable::generate(),
            records_data: Fuzzy::generate_span_lt(10),
        }
    }
}
pub impl InsertsFieldsFuzzable of Fuzzable<InsertsFields> {
    fn blank() -> InsertsFields {
        Default::default()
    }
    fn generate() -> InsertsFields {
        InsertsFields {
            table: Fuzzable::generate(),
            columns: Fuzzy::generate_span_lt(10),
            records_data: Fuzzy::generate_span_lt(10),
        }
    }
}
pub impl InsertFieldGroupFuzzable of Fuzzable<InsertFieldGroup> {
    fn blank() -> InsertFieldGroup {
        Default::default()
    }
    fn generate() -> InsertFieldGroup {
        InsertFieldGroup {
            table: Fuzzable::generate(),
            record: Fuzzable::generate(),
            group: Fuzzable::generate(),
            data: Fuzzy::generate_span_lt(300),
        }
    }
}
pub impl InsertFieldGroupsFuzzable of Fuzzable<InsertFieldGroups> {
    fn blank() -> InsertFieldGroups {
        Default::default()
    }
    fn generate() -> InsertFieldGroups {
        InsertFieldGroups {
            table: Fuzzable::generate(),
            record: Fuzzable::generate(),
            groups: Fuzzy::generate_span_lt(100),
            data: Fuzzy::generate_span_lt(200),
        }
    }
}
pub impl InsertsFieldGroupFuzzable of Fuzzable<InsertsFieldGroup> {
    fn blank() -> InsertsFieldGroup {
        Default::default()
    }
    fn generate() -> InsertsFieldGroup {
        InsertsFieldGroup {
            table: Fuzzable::generate(),
            group: Fuzzable::generate(),
            records_data: Fuzzy::generate_span_lt(10),
        }
    }
}
pub impl InsertsFieldGroupsFuzzable of Fuzzable<InsertsFieldGroups> {
    fn blank() -> InsertsFieldGroups {
        Default::default()
    }
    fn generate() -> InsertsFieldGroups {
        InsertsFieldGroups {
            table: Fuzzable::generate(),
            groups: Fuzzy::generate_span_lt(10),
            records_data: Fuzzy::generate_span_lt(10),
        }
    }
}


pub impl DeleteRecordFuzzable of Fuzzable<DeleteRecord> {
    fn blank() -> DeleteRecord {
        Default::default()
    }
    fn generate() -> DeleteRecord {
        DeleteRecord { table: Fuzzable::generate(), record: Fuzzable::generate() }
    }
}
pub impl DeleteRecordsFuzzable of Fuzzable<DeleteRecords> {
    fn blank() -> DeleteRecords {
        Default::default()
    }
    fn generate() -> DeleteRecords {
        DeleteRecords { table: Fuzzable::generate(), records: Fuzzy::generate_span_lt(10) }
    }
}
pub impl DeleteFieldFuzzable of Fuzzable<DeleteField> {
    fn blank() -> DeleteField {
        Default::default()
    }
    fn generate() -> DeleteField {
        DeleteField {
            table: Fuzzable::generate(), record: Fuzzable::generate(), column: Fuzzable::generate(),
        }
    }
}
pub impl DeleteFieldsFuzzable of Fuzzable<DeleteFields> {
    fn blank() -> DeleteFields {
        Default::default()
    }
    fn generate() -> DeleteFields {
        DeleteFields {
            table: Fuzzable::generate(),
            record: Fuzzable::generate(),
            columns: Fuzzy::generate_span_lt(10),
        }
    }
}
pub impl DeletesFieldFuzzable of Fuzzable<DeletesField> {
    fn blank() -> DeletesField {
        Default::default()
    }
    fn generate() -> DeletesField {
        DeletesField {
            table: Fuzzable::generate(),
            column: Fuzzable::generate(),
            records: Fuzzy::generate_span_lt(10),
        }
    }
}
pub impl DeletesFieldsFuzzable of Fuzzable<DeletesFields> {
    fn blank() -> DeletesFields {
        Default::default()
    }
    fn generate() -> DeletesFields {
        DeletesFields {
            table: Fuzzable::generate(),
            records: Fuzzy::generate_span_lt(10),
            columns: Fuzzy::generate_span_lt(10),
        }
    }
}
pub impl DeleteFieldGroupFuzzable of Fuzzable<DeleteFieldGroup> {
    fn blank() -> DeleteFieldGroup {
        Default::default()
    }
    fn generate() -> DeleteFieldGroup {
        DeleteFieldGroup {
            table: Fuzzable::generate(), record: Fuzzable::generate(), group: Fuzzable::generate(),
        }
    }
}
pub impl DeleteFieldGroupsFuzzable of Fuzzable<DeleteFieldGroups> {
    fn blank() -> DeleteFieldGroups {
        Default::default()
    }
    fn generate() -> DeleteFieldGroups {
        DeleteFieldGroups {
            table: Fuzzable::generate(),
            record: Fuzzable::generate(),
            groups: Fuzzy::generate_span_lt(10),
        }
    }
}
pub impl DeletesFieldGroupFuzzable of Fuzzable<DeletesFieldGroup> {
    fn blank() -> DeletesFieldGroup {
        Default::default()
    }
    fn generate() -> DeletesFieldGroup {
        DeletesFieldGroup {
            table: Fuzzable::generate(),
            group: Fuzzable::generate(),
            records: Fuzzy::generate_span_lt(10),
        }
    }
}
pub impl DeletesFieldGroupsFuzzable of Fuzzable<DeletesFieldGroups> {
    fn blank() -> DeletesFieldGroups {
        Default::default()
    }
    fn generate() -> DeletesFieldGroups {
        DeletesFieldGroups {
            table: Fuzzable::generate(),
            records: Fuzzy::generate_span_lt(10),
            groups: Fuzzy::generate_span_lt(10),
        }
    }
}
