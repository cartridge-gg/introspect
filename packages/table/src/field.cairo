use introspect_types::IdData;
use crate::{MemberTrait, RecordId, Spannable, TableStructure, TupleSnappable};

pub trait RecordField<
    const ID: felt252, impl Struct: TableStructure, impl Member: MemberTrait<Struct, ID>, Entry,
> {
    fn serialize_to_tuple(self: @Entry) -> (felt252, Span<felt252>);
    fn serialize_to_id_data(self: @Entry) -> IdData {
        Self::serialize_to_tuple(self).into()
    }
}

pub trait RecordsField<
    const ID: felt252, impl Struct: TableStructure, impl Member: MemberTrait<Struct, ID>, Entries,
> {
    fn serialise_to_id_data_span(self: Entries) -> Span<IdData>;
}

pub impl RecordFieldImpl<
    const ID: felt252,
    impl Struct: TableStructure,
    impl Member: MemberTrait<Struct, ID>,
    Entry,
    ToId,
    impl RID: RecordId<ToId, Struct>,
    impl TS: TupleSnappable<Entry, (@ToId, @Member::Type)>,
> of RecordField<ID, Struct, Member, Entry> {
    fn serialize_to_tuple(self: @Entry) -> (felt252, Span<felt252>) {
        let (key, field): (@ToId, @Member::Type) = TS::snap_tuple(self);
        let record_id = RID::record_id(key);
        let data = Member::serialize_member_inline(field);
        (record_id, data)
    }
}


impl RecordsFieldImpl<
    const ID: felt252,
    impl Struct: TableStructure,
    impl Member: MemberTrait<Struct, ID>,
    Entries,
    Entry,
    impl IM: RecordField<ID, Struct, Member, Entry>,
    +Spannable<Entries, Entry>,
> of RecordsField<ID, Struct, Member, Entries> {
    fn serialise_to_id_data_span(self: Entries) -> Span<IdData> {
        self.to_span().into_iter().map(|M| IM::serialize_to_id_data(M)).collect::<Array<_>>().span()
    }
}
