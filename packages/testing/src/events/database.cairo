use cgg_utils::testing::{ByteArrayExt, Fuzzy, random_pascal_string, random_snake_string};
use introspect_events::database::{
    AddColumn, AddColumns, CreateColumnSet, CreateIndex, CreateTable, CreateTableFromClass,
    CreateTableFromContract, DeleteField, DeleteFieldSet, DeleteFieldSets, DeleteFields,
    DeleteRecord, DeleteRecords, DeletesField, DeletesFieldSet, DeletesFieldSets, DeletesFields,
    DropColumn, DropColumns, DropIndex, DropTable, IdName, IdTypeDef, InsertField, InsertFieldSet,
    InsertFieldSets, InsertFields, InsertRecord, InsertRecords, InsertsField, InsertsFieldSet,
    InsertsFieldSets, InsertsFields, RenameColumn, RenameColumns, RenamePrimary, RenameTable,
    RetypeColumn, RetypeColumns, RetypePrimary,
};
use introspect_test_utils::types::{
    EntryFuzzable, FuzzableAttribute, FuzzableExtColumnDef, PrimaryDefFuzzable,
    PrimaryTypeDefFuzzable, TypeDefFuzzable, generate_column_attributes,
};
use snforge_std::fuzzable::Fuzzable;

pub impl IdNameFuzzable of Fuzzable<IdName> {
    fn blank() -> IdName {
        Default::default()
    }

    fn generate() -> IdName {
        let name = random_snake_string(31, 4);
        IdName { id: name.selector(), name }
    }
}

pub impl IdTypeAttributesFuzzy<const MAX_DEPTH: u32> of Fuzzy<IdTypeDef> {
    fn generate() -> IdTypeDef {
        IdTypeDef {
            id: Fuzzable::generate(),
            attributes: generate_column_attributes(),
            type_def: TypeDefFuzzable::generate(MAX_DEPTH),
        }
    }
}

pub impl CreateFieldSetFuzzable of Fuzzable<CreateColumnSet> {
    fn blank() -> CreateColumnSet {
        Default::default()
    }

    fn generate() -> CreateColumnSet {
        CreateColumnSet { id: Fuzzable::generate(), columns: Fuzzy::generate_span(300) }
    }
}

pub impl CreateTableFuzzable<
    const MAX_COLUMNS: u32, const MAX_DEPTH: u32,
> of Fuzzable<CreateTable> {
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
            columns: FuzzableExtColumnDef::<MAX_DEPTH>::generate_span(MAX_COLUMNS),
        }
    }
}

pub impl CreateTableFromClassFuzzable of Fuzzable<CreateTableFromClass> {
    fn blank() -> CreateTableFromClass {
        Default::default()
    }

    fn generate() -> CreateTableFromClass {
        let name = random_pascal_string(64, 4);
        CreateTableFromClass { class_hash: Fuzzable::generate(), id: name.selector(), name }
    }
}

pub impl CreateTableFromContractFuzzable of Fuzzable<CreateTableFromContract> {
    fn blank() -> CreateTableFromContract {
        Default::default()
    }

    fn generate() -> CreateTableFromContract {
        let name = random_pascal_string(64, 4);
        CreateTableFromContract {
            contract_address: Fuzzable::generate(), id: name.selector(), name,
        }
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
        CreateIndex {
            table: Fuzzable::generate(),
            id: Fuzzable::generate(),
            attributes: [].span(),
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
            row: Fuzzable::generate(),
            data: Fuzzy::generate_span_lt(300),
        }
    }
}
pub impl InsertRecordsFuzzable of Fuzzable<InsertRecords> {
    fn blank() -> InsertRecords {
        Default::default()
    }
    fn generate() -> InsertRecords {
        InsertRecords { table: Fuzzable::generate(), entries: Fuzzy::generate_span_lt(10) }
    }
}
pub impl InsertFieldFuzzable of Fuzzable<InsertField> {
    fn blank() -> InsertField {
        Default::default()
    }
    fn generate() -> InsertField {
        InsertField {
            table: Fuzzable::generate(),
            row: Fuzzable::generate(),
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
            row: Fuzzable::generate(),
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
            entries: Fuzzy::generate_span_lt(10),
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
            entries: Fuzzy::generate_span_lt(10),
        }
    }
}
pub impl InsertFieldSetFuzzable of Fuzzable<InsertFieldSet> {
    fn blank() -> InsertFieldSet {
        Default::default()
    }
    fn generate() -> InsertFieldSet {
        InsertFieldSet {
            table: Fuzzable::generate(),
            row: Fuzzable::generate(),
            set: Fuzzable::generate(),
            data: Fuzzy::generate_span_lt(300),
        }
    }
}
pub impl InsertFieldSetsFuzzable of Fuzzable<InsertFieldSets> {
    fn blank() -> InsertFieldSets {
        Default::default()
    }
    fn generate() -> InsertFieldSets {
        InsertFieldSets {
            table: Fuzzable::generate(),
            row: Fuzzable::generate(),
            sets: Fuzzy::generate_span_lt(100),
            data: Fuzzy::generate_span_lt(200),
        }
    }
}
pub impl InsertsFieldSetFuzzable of Fuzzable<InsertsFieldSet> {
    fn blank() -> InsertsFieldSet {
        Default::default()
    }
    fn generate() -> InsertsFieldSet {
        InsertsFieldSet {
            table: Fuzzable::generate(),
            set: Fuzzable::generate(),
            entries: Fuzzy::generate_span_lt(10),
        }
    }
}
pub impl InsertsFieldSetsFuzzable of Fuzzable<InsertsFieldSets> {
    fn blank() -> InsertsFieldSets {
        Default::default()
    }
    fn generate() -> InsertsFieldSets {
        InsertsFieldSets {
            table: Fuzzable::generate(),
            sets: Fuzzy::generate_span_lt(10),
            entries: Fuzzy::generate_span_lt(10),
        }
    }
}


pub impl DeleteRecordFuzzable of Fuzzable<DeleteRecord> {
    fn blank() -> DeleteRecord {
        Default::default()
    }
    fn generate() -> DeleteRecord {
        DeleteRecord { table: Fuzzable::generate(), row: Fuzzable::generate() }
    }
}
pub impl DeleteRecordsFuzzable of Fuzzable<DeleteRecords> {
    fn blank() -> DeleteRecords {
        Default::default()
    }
    fn generate() -> DeleteRecords {
        DeleteRecords { table: Fuzzable::generate(), rows: Fuzzy::generate_span_lt(10) }
    }
}
pub impl DeleteFieldFuzzable of Fuzzable<DeleteField> {
    fn blank() -> DeleteField {
        Default::default()
    }
    fn generate() -> DeleteField {
        DeleteField {
            table: Fuzzable::generate(), row: Fuzzable::generate(), column: Fuzzable::generate(),
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
            row: Fuzzable::generate(),
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
            rows: Fuzzy::generate_span_lt(10),
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
            rows: Fuzzy::generate_span_lt(10),
            columns: Fuzzy::generate_span_lt(10),
        }
    }
}
pub impl DeleteFieldSetFuzzable of Fuzzable<DeleteFieldSet> {
    fn blank() -> DeleteFieldSet {
        Default::default()
    }
    fn generate() -> DeleteFieldSet {
        DeleteFieldSet {
            table: Fuzzable::generate(), row: Fuzzable::generate(), set: Fuzzable::generate(),
        }
    }
}
pub impl DeleteFieldSetsFuzzable of Fuzzable<DeleteFieldSets> {
    fn blank() -> DeleteFieldSets {
        Default::default()
    }
    fn generate() -> DeleteFieldSets {
        DeleteFieldSets {
            table: Fuzzable::generate(),
            row: Fuzzable::generate(),
            sets: Fuzzy::generate_span_lt(10),
        }
    }
}
pub impl DeletesFieldSetFuzzable of Fuzzable<DeletesFieldSet> {
    fn blank() -> DeletesFieldSet {
        Default::default()
    }
    fn generate() -> DeletesFieldSet {
        DeletesFieldSet {
            table: Fuzzable::generate(),
            set: Fuzzable::generate(),
            rows: Fuzzy::generate_span_lt(10),
        }
    }
}
pub impl DeletesFieldSetsFuzzable of Fuzzable<DeletesFieldSets> {
    fn blank() -> DeletesFieldSets {
        Default::default()
    }
    fn generate() -> DeletesFieldSets {
        DeletesFieldSets {
            table: Fuzzable::generate(),
            rows: Fuzzy::generate_span_lt(10),
            sets: Fuzzy::generate_span_lt(10),
        }
    }
}
