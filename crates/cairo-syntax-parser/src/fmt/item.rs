use super::{CairoWrite, CairoWriteSlice};
use crate::item::{
    Constant, Enum, ExternFunction, ExternType, FunctionDeclaration, FunctionSignature,
    FunctionWithBody, Impl, ImplAlias, ImplItem, InlineMacroItem, Item, Member, Module, Struct,
    Trait, TraitConstant, TraitFunction, TraitImpl, TraitItem, TraitType, TypeAlias, UseItem,
    UsePath, UsePathLeaf, UsePathSingle, Variant,
};
use std::fmt::{Result, Write};

impl CairoWrite for Item {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            Item::Constant(e) => e.cwrite(buf),
            Item::Module(e) => e.cwrite(buf),
            Item::Use(e) => e.cwrite(buf),
            Item::FreeFunction(e) => e.cwrite(buf),
            Item::ExternFunction(e) => e.cwrite(buf),
            Item::ExternType(e) => e.cwrite(buf),
            Item::Trait(e) => e.cwrite(buf),
            Item::Impl(e) => e.cwrite(buf),
            Item::ImplAlias(e) => e.cwrite(buf),
            Item::Struct(e) => e.cwrite(buf),
            Item::Enum(e) => e.cwrite(buf),
            Item::TypeAlias(e) => e.cwrite(buf),
            Item::InlineMacro(e) => e.cwrite(buf),
            Item::MacroDeclaration | Item::HeaderDoc(_) | Item::Missing => Ok(()),
        }
    }
}

impl CairoWrite for Vec<Item> {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.cwrite_terminated(buf, '\n')
    }
}

impl CairoWrite for Struct {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cwrite(buf)?;
        self.visibility.cwrite(buf)?;
        self.name.cwrite_prefixed_str(buf, "struct ")?;
        self.generic_params.cwrite(buf)?;
        self.members.cwrite_fields_braced(buf)
    }
}

impl CairoWrite for Member {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cwrite(buf)?;
        self.visibility.cwrite(buf)?;
        self.name.cwrite(buf)?;
        self.ty.cwrite_prefixed_str(buf, ": ")
    }
}

impl CairoWrite for Enum {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cwrite(buf)?;
        self.visibility.cwrite(buf)?;
        self.name.cwrite_prefixed_str(buf, "enum ")?;
        self.generic_params.cwrite(buf)?;
        self.variants.cwrite_fields_braced(buf)
    }
}

impl CairoWrite for Variant {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cwrite(buf)?;
        self.name.cwrite(buf)?;
        if let Some(ty) = &self.type_clause {
            ty.cwrite_prefixed_str(buf, ": ")?;
        }
        Ok(())
    }
}

impl CairoWrite for Constant {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cwrite(buf)?;
        self.visibility.cwrite(buf)?;
        self.name.cwrite_prefixed_str(buf, "const ")?;
        self.ty.cwrite_prefixed_str(buf, ": ")?;
        self.value.cwrite_prefixed_str(buf, " = ")?;
        buf.write_char(';')
    }
}

impl CairoWrite for Module {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cwrite(buf)?;
        self.visibility.cwrite(buf)?;
        self.name.cwrite_prefixed_str(buf, "mod ")?;
        match &self.body {
            Some(items) => items.cwrite_block_braced(buf),
            None => buf.write_char(';'),
        }
    }
}

impl CairoWrite for UseItem {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cwrite(buf)?;
        self.visibility.cwrite(buf)?;
        buf.write_str("use ")?;
        if self.dollar {
            buf.write_char('$')?;
        }
        self.path.cwrite_suffixed(buf, ';')
    }
}

impl CairoWrite for UsePath {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            UsePath::Leaf(leaf) => leaf.cwrite(buf),
            UsePath::Single(single) => single.cwrite(buf),
            UsePath::Multi(multi) => multi.cwrite_csv_braced(buf),
            UsePath::Star => buf.write_char('*'),
        }
    }
}

impl CairoWrite for UsePathLeaf {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.ident.cwrite(buf)?;
        if let Some(alias) = &self.alias {
            alias.cwrite_prefixed_str(buf, " as ")?;
        }
        Ok(())
    }
}

impl CairoWrite for UsePathSingle {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.ident.cwrite_suffixed_str(buf, "::")?;
        self.path.cwrite(buf)
    }
}

impl CairoWrite for FunctionWithBody {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cwrite(buf)?;
        self.visibility.cwrite(buf)?;
        self.declaration.cwrite(buf)?;
        self.body.cwrite_prefixed(buf, ' ')
    }
}

impl CairoWrite for FunctionDeclaration {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        if self.is_const {
            buf.write_str("const ")?;
        }
        self.name.cwrite_prefixed_str(buf, "fn ")?;
        self.generic_params.cwrite(buf)?;
        self.signature.cwrite(buf)
    }
}

impl CairoWrite for FunctionSignature {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.parameters.cwrite_csv_parenthesized(buf)?;
        if let Some(ret_type) = &self.return_type {
            ret_type.cwrite_prefixed_str(buf, " -> ")?;
        }
        if let Some(implicits) = &self.implicits_clause {
            buf.write_str(" implicits")?;
            implicits.cwrite_csv_parenthesized(buf)?;
        }
        if self.no_panic {
            buf.write_str(" nopanic")?;
        }
        Ok(())
    }
}

impl CairoWrite for ExternFunction {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cwrite(buf)?;
        self.visibility.cwrite(buf)?;
        buf.write_str("extern ")?;
        self.declaration.cwrite_suffixed(buf, ';')
    }
}

impl CairoWrite for ExternType {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cwrite(buf)?;
        self.visibility.cwrite(buf)?;
        self.name.cwrite_prefixed_str(buf, "extern type ")?;
        self.generic_params.cwrite_suffixed(buf, ';')
    }
}

impl CairoWrite for Trait {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cwrite(buf)?;
        self.visibility.cwrite(buf)?;
        self.name.cwrite_prefixed_str(buf, "trait ")?;
        self.generic_params.cwrite(buf)?;
        match &self.body {
            Some(items) => items.cwrite_block_braced(buf),
            None => buf.write_char(';'),
        }
    }
}

impl CairoWrite for TraitItem {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            TraitItem::Function(e) => e.cwrite(buf),
            TraitItem::Type(e) => e.cwrite(buf),
            TraitItem::Constant(e) => e.cwrite(buf),
            TraitItem::Impl(e) => e.cwrite(buf),
            TraitItem::Missing => Ok(()),
        }
    }
}

impl CairoWrite for TraitFunction {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cwrite(buf)?;
        self.declaration.cwrite(buf)?;
        match &self.body {
            Some(body) => body.cwrite_prefixed(buf, ' '),
            None => buf.write_char(';'),
        }
    }
}

impl CairoWrite for TraitType {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cwrite(buf)?;
        self.name.cwrite_prefixed_str(buf, "type ")?;
        self.generic_params.cwrite_suffixed(buf, ';')
    }
}

impl CairoWrite for TraitConstant {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cwrite(buf)?;
        self.name.cwrite_prefixed_str(buf, "const ")?;
        self.ty.cwrite_prefixed_str(buf, ": ")?;
        buf.write_char(';')
    }
}

impl CairoWrite for TraitImpl {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cwrite(buf)?;
        self.name.cwrite_prefixed_str(buf, "impl ")?;
        self.trait_path.cwrite_suffixed(buf, ';')
    }
}

impl CairoWrite for Impl {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cwrite(buf)?;
        self.visibility.cwrite(buf)?;
        self.name.cwrite_prefixed_str(buf, "impl ")?;
        self.generic_params.cwrite(buf)?;
        self.trait_path.cwrite_prefixed_str(buf, " of ")?;
        match &self.body {
            Some(items) => items.cwrite_block_braced(buf),
            None => buf.write_char(';'),
        }
    }
}

impl CairoWrite for ImplItem {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            ImplItem::Function(e) => e.cwrite(buf),
            ImplItem::Type(e) => e.cwrite(buf),
            ImplItem::Constant(e) => e.cwrite(buf),
            ImplItem::Impl(e) => e.cwrite(buf),
            ImplItem::Module(e) => e.cwrite(buf),
            ImplItem::Use(e) => e.cwrite(buf),
            ImplItem::ExternFunction(e) => e.cwrite(buf),
            ImplItem::ExternType(e) => e.cwrite(buf),
            ImplItem::Trait(e) => e.cwrite(buf),
            ImplItem::Struct(e) => e.cwrite(buf),
            ImplItem::Enum(e) => e.cwrite(buf),
            ImplItem::Missing => Ok(()),
        }
    }
}

impl CairoWrite for TypeAlias {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cwrite(buf)?;
        self.visibility.cwrite(buf)?;
        self.name.cwrite_prefixed_str(buf, "type ")?;
        self.generic_params.cwrite_suffixed_str(buf, " = ")?;
        self.ty.cwrite_suffixed(buf, ';')
    }
}

impl CairoWrite for ImplAlias {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cwrite(buf)?;
        self.visibility.cwrite(buf)?;
        self.name.cwrite_prefixed_str(buf, "impl ")?;
        self.generic_params.cwrite_suffixed_str(buf, " = ")?;
        self.path.cwrite_suffixed(buf, ';')
    }
}

impl CairoWrite for InlineMacroItem {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cwrite(buf)?;
        self.path.cwrite_suffixed(buf, '!')?;
        self.arguments.cwrite(buf)
    }
}
