use crate::i_type::byte_array::{byte_array_felt_len, bytes_to_byte_array_felts, string_to_felts};
use crate::{IntrospectError, IntrospectResult};
use cairo_syntax_parser::{Attribute, AttributesTrait, CairoWrite, CairoWriteSlice};
use std::fmt::{Result as FmtResult, Write};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IAttribute {
    pub name: String,
    pub data: Option<Vec<u8>>,
}

impl IAttribute {
    pub fn new_empty(name: String) -> Self {
        IAttribute { name, data: None }
    }
    pub fn cairo_size(&self) -> u32 {
        byte_array_felt_len(&self.name)
            + match &self.data {
                Some(data) => (data.len() as u32).div_ceil(31),
                None => 0,
            }
    }
}

pub trait IAttributesTrait {
    fn iattributes(&self) -> &[IAttribute];
    fn cwrite_const_attributes<W: Write>(&self, buf: &mut W) -> FmtResult {
        let total_size: u32 = self.iattributes_size();
        write!(buf, "const ATTRIBUTES: [felt252; {total_size}] = ")?;
        self.iattributes().cwrite_csv_bracketed(buf)?;
        buf.write_str(";\n")
    }
    fn iattributes_size(&self) -> u32 {
        self.iattributes().iter().map(IAttribute::cairo_size).sum()
    }
    fn cwrite_attribute_count<W: Write>(&self, buf: &mut W) -> FmtResult {
        let count = self.iattributes().len();
        write!(buf, "const ATTRIBUTES_COUNT: u32 = {count};\n")
    }
}

impl IAttributesTrait for [IAttribute] {
    fn iattributes(&self) -> &[IAttribute] {
        self
    }
}

impl CairoWrite for IAttribute {
    fn cwrite<W: Write>(&self, buf: &mut W) -> FmtResult {
        let (name_felts, final_felt) = string_to_felts(&self.name);
        name_felts.cwrite_terminated(buf, ',')?;
        write_terminal_name_byte31(buf, final_felt, self.data.is_some())?;
        if let Some(data) = &self.data {
            buf.write_char(',')?;
            let (data_felts, final_data_felt) = bytes_to_byte_array_felts(data);
            data_felts.cwrite_terminated(buf, ',')?;
            write_terminal_name_byte31(buf, final_data_felt, false)?;
        }
        Ok(())
    }
}

fn write_terminal_name_byte31<W: Write>(buf: &mut W, bytes: &[u8], data: bool) -> FmtResult {
    let first_byte = if data { 0x06 } else { 0x02 };
    if bytes.len() == 31 {
        write!(buf, "0x{first_byte:02x}",)?;
    } else {
        write!(buf, "0x{:02x}{:02x}", first_byte + 1, bytes.len())?;
        (0..31 - bytes.len())
            .map(|_| buf.write_str("00"))
            .collect::<FmtResult>()?;
    }
    bytes
        .iter()
        .map(|b| write!(buf, "{b:02x}"))
        .collect::<FmtResult>()
}

pub enum AttributeVariant {
    Emit(IAttribute),
    Cairo(Attribute),
}

pub trait MacroAttributeTrait
where
    Self: Sized + Default,
{
}

impl AttributeVariant {
    pub fn empty_i_attribute(name: String) -> Self {
        AttributeVariant::Emit(IAttribute::new_empty(name))
    }
    pub fn i_attribute(name: String, data: Option<Vec<u8>>) -> Self {
        AttributeVariant::Emit(IAttribute { name, data })
    }
    pub fn lazy_empty_i_attribute<E>(name: String) -> Result<Vec<Self>, E> {
        Ok(vec![AttributeVariant::Emit(IAttribute::new_empty(name))])
    }
    pub fn lazy_i_attributes<E>(name: String, data: Option<Vec<u8>>) -> Result<Vec<Self>, E> {
        Ok(vec![AttributeVariant::Emit(IAttribute { name, data })])
    }
}

pub trait AttributeParse {
    fn as_variant_vec(self) -> IntrospectResult<Vec<AttributeVariant>>;
    fn to_single_unnamed_arg(&self) -> IntrospectResult<String>;
}

impl AttributeParse for Attribute {
    fn as_variant_vec(self) -> IntrospectResult<Vec<AttributeVariant>> {
        Ok(vec![AttributeVariant::Cairo(self)])
    }
    fn to_single_unnamed_arg(&self) -> IntrospectResult<String> {
        self.get_single_unnamed_arg()
            .ok_or(IntrospectError::InvalidIntrospectAttributeFormat(
                self.path_str().to_string(),
            ))
            .map(|expr| expr.to_string())
    }
}

// impl<dyn MacroAttributeTrait> Into<Vec<AttributeVariant<T>>> for MacroAttributeTrait {
//     fn into(self) -> Vec<AttributeVariant<T>> {
//         vec![AttributeVariant::Macro(self)]
//     }
// }

impl From<Attribute> for AttributeVariant {
    fn from(attr: Attribute) -> Self {
        AttributeVariant::Cairo(attr)
    }
}

// impl From<Attribute> for Vec<AttributeVariant> {
//     fn from(attr: Attribute) -> Self {
//         vec![AttributeVariant::Cairo(attr)]
//     }
// }

// impl<E> From<Attribute> for Result<Vec<AttributeVariant>, E> {
//     fn from(attr: Attribute) -> Self {
//         Ok(vec![AttributeVariant::Cairo(attr)])
//     }
// }

pub trait ParseAttribute<T> {
    type Error;
    fn parse_attribute(
        &mut self,
        module: &mut T,
        attribute: Attribute,
    ) -> Result<Vec<AttributeVariant>, Self::Error>;
}

pub trait AttributeParser<SyntaxType: AttributesTrait>
where
    Self: Default,
{
    type Error;

    fn parse_attribute(
        &mut self,
        _module: &mut SyntaxType,
        attribute: Attribute,
    ) -> Result<Vec<AttributeVariant>, Self::Error>;

    fn parse_attributes(
        &mut self,
        module: &mut SyntaxType,
    ) -> Result<Vec<IAttribute>, Self::Error> {
        let mut intro_attrs: Vec<IAttribute> = Vec::new();
        let mut cairo_attrs: Vec<Attribute> = Vec::new();
        for attr in module.take_attributes() {
            let parsed_attrs = self.parse_attribute(module, attr)?;
            for parsed_attr in parsed_attrs {
                match parsed_attr {
                    AttributeVariant::Cairo(a) => cairo_attrs.push(a),
                    AttributeVariant::Emit(a) => intro_attrs.push(a),
                }
            }
        }
        module.update_attributes(cairo_attrs);
        Ok(intro_attrs)
    }
}

pub trait ExtractAttributes {
    fn extract_attributes<MacroAttributes>(
        &mut self,
    ) -> Result<(MacroAttributes, Vec<IAttribute>), <MacroAttributes as AttributeParser<Self>>::Error>
    where
        Self: AttributesTrait + Sized,
        MacroAttributes: AttributeParser<Self> + Default;
}

impl<SyntaxType> ExtractAttributes for SyntaxType {
    fn extract_attributes<MacroAttributes>(
        &mut self,
    ) -> Result<(MacroAttributes, Vec<IAttribute>), <MacroAttributes as AttributeParser<Self>>::Error>
    where
        Self: AttributesTrait + Sized,
        MacroAttributes: AttributeParser<Self> + Default,
    {
        let mut macro_attributes = MacroAttributes::default();
        let attributes = macro_attributes.parse_attributes(self)?;
        Ok((macro_attributes, attributes))
    }
}
