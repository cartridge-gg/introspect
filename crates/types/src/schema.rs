use crate::deserialize::FeltToPrimitive;
use crate::parser::TypeParserResult;
use crate::{
    Attribute, Attributes, Bytes31EncodedDef, ElementDef, Primary, PrimaryValue, ResultInto,
    TypeDef, felt_to_bytes31_bytes, felt_to_utf8_string,
};
use blake3::Hash;
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use starknet_types_core::felt::Felt;
use std::ops::Deref;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableSchema {
    pub id: Felt,
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub primary: PrimaryDef,
    pub columns: Vec<ColumnDef>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ColumnDef {
    pub id: Felt,
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub type_def: TypeDef,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ColumnInfo {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub type_def: TypeDef,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PrimaryDef {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub type_def: PrimaryTypeDef,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum PrimaryTypeDef {
    #[default]
    Felt252,
    ShortUtf8,
    Bytes31,
    Bytes31Encoded(Bytes31EncodedDef),
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    ClassHash,
    ContractAddress,
    EthAddress,
    StorageAddress,
    StorageBaseAddress,
}

impl ElementDef for PrimaryTypeDef {}
impl ElementDef for PrimaryDef {}

impl PrimaryTypeDef {
    pub fn item_name(&self) -> &'static str {
        match self {
            PrimaryTypeDef::Felt252 => "Felt252",
            PrimaryTypeDef::ShortUtf8 => "ShortUtf8",
            PrimaryTypeDef::Bytes31 => "Bytes31",
            PrimaryTypeDef::Bytes31Encoded(_) => "Bytes31Encoded",
            PrimaryTypeDef::Bool => "Bool",
            PrimaryTypeDef::U8 => "U8",
            PrimaryTypeDef::U16 => "U16",
            PrimaryTypeDef::U32 => "U32",
            PrimaryTypeDef::U64 => "U64",
            PrimaryTypeDef::U128 => "U128",
            PrimaryTypeDef::I8 => "I8",
            PrimaryTypeDef::I16 => "I16",
            PrimaryTypeDef::I32 => "I32",
            PrimaryTypeDef::I64 => "I64",
            PrimaryTypeDef::I128 => "I128",
            PrimaryTypeDef::ClassHash => "ClassHash",
            PrimaryTypeDef::ContractAddress => "ContractAddress",
            PrimaryTypeDef::EthAddress => "EthAddress",
            PrimaryTypeDef::StorageAddress => "StorageAddress",
            PrimaryTypeDef::StorageBaseAddress => "StorageBaseAddress",
        }
    }

    pub fn to_primary_value(&self, felt: Felt) -> TypeParserResult<PrimaryValue> {
        match self {
            PrimaryTypeDef::Felt252 => Ok(PrimaryValue::Felt252(felt)),
            PrimaryTypeDef::ShortUtf8 => {
                felt_to_utf8_string(felt).map_into(PrimaryValue::ShortUtf8)
            }
            PrimaryTypeDef::Bytes31 => felt_to_bytes31_bytes(felt).map_into(PrimaryValue::Bytes31),
            PrimaryTypeDef::Bytes31Encoded(e) => e
                .to_encoded_bytes_31(felt)
                .map(PrimaryValue::Bytes31Encoded),
            PrimaryTypeDef::Bool => Ok(PrimaryValue::Bool(!felt.is_zero())),
            PrimaryTypeDef::U8 => felt.to_primitive().map_into(PrimaryValue::U8),
            PrimaryTypeDef::U16 => felt.to_primitive().map_into(PrimaryValue::U16),
            PrimaryTypeDef::U32 => felt.to_primitive().map_into(PrimaryValue::U32),
            PrimaryTypeDef::U64 => felt.to_primitive().map_into(PrimaryValue::U64),
            PrimaryTypeDef::U128 => felt.to_primitive().map_into(PrimaryValue::U128),
            PrimaryTypeDef::I8 => felt.to_primitive().map_into(PrimaryValue::I8),
            PrimaryTypeDef::I16 => felt.to_primitive().map_into(PrimaryValue::I16),
            PrimaryTypeDef::I32 => felt.to_primitive().map_into(PrimaryValue::I32),
            PrimaryTypeDef::I64 => felt.to_primitive().map_into(PrimaryValue::I64),
            PrimaryTypeDef::I128 => felt.to_primitive().map_into(PrimaryValue::I128),
            PrimaryTypeDef::ClassHash => Ok(PrimaryValue::ClassHash(felt)),
            PrimaryTypeDef::ContractAddress => Ok(PrimaryValue::ContractAddress(felt)),
            PrimaryTypeDef::EthAddress => Ok(PrimaryValue::EthAddress(felt)),
            PrimaryTypeDef::StorageAddress => Ok(PrimaryValue::StorageAddress(felt)),
            PrimaryTypeDef::StorageBaseAddress => Ok(PrimaryValue::StorageBaseAddress(felt)),
        }
    }
}

impl From<PrimaryTypeDef> for TypeDef {
    fn from(value: PrimaryTypeDef) -> Self {
        match value {
            PrimaryTypeDef::Felt252 => TypeDef::Felt252,
            PrimaryTypeDef::ShortUtf8 => TypeDef::ShortUtf8,
            PrimaryTypeDef::Bytes31 => TypeDef::Bytes31,
            PrimaryTypeDef::Bytes31Encoded(e) => TypeDef::Bytes31Encoded(e),
            PrimaryTypeDef::Bool => TypeDef::Bool,
            PrimaryTypeDef::U8 => TypeDef::U8,
            PrimaryTypeDef::U16 => TypeDef::U16,
            PrimaryTypeDef::U32 => TypeDef::U32,
            PrimaryTypeDef::U64 => TypeDef::U64,
            PrimaryTypeDef::U128 => TypeDef::U128,
            PrimaryTypeDef::I8 => TypeDef::I8,
            PrimaryTypeDef::I16 => TypeDef::I16,
            PrimaryTypeDef::I32 => TypeDef::I32,
            PrimaryTypeDef::I64 => TypeDef::I64,
            PrimaryTypeDef::I128 => TypeDef::I128,
            PrimaryTypeDef::ClassHash => TypeDef::ClassHash,
            PrimaryTypeDef::ContractAddress => TypeDef::ContractAddress,
            PrimaryTypeDef::EthAddress => TypeDef::EthAddress,
            PrimaryTypeDef::StorageAddress => TypeDef::StorageAddress,
            PrimaryTypeDef::StorageBaseAddress => TypeDef::StorageBaseAddress,
        }
    }
}

impl From<&PrimaryTypeDef> for TypeDef {
    fn from(value: &PrimaryTypeDef) -> Self {
        match value {
            PrimaryTypeDef::Felt252 => TypeDef::Felt252,
            PrimaryTypeDef::ShortUtf8 => TypeDef::ShortUtf8,
            PrimaryTypeDef::Bytes31 => TypeDef::Bytes31,
            PrimaryTypeDef::Bytes31Encoded(e) => TypeDef::Bytes31Encoded(e.clone()),
            PrimaryTypeDef::Bool => TypeDef::Bool,
            PrimaryTypeDef::U8 => TypeDef::U8,
            PrimaryTypeDef::U16 => TypeDef::U16,
            PrimaryTypeDef::U32 => TypeDef::U32,
            PrimaryTypeDef::U64 => TypeDef::U64,
            PrimaryTypeDef::U128 => TypeDef::U128,
            PrimaryTypeDef::I8 => TypeDef::I8,
            PrimaryTypeDef::I16 => TypeDef::I16,
            PrimaryTypeDef::I32 => TypeDef::I32,
            PrimaryTypeDef::I64 => TypeDef::I64,
            PrimaryTypeDef::I128 => TypeDef::I128,
            PrimaryTypeDef::ClassHash => TypeDef::ClassHash,
            PrimaryTypeDef::ContractAddress => TypeDef::ContractAddress,
            PrimaryTypeDef::EthAddress => TypeDef::EthAddress,
            PrimaryTypeDef::StorageAddress => TypeDef::StorageAddress,
            PrimaryTypeDef::StorageBaseAddress => TypeDef::StorageBaseAddress,
        }
    }
}

impl PrimaryDef {
    pub fn new(name: String, attributes: Vec<Attribute>, type_def: PrimaryTypeDef) -> Self {
        PrimaryDef {
            name,
            attributes,
            type_def,
        }
    }
    pub fn to_primary(&self, felt: Felt) -> TypeParserResult<Primary> {
        Ok(Primary {
            name: self.name.clone(),
            attributes: self.attributes.clone(),
            value: self.type_def.to_primary_value(felt)?,
        })
    }

    pub fn to_primary_value(&self, felt: Felt) -> TypeParserResult<PrimaryValue> {
        self.type_def.to_primary_value(felt)
    }
}

impl ColumnDef {
    pub fn new(id: Felt, name: String, attributes: Vec<Attribute>, type_def: TypeDef) -> Self {
        ColumnDef {
            id,
            name,
            attributes,
            type_def,
        }
    }
}

impl ColumnInfo {
    pub fn new(name: String, attributes: Vec<Attribute>, type_def: TypeDef) -> Self {
        ColumnInfo {
            name,
            attributes,
            type_def,
        }
    }
}

// pub trait ColumnDefs {
//     fn as_hash_map(self) -> HashMap<Felt, ColumnDef>;
//     fn as_info_map(self) -> HashMap<Felt, ColumnInfo>;
// }

// impl ColumnDefs for Vec<ColumnDef> {
//     fn as_hash_map(self) -> HashMap<Felt, ColumnDef> {
//         self.into_iter().map(|col| (col.id.clone(), col)).collect()
//     }
//     fn as_info_map(self) -> HashMap<Felt, ColumnInfo> {
//         self.into_iter().map(Into::into).collect()
//     }
// }

impl Attributes for TableSchema {
    fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

impl Attributes for ColumnDef {
    fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

impl Attributes for PrimaryDef {
    fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

impl Attributes for ColumnInfo {
    fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

impl From<ColumnDef> for (Felt, ColumnInfo) {
    fn from(value: ColumnDef) -> Self {
        (
            value.id,
            ColumnInfo {
                name: value.name,
                attributes: value.attributes,
                type_def: value.type_def,
            },
        )
    }
}

impl From<(Felt, ColumnInfo)> for ColumnDef {
    fn from(value: (Felt, ColumnInfo)) -> Self {
        ColumnDef {
            id: value.0,
            name: value.1.name,
            attributes: value.1.attributes,
            type_def: value.1.type_def,
        }
    }
}

pub trait FeltIds {
    fn ids(&self) -> Vec<Felt>;
    fn hash(&self) -> Hash {
        let ids = self.ids();
        match ids.len() {
            0 => Hash::from([0; 32]),
            1 => Hash::from(ids[0].to_bytes_be()),
            _ => blake3::hash(
                &self
                    .ids()
                    .into_iter()
                    .flat_map(|id| id.to_bytes_be())
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

pub trait FeltId {
    fn id(&self) -> Felt;
}

pub trait Name {
    fn name(&self) -> &str;
}

pub trait Names {
    fn names(&self) -> Vec<&str>;
}

pub trait TypeDefTrait {
    fn type_def(&self) -> &TypeDef;
}

pub trait TypeDefs {
    fn type_defs(&self) -> Vec<&TypeDef>;
}

impl FeltId for Felt {
    fn id(&self) -> Felt {
        *self
    }
}

impl FeltId for ColumnDef {
    fn id(&self) -> Felt {
        self.id
    }
}

impl FeltId for (Felt, ColumnInfo) {
    fn id(&self) -> Felt {
        self.0
    }
}

impl<C> FeltId for (&Felt, C)
where
    C: Deref<Target = ColumnInfo>,
{
    fn id(&self) -> Felt {
        *self.0
    }
}

impl Name for TableSchema {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Name for ColumnDef {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Name for ColumnInfo {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Name for (Felt, ColumnInfo) {
    fn name(&self) -> &str {
        &self.1.name
    }
}

impl<C> Name for (&Felt, C)
where
    C: Deref<Target = ColumnInfo>,
{
    fn name(&self) -> &str {
        &self.1.name
    }
}

impl Name for PrimaryDef {
    fn name(&self) -> &str {
        &self.name
    }
}

impl TypeDefTrait for ColumnDef {
    fn type_def(&self) -> &TypeDef {
        &self.type_def
    }
}

impl TypeDefTrait for ColumnInfo {
    fn type_def(&self) -> &TypeDef {
        &self.type_def
    }
}

impl TypeDefTrait for (Felt, ColumnInfo) {
    fn type_def(&self) -> &TypeDef {
        &self.1.type_def
    }
}

impl<C> TypeDefTrait for (&Felt, C)
where
    C: Deref<Target = ColumnInfo>,
{
    fn type_def(&self) -> &TypeDef {
        &self.1.type_def
    }
}

/// Blanket-implements a single-item trait for `&T`, `Arc<T>`, and `Rc<T>`.
macro_rules! impl_wrappers {
    ($trait:ident :: $method:ident -> $ret:ty) => {
        impl<T: $trait> $trait for &T {
            fn $method(&self) -> $ret {
                (**self).$method()
            }
        }
        impl<T: $trait> $trait for std::sync::Arc<T> {
            fn $method(&self) -> $ret {
                (**self).$method()
            }
        }
        impl<T: $trait> $trait for std::rc::Rc<T> {
            fn $method(&self) -> $ret {
                (**self).$method()
            }
        }
    };
}

/// Blanket-implements a collection trait for `Vec<R>` and `&[R]`.
macro_rules! impl_collections {
    ($col_trait:ident :: $col_method:ident -> Vec<$ret:ty>, $item_trait:ident :: $item_method:ident) => {
        impl<R: $item_trait> $col_trait for Vec<R> {
            fn $col_method(&self) -> Vec<$ret> {
                self.iter().map(|item| item.$item_method()).collect()
            }
        }
        impl<R: $item_trait> $col_trait for &[R] {
            fn $col_method(&self) -> Vec<$ret> {
                self.iter().map(|item| item.$item_method()).collect()
            }
        }
    };
}

impl_wrappers!(Name::name -> &str);
impl_wrappers!(FeltId::id -> Felt);
impl_wrappers!(TypeDefTrait::type_def -> &TypeDef);
impl_collections!(Names::names -> Vec<&str>, Name::name);
impl_collections!(FeltIds::ids -> Vec<Felt>, FeltId::id);
impl_collections!(TypeDefs::type_defs -> Vec<&TypeDef>, TypeDefTrait::type_def);

#[cfg(test)]
mod tests {
    use super::*;
    use starknet_types_core::felt::Felt;

    fn felt(v: u64) -> Felt {
        Felt::from(v)
    }

    fn col_def(id: u64, name: &str) -> ColumnDef {
        ColumnDef::new(felt(id), name.to_string(), vec![], TypeDef::Felt252)
    }

    fn col_info(name: &str) -> ColumnInfo {
        ColumnInfo::new(name.to_string(), vec![], TypeDef::Felt252)
    }

    // ===== FeltId =====

    #[test]
    fn felt_id_felt() {
        assert_eq!(felt(42).id(), felt(42));
    }

    #[test]
    fn felt_id_felt_ref() {
        let f = felt(42);
        assert_eq!((&f).id(), felt(42));
    }

    #[test]
    fn felt_id_column_def() {
        assert_eq!(FeltId::id(&col_def(7, "x")), felt(7));
    }

    #[test]
    fn felt_id_column_def_ref() {
        let c = col_def(7, "x");
        assert_eq!(FeltId::id(&&c), felt(7));
    }

    #[test]
    fn felt_id_tuple_owned() {
        let t = (felt(99), col_info("z"));
        assert_eq!(t.id(), felt(99));
    }

    #[test]
    fn felt_id_tuple_owned_ref() {
        let t = (felt(99), col_info("z"));
        assert_eq!((&t).id(), felt(99));
    }

    #[test]
    fn felt_id_tuple_ref() {
        let f = felt(55);
        let ci = col_info("r");
        assert_eq!((&f, &ci).id(), felt(55));
    }

    // ===== FeltIds — Vec =====

    #[test]
    fn felt_ids_vec_felt() {
        let v = vec![felt(1), felt(2), felt(3)];
        assert_eq!(v.ids(), vec![felt(1), felt(2), felt(3)]);
    }

    #[test]
    fn felt_ids_vec_column_def() {
        let v = vec![col_def(1, "a"), col_def(2, "b")];
        assert_eq!(v.ids(), vec![felt(1), felt(2)]);
    }

    #[test]
    fn felt_ids_vec_tuple_owned() {
        let v = vec![(felt(3), col_info("a")), (felt(4), col_info("b"))];
        assert_eq!(v.ids(), vec![felt(3), felt(4)]);
    }

    #[test]
    fn felt_ids_vec_tuple_ref() {
        let f1 = felt(1);
        let f2 = felt(2);
        let ci1 = col_info("a");
        let ci2 = col_info("b");
        let v = vec![(&f1, &ci1), (&f2, &ci2)];
        assert_eq!(v.ids(), vec![felt(1), felt(2)]);
    }

    // ===== FeltIds — &[T] slices =====

    #[test]
    fn felt_ids_slice_felt() {
        let v = vec![felt(10), felt(20)];
        let s: &[Felt] = &v;
        assert_eq!(s.ids(), vec![felt(10), felt(20)]);
    }

    #[test]
    fn felt_ids_slice_column_def() {
        let v = vec![col_def(5, "m"), col_def(6, "n")];
        let s: &[ColumnDef] = &v;
        assert_eq!(s.ids(), vec![felt(5), felt(6)]);
    }

    #[test]
    fn felt_ids_slice_tuple_owned() {
        let v = vec![(felt(11), col_info("x")), (felt(12), col_info("y"))];
        let s: &[(Felt, ColumnInfo)] = &v;
        assert_eq!(s.ids(), vec![felt(11), felt(12)]);
    }

    #[test]
    fn felt_ids_slice_tuple_ref() {
        let f1 = felt(8);
        let f2 = felt(9);
        let ci1 = col_info("p");
        let ci2 = col_info("q");
        let v = vec![(&f1, &ci1), (&f2, &ci2)];
        let s: &[(&Felt, &ColumnInfo)] = &v;
        assert_eq!(s.ids(), vec![felt(8), felt(9)]);
    }

    // ===== FeltIds — Vec<&T> =====

    #[test]
    fn felt_ids_vec_ref_felt() {
        let f1 = felt(1);
        let f2 = felt(2);
        let v: Vec<&Felt> = vec![&f1, &f2];
        assert_eq!(v.ids(), vec![felt(1), felt(2)]);
    }

    #[test]
    fn felt_ids_vec_ref_column_def() {
        let c1 = col_def(3, "a");
        let c2 = col_def(4, "b");
        let v: Vec<&ColumnDef> = vec![&c1, &c2];
        assert_eq!(v.ids(), vec![felt(3), felt(4)]);
    }

    #[test]
    fn felt_ids_vec_ref_tuple_owned() {
        let t1 = (felt(5), col_info("a"));
        let t2 = (felt(6), col_info("b"));
        let v: Vec<&(Felt, ColumnInfo)> = vec![&t1, &t2];
        assert_eq!(v.ids(), vec![felt(5), felt(6)]);
    }

    #[test]
    fn felt_ids_vec_ref_tuple_ref() {
        let f1 = felt(7);
        let f2 = felt(8);
        let ci1 = col_info("a");
        let ci2 = col_info("b");
        let t1 = (&f1, &ci1);
        let t2 = (&f2, &ci2);
        let v: Vec<&(&Felt, &ColumnInfo)> = vec![&t1, &t2];
        assert_eq!(v.ids(), vec![felt(7), felt(8)]);
    }

    // ===== FeltIds — &[&T] slices =====

    #[test]
    fn felt_ids_slice_ref_felt() {
        let f1 = felt(1);
        let f2 = felt(2);
        let v: Vec<&Felt> = vec![&f1, &f2];
        let s: &[&Felt] = &v;
        assert_eq!(s.ids(), vec![felt(1), felt(2)]);
    }

    #[test]
    fn felt_ids_slice_ref_column_def() {
        let c1 = col_def(3, "a");
        let c2 = col_def(4, "b");
        let v: Vec<&ColumnDef> = vec![&c1, &c2];
        let s: &[&ColumnDef] = &v;
        assert_eq!(s.ids(), vec![felt(3), felt(4)]);
    }

    #[test]
    fn felt_ids_slice_ref_tuple_owned() {
        let t1 = (felt(5), col_info("a"));
        let t2 = (felt(6), col_info("b"));
        let v: Vec<&(Felt, ColumnInfo)> = vec![&t1, &t2];
        let s: &[&(Felt, ColumnInfo)] = &v;
        assert_eq!(s.ids(), vec![felt(5), felt(6)]);
    }

    #[test]
    fn felt_ids_slice_ref_tuple_ref() {
        let f1 = felt(7);
        let f2 = felt(8);
        let ci1 = col_info("a");
        let ci2 = col_info("b");
        let t1 = (&f1, &ci1);
        let t2 = (&f2, &ci2);
        let v: Vec<&(&Felt, &ColumnInfo)> = vec![&t1, &t2];
        let s: &[&(&Felt, &ColumnInfo)] = &v;
        assert_eq!(s.ids(), vec![felt(7), felt(8)]);
    }

    // ===== FeltIds — &Vec<T> (ref of vec) =====

    #[test]
    fn felt_ids_ref_vec_felt() {
        let v = vec![felt(1), felt(2)];
        let r = &v;
        assert_eq!(r.ids(), vec![felt(1), felt(2)]);
    }

    #[test]
    fn felt_ids_ref_vec_column_def() {
        let v = vec![col_def(1, "a"), col_def(2, "b")];
        let r = &v;
        assert_eq!(r.ids(), vec![felt(1), felt(2)]);
    }

    #[test]
    fn felt_ids_ref_vec_tuple_owned() {
        let v = vec![(felt(3), col_info("a")), (felt(4), col_info("b"))];
        let r = &v;
        assert_eq!(r.ids(), vec![felt(3), felt(4)]);
    }

    #[test]
    fn felt_ids_ref_vec_tuple_ref() {
        let f1 = felt(5);
        let f2 = felt(6);
        let ci1 = col_info("a");
        let ci2 = col_info("b");
        let v = vec![(&f1, &ci1), (&f2, &ci2)];
        let r = &v;
        assert_eq!(r.ids(), vec![felt(5), felt(6)]);
    }

    // ===== Name =====

    #[test]
    fn name_column_def() {
        assert_eq!(Name::name(&col_def(1, "alpha")), "alpha");
    }

    #[test]
    fn name_column_def_ref() {
        let c = col_def(1, "alpha");
        assert_eq!(Name::name(&&c), "alpha");
    }

    #[test]
    fn name_column_info() {
        assert_eq!(col_info("beta").name(), "beta");
    }

    #[test]
    fn name_column_info_ref() {
        let ci = col_info("beta");
        assert_eq!((&ci).name(), "beta");
    }

    #[test]
    fn name_tuple_owned() {
        let t = (felt(1), col_info("gamma"));
        assert_eq!(t.name(), "gamma");
    }

    #[test]
    fn name_tuple_owned_ref() {
        let t = (felt(1), col_info("gamma"));
        assert_eq!((&t).name(), "gamma");
    }

    #[test]
    fn name_tuple_ref() {
        let f = felt(1);
        let ci = col_info("delta");
        assert_eq!((&f, &ci).name(), "delta");
    }

    // ===== Names — Vec =====

    #[test]
    fn names_vec_column_def() {
        let v = vec![col_def(1, "a"), col_def(2, "b"), col_def(3, "c")];
        assert_eq!(v.names(), vec!["a", "b", "c"]);
    }

    #[test]
    fn names_vec_column_info() {
        let v = vec![col_info("x"), col_info("y")];
        assert_eq!(v.names(), vec!["x", "y"]);
    }

    #[test]
    fn names_vec_tuple_owned() {
        let v = vec![(felt(1), col_info("a")), (felt(2), col_info("b"))];
        assert_eq!(v.names(), vec!["a", "b"]);
    }

    #[test]
    fn names_vec_tuple_ref() {
        let f1 = felt(1);
        let f2 = felt(2);
        let ci1 = col_info("a");
        let ci2 = col_info("b");
        let v = vec![(&f1, &ci1), (&f2, &ci2)];
        assert_eq!(v.names(), vec!["a", "b"]);
    }

    // ===== Names — &[T] slices =====

    #[test]
    fn names_slice_column_def() {
        let v = vec![col_def(1, "x"), col_def(2, "y")];
        let s: &[ColumnDef] = &v;
        assert_eq!(s.names(), vec!["x", "y"]);
    }

    #[test]
    fn names_slice_column_info() {
        let v = vec![col_info("m"), col_info("n")];
        let s: &[ColumnInfo] = &v;
        assert_eq!(s.names(), vec!["m", "n"]);
    }

    #[test]
    fn names_slice_tuple_owned() {
        let v = vec![(felt(1), col_info("a")), (felt(2), col_info("b"))];
        let s: &[(Felt, ColumnInfo)] = &v;
        assert_eq!(s.names(), vec!["a", "b"]);
    }

    #[test]
    fn names_slice_tuple_ref() {
        let f1 = felt(1);
        let f2 = felt(2);
        let ci1 = col_info("p");
        let ci2 = col_info("q");
        let v = vec![(&f1, &ci1), (&f2, &ci2)];
        let s: &[(&Felt, &ColumnInfo)] = &v;
        assert_eq!(s.names(), vec!["p", "q"]);
    }

    // ===== Names — Vec<&T> =====

    #[test]
    fn names_vec_ref_column_def() {
        let c1 = col_def(1, "a");
        let c2 = col_def(2, "b");
        let v: Vec<&ColumnDef> = vec![&c1, &c2];
        assert_eq!(v.names(), vec!["a", "b"]);
    }

    #[test]
    fn names_vec_ref_column_info() {
        let ci1 = col_info("x");
        let ci2 = col_info("y");
        let v: Vec<&ColumnInfo> = vec![&ci1, &ci2];
        assert_eq!(v.names(), vec!["x", "y"]);
    }

    #[test]
    fn names_vec_ref_tuple_owned() {
        let t1 = (felt(1), col_info("a"));
        let t2 = (felt(2), col_info("b"));
        let v: Vec<&(Felt, ColumnInfo)> = vec![&t1, &t2];
        assert_eq!(v.names(), vec!["a", "b"]);
    }

    #[test]
    fn names_vec_ref_tuple_ref() {
        let f1 = felt(1);
        let f2 = felt(2);
        let ci1 = col_info("a");
        let ci2 = col_info("b");
        let t1 = (&f1, &ci1);
        let t2 = (&f2, &ci2);
        let v: Vec<&(&Felt, &ColumnInfo)> = vec![&t1, &t2];
        assert_eq!(v.names(), vec!["a", "b"]);
    }

    // ===== Names — &[&T] slices =====

    #[test]
    fn names_slice_ref_column_def() {
        let c1 = col_def(1, "x");
        let c2 = col_def(2, "y");
        let v: Vec<&ColumnDef> = vec![&c1, &c2];
        let s: &[&ColumnDef] = &v;
        assert_eq!(s.names(), vec!["x", "y"]);
    }

    #[test]
    fn names_slice_ref_column_info() {
        let ci1 = col_info("m");
        let ci2 = col_info("n");
        let v: Vec<&ColumnInfo> = vec![&ci1, &ci2];
        let s: &[&ColumnInfo] = &v;
        assert_eq!(s.names(), vec!["m", "n"]);
    }

    #[test]
    fn names_slice_ref_tuple_owned() {
        let t1 = (felt(1), col_info("a"));
        let t2 = (felt(2), col_info("b"));
        let v: Vec<&(Felt, ColumnInfo)> = vec![&t1, &t2];
        let s: &[&(Felt, ColumnInfo)] = &v;
        assert_eq!(s.names(), vec!["a", "b"]);
    }

    #[test]
    fn names_slice_ref_tuple_ref() {
        let f1 = felt(1);
        let f2 = felt(2);
        let ci1 = col_info("p");
        let ci2 = col_info("q");
        let t1 = (&f1, &ci1);
        let t2 = (&f2, &ci2);
        let v: Vec<&(&Felt, &ColumnInfo)> = vec![&t1, &t2];
        let s: &[&(&Felt, &ColumnInfo)] = &v;
        assert_eq!(s.names(), vec!["p", "q"]);
    }

    // ===== Names — &Vec<T> (ref of vec) =====

    #[test]
    fn names_ref_vec_column_def() {
        let v = vec![col_def(1, "a"), col_def(2, "b")];
        let r = &v;
        assert_eq!(r.names(), vec!["a", "b"]);
    }

    #[test]
    fn names_ref_vec_tuple_owned() {
        let v = vec![(felt(1), col_info("a")), (felt(2), col_info("b"))];
        let r = &v;
        assert_eq!(r.names(), vec!["a", "b"]);
    }

    // ===== FeltIds::hash =====

    #[test]
    fn hash_empty_vec() {
        let v: Vec<Felt> = vec![];
        assert_eq!(v.hash(), blake3::Hash::from([0; 32]));
    }

    #[test]
    fn hash_single_felt() {
        let v = vec![felt(42)];
        assert_eq!(v.hash(), blake3::Hash::from(felt(42).to_bytes_be()));
    }

    #[test]
    fn hash_multiple_felts() {
        let v = vec![felt(1), felt(2), felt(3)];
        let expected = blake3::hash(&v.iter().flat_map(|f| f.to_bytes_be()).collect::<Vec<_>>());
        assert_eq!(v.hash(), expected);
    }

    #[test]
    fn hash_column_def_slice() {
        let v = vec![col_def(10, "a"), col_def(20, "b")];
        let s: &[ColumnDef] = &v;
        let expected = blake3::hash(
            &[felt(10), felt(20)]
                .iter()
                .flat_map(|f| f.to_bytes_be())
                .collect::<Vec<_>>(),
        );
        assert_eq!(s.hash(), expected);
    }

    #[test]
    fn hash_tuple_owned_vec() {
        let v = vec![(felt(10), col_info("a")), (felt(20), col_info("b"))];
        let expected = blake3::hash(
            &[felt(10), felt(20)]
                .iter()
                .flat_map(|f| f.to_bytes_be())
                .collect::<Vec<_>>(),
        );
        assert_eq!(v.hash(), expected);
    }

    // ===== From conversions =====

    #[test]
    fn column_def_to_tuple() {
        let c = col_def(5, "test");
        let (id, info): (Felt, ColumnInfo) = c.into();
        assert_eq!(id, felt(5));
        assert_eq!(info.name, "test");
    }

    #[test]
    fn tuple_to_column_def() {
        let t = (felt(7), col_info("back"));
        let c: ColumnDef = t.into();
        assert_eq!(c.id, felt(7));
        assert_eq!(c.name, "back");
    }

    // ===== Arc — FeltId =====

    #[test]
    fn felt_id_arc_felt() {
        let f = std::sync::Arc::new(felt(42));
        assert_eq!(f.id(), felt(42));
    }

    #[test]
    fn felt_id_arc_column_def() {
        let c = std::sync::Arc::new(col_def(7, "x"));
        assert_eq!(FeltId::id(&*c), felt(7));
    }

    #[test]
    fn felt_id_arc_tuple_owned() {
        let t = std::sync::Arc::new((felt(99), col_info("z")));
        assert_eq!(t.id(), felt(99));
    }

    // ===== Arc — FeltIds =====

    #[test]
    fn felt_ids_vec_arc_felt() {
        let v: Vec<std::sync::Arc<Felt>> =
            vec![std::sync::Arc::new(felt(1)), std::sync::Arc::new(felt(2))];
        assert_eq!(v.ids(), vec![felt(1), felt(2)]);
    }

    #[test]
    fn felt_ids_vec_arc_column_def() {
        let v: Vec<std::sync::Arc<ColumnDef>> = vec![
            std::sync::Arc::new(col_def(3, "a")),
            std::sync::Arc::new(col_def(4, "b")),
        ];
        assert_eq!(v.ids(), vec![felt(3), felt(4)]);
    }

    #[test]
    fn felt_ids_vec_arc_tuple_owned() {
        let v: Vec<std::sync::Arc<(Felt, ColumnInfo)>> = vec![
            std::sync::Arc::new((felt(5), col_info("a"))),
            std::sync::Arc::new((felt(6), col_info("b"))),
        ];
        assert_eq!(v.ids(), vec![felt(5), felt(6)]);
    }

    // ===== Arc — Name =====

    #[test]
    fn name_arc_column_def() {
        let c = std::sync::Arc::new(col_def(1, "alpha"));
        assert_eq!(Name::name(&*c), "alpha");
    }

    #[test]
    fn name_arc_column_info() {
        let ci = std::sync::Arc::new(col_info("beta"));
        assert_eq!(ci.name(), "beta");
    }

    #[test]
    fn name_arc_tuple_owned() {
        let t = std::sync::Arc::new((felt(1), col_info("gamma")));
        assert_eq!(t.name(), "gamma");
    }

    // ===== Arc — Names =====

    #[test]
    fn names_vec_arc_column_info() {
        let v: Vec<std::sync::Arc<ColumnInfo>> = vec![
            std::sync::Arc::new(col_info("x")),
            std::sync::Arc::new(col_info("y")),
        ];
        assert_eq!(v.names(), vec!["x", "y"]);
    }

    #[test]
    fn names_vec_arc_tuple_owned() {
        let v: Vec<std::sync::Arc<(Felt, ColumnInfo)>> = vec![
            std::sync::Arc::new((felt(1), col_info("a"))),
            std::sync::Arc::new((felt(2), col_info("b"))),
        ];
        assert_eq!(v.names(), vec!["a", "b"]);
    }

    // ===== Rc — FeltId =====

    #[test]
    fn felt_id_rc_felt() {
        let f = std::rc::Rc::new(felt(42));
        assert_eq!(f.id(), felt(42));
    }

    #[test]
    fn felt_id_rc_column_def() {
        let c = std::rc::Rc::new(col_def(7, "x"));
        assert_eq!(FeltId::id(&*c), felt(7));
    }

    #[test]
    fn felt_id_rc_tuple_owned() {
        let t = std::rc::Rc::new((felt(99), col_info("z")));
        assert_eq!(t.id(), felt(99));
    }

    // ===== Rc — FeltIds =====

    #[test]
    fn felt_ids_vec_rc_felt() {
        let v: Vec<std::rc::Rc<Felt>> = vec![std::rc::Rc::new(felt(1)), std::rc::Rc::new(felt(2))];
        assert_eq!(v.ids(), vec![felt(1), felt(2)]);
    }

    #[test]
    fn felt_ids_vec_rc_column_def() {
        let v: Vec<std::rc::Rc<ColumnDef>> = vec![
            std::rc::Rc::new(col_def(3, "a")),
            std::rc::Rc::new(col_def(4, "b")),
        ];
        assert_eq!(v.ids(), vec![felt(3), felt(4)]);
    }

    #[test]
    fn felt_ids_vec_rc_tuple_owned() {
        let v: Vec<std::rc::Rc<(Felt, ColumnInfo)>> = vec![
            std::rc::Rc::new((felt(5), col_info("a"))),
            std::rc::Rc::new((felt(6), col_info("b"))),
        ];
        assert_eq!(v.ids(), vec![felt(5), felt(6)]);
    }

    // ===== Rc — Name =====

    #[test]
    fn name_rc_column_def() {
        let c = std::rc::Rc::new(col_def(1, "alpha"));
        assert_eq!(Name::name(&*c), "alpha");
    }

    #[test]
    fn name_rc_column_info() {
        let ci = std::rc::Rc::new(col_info("beta"));
        assert_eq!(ci.name(), "beta");
    }

    #[test]
    fn name_rc_tuple_owned() {
        let t = std::rc::Rc::new((felt(1), col_info("gamma")));
        assert_eq!(t.name(), "gamma");
    }

    // ===== Rc — Names =====

    #[test]
    fn names_vec_rc_column_info() {
        let v: Vec<std::rc::Rc<ColumnInfo>> = vec![
            std::rc::Rc::new(col_info("x")),
            std::rc::Rc::new(col_info("y")),
        ];
        assert_eq!(v.names(), vec!["x", "y"]);
    }

    #[test]
    fn names_vec_rc_tuple_owned() {
        let v: Vec<std::rc::Rc<(Felt, ColumnInfo)>> = vec![
            std::rc::Rc::new((felt(1), col_info("a"))),
            std::rc::Rc::new((felt(2), col_info("b"))),
        ];
        assert_eq!(v.names(), vec!["a", "b"]);
    }
}
