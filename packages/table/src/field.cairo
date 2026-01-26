use core_ext::{SnapForwardTo, ToSpan};
use introspect_types::Entry;
use crate::{Member, RecordId, TableStructure};

pub trait RecordField<
    impl Table: TableStructure,
    const ID: felt252,
    impl Member: Member<Table, Table::Record, ID>,
    Item,
> {
    fn serialize_to_tuple(self: @Item) -> (felt252, Span<felt252>);
    #[inline(always)]
    fn serialize_to_entry(self: @Item) -> Entry {
        Self::serialize_to_tuple(self).into()
    }
}

pub trait RecordsField<
    impl Table: TableStructure,
    const ID: felt252,
    impl Member: Member<Table, Table::Record, ID>,
    Items,
> {
    fn serialise_to_entries(self: Items) -> Span<Entry>;
}

pub impl RecordFieldImpl<
    impl Table: TableStructure,
    const ID: felt252,
    impl Member: Member<Table, Table::Record, ID>,
    Tuple,
    AsId,
    impl Id: RecordId<Table, AsId>,
    impl TS: SnapForwardTo<Tuple, (@AsId, @Member::Type)>,
> of RecordField<Table, ID, Member, Tuple> {
    fn serialize_to_tuple(self: @Tuple) -> (felt252, Span<felt252>) {
        let (key, field): (@AsId, @Member::Type) = TS::snap_forward(self);
        let record_id = Id::record_id(key);
        let data = Member::serialize_member_inline(field);
        (record_id, data)
    }
}


impl RecordsFieldImpl<
    const ID: felt252,
    impl Table: TableStructure,
    impl Member: Member<Table, Table::Record, ID>,
    Tuples,
    Tuple,
    impl IM: RecordField<Table, ID, Member, Tuple>,
    +ToSpan<Tuples, Tuple>,
> of RecordsField<Table, ID, Member, Tuples> {
    fn serialise_to_entries(self: Tuples) -> Span<Entry> {
        self.to_span().into_iter().map(|M| IM::serialize_to_entry(M)).collect::<Array<_>>().span()
    }
}
