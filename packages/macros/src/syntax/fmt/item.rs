use super::{CairoCollectionFormat, CairoFormat, CodeBuffer};
use crate::syntax::module::{
    Constant, Enum, ExternFunction, ExternType, FunctionDeclaration, FunctionSignature,
    FunctionWithBody, Impl, ImplAlias, ImplItem, InlineMacroItem, Item, Member, Module, Struct,
    Trait, TraitConstant, TraitFunction, TraitImpl, TraitItem, TraitType, TypeAlias, UseItem,
    UsePath, UsePathLeaf, UsePathSingle, Variant,
};

impl<T: CodeBuffer> CairoFormat<T> for Item {
    fn cfmt(&self, buf: &mut T) {
        match self {
            Item::Constant(e) => e.cfmt(buf),
            Item::Module(e) => e.cfmt(buf),
            Item::Use(e) => e.cfmt(buf),
            Item::FreeFunction(e) => e.cfmt(buf),
            Item::ExternFunction(e) => e.cfmt(buf),
            Item::ExternType(e) => e.cfmt(buf),
            Item::Trait(e) => e.cfmt(buf),
            Item::Impl(e) => e.cfmt(buf),
            Item::ImplAlias(e) => e.cfmt(buf),
            Item::Struct(e) => e.cfmt(buf),
            Item::Enum(e) => e.cfmt(buf),
            Item::TypeAlias(e) => e.cfmt(buf),
            Item::InlineMacro(e) => e.cfmt(buf),
            Item::MacroDeclaration | Item::HeaderDoc(_) | Item::Missing => {}
        }
    }
}

impl<T: CodeBuffer> CairoFormat<T> for Struct {
    fn cfmt(&self, buf: &mut T) {
        self.attributes.cfmt(buf);
        self.visibility.cfmt(buf);
        self.name.cfmt_prefixed_str(buf, "struct ");
        self.generic_params.cfmt(buf);
        self.members.cfmt_fields_braced(buf);
    }
}

impl<T: CodeBuffer> CairoFormat<T> for Member {
    fn cfmt(&self, buf: &mut T) {
        self.attributes.cfmt(buf);
        self.visibility.cfmt(buf);
        self.name.cfmt(buf);
        self.ty.cfmt_prefixed_str(buf, ": ");
    }
}

impl<T: CodeBuffer> CairoFormat<T> for Enum {
    fn cfmt(&self, buf: &mut T) {
        self.attributes.cfmt(buf);
        self.visibility.cfmt(buf);
        self.name.cfmt_prefixed_str(buf, "enum ");
        self.generic_params.cfmt(buf);
        self.variants.cfmt_fields_braced(buf);
    }
}

impl<T: CodeBuffer> CairoFormat<T> for Variant {
    fn cfmt(&self, buf: &mut T) {
        self.attributes.cfmt(buf);
        self.name.cfmt(buf);
        if let Some(ty) = &self.type_clause {
            ty.cfmt_prefixed_str(buf, ": ");
        }
    }
}

impl<T: CodeBuffer> CairoFormat<T> for Constant {
    fn cfmt(&self, buf: &mut T) {
        self.attributes.cfmt(buf);
        self.visibility.cfmt(buf);
        self.name.cfmt_prefixed_str(buf, "const ");
        self.ty.cfmt_prefixed_str(buf, ": ");
        self.value.cfmt_prefixed_str(buf, " = ");
        buf.push_token_char(';');
    }
}
impl<T: CodeBuffer> CairoFormat<T> for Module {
    fn cfmt(&self, buf: &mut T) {
        self.attributes.cfmt(buf);
        self.visibility.cfmt(buf);
        self.name.cfmt_prefixed_str(buf, "mod ");
        match &self.body {
            Some(items) => items.cfmt_block_braced(buf),
            None => buf.push_token_char(';'),
        }
    }
}

impl<T: CodeBuffer> CairoFormat<T> for UseItem {
    fn cfmt(&self, buf: &mut T) {
        self.attributes.cfmt(buf);
        self.visibility.cfmt(buf);
        buf.push_token_str("use ");
        if self.dollar {
            buf.push_token_char('$');
        }
        self.path.cfmt_suffixed(buf, ';');
    }
}

impl<T: CodeBuffer> CairoFormat<T> for UsePath {
    fn cfmt(&self, buf: &mut T) {
        match self {
            UsePath::Leaf(leaf) => leaf.cfmt(buf),
            UsePath::Single(single) => single.cfmt(buf),
            UsePath::Multi(multi) => multi.cfmt_csv_braced(buf),
            UsePath::Star => buf.push_token_char('*'),
        }
    }
}

impl<T: CodeBuffer> CairoFormat<T> for UsePathLeaf {
    fn cfmt(&self, buf: &mut T) {
        self.ident.cfmt(buf);
        if let Some(alias) = &self.alias {
            alias.cfmt_prefixed_str(buf, " as ");
        }
    }
}

impl<T: CodeBuffer> CairoFormat<T> for UsePathSingle {
    fn cfmt(&self, buf: &mut T) {
        self.ident.cfmt_suffixed_str(buf, "::");
        self.path.cfmt(buf);
    }
}

impl<T: CodeBuffer> CairoFormat<T> for FunctionWithBody {
    fn cfmt(&self, buf: &mut T) {
        self.attributes.cfmt(buf);
        self.visibility.cfmt(buf);
        self.declaration.cfmt(buf);
        self.body.cfmt_prefixed(buf, ' ');
    }
}

impl<T: CodeBuffer> CairoFormat<T> for FunctionDeclaration {
    fn cfmt(&self, buf: &mut T) {
        if self.is_const {
            buf.push_token_str("const ");
        }
        self.name.cfmt_prefixed_str(buf, "fn ");
        self.generic_params.cfmt(buf);
        self.signature.cfmt(buf);
    }
}

impl<T: CodeBuffer> CairoFormat<T> for FunctionSignature {
    fn cfmt(&self, buf: &mut T) {
        self.parameters.cfmt_csv_parenthesized(buf);
        if let Some(ret_type) = &self.return_type {
            ret_type.cfmt_prefixed_str(buf, " -> ");
        }
        if let Some(implicits) = &self.implicits_clause {
            buf.push_token_str(" implicits");
            implicits.cfmt_csv_parenthesized(buf);
        }
        if self.no_panic {
            buf.push_token_str(" nopanic");
        }
    }
}

impl<T: CodeBuffer> CairoFormat<T> for ExternFunction {
    fn cfmt(&self, buf: &mut T) {
        self.attributes.cfmt(buf);
        self.visibility.cfmt(buf);
        buf.push_token_str("extern ");
        self.declaration.cfmt_suffixed(buf, ';');
    }
}

impl<T: CodeBuffer> CairoFormat<T> for ExternType {
    fn cfmt(&self, buf: &mut T) {
        self.attributes.cfmt(buf);
        self.visibility.cfmt(buf);
        buf.push_token_str("extern ");
        self.name.cfmt_prefixed_str(buf, "type ");
        self.generic_params.cfmt_suffixed(buf, ';');
    }
}

impl<T: CodeBuffer> CairoFormat<T> for Trait {
    fn cfmt(&self, buf: &mut T) {
        self.attributes.cfmt(buf);
        self.visibility.cfmt(buf);
        self.name.cfmt_prefixed_str(buf, "trait ");
        self.generic_params.cfmt(buf);
        match &self.body {
            Some(items) => items.cfmt_block_braced(buf),
            None => buf.push_token_char(';'),
        }
    }
}

impl<T: CodeBuffer> CairoFormat<T> for TraitItem {
    fn cfmt(&self, buf: &mut T) {
        match self {
            TraitItem::Function(e) => e.cfmt(buf),
            TraitItem::Type(e) => e.cfmt(buf),
            TraitItem::Constant(e) => e.cfmt(buf),
            TraitItem::Impl(e) => e.cfmt(buf),
            TraitItem::Missing => {}
        }
    }
}

impl<T: CodeBuffer> CairoFormat<T> for TraitFunction {
    fn cfmt(&self, buf: &mut T) {
        self.attributes.cfmt(buf);
        self.declaration.cfmt(buf);
        match &self.body {
            Some(body) => body.cfmt_prefixed(buf, ' '),
            None => buf.push_token_char(';'),
        }
    }
}

impl<T: CodeBuffer> CairoFormat<T> for TraitType {
    fn cfmt(&self, buf: &mut T) {
        self.attributes.cfmt(buf);
        self.name.cfmt_prefixed_str(buf, "type ");
        self.generic_params.cfmt_suffixed(buf, ';');
    }
}

impl<T: CodeBuffer> CairoFormat<T> for TraitConstant {
    fn cfmt(&self, buf: &mut T) {
        self.attributes.cfmt(buf);
        self.name.cfmt_prefixed_str(buf, "const ");
        self.ty.cfmt_prefixed_str(buf, ": ");
        buf.push_token_char(';');
    }
}

impl<T: CodeBuffer> CairoFormat<T> for TraitImpl {
    fn cfmt(&self, buf: &mut T) {
        self.attributes.cfmt(buf);
        self.name.cfmt_prefixed_str(buf, "impl ");
        self.trait_path.cfmt_suffixed(buf, ';');
    }
}

impl<T: CodeBuffer> CairoFormat<T> for Impl {
    fn cfmt(&self, buf: &mut T) {
        self.attributes.cfmt(buf);
        self.visibility.cfmt(buf);
        self.name.cfmt_prefixed_str(buf, "impl ");
        self.generic_params.cfmt(buf);
        self.trait_path.cfmt_prefixed_str(buf, " of ");
        match &self.body {
            Some(items) => items.cfmt_block_braced(buf),
            None => buf.push_token_char(';'),
        }
    }
}

impl<T: CodeBuffer> CairoFormat<T> for ImplItem {
    fn cfmt(&self, buf: &mut T) {
        match self {
            ImplItem::Function(e) => e.cfmt(buf),
            ImplItem::Type(e) => e.cfmt(buf),
            ImplItem::Constant(e) => e.cfmt(buf),
            ImplItem::Impl(e) => e.cfmt(buf),
            ImplItem::Module(e) => e.cfmt(buf),
            ImplItem::Use(e) => e.cfmt(buf),
            ImplItem::ExternFunction(e) => e.cfmt(buf),
            ImplItem::ExternType(e) => e.cfmt(buf),
            ImplItem::Trait(e) => e.cfmt(buf),
            ImplItem::Struct(e) => e.cfmt(buf),
            ImplItem::Enum(e) => e.cfmt(buf),
            ImplItem::Missing => {}
        }
    }
}

impl<T: CodeBuffer> CairoFormat<T> for TypeAlias {
    fn cfmt(&self, buf: &mut T) {
        self.attributes.cfmt(buf);
        self.visibility.cfmt(buf);
        self.name.cfmt_prefixed_str(buf, "type ");
        self.generic_params.cfmt_suffixed_str(buf, " = ");
        self.ty.cfmt_suffixed(buf, ';');
    }
}

impl<T: CodeBuffer> CairoFormat<T> for ImplAlias {
    fn cfmt(&self, buf: &mut T) {
        self.attributes.cfmt(buf);
        self.visibility.cfmt(buf);
        self.name.cfmt_prefixed_str(buf, "impl ");
        self.generic_params.cfmt_suffixed_str(buf, " = ");
        self.path.cfmt_suffixed(buf, ';');
    }
}

impl<T: CodeBuffer> CairoFormat<T> for InlineMacroItem {
    fn cfmt(&self, buf: &mut T) {
        self.attributes.cfmt(buf);
        self.path.cfmt_suffixed(buf, '!');
        self.arguments.cfmt(buf);
    }
}
