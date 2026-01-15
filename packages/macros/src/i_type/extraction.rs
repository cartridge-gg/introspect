use crate::{AttributeArg, AttributeCallType, IntrospectError, SyntaxItemTrait};
use cairo_lang_macro::TokenStream;
use salsa::Database;

pub trait IExtractor {
    type Error;
    fn get_attribute_call_type(&self) -> &AttributeCallType;
    fn derive_call_error(&self) -> Self::Error;
}

pub trait IExtract<T> {
    type SyntaxType;
    type Error;
    fn iextract(&self, module: &mut Self::SyntaxType) -> Result<T, Self::Error>;
    fn iextracts(&self, modules: &mut [Self::SyntaxType]) -> Result<Vec<T>, Self::Error> {
        modules.iter_mut().map(|m| self.iextract(m)).collect()
    }
}

pub trait IExtractWithArgs<T> {
    type SyntaxType;
    type Error;
    fn iextract_with_args(
        &self,
        module: &mut Self::SyntaxType,
        attributes: &Vec<AttributeArg>,
    ) -> Result<T, Self::Error>;
}

pub trait IExtractWith<T, C> {
    type SyntaxType;
    type Error;
    fn iextract_with(&self, module: &mut Self::SyntaxType, context: &C) -> Result<T, Self::Error>;
    fn iextracts_with(
        &self,
        modules_with: &mut [(Self::SyntaxType, C)],
    ) -> Result<Vec<T>, Self::Error> {
        modules_with
            .iter_mut()
            .map(|(m, c)| self.iextract_with(m, c))
            .collect()
    }
}

pub trait IExtractFromTokenStream<T> {
    type Error: From<IntrospectError>;
    fn iextract_from_file_node(
        &self,
        db: &dyn Database,
        node: cairo_lang_syntax::node::SyntaxNode,
    ) -> Result<T, Self::Error>;
    fn iextract_from_token_stream(&self, token_stream: TokenStream) -> Result<T, Self::Error> {
        let db = cairo_lang_parser::utils::SimpleParserDatabase::default();
        let (node, _diagnostics) = db.parse_virtual_with_diagnostics(token_stream.clone());
        self.iextract_from_file_node(&db, node)
    }
}

impl<E, T> IExtractFromTokenStream<T> for E
where
    E: IExtract<T>,
    E::SyntaxType: SyntaxItemTrait,
    E::Error: From<IntrospectError>,
{
    type Error = E::Error;
    fn iextract_from_file_node(
        &self,
        db: &dyn Database,
        node: cairo_lang_syntax::node::SyntaxNode,
    ) -> Result<T, Self::Error> {
        let mut item = E::SyntaxType::from_file_node(db, node)?;
        self.iextract(&mut item)
    }
}

impl<E, T> IExtractWith<T, ()> for E
where
    E: IExtract<T>,
{
    type Error = E::Error;
    type SyntaxType = E::SyntaxType;
    fn iextract_with(&self, module: &mut E::SyntaxType, _context: &()) -> Result<T, Self::Error> {
        self.iextract(module)
    }
}
