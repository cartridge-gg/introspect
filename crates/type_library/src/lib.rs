use introspect_types::ty::{Enum, Field, FixedArray, Member, Struct, Ty};

pub trait TypeLibrary {
    fn get_ref_type(&self, id: String) -> Ty;
    fn set_ref_type(&mut self, id: String, ty: Ty);
    fn get_ref_expanded(&self, id: String) -> Ty {
        self.expand_type(self.get_ref_type(id))
    }
    fn expand_type(&self, ty: Ty) -> Ty {
        match ty {
            Ty::Tuple(inner) => Ty::Tuple(inner.into_iter().map(|e| self.expand_type(e)).collect()),
            Ty::Array(inner) => Ty::Array(self.expand_boxed_type(inner)),
            Ty::FixedArray(inner) => Ty::FixedArray(self.expand_fixed_array(inner)),
            Ty::Felt252Dict(inner) => Ty::Felt252Dict(self.expand_boxed_type(inner)),
            Ty::Struct(s) => Ty::Struct(self.expand_struct(s)),
            Ty::Enum(inner) => Ty::Enum(self.expand_enum(inner)),
            Ty::Ref(inner) => self.get_ref_type(inner),
            Ty::Option(inner) => Ty::Option(self.expand_boxed_type(inner)),
            Ty::Result(inner) => Ty::Result(self.expand_result(inner)),
            Ty::Nullable(inner) => Ty::Nullable(self.expand_boxed_type(inner)),
            _ => ty,
        }
    }

    fn expand_type_in_place(&self, ty: &mut Ty) {
        match ty {
            Ty::Tuple(inner) => {
                inner.iter_mut().for_each(|e| self.expand_type_in_place(e));
            }
            Ty::Array(inner) => self.expand_boxed_type_in_place(inner),
            Ty::FixedArray(inner) => {
                self.expand_fixed_array_in_place(inner);
            }
            Ty::Felt252Dict(inner) => self.expand_boxed_type_in_place(inner),
            Ty::Struct(inner) => self.expand_struct_in_place(inner),
            Ty::Enum(inner) => self.expand_enum_in_place(inner),
            Ty::Ref(inner) => *ty = self.get_ref_expanded(inner.clone()),
            Ty::Option(inner) => self.expand_boxed_type_in_place(inner),
            Ty::Result(inner) => self.expand_result_in_place(inner),
            Ty::Nullable(inner) => self.expand_boxed_type_in_place(inner),
            _ => {}
        }
    }

    fn expand_boxed_type(&self, ty: Box<Ty>) -> Box<Ty> {
        Box::new(self.expand_type(*ty))
    }
    fn expand_boxed_type_in_place(&self, ty: &mut Box<Ty>) {
        self.expand_type_in_place(&mut *ty);
    }
    fn expand_fixed_array(&self, fa: FixedArray) -> FixedArray {
        FixedArray {
            ty: self.expand_boxed_type(fa.ty),
            size: fa.size,
        }
    }
    fn expand_fixed_array_in_place(&self, fa: &mut FixedArray) {
        self.expand_boxed_type_in_place(&mut fa.ty);
    }
    fn expand_struct(&self, s: Struct) -> Struct {
        Struct {
            name: s.name,
            attrs: s.attrs,
            children: s
                .children
                .into_iter()
                .map(|member| self.expand_member(member))
                .collect(),
        }
    }
    fn expand_struct_in_place(&self, s: &mut Struct) {
        s.children
            .iter_mut()
            .for_each(|member| self.expand_member_in_place(member));
    }
    fn expand_enum(&self, e: Enum) -> Enum {
        Enum {
            name: e.name,
            attrs: e.attrs,
            children: e
                .children
                .into_iter()
                .map(|field| self.expand_field(field))
                .collect(),
        }
    }
    fn expand_enum_in_place(&self, e: &mut Enum) {
        e.children
            .iter_mut()
            .for_each(|field| self.expand_field_in_place(field));
    }

    fn expand_field(&self, field: Field) -> Field {
        Field {
            selector: field.selector,
            name: field.name,
            attrs: field.attrs,
            ty: self.expand_type(field.ty),
        }
    }
    fn expand_field_in_place(&self, field: &mut Field) {
        self.expand_type_in_place(&mut field.ty);
    }
    fn expand_member(&self, member: Member) -> Member {
        Member {
            name: member.name,
            attrs: member.attrs,
            ty: self.expand_type(member.ty),
        }
    }
    fn expand_member_in_place(&self, member: &mut Member) {
        self.expand_type_in_place(&mut member.ty);
    }
    fn expand_result(
        &self,
        result: introspect_types::ty::CairoResult,
    ) -> introspect_types::ty::CairoResult {
        introspect_types::ty::CairoResult {
            ok: self.expand_boxed_type(result.ok),
            err: self.expand_boxed_type(result.err),
        }
    }
    fn expand_result_in_place(&self, result: &mut introspect_types::ty::CairoResult) {
        self.expand_boxed_type_in_place(&mut result.ok);
        self.expand_boxed_type_in_place(&mut result.err);
    }
}
