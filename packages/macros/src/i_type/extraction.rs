use crate::i_type::TypeDefVariant;
use crate::{Attribute, IAttribute, Result, SyntaxItemTrait, Ty};
use cairo_lang_macro::TokenStream;
use salsa::Database;

pub trait IExtract<T> {
    type SyntaxType;
    fn iextract(&self, module: &mut Self::SyntaxType) -> Result<T>;
    fn iextracts(&self, modules: &mut [Self::SyntaxType]) -> Result<Vec<T>> {
        modules.iter_mut().map(|m| self.iextract(m)).collect()
    }
}

pub trait IExtractWith<T, C> {
    type SyntaxType;
    fn iextract_with(&self, module: &mut Self::SyntaxType, context: &C) -> Result<T>;
    fn iextracts_with(&self, modules_with: &mut [(Self::SyntaxType, C)]) -> Result<Vec<T>> {
        modules_with
            .iter_mut()
            .map(|(m, c)| self.iextract_with(m, c))
            .collect()
    }
}

pub trait IExtractFromTokenStream<T> {
    fn iextract_from_file_node(
        &self,
        db: &dyn Database,
        node: cairo_lang_syntax::node::SyntaxNode,
    ) -> Result<T>;
    fn iextract_from_token_stream(&self, token_stream: TokenStream) -> Result<T> {
        let db = cairo_lang_parser::utils::SimpleParserDatabase::default();
        let (node, _diagnostics) = db.parse_virtual_with_diagnostics(token_stream.clone());
        self.iextract_from_file_node(&db, node)
    }
}

impl<E, T> IExtractFromTokenStream<T> for E
where
    E: IExtract<T>,
    E::SyntaxType: SyntaxItemTrait,
{
    fn iextract_from_file_node(
        &self,
        db: &dyn Database,
        node: cairo_lang_syntax::node::SyntaxNode,
    ) -> Result<T> {
        let mut item = E::SyntaxType::from_file_node(db, node)?;
        self.iextract(&mut item)
    }
}

impl<E, T> IExtractWith<T, ()> for E
where
    E: IExtract<T>,
{
    type SyntaxType = E::SyntaxType;
    fn iextract_with(&self, module: &mut E::SyntaxType, _context: &()) -> Result<T> {
        self.iextract(module)
    }
}

pub struct DefaultIExtractor;

pub enum AttributeVariant {
    Emit(IAttribute),
    Macro(MacroAttribute),
    Ignore(Attribute),
}

pub enum MacroAttribute {
    Raw,
    Encoded(String),
}

pub fn sort_attribute_variants(
    attributes: Vec<AttributeVariant>,
) -> (Vec<Attribute>, Vec<IAttribute>, Vec<MacroAttribute>) {
    let mut macro_attributes = Vec::new();
    let mut iattributes = Vec::new();
    let mut other_attributes = Vec::new();
    for attr in attributes {
        match attr {
            AttributeVariant::Macro(m) => macro_attributes.push(m),
            AttributeVariant::Emit(i) => iattributes.push(i),
            AttributeVariant::Ignore(o) => other_attributes.push(o),
        }
    }
    (other_attributes, iattributes, macro_attributes)
}

impl DefaultIExtractor {
    pub fn new() -> Self {
        DefaultIExtractor {}
    }

    pub fn parse_type_def(&self, _ty: &Ty, _attributes: &[MacroAttribute]) -> TypeDefVariant {
        TypeDefVariant::Default
    }

    pub fn extract_attributes(
        &self,
        attributes: Vec<Attribute>,
    ) -> Result<(Vec<Attribute>, Vec<IAttribute>, Vec<MacroAttribute>)> {
        attributes
            .into_iter()
            .map(|a| self.parse_attribute(a))
            .collect::<Result<Vec<_>>>()
            .map(|v| sort_attribute_variants(v.into_iter().flatten().collect()))
    }

    pub fn parse_attribute(&self, attribute: Attribute) -> Result<Vec<AttributeVariant>> {
        match (attribute.name.as_str(), &attribute.args) {
            ("i_raw", None) => Ok(vec![AttributeVariant::Macro(MacroAttribute::Raw)]),
            ("encoded", Some(_)) => attribute
                .single_unnamed_arg()
                .map(|arg| vec![AttributeVariant::Macro(MacroAttribute::Encoded(arg))]),
            ("encoded" | "i_raw", _) => attribute.format_err(),
            _ => Ok(vec![AttributeVariant::Ignore(attribute)]),
        }
    }
}
