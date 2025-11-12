use introspect_types::{
    EnumDef, FieldDef, FixedArrayDef, MemberDef, ResultDef, StructDef, TypeDef, VariantDef,
};

pub trait TypeLibrary {
    fn get_ref_type(&self, id: String) -> TypeDef;
    fn set_ref_type(&mut self, id: String, type_def: TypeDef);
    fn get_ref_expanded(&self, id: String) -> TypeDef {
        self.expand_type(self.get_ref_type(id))
    }
    fn expand_type(&self, type_def: TypeDef) -> TypeDef {
        match type_def {
            TypeDef::Tuple(inner) => {
                TypeDef::Tuple(inner.into_iter().map(|e| self.expand_type(e)).collect())
            }
            TypeDef::Array(inner) => TypeDef::Array(self.expand_boxed_type(inner)),
            TypeDef::FixedArray(inner) => TypeDef::FixedArray(self.expand_fixed_array(inner)),
            TypeDef::Felt252Dict(inner) => TypeDef::Felt252Dict(self.expand_boxed_type(inner)),
            TypeDef::Struct(s) => TypeDef::Struct(self.expand_struct(s)),
            TypeDef::Enum(inner) => TypeDef::Enum(self.expand_enum(inner)),
            TypeDef::Ref(inner) => self.get_ref_type(inner),
            TypeDef::Option(inner) => TypeDef::Option(self.expand_boxed_type(inner)),
            TypeDef::Result(inner) => TypeDef::Result(self.expand_result(inner)),
            TypeDef::Nullable(inner) => TypeDef::Nullable(self.expand_boxed_type(inner)),
            _ => type_def,
        }
    }

    fn expand_type_in_place(&self, type_def: &mut TypeDef) {
        match type_def {
            TypeDef::Tuple(inner) => {
                inner.iter_mut().for_each(|e| self.expand_type_in_place(e));
            }
            TypeDef::Array(inner) => self.expand_boxed_type_in_place(inner),
            TypeDef::FixedArray(inner) => {
                self.expand_fixed_array_in_place(inner);
            }
            TypeDef::Felt252Dict(inner) => self.expand_boxed_type_in_place(inner),
            TypeDef::Struct(inner) => self.expand_struct_in_place(inner),
            TypeDef::Enum(inner) => self.expand_enum_in_place(inner),
            TypeDef::Ref(inner) => *type_def = self.get_ref_expanded(inner.clone()),
            TypeDef::Option(inner) => self.expand_boxed_type_in_place(inner),
            TypeDef::Result(inner) => self.expand_result_in_place(inner),
            TypeDef::Nullable(inner) => self.expand_boxed_type_in_place(inner),
            _ => {}
        }
    }

    fn expand_boxed_type(&self, type_def: Box<TypeDef>) -> Box<TypeDef> {
        Box::new(self.expand_type(*type_def))
    }
    fn expand_boxed_type_in_place(&self, type_def: &mut Box<TypeDef>) {
        self.expand_type_in_place(&mut *type_def);
    }
    fn expand_fixed_array(&self, fa: FixedArrayDef) -> FixedArrayDef {
        FixedArrayDef {
            type_def: self.expand_boxed_type(fa.type_def),
            size: fa.size,
        }
    }
    fn expand_fixed_array_in_place(&self, fa: &mut FixedArrayDef) {
        self.expand_boxed_type_in_place(&mut fa.type_def);
    }
    fn expand_struct(&self, s: StructDef) -> StructDef {
        StructDef {
            name: s.name,
            attributes: s.attributes,
            variants: s
                .variants
                .into_iter()
                .map(|member| self.expand_member(member))
                .collect(),
        }
    }
    fn expand_struct_in_place(&self, s: &mut StructDef) {
        s.variants
            .iter_mut()
            .for_each(|member| self.expand_member_in_place(member));
    }
    fn expand_enum(&self, e: EnumDef) -> EnumDef {
        EnumDef {
            name: e.name,
            attributes: e.attributes,
            variants: e
                .variants
                .into_iter()
                .map(|(id, field)| (id.clone(), self.expand_variant(field)))
                .collect(),
        }
    }
    fn expand_enum_in_place(&self, e: &mut EnumDef) {
        e.variants
            .iter_mut()
            .for_each(|(_, field)| self.expand_variant_in_place(field));
    }

    fn expand_variant(&self, variant: VariantDef) -> VariantDef {
        VariantDef {
            name: variant.name,
            attributes: variant.attributes,
            type_def: self.expand_type(variant.type_def),
        }
    }

    fn expand_variant_in_place(&self, variant: &mut VariantDef) {
        self.expand_type_in_place(&mut variant.type_def);
    }

    fn expand_field(&self, field: FieldDef) -> FieldDef {
        FieldDef {
            selector: field.selector,
            name: field.name,
            attributes: field.attributes,
            type_def: self.expand_type(field.type_def),
        }
    }
    fn expand_field_in_place(&self, field: &mut FieldDef) {
        self.expand_type_in_place(&mut field.type_def);
    }
    fn expand_member(&self, member: MemberDef) -> MemberDef {
        MemberDef {
            name: member.name,
            attributes: member.attributes,
            type_def: self.expand_type(member.type_def),
        }
    }
    fn expand_member_in_place(&self, member: &mut MemberDef) {
        self.expand_type_in_place(&mut member.type_def);
    }
    fn expand_result(&self, result: ResultDef) -> ResultDef {
        ResultDef {
            ok: self.expand_boxed_type(result.ok),
            err: self.expand_boxed_type(result.err),
        }
    }
    fn expand_result_in_place(&self, result: &mut ResultDef) {
        self.expand_boxed_type_in_place(&mut result.ok);
        self.expand_boxed_type_in_place(&mut result.err);
    }
}
