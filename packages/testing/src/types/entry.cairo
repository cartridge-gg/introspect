use cgg_utils::testing::Fuzzy;
use introspect_types::Entry;
use snforge_std::fuzzable::Fuzzable;


pub impl IdDataFuzzable of Fuzzable<Entry> {
    fn blank() -> Entry {
        Default::default()
    }
    fn generate() -> Entry {
        Entry { row: Fuzzable::generate(), data: Fuzzy::generate_span_lt(28) }
    }
}
