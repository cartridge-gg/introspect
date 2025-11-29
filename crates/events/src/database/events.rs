use introspect_types::schema::{PrimaryDef, PrimaryTypeDef};
use introspect_types::utils::ideserialize_utf8_string;
use introspect_types::{Attribute, ColumnDef, FeltIterator, ISerde, TypeDef};
use serde::{Deserialize, Serialize};
use starknet_types_core::felt::Felt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateFieldGroup {
    pub id: Felt,
    pub columns: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTable {
    pub id: Felt,
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub primary: PrimaryDef,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTableWithColumns {
    pub id: Felt,
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub primary: PrimaryDef,
    pub columns: Vec<ColumnDef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTableFromClassHash {
    pub id: Felt,
    pub name: String,
    pub class_hash: Felt,
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
    pub columns: Vec<IdTypeAttributes>,
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
    pub name: Felt,
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
    pub record: Felt,
    pub data: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsertRecords {
    pub table: Felt,
    pub records_data: Vec<IdData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsertField {
    pub table: Felt,
    pub column: Felt,
    pub record: Felt,
    pub data: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsertFields {
    pub table: Felt,
    pub record: Felt,
    pub columns: Vec<Felt>,
    pub data: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsertsField {
    pub table: Felt,
    pub column: Felt,
    pub records_data: Vec<IdData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsertsFields {
    pub table: Felt,
    pub columns: Vec<Felt>,
    pub records_data: Vec<IdData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsertFieldGroup {
    pub table: Felt,
    pub record: Felt,
    pub group: Felt,
    pub data: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsertFieldGroups {
    pub table: Felt,
    pub record: Felt,
    pub groups: Vec<Felt>,
    pub data: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsertsFieldGroup {
    pub table: Felt,
    pub group: Felt,
    pub records_data: Vec<IdData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsertsFieldGroups {
    pub table: Felt,
    pub record: Felt,
    pub groups: Vec<Felt>,
    pub records_data: Vec<IdData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteRecord {
    pub table: Felt,
    pub record: Felt,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteRecords {
    pub table: Felt,
    pub records: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteField {
    pub table: Felt,
    pub record: Felt,
    pub column: Felt,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteFields {
    pub table: Felt,
    pub record: Felt,
    pub columns: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeletesField {
    pub table: Felt,
    pub column: Felt,
    pub records: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeletesFields {
    pub table: Felt,
    pub records: Vec<Felt>,
    pub columns: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteFieldGroup {
    pub table: Felt,
    pub record: Felt,
    pub group: Felt,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteFieldGroups {
    pub table: Felt,
    pub record: Felt,
    pub groups: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeletesFieldGroup {
    pub table: Felt,
    pub group: Felt,
    pub records: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeletesFieldGroups {
    pub table: Felt,
    pub records: Vec<Felt>,
    pub groups: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdName {
    pub id: Felt,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdTypeAttributes {
    pub id: Felt,
    pub attributes: Vec<Attribute>,
    pub type_def: TypeDef,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdData {
    pub id: Felt,
    pub data: Vec<Felt>,
}

impl ISerde for IdName {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        let id = data.next()?;
        let name = ideserialize_utf8_string(data)?;
        Some(IdName { id, name })
    }
}

impl ISerde for IdTypeAttributes {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        Some(IdTypeAttributes {
            id: data.next()?,
            attributes: Vec::<Attribute>::ideserialize(data)?,
            type_def: TypeDef::ideserialize(data)?,
        })
    }
}

impl ISerde for IdData {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        Some(IdData {
            id: data.next()?,
            data: Vec::<Felt>::ideserialize(data)?,
        })
    }
}
