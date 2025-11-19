use crate::type_def::SingletonTypeDefTrait;
use crate::{
    EnumDef, FixedArrayDef, ItemDefTrait, MemberDef, StructDef, TupleDef, TypeDef, VariantDef,
};
use starknet_types_core::felt::Felt;
use std::collections::HashMap;
use std::hash::Hash;

pub trait GetRefTypeDef {
    fn get_type_def(&self, id: Felt) -> Option<TypeDef>;
}

pub trait DerefDefTrait<TD> {
    fn deref_def(&self, def: TD) -> Option<TD>;
}

impl<TD, T: DerefDefTrait<TD>> DerefDefTrait<Vec<TD>> for T
where
    T: GetRefTypeDef,
{
    fn deref_def(&self, defs: Vec<TD>) -> Option<Vec<TD>> {
        defs.into_iter()
            .map(|def| self.deref_def(def))
            .collect::<Option<Vec<TD>>>()
    }
}

impl<TD, T: DerefDefTrait<TD>, K: Eq + Hash> DerefDefTrait<HashMap<K, TD>> for T
where
    T: GetRefTypeDef,
{
    fn deref_def(&self, defs: HashMap<K, TD>) -> Option<HashMap<K, TD>> {
        defs.into_iter()
            .map(|(k, def)| self.deref_def(def).map(|d| (k, d)))
            .collect::<Option<HashMap<K, TD>>>()
    }
}

trait DeRefItemTrait<TD> {
    fn deref_item(&self, item: TD) -> Option<TypeDef>;
}

impl<T: GetRefTypeDef + DerefDefTrait<TD>, TD: ItemDefTrait> DeRefItemTrait<TD> for T {
    fn deref_item(&self, type_def: TD) -> Option<TypeDef> {
        self.deref_def(type_def).map(TD::wrap_to_type_def)
    }
}

impl<T: GetRefTypeDef> DerefDefTrait<TypeDef> for T {
    fn deref_def(&self, type_def: TypeDef) -> Option<TypeDef> {
        match type_def {
            TypeDef::Ref(def) => self.get_type_def(def.id),
            TypeDef::Tuple(def) => self.deref_item(def),
            TypeDef::Array(def) => self.deref_item(*def),
            TypeDef::FixedArray(def) => self.deref_item(*def),
            TypeDef::Felt252Dict(def) => self.deref_item(*def),
            TypeDef::Struct(def) => self.deref_item(def),
            TypeDef::Enum(def) => self.deref_item(def),
            _ => Some(type_def),
        }
    }
}

impl<T: GetRefTypeDef> DerefDefTrait<TupleDef> for T {
    fn deref_def(&self, type_def: TupleDef) -> Option<TupleDef> {
        self.deref_def(type_def.elements).map(TupleDef::new)
    }
}

impl<T: GetRefTypeDef, TD: SingletonTypeDefTrait> DerefDefTrait<TD> for T {
    fn deref_def(&self, def: TD) -> Option<TD> {
        self.deref_def(def.inner_type_def())
            .map(TD::from_inner_type_def)
    }
}

impl<T: GetRefTypeDef> DerefDefTrait<StructDef> for T {
    fn deref_def(&self, def: StructDef) -> Option<StructDef> {
        Some(StructDef {
            name: def.name,
            attributes: def.attributes,
            members: self.deref_def(def.members)?,
        })
    }
}

impl<T: GetRefTypeDef> DerefDefTrait<MemberDef> for T {
    fn deref_def(&self, def: MemberDef) -> Option<MemberDef> {
        Some(MemberDef {
            name: def.name,
            attributes: def.attributes,
            type_def: self.deref_def(def.type_def)?,
        })
    }
}

impl<T: GetRefTypeDef> DerefDefTrait<EnumDef> for T {
    fn deref_def(&self, def: EnumDef) -> Option<EnumDef> {
        Some(EnumDef {
            name: def.name,
            attributes: def.attributes,
            variants: self.deref_def(def.variants)?,
            order: def.order,
        })
    }
}

impl<T: GetRefTypeDef> DerefDefTrait<VariantDef> for T {
    fn deref_def(&self, def: VariantDef) -> Option<VariantDef> {
        Some(VariantDef {
            name: def.name,
            attributes: def.attributes,
            type_def: self.deref_def(def.type_def)?,
        })
    }
}

impl<T: GetRefTypeDef> DerefDefTrait<FixedArrayDef> for T {
    fn deref_def(&self, def: FixedArrayDef) -> Option<FixedArrayDef> {
        Some(FixedArrayDef {
            type_def: self.deref_def(def.type_def)?,
            size: def.size,
        })
    }
}
