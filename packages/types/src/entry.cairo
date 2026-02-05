use crate::ISerde;
use crate::utils::SpanDefault;

pub struct EntryConst<const SIZE: usize> {
    pub row: felt252,
    pub data: [felt252; SIZE],
}

#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct Entry {
    pub row: felt252,
    pub data: Span<felt252>,
}


impl EntryConstToEntry<
    const SIZE: usize, impl ToSpan: ToSpanTrait<[felt252; SIZE], felt252>,
> of Into<EntryConst<SIZE>, Entry> {
    fn into(self: EntryConst<SIZE>) -> Entry {
        Entry { row: self.row, data: ToSpan::span(@self.data) }
    }
}

impl EntryIntoTuple of Into<Entry, (felt252, Span<felt252>)> {
    const fn into(self: Entry) -> (felt252, Span<felt252>) nopanic {
        (self.row, self.data)
    }
}

impl TupleIntoEntry of Into<(felt252, Span<felt252>), Entry> {
    const fn into(self: (felt252, Span<felt252>)) -> Entry nopanic {
        let (row, data) = self;
        Entry { row, data }
    }
}

impl EntryISerde of ISerde<Entry> {
    const SIZE_HINT: Option<u32> = None;
    fn iserialize(self: @Entry, ref output: Array<felt252>) {
        output.append(*self.row);
        self.data.iserialize(ref output);
    }

    fn ideserialize(ref serialized: Span<felt252>) -> Option<Entry> {
        let row = *serialized.pop_front()?;
        let data = ISerde::ideserialize(ref serialized)?;
        Some(Entry { row, data })
    }
    fn iserialized_size(self: @Entry) -> u32 {
        1 + self.data.iserialized_size()
    }
}
