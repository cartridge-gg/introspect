use crate::{Attribute, AttributesTrait, IAttribute};

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

impl From<Attribute> for Vec<AttributeVariant> {
    fn from(attr: Attribute) -> Self {
        vec![AttributeVariant::Cairo(attr)]
    }
}

impl<E> From<Attribute> for Result<Vec<AttributeVariant>, E> {
    fn from(attr: Attribute) -> Self {
        Ok(vec![AttributeVariant::Cairo(attr)])
    }
}

pub trait AttributeParser<T: AttributesTrait, MacroAttribute: Default> {
    type Error;
    fn parse_attribute(
        &self,
        item: &mut T,
        macro_attributes: &mut MacroAttribute,
        attribute: Attribute,
    ) -> Result<Vec<AttributeVariant>, Self::Error>;
    fn parse_attributes(
        &self,
        item: &mut T,
    ) -> Result<(MacroAttribute, Vec<IAttribute>), Self::Error> {
        let mut macro_attrs: MacroAttribute = Default::default();
        let mut intro_attrs: Vec<IAttribute> = Vec::new();
        let mut cairo_attrs: Vec<Attribute> = Vec::new();
        for attr in item.take_attributes() {
            let parsed_attrs = self.parse_attribute(item, &mut macro_attrs, attr)?;
            for parsed_attr in parsed_attrs {
                match parsed_attr {
                    AttributeVariant::Cairo(a) => cairo_attrs.push(a),
                    AttributeVariant::Emit(a) => intro_attrs.push(a),
                }
            }
        }
        item.update_attributes(cairo_attrs);
        Ok((macro_attrs, intro_attrs))
    }
}
