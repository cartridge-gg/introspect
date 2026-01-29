use crate::IAttribute;
use cairo_syntax_parser::{Attribute, AttributesTrait};

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
