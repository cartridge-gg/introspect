use introspect_events::EmitEvent;
use introspect_events::database::{
    CreateTableWithColumns, DeleteField, DeleteFields, DeleteRecord, DeleteRecords, DeletesField,
    DeletesFields, InsertField, InsertFieldGroup, InsertFields, InsertRecord, InsertRecords,
    InsertsField, InsertsFieldGroup, InsertsFields,
};
use introspect_types::{Attribute, ChildDefs, ColumnDef, IdData, PrimaryDef, PrimaryTrait};
use crate::{Snapable, Spannable};

pub trait TableStructure {
    type Primary;
    type Record;
    fn attributes() -> Span<Attribute> {
        [].span()
    }
    fn primary() -> PrimaryDef {
        introspect_types::PrimaryDef {
            name: "__id",
            type_def: introspect_types::PrimaryTypeDef::Felt252,
            attributes: [].span(),
        }
    }
    fn columns() -> Span<ColumnDef>;
    fn collect_child_defs(ref defs: ChildDefs);
}

pub trait KeySpanToPrimary<R, impl Struct: TableStructure> {
    fn key_span_to_primary(self: Span<felt252>) -> Struct::Primary;
}

pub trait KeySpanToId<R, impl Struct: TableStructure> {
    fn key_span_to_id(self: Span<felt252>) -> felt252;
}

impl KeySpanToIdImpl<
    K,
    impl Struct: TableStructure,
    impl KP: KeySpanToPrimary<Struct::Record, Struct>,
    +PrimaryTrait<Struct::Primary>,
    +Drop<Struct::Primary>,
> of KeySpanToId<K, Struct> {
    fn key_span_to_id(self: Span<felt252>) -> felt252 {
        KP::key_span_to_primary(self).to_felt252()
    }
}

pub trait RecordId<K, impl Struct: TableStructure> {
    fn record_id(self: @K) -> felt252;
}

// pub mod record_id_felt252 {
//     pub impl Impl<impl T: super::TableSchema> of super::RecordId<felt252, T> {
//         fn record_id(self: @felt252) -> felt252 {
//             *self
//         }
//     }
//     pub impl PrimaryImpl<
//         K, impl T: super::TableSchema, +super::PrimaryTrait<K>,
//     > of super::RecordId<K, T> {
//         fn record_id(self: @K) -> felt252 {
//             self.to_felt252()
//         }
//     }
// }

pub trait RecordIds<KS, impl Struct: TableStructure> {
    fn record_ids(self: KS) -> Span<felt252>;
}

pub trait RecordData<V, impl Struct: TableStructure> {
    fn record_data(self: @V) -> Span<felt252>;
}

pub trait RecordIdData<R, impl Struct: TableStructure> {
    fn record_tuple(self: @R) -> (felt252, Span<felt252>);
    fn record_id_data(self: @R) -> IdData {
        Self::record_tuple(self).into()
    }
}

pub trait MultiIdData<RS, R, impl Struct: TableStructure> {
    fn multi_id_data(self: RS) -> Span<IdData>;
}

pub trait SerialisedKey<R, K, impl Struct: TableStructure> {
    fn serialize_key(self: @K, ref data: Array<felt252>);
}

pub trait RecordKey<R, K, impl Struct: TableStructure> {
    type Key;
    fn record_key(self: @R) -> K;
}
pub trait SerialisedRecordKey<R, impl Struct: TableStructure> {
    fn serialize_record_key(self: @R, ref data: Array<felt252>);
}

impl SerialisedRecordKeyImpl<
    SR,
    K,
    impl Struct: TableStructure,
    impl RK: RecordKey<Struct::Record, K, Struct>,
    impl SK: SerialisedKey<Struct::Record, K, Struct>,
    impl SS: Snapable<@SR, Struct::Record>,
    +Drop<SR>,
    +Drop<K>,
> of SerialisedRecordKey<SR, Struct> {
    fn serialize_record_key(self: @SR, ref data: Array<felt252>) {
        let key = RK::record_key(SS::snapshot(self));
        SK::serialize_key(@key, ref data);
    }
}

pub trait RecordValuesSpanTrait<impl Struct: TableStructure, R> {
    fn serialize_values(self: @Struct::Record, ref data: Array<felt252>);
    fn serialize_values_inline(
        self: @Struct::Record,
    ) -> Span<
        felt252,
    > {
        let mut data: Array<felt252> = Default::default();
        Self::serialize_values(self, ref data);
        data.span()
    }
}

pub impl TablePrimaryIdImpl<
    K,
    impl Struct: TableStructure,
    +PrimaryTrait<Struct::Primary>,
    impl SS: Snapable<@K, Struct::Primary>,
> of RecordId<K, Struct> {
    fn record_id(self: @K) -> felt252 {
        SS::snapshot(self).to_felt252()
    }
}

pub impl RecordIdsImpl<
    KS, K, impl Struct: TableStructure, impl TID: RecordId<K, Struct>, +Spannable<KS, K>,
> of RecordIds<KS, Struct> {
    fn record_ids(self: KS) -> Span<felt252> {
        self.to_span().into_iter().map(|k| TID::record_id(k)).collect::<Array<_>>().span()
    }
}

pub impl RecordIdDatasImpl<
    RS, R, impl Struct: TableStructure, impl TID: RecordIdData<R, Struct>, +Spannable<RS, R>,
> of MultiIdData<RS, R, Struct> {
    fn multi_id_data(self: RS) -> Span<IdData> {
        self
            .to_span()
            .into_iter()
            .map(|r| TID::record_id_data(r.snapshot()))
            .collect::<Array<_>>()
            .span()
    }
}

impl TableIdDataSSImpl<
    R, impl Struct: TableStructure, impl TID: RecordIdData<R, Struct>,
> of RecordIdData<@R, Struct> {
    fn record_tuple(self: @@R) -> (felt252, Span<felt252>) {
        TID::record_tuple(*self)
    }
}

pub impl IdDataImpl<
    R,
    impl Struct: TableStructure,
    impl TID: RecordId<R, Struct>,
    impl TV: RecordValuesSpanTrait<Struct, Struct::Record>,
    +Snapable<@R, Struct::Record>,
> of RecordIdData<R, Struct> {
    fn record_tuple(self: @R) -> (felt252, Span<felt252>) {
        (TID::record_id(self), TV::serialize_values_inline(self.snapshot()))
    }
}

pub impl KeySpanToPrimaryTableIdDataImpl<
    impl Struct: TableStructure,
    impl SK: SerialisedRecordKey<Struct::Record, Struct>,
    impl KT: KeySpanToId<Struct::Record, Struct>,
    impl RV: RecordValuesSpanTrait<Struct, Struct::Record>,
> of RecordIdData<Struct::Record, Struct> {
    fn record_tuple(self: @Struct::Record) -> (felt252, Span<felt252>) {
        let mut data: Array<felt252> = Default::default();
        SK::serialize_record_key(self, ref data);
        let id = KT::key_span_to_id(data.span());
        RV::serialize_values(self, ref data);
        (id, data.span())
    }
}

pub impl IdDataTupleImpl<
    K,
    V,
    impl Struct: TableStructure,
    impl TID: RecordId<K, Struct>,
    impl TV: RecordValuesSpanTrait<Struct, Struct::Record>,
    +Snapable<@V, Struct::Record>,
> of RecordIdData<(K, V), Struct> {
    fn record_tuple(self: @(K, V)) -> (felt252, Span<felt252>) {
        let (key, value) = self;
        (TID::record_id(key), TV::serialize_values_inline(value.snapshot()))
    }
}

pub trait ColumnId<C, impl Struct: TableStructure> {
    const fn column_id(self: @C) -> felt252;
}

impl ColumnSSFelt252Id<
    C, impl Struct: TableStructure, impl SS: Snapable<@C, felt252>,
> of introspect_table::table::ColumnId<C, Struct> {
    const fn column_id(self: @C) -> felt252 {
        *SS::snapshot(self)
    }
}

trait ColumnIds<CS, impl Struct: TableStructure> {
    fn columns_ids(self: CS) -> Span<felt252>;
}

impl ColumnIdsImpl<
    C, CS, impl Struct: TableStructure, impl CID: ColumnId<C, Struct>, +Spannable<CS, C>,
> of ColumnIds<CS, Struct> {
    fn columns_ids(self: CS) -> Span<felt252> {
        self.to_span().into_iter().map(|c| CID::column_id(c)).collect::<Array<_>>().span()
    }
}


pub impl TableKeyIdImpl<
    K,
    impl Struct: TableStructure,
    impl SK: SerialisedKey<Struct::Record, K, Struct>,
    impl KP: KeySpanToId<Struct::Record, Struct>,
    +Drop<K>,
> of RecordId<K, Struct> {
    fn record_id(self: @K) -> felt252 {
        let mut data: Array<felt252> = Default::default();
        SK::serialize_key(self, ref data);
        KP::key_span_to_id(data.span())
    }
}

pub trait FieldOnlyColumnGroup<C, const SIZE: usize, impl Struct: TableStructure> {
    const GROUP_ID: felt252;
    const COLUMN_IDS: [felt252; SIZE];
    fn group_data(self: @C) -> Span<felt252>;
}


impl SSFieldOnlyColumnGroupImpl<
    C,
    const SIZE: usize,
    impl Struct: TableStructure,
    impl FOCG: FieldOnlyColumnGroup<C, SIZE, Struct>,
> of FieldOnlyColumnGroup<@C, SIZE, Struct> {
    const GROUP_ID: felt252 = FOCG::GROUP_ID;
    const COLUMN_IDS: [felt252; SIZE] = FOCG::COLUMN_IDS;
    fn group_data(self: @@C) -> Span<felt252> {
        FOCG::group_data(*self)
    }
}

pub trait IdColumnGroup<C, const SIZE: usize, impl Struct: TableStructure> {
    const GROUP_ID: felt252;
    const COLUMN_IDS: [felt252; SIZE];
    fn group_tuple(self: @C) -> (felt252, Span<felt252>);
}

impl SSIdColumnGroupImpl<
    C, const SIZE: usize, impl Struct: TableStructure, impl ICG: IdColumnGroup<C, SIZE, Struct>,
> of IdColumnGroup<@C, SIZE, Struct> {
    const GROUP_ID: felt252 = ICG::GROUP_ID;
    const COLUMN_IDS: [felt252; SIZE] = ICG::COLUMN_IDS;
    fn group_tuple(self: @@C) -> (felt252, Span<felt252>) {
        ICG::group_tuple(*self)
    }
}

pub impl FieldOnlyTupleColumnGroup<
    K,
    CG,
    const SIZE: usize,
    impl Struct: TableStructure,
    impl RID: RecordId<K, Struct>,
    impl FOCG: FieldOnlyColumnGroup<CG, SIZE, Struct>,
> of IdColumnGroup<(K, CG), SIZE> {
    const GROUP_ID: felt252 = FOCG::GROUP_ID;
    const COLUMN_IDS: [felt252; SIZE] = FOCG::COLUMN_IDS;
    fn group_tuple(self: @(K, CG)) -> (felt252, Span<felt252>) {
        let (id, scg): @(K, CG) = self;
        (RID::record_id(id), FOCG::group_data(scg))
    }
}


impl MultiIdColumnGroup<
    GS,
    G,
    impl Struct: TableStructure,
    const SIZE: usize,
    impl CG: IdColumnGroup<G, SIZE, Struct>,
    +Spannable<GS, G>,
> of MultiIdData<GS, G, Struct> {
    fn multi_id_data(self: GS) -> Span<IdData> {
        self.to_span().into_iter().map(|g| CG::group_tuple(g).into()).collect::<Array<_>>().span()
    }
}


pub trait RecordTrait<R, impl Struct: TableStructure> {}

impl RecordImpl<impl Struct: TableStructure> of RecordTrait<Struct::Record, Struct> {}
impl RecordSSImpl<R, impl Struct: TableStructure, +RecordTrait<R>> of RecordTrait<@R, Struct> {}

impl RecordTupleImpl<
    K,
    V,
    impl Struct: TableStructure,
    +Snapable<@K, felt252>,
    +Snapable<@V, Struct::Record>,
    -RecordId<Struct::Record>,
> of RecordTrait<(K, V), Struct> {}

trait RecordsField<
    const ID: felt252,
    KMS,
    impl Struct: TableStructure,
    impl M: MemberTrait<Struct::Record, Struct, ID>,
> {
    fn records_field_datas(self: KMS) -> Span<IdData>;
}

impl RecordsFieldImpl<
    KFS,
    KF,
    const ID: felt252,
    impl Struct: TableStructure,
    impl MT: MemberTrait<Struct::Record, Struct, ID>,
    impl IM: RecordField<ID, KF, Struct>,
    +Spannable<KFS, KF>,
> of RecordsField<ID, KFS, Struct, MT> {
    fn records_field_datas(self: KFS) -> Span<IdData> {
        self
            .to_span()
            .into_iter()
            .map(|M| IM::serialize_id_field(M).into())
            .collect::<Array<_>>()
            .span()
    }
}

pub trait MemberTrait<R, impl Struct: TableStructure, const ID: felt252> {
    type Type;
    fn serialize_member<F, +Snapable<F, Self::Type>, +Drop<F>>(self: F, ref data: Array<felt252>);
    fn serialize_member_inline<F, +Snapable<F, Self::Type>, +Drop<F>>(
        self: F,
    ) -> Span<
        felt252,
    > {
        let mut data: Array<felt252> = Default::default();
        Self::serialize_member(self, ref data);
        data.span()
    }
}


pub trait RecordField<
    const ID: felt252,
    KF,
    impl Struct: TableStructure,
    impl M: MemberTrait<Struct::Record, Struct, ID>,
> {
    fn serialize_id_field(self: @KF) -> (felt252, Span<felt252>);
}

pub impl RecordFieldImpl<
    const ID: felt252,
    KF,
    F,
    K,
    impl Struct: TableStructure,
    impl M: MemberTrait<Struct::Record, Struct, ID>,
    impl RID: RecordId<K, Struct>,
    +Snapable<@KF, (K, F)>,
    +Snapable<@F, M::Type>,
    +Drop<F>,
> of RecordField<ID, KF, Struct, M> {
    fn serialize_id_field(self: @KF) -> (felt252, Span<felt252>) {
        let (key, field) = Snapable::snapshot(self);
        (RID::record_id(key), M::serialize_member_inline(field))
    }
}

pub mod table_member {
    use introspect_types::ISerde;
    use crate::Snapable;
    pub impl Impl<
        impl Struct: super::TableStructure, const ID: felt252, M, +ISerde<M>,
    > of super::MemberTrait<Struct::Record, Struct, ID> {
        type Type = M;
        #[inline]
        fn serialize_member<F, +Snapable<F, Self::Type>, +Drop<F>>(
            self: F, ref data: Array<felt252>,
        ) {
            ISerde::<M>::iserialize(self.snapshot(), ref data);
        }
    }
}

pub trait RecordableEvent<R, impl Struct: TableStructure, const ID: felt252> {
    fn emit_recordable(record: @R);
}

pub trait RecordablesEvent<RS, impl Struct: TableStructure, const ID: felt252> {
    fn emit_recordables(records: RS);
}


pub impl EmitRecordableRecordImpl<
    R,
    impl Struct: TableStructure,
    const ID: felt252,
    impl RT: RecordTrait<R, Struct>,
    impl IDD: RecordIdData<R, Struct>,
    +Drop<R>,
> of RecordableEvent<R, Struct, ID> {
    fn emit_recordable(record: @R) {
        let id_data = IDD::record_id_data(record);
        InsertRecord { table: ID, record: id_data.id, data: id_data.data }.emit_event();
    }
}

pub impl EmitRecordableRecordsImpl<
    RS,
    R,
    impl Struct: TableStructure,
    const ID: felt252,
    impl IDD: RecordIdData<R, Struct>,
    +RecordTrait<R>,
    +Spannable<RS, R>,
    +Drop<RS>,
> of RecordablesEvent<RS, Struct, ID> {
    fn emit_recordables(records: RS) {
        let records_data = RecordIdDatasImpl::multi_id_data(records);
        InsertRecords { table: ID, records_data }.emit_event();
    }
}

pub impl ColumnGroupRecordable<
    R,
    impl Struct: TableStructure,
    const ID: felt252,
    const SIZE: usize,
    impl CG: IdColumnGroup<R, SIZE, Struct>,
    +Drop<R>,
> of RecordableEvent<R, Struct, ID> {
    fn emit_recordable(record: @R) {
        let (record, data) = CG::group_tuple(record);
        InsertFieldGroup { table: ID, record, group: CG::GROUP_ID, data }.emit_event();
    }
}

pub impl ColumnGroupRecordables<
    RS,
    R,
    impl Struct: TableStructure,
    const ID: felt252,
    const SIZE: usize,
    impl G: IdColumnGroup<R, SIZE>,
    +Spannable<RS, R>,
    +Drop<RS>,
> of RecordablesEvent<RS, Struct, ID> {
    fn emit_recordables(records: RS) {
        let records_data = MultiIdColumnGroup::multi_id_data(records);
        InsertsFieldGroup { table: ID, group: G::GROUP_ID, records_data }.emit_event();
    }
}

trait RecordFieldsEvent<R, impl Struct: TableStructure, const ID: felt252> {
    fn emit_record_fields(record_fields: @R);
}

trait RecordsFieldsEvent<RS, impl Struct: TableStructure, const ID: felt252> {
    fn emit_records_fields(records_fields: RS);
}

pub impl ColumnGroupRecordFields<
    R,
    impl Struct: TableStructure,
    const ID: felt252,
    const SIZE: usize,
    impl CG: IdColumnGroup<R, SIZE, Struct>,
    +Drop<R>,
    impl S: ToSpanTrait<[felt252; SIZE], felt252>,
> of RecordFieldsEvent<R, Struct, ID> {
    fn emit_record_fields(record_fields: @R) {
        let (record, data) = CG::group_tuple(record_fields);
        InsertFields { table: ID, record, columns: S::span(@CG::COLUMN_IDS), data }.emit_event();
    }
}

pub impl ColumnGroupRecordsFields<
    RS,
    R,
    const SIZE: usize,
    impl Struct: TableStructure,
    const ID: felt252,
    impl CG: IdColumnGroup<R, SIZE, Struct>,
    +Spannable<RS, R>,
    +Drop<RS>,
    impl S: ToSpanTrait<[felt252; SIZE], felt252>,
> of RecordsFieldsEvent<RS, Struct, ID> {
    fn emit_records_fields(records_fields: RS) {
        let records_data = MultiIdColumnGroup::multi_id_data(records_fields);
        InsertsFields { table: ID, columns: S::span(@CG::COLUMN_IDS), records_data }.emit_event();
    }
}


pub trait ITable {
    impl Struct: TableStructure;
    const ID: felt252;
    fn name() -> ByteArray;
    fn register_table() {
        CreateTableWithColumns {
            id: Self::ID,
            name: Self::name(),
            attributes: Self::Struct::attributes(),
            primary: Self::Struct::primary(),
            columns: Self::Struct::columns(),
        }
            .emit_event();
    }
    fn insert<R, impl RE: RecordableEvent<R, Self::Struct, Self::ID>, +Drop<R>>(
        record: R,
    ) {
        RE::emit_recordable(@record);
    }
    fn inserts<RS, impl RE: RecordablesEvent<RS, Self::Struct, Self::ID>, +Drop<RS>>(
        records: RS,
    ) {
        RE::emit_recordables(records);
    }
    fn insert_field<
        const ID: felt252,
        K,
        F,
        impl TID: RecordId<K, Self::Struct>,
        impl M: MemberTrait<Self::Struct::Record, Self::Struct, ID>,
        +Snapable<F, M::Type>,
        +Drop<K>,
        +Drop<F>,
    >(
        id: K, field: F,
    ) {
        InsertField {
            table: Self::ID,
            record: TID::record_id(@id),
            column: ID,
            data: M::serialize_member_inline(field),
        }
            .emit_event();
    }
    fn insert_fields<R, impl RE: RecordFieldsEvent<R, Self::Struct, Self::ID>, +Drop<R>>(
        record: R,
    ) {
        RE::emit_record_fields(@record);
    }
    fn inserts_field<
        const ID: felt252,
        RFS,
        impl M: MemberTrait<Self::Struct::Record, Self::Struct, ID>,
        impl RF: RecordsField<ID, RFS, Self::Struct, M>,
    >(
        id_fields: RFS,
    ) {
        let records_data = RF::records_field_datas(id_fields);
        InsertsField { table: Self::ID, column: ID, records_data }.emit_event();
    }
    fn inserts_fields<RS, impl RE: RecordsFieldsEvent<RS, Self::Struct, Self::ID>, +Drop<RS>>(
        records: RS,
    ) {
        RE::emit_records_fields(records);
    }
    fn delete_record<K, impl RID: RecordId<K, Self::Struct>, +Drop<K>>(
        id: K,
    ) {
        DeleteRecord { table: Self::ID, record: RID::record_id(@id) }.emit_event();
    }
    fn delete_records<KS, impl RID: RecordIds<KS, Self::Struct>, +Drop<KS>>(
        ids: KS,
    ) {
        DeleteRecords { table: Self::ID, records: RID::record_ids(ids) }.emit_event();
    }
    fn delete_field<
        K,
        C,
        impl RID: RecordId<K, Self::Struct>,
        impl CID: ColumnId<C, Self::Struct>,
        +Drop<K>,
        +Drop<C>,
    >(
        id: K, column: C,
    ) {
        DeleteField {
            table: Self::ID, record: RID::record_id(@id), column: CID::column_id(@column),
        }
            .emit_event();
    }
    fn delete_fields<
        K,
        CS,
        impl RID: RecordId<K, Self::Struct>,
        impl CID: ColumnIds<CS, Self::Struct>,
        +Drop<K>,
        +Drop<CS>,
    >(
        id: K, columns: CS,
    ) {
        DeleteFields {
            table: Self::ID, record: RID::record_id(@id), columns: CID::columns_ids(columns),
        }
            .emit_event();
    }
    fn deletes_field<
        KS, C, impl TID: RecordIds<KS, Self::Struct>, impl CID: ColumnId<C, Self::Struct>, +Drop<C>,
    >(
        ids: KS, column: C,
    ) {
        DeletesField {
            table: Self::ID, records: TID::record_ids(ids), column: CID::column_id(@column),
        }
            .emit_event();
    }
    fn deletes_fields<
        KS,
        CS,
        impl RID: RecordIds<KS, Self::Struct>,
        impl CID: ColumnIds<CS, Self::Struct>,
        +Drop<KS>,
        +Drop<CS>,
    >(
        ids: KS, columns: CS,
    ) {
        DeletesFields {
            table: Self::ID, records: RID::record_ids(ids), columns: CID::columns_ids(columns),
        }
            .emit_event();
    }
}
