use introspect_types::schema::{PrimaryDef, PrimaryTypeDef};
use introspect_types::{
    Attribute, CairoDeserialize, CairoDeserializer, ColumnDef, DecodeResult, TypeDef,
};
use starknet_types_core::felt::Felt;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateColumnSet {
    pub id: Felt,
    pub columns: Vec<Felt>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateTable {
    pub id: Felt,
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub primary: PrimaryDef,
    pub columns: Vec<ColumnDef>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateTableFromClass {
    pub id: Felt,
    pub name: String,
    pub class_hash: Felt,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateTableFromContract {
    pub id: Felt,
    pub name: String,
    pub contract_address: Felt,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RenameTable {
    pub id: Felt,
    pub name: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DropTable {
    pub id: Felt,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RenamePrimary {
    pub table: Felt,
    pub name: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RetypePrimary {
    pub table: Felt,
    pub attributes: Vec<Attribute>,
    pub type_def: PrimaryTypeDef,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AddColumn {
    pub table: Felt,
    pub id: Felt,
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub type_def: TypeDef,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AddColumns {
    pub table: Felt,
    pub columns: Vec<ColumnDef>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RenameColumn {
    pub table: Felt,
    pub id: Felt,
    pub name: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RenameColumns {
    pub table: Felt,
    pub columns: Vec<IdName>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RetypeColumn {
    pub table: Felt,
    pub id: Felt,
    pub attributes: Vec<Attribute>,
    pub type_def: TypeDef,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RetypeColumns {
    pub table: Felt,
    pub columns: Vec<IdTypeDef>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DropColumn {
    pub table: Felt,
    pub id: Felt,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DropColumns {
    pub table: Felt,
    pub ids: Vec<Felt>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateIndex {
    pub table: Felt,
    pub id: Felt,
    pub attributes: Vec<Attribute>,
    pub columns: Vec<Felt>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DropIndex {
    pub table: Felt,
    pub id: Felt,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InsertRecord {
    pub table: Felt,
    pub row: Felt,
    pub data: Vec<Felt>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InsertRecords {
    pub table: Felt,
    pub entries: Vec<Entry>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InsertField {
    pub table: Felt,
    pub row: Felt,
    pub column: Felt,
    pub data: Vec<Felt>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InsertFields {
    pub table: Felt,
    pub row: Felt,
    pub columns: Vec<Felt>,
    pub data: Vec<Felt>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InsertsField {
    pub table: Felt,
    pub column: Felt,
    pub entries: Vec<Entry>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InsertsFields {
    pub table: Felt,
    pub columns: Vec<Felt>,
    pub entries: Vec<Entry>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InsertFieldSet {
    pub table: Felt,
    pub row: Felt,
    pub set: Felt,
    pub data: Vec<Felt>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InsertFieldSets {
    pub table: Felt,
    pub row: Felt,
    pub sets: Vec<Felt>,
    pub data: Vec<Felt>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InsertsFieldSet {
    pub table: Felt,
    pub set: Felt,
    pub entries: Vec<Entry>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InsertsFieldSets {
    pub table: Felt,
    pub sets: Vec<Felt>,
    pub entries: Vec<Entry>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeleteRecord {
    pub table: Felt,
    pub row: Felt,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeleteRecords {
    pub table: Felt,
    pub rows: Vec<Felt>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeleteField {
    pub table: Felt,
    pub row: Felt,
    pub column: Felt,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeleteFields {
    pub table: Felt,
    pub row: Felt,
    pub columns: Vec<Felt>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletesField {
    pub table: Felt,
    pub rows: Vec<Felt>,
    pub column: Felt,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletesFields {
    pub table: Felt,
    pub rows: Vec<Felt>,
    pub columns: Vec<Felt>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeleteFieldSet {
    pub table: Felt,
    pub row: Felt,
    pub set: Felt,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeleteFieldSets {
    pub table: Felt,
    pub row: Felt,
    pub sets: Vec<Felt>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletesFieldSet {
    pub table: Felt,
    pub rows: Vec<Felt>,
    pub set: Felt,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletesFieldSets {
    pub table: Felt,
    pub rows: Vec<Felt>,
    pub sets: Vec<Felt>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IdName {
    pub id: Felt,
    pub name: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IdTypeDef {
    pub id: Felt,
    pub attributes: Vec<Attribute>,
    pub type_def: TypeDef,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Entry {
    pub row: Felt,
    pub data: Vec<Felt>,
}

impl<D: CairoDeserializer> CairoDeserialize<D> for IdName {
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        let id = deserializer.next_felt()?;
        let name = deserializer.next_string()?;
        Ok(IdName { id, name })
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for IdTypeDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        Ok(IdTypeDef {
            id: deserializer.next_felt()?,
            attributes: Vec::<Attribute>::deserialize(deserializer)?,
            type_def: TypeDef::deserialize(deserializer)?,
        })
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for Entry {
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        Ok(Entry {
            row: deserializer.next_felt()?,
            data: deserializer.next_array()?,
        })
    }
}
