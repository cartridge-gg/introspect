use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::ast::OptionWrappedGenericParamList;
use salsa::Database;

pub fn parse_params<'db>(
    maybe_params: OptionWrappedGenericParamList<'db>,
    db: &'db dyn Database,
) -> Option<Vec<String>> {
    match maybe_params {
        OptionWrappedGenericParamList::Empty(_) => None,
        OptionWrappedGenericParamList::WrappedGenericParamList(params) => Some(
            params
                .generic_params(db)
                .elements(db)
                .map(|p| p.as_syntax_node().get_text_without_all_comment_trivia(db))
                .collect(),
        ),
    }
}
