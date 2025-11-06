use cairo_lang_syntax::attribute::structured::AttributeArg;
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::ast::{
    Attribute as AstAttribute, AttributeList, OptionArgListParenthesized,
};
use salsa::Database;

pub struct Attribute<'db> {
    pub db: &'db dyn Database,
    pub name: String,
    pub args: Option<Vec<AttributeArg<'db>>>,
}

pub fn parse_attributes<'db>(
    attrs: AttributeList<'db>,
    db: &'db dyn Database,
) -> Vec<Attribute<'db>> {
    attrs
        .elements(db)
        .map(|attr| Attribute::new(attr, db))
        .collect()
}

impl<'db> Attribute<'db> {
    pub fn new(attribute: AstAttribute<'db>, db: &'db dyn Database) -> Self {
        Self {
            db,
            name: attribute
                .attr(db)
                .as_syntax_node()
                .get_text_without_trivia(db)
                .to_string(db),
            args: match attribute.arguments(db) {
                OptionArgListParenthesized::Empty(_) => None,
                OptionArgListParenthesized::ArgListParenthesized(arg_list) => Some(
                    arg_list
                        .arguments(db)
                        .elements(db)
                        .map(|arg| AttributeArg::from_ast(arg, db))
                        .collect(),
                ),
            },
        }
    }
}

impl<'db> ToString for Attribute<'db> {
    fn to_string(&self) -> String {
        match &self.args {
            Some(args) => {
                let args_str = args
                    .iter()
                    .map(|arg| arg.text(self.db))
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("#[{}({})]", self.name, args_str)
            }
            None => format!("#[{}]", self.name),
        }
    }
}

pub fn attributes_to_string<'db>(attributes: &[Attribute<'db>], indent: usize) -> String {
    if attributes.is_empty() {
        "".to_string()
    } else {
        let line = &("\n".to_string() + "    ".repeat(indent).as_str());
        attributes
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(line)
            + line
    }
}
