use introspect::Entry;
use snforge_std::fuzzable::Fuzzable;
use super::Fuzzy;


pub impl IdDataFuzzable of Fuzzable<Entry> {
    fn blank() -> Entry {
        Default::default()
    }
    fn generate() -> Entry {
        Entry { row: Fuzzable::generate(), data: Fuzzy::generate_span_lt(28) }
    }
}
