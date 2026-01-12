pub impl Gen{{name}}FuzzableImpl{{impl_params}} of snforge_std::fuzzable::Fuzzable<{{full_name}}> {
    fn blank() -> {{full_name}} {
        Default::default()
    }

    fn generate() -> {{full_name}} {
        {{body}}
    }
}
