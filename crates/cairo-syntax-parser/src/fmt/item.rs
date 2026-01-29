use super::{CairoWrite, CairoSliceFormat};
use crate::fmt::fmt::{OptionSizeHint, SizeHint};
use crate::item::{
    Constant, Enum, ExternFunction, ExternType, FunctionDeclaration, FunctionSignature,
    FunctionWithBody, Impl, ImplAlias, ImplItem, InlineMacroItem, Item, Member, Module, Struct,
    Trait, TraitConstant, TraitFunction, TraitImpl, TraitItem, TraitType, TypeAlias, UseItem,
    UsePath, UsePathLeaf, UsePathSingle, Variant,
};
use std::fmt::{Result, Write};

impl CairoWrite for Item {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
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
            Item::MacroDeclaration | Item::HeaderDoc(_) | Item::Missing => Ok(()),
        }
    }

    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        match self {
            Item::Constant(e) => e.size_hint(),
            Item::Module(e) => e.size_hint(),
            Item::Use(e) => e.size_hint(),
            Item::FreeFunction(e) => e.size_hint(),
            Item::ExternFunction(e) => e.size_hint(),
            Item::ExternType(e) => e.size_hint(),
            Item::Trait(e) => e.size_hint(),
            Item::Impl(e) => e.size_hint(),
            Item::ImplAlias(e) => e.size_hint(),
            Item::Struct(e) => e.size_hint(),
            Item::Enum(e) => e.size_hint(),
            Item::TypeAlias(e) => e.size_hint(),
            Item::InlineMacro(e) => e.size_hint(),
            Item::MacroDeclaration | Item::HeaderDoc(_) | Item::Missing => 0,
        }
    }
}

impl CairoWrite for Vec<Item> {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.cfmt_terminated(buf, '\n')
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.size_hint_slice::<1>()
    }
}

impl CairoWrite for Struct {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cfmt(buf)?;
        self.visibility.cfmt(buf)?;
        self.name.cfmt_prefixed_str(buf, "struct ")?;
        self.generic_params.cfmt(buf)?;
        self.members.cfmt_fields_braced(buf)
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.attributes.size_hint()
            + self.visibility.size_hint()
            + 7
            + self.name.size_hint()
            + self.generic_params.size_hint()
            + self.members.size_hint()
    }
}

impl CairoWrite for Member {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cfmt(buf)?;
        self.visibility.cfmt(buf)?;
        self.name.cfmt(buf)?;
        self.ty.cfmt_prefixed_str(buf, ": ")
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.attributes.size_hint()
            + self.visibility.size_hint()
            + self.name.size_hint()
            + 2
            + self.ty.size_hint()
    }
}

impl SizeHint for Vec<Member> {
    fn size_hint(&self) -> usize {
        if self.is_empty() {
            2
        } else {
            self.size_hint_slice::<2>() + 3
        }
    }
}

impl CairoWrite for Enum {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cfmt(buf)?;
        self.visibility.cfmt(buf)?;
        self.name.cfmt_prefixed_str(buf, "enum ")?;
        self.generic_params.cfmt(buf)?;
        self.variants.cfmt_fields_braced(buf)
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.attributes.size_hint()
            + self.visibility.size_hint()
            + 6
            + self.name.size_hint()
            + self.generic_params.size_hint()
            + self.variants.size_hint()
    }
}

impl SizeHint for Vec<Variant> {
    fn size_hint(&self) -> usize {
        if self.is_empty() {
            2
        } else {
            self.size_hint_slice::<2>() + 3
        }
    }
}

impl CairoWrite for Variant {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cfmt(buf)?;
        self.name.cfmt(buf)?;
        if let Some(ty) = &self.type_clause {
            ty.cfmt_prefixed_str(buf, ": ")?;
        }
        Ok(())
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.attributes.size_hint()
            + self.name.size_hint()
            + self.type_clause.size_hint_option::<2, 0>()
    }
}

impl CairoWrite for Constant {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cfmt(buf)?;
        self.visibility.cfmt(buf)?;
        self.name.cfmt_prefixed_str(buf, "const ")?;
        self.ty.cfmt_prefixed_str(buf, ": ")?;
        self.value.cfmt_prefixed_str(buf, " = ")?;
        buf.write_char(';')
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        13 + self.attributes.size_hint()
            + self.visibility.size_hint()
            + self.name.size_hint()
            + self.ty.size_hint()
            + self.value.size_hint()
    }
}
impl CairoWrite for Module {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cfmt(buf)?;
        self.visibility.cfmt(buf)?;
        self.name.cfmt_prefixed_str(buf, "mod ")?;
        match &self.body {
            Some(items) => items.cfmt_block_braced(buf),
            None => buf.write_char(';'),
        }
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.attributes.size_hint()
            + self.visibility.size_hint()
            + 4
            + self.name.size_hint()
            + self.body.size_hint_option::<2, 0>()
    }
}

impl CairoWrite for UseItem {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cfmt(buf)?;
        self.visibility.cfmt(buf)?;
        buf.write_str("use ")?;
        if self.dollar {
            buf.write_char('$')?;
        }
        self.path.cfmt_suffixed(buf, ';')
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.attributes.size_hint()
            + self.visibility.size_hint()
            + 5
            + if self.dollar { 1 } else { 0 }
            + self.path.size_hint()
            + 1
    }
}

impl CairoWrite for UsePath {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            UsePath::Leaf(leaf) => leaf.cfmt(buf),
            UsePath::Single(single) => single.cfmt(buf),
            UsePath::Multi(multi) => multi.cfmt_csv_braced(buf),
            UsePath::Star => buf.write_char('*'),
        }
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        match self {
            UsePath::Leaf(leaf) => leaf.size_hint(),
            UsePath::Single(single) => single.size_hint(),
            UsePath::Multi(multi) => 2 + multi.size_hint_slice::<2>(),
            UsePath::Star => 1,
        }
    }
}

impl CairoWrite for UsePathLeaf {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.ident.cfmt(buf)?;
        if let Some(alias) = &self.alias {
            alias.cfmt_prefixed_str(buf, " as ")?;
        }
        Ok(())
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.ident.size_hint() + self.alias.size_hint_option::<4, 0>()
    }
}

impl CairoWrite for UsePathSingle {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.ident.cfmt_suffixed_str(buf, "::")?;
        self.path.cfmt(buf)
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.ident.size_hint() + 2 + self.path.size_hint()
    }
}

impl CairoWrite for FunctionWithBody {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cfmt(buf)?;
        self.visibility.cfmt(buf)?;
        self.declaration.cfmt(buf)?;
        self.body.cfmt_prefixed(buf, ' ')
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.attributes.size_hint()
            + self.visibility.size_hint()
            + self.declaration.size_hint()
            + self.body.size_hint()
            + 1
    }
}

impl CairoWrite for FunctionDeclaration {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        if self.is_const {
            buf.write_str("const ")?;
        }
        self.name.cfmt_prefixed_str(buf, "fn ")?;
        self.generic_params.cfmt(buf)?;
        self.signature.cfmt(buf)
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        (if self.is_const { 10 } else { 4 })
            + self.name.size_hint()
            + self.generic_params.size_hint()
            + self.signature.size_hint()
    }
}

impl CairoWrite for FunctionSignature {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.parameters.cfmt_csv_parenthesized(buf)?;
        if let Some(ret_type) = &self.return_type {
            ret_type.cfmt_prefixed_str(buf, " -> ")?;
        }
        if let Some(implicits) = &self.implicits_clause {
            buf.write_str(" implicits")?;
            implicits.cfmt_csv_parenthesized(buf)?;
        }
        if self.no_panic {
            buf.write_str(" nopanic")?;
        }
        Ok(())
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.parameters.size_hint()
            + self.return_type.size_hint_option::<4, 0>()
            + self.implicits_clause.size_hint_option::<11, 0>()
            + if self.no_panic { 10 } else { 2 }
    }
}





impl CairoWrite for ExternFunction {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cfmt(buf)?;
        self.visibility.cfmt(buf)?;
        buf.write_str("extern ")?;
        self.declaration.cfmt_suffixed(buf, ';')
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.attributes.size_hint() + self.visibility.size_hint() + self.declaration.size_hint() + 8
    }
}

impl CairoWrite for ExternType {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cfmt(buf)?;
        self.visibility.cfmt(buf)?;
        self.name.cfmt_prefixed_str(buf, "extern type ")?;
        self.generic_params.cfmt_suffixed(buf, ';')
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.attributes.size_hint()
            + self.visibility.size_hint()
            + self.name.size_hint()
            + self.generic_params.size_hint()
            + 13
    }
}

impl CairoWrite for Trait {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cfmt(buf)?;
        self.visibility.cfmt(buf)?;
        self.name.cfmt_prefixed_str(buf, "trait ")?;
        self.generic_params.cfmt(buf)?;
        match &self.body {
            Some(items) => items.cfmt_block_braced(buf),
            None => buf.write_char(';'),
        }
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.attributes.size_hint()
            + self.visibility.size_hint()
            + 6
            + self.name.size_hint()
            + self.generic_params.size_hint()
            + self.body.size_hint_option::<2, 1>()
    }
}

impl SizeHint for Vec<TraitItem> {
    fn size_hint(&self) -> usize {
        self.size_hint_block::<2>()
    }
}

impl CairoWrite for TraitItem {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        match self {
            TraitItem::Function(e) => e.cfmt(buf),
            TraitItem::Type(e) => e.cfmt(buf),
            TraitItem::Constant(e) => e.cfmt(buf),
            TraitItem::Impl(e) => e.cfmt(buf),
            TraitItem::Missing => Ok(()),
        }
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        match self {
            TraitItem::Function(e) => e.size_hint(),
            TraitItem::Type(e) => e.size_hint(),
            TraitItem::Constant(e) => e.size_hint(),
            TraitItem::Impl(e) => e.size_hint(),
            TraitItem::Missing => 0,
        }
    }
}

impl CairoWrite for TraitFunction {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cfmt(buf)?;
        self.declaration.cfmt(buf)?;
        match &self.body {
            Some(body) => body.cfmt_prefixed(buf, ' '),
            None => buf.write_char(';'),
        }
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.attributes.size_hint()
            + self.declaration.size_hint()
            + self.body.size_hint_option::<1, 1>()
    }
}

impl CairoWrite for TraitType {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cfmt(buf)?;
        self.name.cfmt_prefixed_str(buf, "type ")?;
        self.generic_params.cfmt_suffixed(buf, ';')
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.attributes.size_hint() + self.name.size_hint() + self.generic_params.size_hint() + 6
    }
}

impl CairoWrite for TraitConstant {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cfmt(buf)?;
        self.name.cfmt_prefixed_str(buf, "const ")?;
        self.ty.cfmt_prefixed_str(buf, ": ")?;
        buf.write_char(';')
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.attributes.size_hint() + self.name.size_hint() + self.ty.size_hint() + 10
    }
}

impl CairoWrite for TraitImpl {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cfmt(buf)?;
        self.name.cfmt_prefixed_str(buf, "impl ")?;
        self.trait_path.cfmt_suffixed(buf, ';')
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.attributes.size_hint() + self.name.size_hint() + self.trait_path.size_hint() + 6
    }
}

impl CairoWrite for Impl {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cfmt(buf)?;
        self.visibility.cfmt(buf)?;
        self.name.cfmt_prefixed_str(buf, "impl ")?;
        self.generic_params.cfmt(buf)?;
        self.trait_path.cfmt_prefixed_str(buf, " of ")?;
        match &self.body {
            Some(items) => items.cfmt_block_braced(buf),
            None => buf.write_char(';'),
        }
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.attributes.size_hint()
            + self.visibility.size_hint()
            + self.name.size_hint()
            + self.generic_params.size_hint()
            + self.trait_path.size_hint()
            + self.body.size_hint_option::<2, 1>()
    }
}

impl CairoWrite for ImplItem {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
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
            ImplItem::Missing => Ok(()),
        }
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        match self {
            ImplItem::Function(e) => e.size_hint(),
            ImplItem::Type(e) => e.size_hint(),
            ImplItem::Constant(e) => e.size_hint(),
            ImplItem::Impl(e) => e.size_hint(),
            ImplItem::Module(e) => e.size_hint(),
            ImplItem::Use(e) => e.size_hint(),
            ImplItem::ExternFunction(e) => e.size_hint(),
            ImplItem::ExternType(e) => e.size_hint(),
            ImplItem::Trait(e) => e.size_hint(),
            ImplItem::Struct(e) => e.size_hint(),
            ImplItem::Enum(e) => e.size_hint(),
            ImplItem::Missing => 0,
        }
    }
}

impl SizeHint for Vec<ImplItem> {
    fn size_hint(&self) -> usize {
        self.size_hint_block::<2>()
    }
}

impl CairoWrite for TypeAlias {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cfmt(buf)?;
        self.visibility.cfmt(buf)?;
        self.name.cfmt_prefixed_str(buf, "type ")?;
        self.generic_params.cfmt_suffixed_str(buf, " = ")?;
        self.ty.cfmt_suffixed(buf, ';')
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.attributes.size_hint()
            + self.visibility.size_hint()
            + self.name.size_hint()
            + self.generic_params.size_hint()
            + self.ty.size_hint()
            + 8
    }
}

impl CairoWrite for ImplAlias {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cfmt(buf)?;
        self.visibility.cfmt(buf)?;
        self.name.cfmt_prefixed_str(buf, "impl ")?;
        self.generic_params.cfmt_suffixed_str(buf, " = ")?;
        self.path.cfmt_suffixed(buf, ';')
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.attributes.size_hint()
            + self.visibility.size_hint()
            + self.name.size_hint()
            + self.generic_params.size_hint()
            + self.path.size_hint()
            + 9
    }
}

impl CairoWrite for InlineMacroItem {
    fn cfmt<W: Write>(&self, buf: &mut W) -> Result {
        self.attributes.cfmt(buf)?;
        self.path.cfmt_suffixed(buf, '!')?;
        self.arguments.cfmt(buf)
    }
    }
impl SizeHint for  {
    fn size_hint(&self) -> usize {
        self.attributes.size_hint() + self.path.size_hint() + 1 + self.arguments.size_hint()
    }
}
