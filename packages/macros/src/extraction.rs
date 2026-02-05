use cairo_syntax_parser::Arg;

pub trait IExtract
where
    Self: Sized,
{
    type SyntaxType;
    type Error;
    fn iextract(module: &mut Self::SyntaxType) -> Result<Self, Self::Error>;
    fn iextracts(modules: &mut [Self::SyntaxType]) -> Result<Vec<Self>, Self::Error> {
        modules.iter_mut().map(|m| Self::iextract(m)).collect()
    }
}

pub trait IExtractWithArgs<T> {
    type SyntaxType;
    type Error;
    fn iextract_with_args(
        module: &mut Self::SyntaxType,
        attributes: &Vec<Arg>,
    ) -> Result<T, Self::Error>;
}

pub trait IExtractWith
where
    Self: Sized,
{
    type Context;
    type SyntaxType;
    type Error;
    fn iextract_with(
        module: &mut Self::SyntaxType,
        context: &Self::Context,
    ) -> Result<Self, Self::Error>;
    fn iextracts_with(
        modules: &mut [Self::SyntaxType],
        context: &Self::Context,
    ) -> Result<Vec<Self>, Self::Error> {
        modules
            .iter_mut()
            .map(|m| Self::iextract_with(m, context))
            .collect()
    }
    fn iextracts_withs(
        modules_with: &mut [(Self::SyntaxType, Self::Context)],
    ) -> Result<Vec<Self>, Self::Error> {
        modules_with
            .iter_mut()
            .map(|(m, c)| Self::iextract_with(m, c))
            .collect()
    }
}

pub trait IExtractable {
    fn iextract<I: IExtract<SyntaxType = Self>>(&mut self) -> Result<I, I::Error>;
    fn iextract_with<I: IExtractWith<SyntaxType = Self>>(
        &mut self,
        context: &I::Context,
    ) -> Result<I, I::Error>;
}

pub trait IExtractables<I>
where
    I: IExtract,
{
    fn iextracts(&mut self) -> Result<Vec<I>, I::Error>;
}

pub trait IExtractablesContext<I>
where
    I: IExtractWith,
{
    fn iextracts_with(&mut self, context: &I::Context) -> Result<Vec<I>, I::Error>;
}

impl<T, I> IExtractables<I> for [T]
where
    I: IExtract<SyntaxType = T>,
{
    fn iextracts(&mut self) -> Result<Vec<I>, I::Error> {
        self.iter_mut().map(|item| I::iextract(item)).collect()
    }
}

impl<T, I> IExtractablesContext<I> for [T]
where
    I: IExtractWith<SyntaxType = T>,
{
    fn iextracts_with(&mut self, context: &I::Context) -> Result<Vec<I>, I::Error> {
        self.iter_mut()
            .map(|item| I::iextract_with(item, context))
            .collect()
    }
}

impl<T> IExtractable for T {
    fn iextract<I: IExtract<SyntaxType = Self>>(&mut self) -> Result<I, I::Error> {
        I::iextract(self)
    }
    fn iextract_with<I: IExtractWith<SyntaxType = Self>>(
        &mut self,
        context: &I::Context,
    ) -> Result<I, I::Error> {
        I::iextract_with(self, context)
    }
}

// impl<I> IExtractFromTokenStream for I
// where
//     I: IExtract,
//     I::SyntaxType: SyntaxItemTrait,
//     I::Error: From<IntrospectError>,
// {
//     type Error = I::Error;
//     fn iextract_from_file_node(
//         db: &dyn Database,
//         node: cairo_lang_syntax::node::SyntaxNode,
//     ) -> Result<Self, Self::Error> {
//         let mut item = I::SyntaxType::from_file_node(db, node)?;
//         I::iextract(&mut item)
//     }
// }
