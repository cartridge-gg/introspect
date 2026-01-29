use crate::fuzzable::FuzzableImpl;
use cairo_syntax_parser::{Enum, Variant};

impl FuzzableImpl for Enum {
    fn fuzzable_body(&self) -> String {
        let variants: String = self
            .variants
            .iter()
            .enumerate()
            .map(|(i, v)| fuzzable_variant(v, &self.name, i))
            .collect();
        format!(
            "match snforge_std::fuzzable::generate_arg(0_u32, {}){{{variants} _=> Default::default()}}",
            self.variants.len() - 1
        )
    }
}

fn fuzzable_variant(variant: &Variant, enum_name: &str, index: usize) -> String {
    match &variant.type_clause {
        None => format!("{index} => {enum_name}::{},", variant.name),
        Some(_) => format!(
            "{index} => {enum_name}::{}(snforge_std::fuzzable::Fuzzable::generate()),",
            variant.name
        ),
    }
}
