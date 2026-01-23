use introspect_types::schema::{PrimaryDef, PrimaryTypeDef};
use introspect_types::utils::ideserialize_utf8_string;
use introspect_types::{Attribute, ColumnDef, FeltIterator, ISerde, TypeDef};
use serde::{Deserialize, Serialize};
use starknet_types_core::felt::Felt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateColumnSet {
    pub id: Felt,
    pub columns: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTable {
    pub id: Felt,
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub primary: PrimaryDef,
    pub columns: Vec<ColumnDef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTableFromClass {
    pub id: Felt,
    pub name: String,
    pub class_hash: Felt,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTableFromContract {
    pub id: Felt,
    pub name: String,
    pub contract_address: Felt,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RenameTable {
    pub id: Felt,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DropTable {
    pub id: Felt,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RenamePrimary {
    pub table: Felt,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RetypePrimary {
    pub table: Felt,
    pub attributes: Vec<Attribute>,
    pub type_def: PrimaryTypeDef,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddColumn {
    pub table: Felt,
    pub id: Felt,
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub type_def: TypeDef,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddColumns {
    pub table: Felt,
    pub columns: Vec<ColumnDef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RenameColumn {
    pub table: Felt,
    pub id: Felt,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RenameColumns {
    pub table: Felt,
    pub columns: Vec<IdName>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RetypeColumn {
    pub table: Felt,
    pub id: Felt,
    pub attributes: Vec<Attribute>,
    pub type_def: TypeDef,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RetypeColumns {
    pub table: Felt,
    pub columns: Vec<IdTypeDef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DropColumn {
    pub table: Felt,
    pub id: Felt,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DropColumns {
    pub table: Felt,
    pub ids: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateIndex {
    pub table: Felt,
    pub id: Felt,
    pub name: String,
    pub columns: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DropIndex {
    pub table: Felt,
    pub id: Felt,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsertRecord {
    pub table: Felt,
    pub row: Felt,
    pub data: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsertRecords {
    pub table: Felt,
    pub entries: Vec<Entry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsertField {
    pub table: Felt,
    pub row: Felt,
    pub column: Felt,
    pub data: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsertFields {
    pub table: Felt,
    pub row: Felt,
    pub columns: Vec<Felt>,
    pub data: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsertsField {
    pub table: Felt,
    pub column: Felt,
    pub entries: Vec<Entry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsertsFields {
    pub table: Felt,
    pub columns: Vec<Felt>,
    pub entries: Vec<Entry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsertFieldSet {
    pub table: Felt,
    pub row: Felt,
    pub set: Felt,
    pub data: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsertFieldSets {
    pub table: Felt,
    pub row: Felt,
    pub sets: Vec<Felt>,
    pub data: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsertsFieldSet {
    pub table: Felt,
    pub set: Felt,
    pub entries: Vec<Entry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsertsFieldSets {
    pub table: Felt,
    pub sets: Vec<Felt>,
    pub entries: Vec<Entry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteRecord {
    pub table: Felt,
    pub row: Felt,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteRecords {
    pub table: Felt,
    pub rows: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteField {
    pub table: Felt,
    pub row: Felt,
    pub column: Felt,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteFields {
    pub table: Felt,
    pub row: Felt,
    pub columns: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeletesField {
    pub table: Felt,
    pub rows: Vec<Felt>,
    pub column: Felt,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeletesFields {
    pub table: Felt,
    pub rows: Vec<Felt>,
    pub columns: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteFieldSet {
    pub table: Felt,
    pub row: Felt,
    pub set: Felt,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteFieldSets {
    pub table: Felt,
    pub row: Felt,
    pub sets: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeletesFieldSet {
    pub table: Felt,
    pub rows: Vec<Felt>,
    pub set: Felt,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeletesFieldSets {
    pub table: Felt,
    pub rows: Vec<Felt>,
    pub sets: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdName {
    pub id: Felt,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdTypeDef {
    pub id: Felt,
    pub attributes: Vec<Attribute>,
    pub type_def: TypeDef,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entry {
    pub row: Felt,
    pub data: Vec<Felt>,
}

impl ISerde for IdName {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        let id = data.next()?;
        let name = ideserialize_utf8_string(data)?;
        Some(IdName { id, name })
    }
}

impl ISerde for IdTypeDef {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        Some(IdTypeDef {
            id: data.next()?,
            attributes: Vec::<Attribute>::ideserialize(data)?,
            type_def: TypeDef::ideserialize(data)?,
        })
    }
}

impl ISerde for Entry {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        Some(Entry {
            row: data.next()?,
            data: Vec::<Felt>::ideserialize(data)?,
        })
    }
}
