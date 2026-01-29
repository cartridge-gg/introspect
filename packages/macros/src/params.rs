use std::ops::Deref;

use crate::{AstInto, FromAst, typed_syntax_node_to_string_without_trivia, vec_from_element_list};
use cairo_lang_syntax::node::ast::OptionWrappedGenericParamList;
use cairo_syntax_parser::CairoWrite;
use salsa::Database;

vec_from_element_list!(WrappedGenericParamList.generic_params, String);

typed_syntax_node_to_string_without_trivia!(GenericParam);

#[derive(Clone, Debug, PartialEq)]
pub struct GenericParams(pub Vec<String>);

impl GenericParams {
    pub fn as_cairo_callable(&self) -> String {
        let mut buf = String::new();
        if !self.is_empty() {
            buf.push_str("::");
            self.cfmt_csv_angled(&mut buf);
        }
        buf
    }

    pub fn with_trait_bounds(&self, traits: &[&str]) -> String {
        match &self.0.len() {
            0 => "".to_string(),
            _ => {
                let items = self
                    .0
                    .iter()
                    .flat_map(|p| traits.iter().map(move |t| format!("+{t}<{p}>")))
                    .collect::<Vec<_>>();
                format!("<{}, {}>", self.0.join(", "), items.join(", "))
            }
        }
    }
}

impl Deref for GenericParams {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'db> FromAst<'db, OptionWrappedGenericParamList<'db>> for GenericParams {
    fn from_ast(ast: OptionWrappedGenericParamList<'db>, db: &'db dyn Database) -> Self {
        match ast {
            OptionWrappedGenericParamList::Empty(_) => GenericParams(vec![]),
            OptionWrappedGenericParamList::WrappedGenericParamList(arg_list) => {
                GenericParams(arg_list.ast_into(db))
            }
        }
    }
}

impl CairoWrite for GenericParams {
    fn cfmt(&self, buf: &mut String) {
        if !self.is_empty() {
            self.cfmt_csv_angled(buf);
        }
    }
}
