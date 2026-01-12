use introspect::IdData;
use snforge_std::fuzzable::Fuzzable;
use super::Fuzzy;


pub impl IdDataFuzzable of Fuzzable<IdData> {
    fn blank() -> IdData {
        Default::default()
    }
    fn generate() -> IdData {
        IdData { id: Fuzzable::generate(), data: Fuzzy::generate_span_lt(28) }
    }
}
