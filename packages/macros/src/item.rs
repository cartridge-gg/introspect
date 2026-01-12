use crate::{AsCairo, GenericParams};

pub trait ItemTrait {
    fn name(&self) -> &str;
    fn generic_params(&self) -> &GenericParams;
    fn generics_clause(&self) -> String {
        self.generic_params().as_cairo()
    }
    fn full_name(&self) -> String {
        format!("{}{}", self.name(), self.generics_clause())
    }
    fn generics_call(&self) -> String {
        self.generic_params().as_cairo_callable()
    }
    fn full_call(&self) -> String {
        format!("{}{}", self.name(), self.generics_call())
    }
    fn generics_with_traits(&self, traits: &[&str]) -> String {
        self.generic_params().with_trait_bounds(traits)
    }
}
