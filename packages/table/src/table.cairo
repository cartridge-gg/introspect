use introspect_events::EmitEvent;
use introspect_events::database::{
    CreateTableWithColumns, DeleteField, DeleteFields, DeleteRecord, DeleteRecords, DeletesField,
    DeletesFields, InsertField, InsertFieldGroup, InsertFields, InsertRecord, InsertRecords,
    InsertsField, InsertsFieldGroup, InsertsFields,
};
use introspect_types::{Attribute, ColumnDef, IdData, PrimaryDef, PrimaryTrait, TypeDef};
use crate::{Snapable, Spannable};


pub trait TablePrimary {
    type Primary;
    fn primary_def() -> PrimaryDef;
}

pub trait TableMeta {
    const ID: felt252;
    fn name() -> ByteArray;
    fn attributes() -> Span<Attribute>;
}

pub trait TableColumns {
    fn columns() -> Span<ColumnDef>;
    fn child_defs() -> Array<(felt252, TypeDef)>;
}

pub mod table_primary {
    use introspect_types::{PrimaryDef, PrimaryTypeDef};
    pub impl Default of super::TablePrimary {
        type Primary = felt252;
        fn primary_def() -> introspect_types::PrimaryDef {
            PrimaryDef { name: "__id", type_def: PrimaryTypeDef::Felt252, attributes: [].span() }
        }
    }
}

pub trait KeySpanToPrimary<R, impl T: Table> {
    fn key_span_to_primary(self: Span<felt252>) -> T::Primary;
}

pub trait KeySpanToId<R, impl T: Table> {
    fn key_span_to_id(self: Span<felt252>) -> felt252;
}

impl KeySpanToIdImpl<
    K,
    impl T: Table,
    impl KP: KeySpanToPrimary<T::Record, T>,
    +PrimaryTrait<T::Primary>,
    +Drop<T::Primary>,
> of KeySpanToId<K, T> {
    fn key_span_to_id(self: Span<felt252>) -> felt252 {
        KP::key_span_to_primary(self).to_felt252()
    }
}

pub trait RecordId<K, impl T: Table> {
    fn record_id(self: @K) -> felt252;
}

pub mod record_id_felt252 {
    pub impl Impl<impl T: super::Table> of super::RecordId<felt252, T> {
        fn record_id(self: @felt252) -> felt252 {
            *self
        }
    }
}

pub trait RecordIds<KS, impl T: Table> {
    fn record_ids(self: KS) -> Span<felt252>;
}

pub trait RecordData<V, impl T: Table> {
    fn record_data(self: @V) -> Span<felt252>;
}

pub trait RecordIdData<R, impl T: Table> {
    fn record_tuple(self: @R) -> (felt252, Span<felt252>);
    fn record_id_data(self: @R) -> IdData {
        Self::record_tuple(self).into()
    }
}

pub trait MultiIdData<RS, R, impl T: Table> {
    fn multi_id_data(self: RS) -> Span<IdData>;
}

pub trait SerialisedKey<R, K, impl T: Table> {
    fn serialize_key(self: @K, ref data: Array<felt252>);
}

pub trait RecordKey<R, K, impl T: Table> {
    type Key;
    fn record_key(self: @R) -> K;
}
pub trait SerialisedRecordKey<R, impl T: Table> {
    fn serialize_record_key(self: @R, ref data: Array<felt252>);
}

impl SerialisedRecordKeyImpl<
    SR,
    K,
    impl T: Table,
    impl RK: RecordKey<T::Record, K, T>,
    impl SK: SerialisedKey<T::Record, K, T>,
    impl SS: Snapable<@SR, T::Record>,
    +Drop<SR>,
    +Drop<K>,
> of SerialisedRecordKey<SR, T> {
    fn serialize_record_key(self: @SR, ref data: Array<felt252>) {
        let key = RK::record_key(SS::snapshot(self));
        SK::serialize_key(@key, ref data);
    }
}

pub trait RecordValuesSpanTrait<R, impl T: Table> {
    fn serialize_values(self: @R, ref data: Array<felt252>);
    fn serialize_values_inline(
        self: @R,
    ) -> Span<
        felt252,
    > {
        let mut data: Array<felt252> = Default::default();
        Self::serialize_values(self, ref data);
        data.span()
    }
}


pub impl TablePrimaryIdImpl<
    K, impl T: Table, +PrimaryTrait<T::Primary>, impl SS: Snapable<@K, T::Primary>,
> of RecordId<K, T> {
    fn record_id(self: @K) -> felt252 {
        SS::snapshot(self).to_felt252()
    }
}

pub impl RecordIdsImpl<
    KS, K, impl T: Table, impl TID: RecordId<K, T>, +Spannable<KS, K>,
> of RecordIds<KS, T> {
    fn record_ids(self: KS) -> Span<felt252> {
        self.to_span().into_iter().map(|k| TID::record_id(k)).collect::<Array<_>>().span()
    }
}

pub impl RecordIdDatasImpl<
    RS, R, impl T: Table, impl TID: RecordIdData<R, T>, +Spannable<RS, R>,
> of MultiIdData<RS, R, T> {
    fn multi_id_data(self: RS) -> Span<IdData> {
        self
            .to_span()
            .into_iter()
            .map(|r| TID::record_id_data(r.snapshot()))
            .collect::<Array<_>>()
            .span()
    }
}

impl TableIdDataSSImpl<R, impl T: Table, impl TID: RecordIdData<R, T>> of RecordIdData<@R, T> {
    fn record_tuple(self: @@R) -> (felt252, Span<felt252>) {
        TID::record_tuple(*self)
    }
}

pub impl IdDataImpl<
    R, impl T: Table, impl TID: RecordId<R, T>, impl TV: RecordValuesSpanTrait<R, T>,
> of RecordIdData<R, T> {
    fn record_tuple(self: @R) -> (felt252, Span<felt252>) {
        (TID::record_id(self), TV::serialize_values_inline(self))
    }
}

pub impl KeySpanToPrimaryTableIdDataImpl<
    impl T: Table,
    impl SK: SerialisedRecordKey<T::Record, T>,
    impl KT: KeySpanToId<T::Record, T>,
    impl RV: RecordValuesSpanTrait<T::Record, T>,
> of RecordIdData<T::Record, T> {
    fn record_tuple(self: @T::Record) -> (felt252, Span<felt252>) {
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
    impl T: Table,
    impl TID: RecordId<K, T>,
    impl TV: RecordValuesSpanTrait<T::Record, T>,
    +Snapable<@V, T::Record>,
> of RecordIdData<(K, V), T> {
    fn record_tuple(self: @(K, V)) -> (felt252, Span<felt252>) {
        let (key, value) = self;
        (TID::record_id(key), TV::serialize_values_inline(value.snapshot()))
    }
}

pub trait ColumnId<C, impl T: Table> {
    const fn column_id(self: @C) -> felt252;
}

impl ColumnSSFelt252Id<
    C, impl T: Table, impl SS: Snapable<@C, felt252>,
> of introspect_table::table::ColumnId<C, T> {
    const fn column_id(self: @C) -> felt252 {
        *SS::snapshot(self)
    }
}

trait ColumnIds<CS, impl T: Table> {
    fn columns_ids(self: CS) -> Span<felt252>;
}

impl ColumnIdsImpl<
    C, CS, impl T: Table, impl CID: ColumnId<C, T>, +Spannable<CS, C>,
> of ColumnIds<CS, T> {
    fn columns_ids(self: CS) -> Span<felt252> {
        self.to_span().into_iter().map(|c| CID::column_id(c)).collect::<Array<_>>().span()
    }
}


pub impl TableKeyIdImpl<
    K,
    impl T: Table,
    impl SK: SerialisedKey<T::Record, K, T>,
    impl KP: KeySpanToId<T::Record, T>,
    +Drop<K>,
> of RecordId<K, T> {
    fn record_id(self: @K) -> felt252 {
        let mut data: Array<felt252> = Default::default();
        SK::serialize_key(self, ref data);
        KP::key_span_to_id(data.span())
    }
}

pub trait FieldOnlyColumnGroup<C, const SIZE: usize, impl T: Table> {
    const GROUP_ID: felt252;
    const COLUMN_IDS: [felt252; SIZE];
    fn group_data(self: @C) -> Span<felt252>;
}


impl SSFieldOnlyColumnGroupImpl<
    C, const SIZE: usize, impl T: Table, impl FOCG: FieldOnlyColumnGroup<C, SIZE, T>,
> of FieldOnlyColumnGroup<@C, SIZE, T> {
    const GROUP_ID: felt252 = FOCG::GROUP_ID;
    const COLUMN_IDS: [felt252; SIZE] = FOCG::COLUMN_IDS;
    fn group_data(self: @@C) -> Span<felt252> {
        FOCG::group_data(*self)
    }
}

pub trait IdColumnGroup<C, const SIZE: usize, impl T: Table> {
    const GROUP_ID: felt252;
    const COLUMN_IDS: [felt252; SIZE];
    fn group_tuple(self: @C) -> (felt252, Span<felt252>);
}

impl SSIdColumnGroupImpl<
    C, const SIZE: usize, impl T: Table, impl ICG: IdColumnGroup<C, SIZE, T>,
> of IdColumnGroup<@C, SIZE, T> {
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
    impl T: Table,
    impl RID: RecordId<K, T>,
    impl FOCG: FieldOnlyColumnGroup<CG, SIZE, T>,
> of IdColumnGroup<(K, CG), SIZE> {
    const GROUP_ID: felt252 = FOCG::GROUP_ID;
    const COLUMN_IDS: [felt252; SIZE] = FOCG::COLUMN_IDS;
    fn group_tuple(self: @(K, CG)) -> (felt252, Span<felt252>) {
        let (id, scg): @(K, CG) = self;
        (RID::record_id(id), FOCG::group_data(scg))
    }
}


impl MultiIdColumnGroup<
    GS, G, impl T: Table, const SIZE: usize, impl CG: IdColumnGroup<G, SIZE, T>, +Spannable<GS, G>,
> of MultiIdData<GS, G, T> {
    fn multi_id_data(self: GS) -> Span<IdData> {
        self.to_span().into_iter().map(|g| CG::group_tuple(g).into()).collect::<Array<_>>().span()
    }
}

pub trait RecordTrait<R, impl T: Table> {}

impl RecordImpl<impl T: Table> of RecordTrait<T::Record, T> {}
impl RecordSSImpl<R, impl T: Table, +RecordTrait<R>> of RecordTrait<@R, T> {}

impl RecordTupleImpl<
    K, V, impl T: Table, +Snapable<@K, felt252>, +Snapable<@V, T::Record>, -RecordId<T::Record>,
> of RecordTrait<(K, V), T> {}


trait RecordsField<const ID: felt252, KMS, impl T: Table, impl M: MemberTrait<T::Record, T, ID>> {
    fn records_field_datas(self: KMS) -> Span<IdData>;
}

impl RecordsFieldImpl<
    KFS,
    KF,
    const ID: felt252,
    impl T: Table,
    impl MT: MemberTrait<T::Record, T, ID>,
    impl IM: RecordField<ID, KF, T>,
    +Spannable<KFS, KF>,
> of RecordsField<ID, KFS, T, MT> {
    fn records_field_datas(self: KFS) -> Span<IdData> {
        self
            .to_span()
            .into_iter()
            .map(|M| IM::serialize_id_field(M).into())
            .collect::<Array<_>>()
            .span()
    }
}

pub trait MemberTrait<R, impl T: Table, const ID: felt252> {
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


pub trait RecordField<const ID: felt252, KF, impl T: Table, impl M: MemberTrait<T::Record, T, ID>> {
    fn serialize_id_field(self: @KF) -> (felt252, Span<felt252>);
}

pub impl RecordFieldImpl<
    const ID: felt252,
    KF,
    F,
    K,
    impl T: Table,
    impl M: MemberTrait<T::Record, T, ID>,
    impl RID: RecordId<K, T>,
    +Snapable<@KF, (K, F)>,
    +Snapable<@F, M::Type>,
    +Drop<F>,
> of RecordField<ID, KF, T, M> {
    fn serialize_id_field(self: @KF) -> (felt252, Span<felt252>) {
        let (key, field) = Snapable::snapshot(self);
        (RID::record_id(key), M::serialize_member_inline(field))
    }
}

pub mod table_member {
    use introspect_types::ISerde;
    use crate::Snapable;
    pub impl Impl<
        impl T: super::Table, const ID: felt252, M, +ISerde<M>,
    > of super::MemberTrait<T::Record, T, ID> {
        type Type = M;
        fn serialize_member<F, +Snapable<F, Self::Type>, +Drop<F>>(
            self: F, ref data: Array<felt252>,
        ) {
            ISerde::<M>::iserialize(self.snapshot(), ref data);
        }
    }
}

pub trait RecordableEvent<R, impl T: Table> {
    fn emit_recordable(record: @R);
}

pub trait RecordablesEvent<RS, impl T: Table> {
    fn emit_recordables(records: RS);
}


pub impl EmitRecordableRecordImpl<
    R, impl T: Table, impl RT: RecordTrait<R, T>, impl IDD: RecordIdData<R, T>, +Drop<R>,
> of RecordableEvent<R, T> {
    fn emit_recordable(record: @R) {
        let id_data = IDD::record_id_data(record);
        InsertRecord { table: T::ID, record: id_data.id, data: id_data.data }.emit_event();
    }
}

pub impl EmitRecordableRecordsImpl<
    RS,
    R,
    impl T: Table,
    impl IDD: RecordIdData<R, T>,
    +RecordTrait<R>,
    +Spannable<RS, R>,
    +Drop<RS>,
> of RecordablesEvent<RS, T> {
    fn emit_recordables(records: RS) {
        let records_data = RecordIdDatasImpl::multi_id_data(records);
        InsertRecords { table: T::ID, records_data }.emit_event();
    }
}

pub impl ColumnGroupRecordable<
    R, const SIZE: usize, impl T: Table, impl CG: IdColumnGroup<R, SIZE, T>, +Drop<R>,
> of RecordableEvent<R, T> {
    fn emit_recordable(record: @R) {
        let (record, data) = CG::group_tuple(record);
        InsertFieldGroup { table: T::ID, record, group: CG::GROUP_ID, data }.emit_event();
    }
}

pub impl ColumnGroupRecordables<
    RS,
    R,
    const SIZE: usize,
    impl T: Table,
    impl G: IdColumnGroup<R, SIZE>,
    +Spannable<RS, R>,
    +Drop<RS>,
> of RecordablesEvent<RS, T> {
    fn emit_recordables(records: RS) {
        let records_data = MultiIdColumnGroup::multi_id_data(records);
        InsertsFieldGroup { table: T::ID, group: G::GROUP_ID, records_data }.emit_event();
    }
}

trait RecordFieldsEvent<R, impl T: Table> {
    fn emit_record_fields(record_fields: @R);
}

trait RecordsFieldsEvent<RS, impl T: Table> {
    fn emit_records_fields(records_fields: RS);
}

pub impl ColumnGroupRecordFields<
    R,
    const SIZE: usize,
    impl T: Table,
    impl CG: IdColumnGroup<R, SIZE, T>,
    +Drop<R>,
    impl S: ToSpanTrait<[felt252; SIZE], felt252>,
> of RecordFieldsEvent<R, T> {
    fn emit_record_fields(record_fields: @R) {
        let (record, data) = CG::group_tuple(record_fields);
        InsertFields { table: T::ID, record, columns: S::span(@CG::COLUMN_IDS), data }.emit_event();
    }
}

pub impl ColumnGroupRecordsFields<
    RS,
    R,
    const SIZE: usize,
    impl T: Table,
    impl CG: IdColumnGroup<R, SIZE>,
    +Spannable<RS, R>,
    +Drop<RS>,
    impl S: ToSpanTrait<[felt252; SIZE], felt252>,
> of RecordsFieldsEvent<RS, T> {
    fn emit_records_fields(records_fields: RS) {
        let records_data = MultiIdColumnGroup::multi_id_data(records_fields);
        InsertsFields { table: T::ID, columns: S::span(@CG::COLUMN_IDS), records_data }
            .emit_event();
    }
}

pub trait Table {
    type Primary;
    type Record;
    const ID: felt252;
    fn name() -> ByteArray;
    fn attributes() -> Span<Attribute>;
    fn primary() -> PrimaryDef;
    fn columns() -> Span<ColumnDef>;
    fn child_defs() -> Array<(felt252, TypeDef)>;
}


pub trait ITable {
    impl T: Table;
    const ID: felt252;
    fn register_table();
    fn insert<R, +RecordableEvent<R, Self::T>, +Drop<R>>(record: R);
    fn inserts<RS, +RecordablesEvent<RS, Self::T>, +Drop<RS>>(records: RS);
    fn insert_field<
        const ID: felt252,
        K,
        F,
        +RecordId<K, Self::T>,
        impl M: MemberTrait<Self::T::Record, Self::T, ID>,
        +Snapable<F, M::Type>,
        +Drop<K>,
        +Drop<F>,
    >(
        id: K, field: F,
    );
    fn insert_fields<R, +RecordFieldsEvent<R, Self::T>, +Drop<R>>(record: R);
    fn inserts_field<
        const ID: felt252,
        RFS,
        impl M: MemberTrait<Self::T::Record, Self::T, ID>,
        +RecordsField<ID, RFS, Self::T, M>,
    >(
        id_fields: RFS,
    );
    fn inserts_fields<RS, +RecordsFieldsEvent<RS, Self::T>, +Drop<RS>>(records: RS);
    fn delete_record<K, +RecordId<K, Self::T>, +Drop<K>>(id: K);
    fn delete_records<KS, +RecordIds<KS, Self::T>, +Drop<KS>>(ids: KS);
    fn delete_field<K, C, +RecordId<K, Self::T>, +ColumnId<C, Self::T>, +Drop<K>, +Drop<C>>(
        id: K, column: C,
    );
    fn delete_fields<K, CS, +RecordId<K, Self::T>, +ColumnIds<CS, Self::T>, +Drop<K>, +Drop<CS>>(
        id: K, columns: CS,
    );
    fn deletes_field<KS, C, impl TID: RecordIds<KS, Self::T>, +ColumnId<C, Self::T>, +Drop<C>>(
        ids: KS, column: C,
    );
    fn deletes_fields<
        KS, CS, +RecordIds<KS, Self::T>, +ColumnIds<CS, Self::T>, +Drop<KS>, +Drop<CS>,
    >(
        ids: KS, columns: CS,
    );
}


pub impl TableImpl<
    Record, impl Meta: TableMeta, impl Primary: TablePrimary, impl Columns: TableColumns,
> of Table {
    type Primary = Primary::Primary;
    type Record = Record;
    const ID: felt252 = Meta::ID;
    fn name() -> ByteArray {
        Meta::name()
    }
    fn attributes() -> Span<Attribute> {
        Meta::attributes()
    }
    fn primary() -> PrimaryDef {
        Primary::primary_def()
    }
    fn columns() -> Span<ColumnDef> {
        Columns::columns()
    }
    fn child_defs() -> Array<(felt252, TypeDef)> {
        Columns::child_defs()
    }
}

pub impl ITableImpl<impl T: Table> of ITable {
    impl T = T;
    const ID: felt252 = T::ID;
    fn register_table() {
        CreateTableWithColumns {
            id: T::ID,
            name: T::name(),
            attributes: T::attributes(),
            primary: T::primary(),
            columns: T::columns(),
        }
            .emit_event();
    }
    fn insert<R, impl RE: RecordableEvent<R, T>, +Drop<R>>(record: R) {
        RE::emit_recordable(@record);
    }
    fn inserts<RS, impl RE: RecordablesEvent<RS, T>, +Drop<RS>>(records: RS) {
        RE::emit_recordables(records);
    }
    fn insert_field<
        const ID: felt252,
        K,
        F,
        impl TID: RecordId<K, Self::T>,
        impl M: MemberTrait<Self::T::Record, Self::T, ID>,
        +Snapable<F, M::Type>,
        +Drop<K>,
        +Drop<F>,
    >(
        id: K, field: F,
    ) {
        InsertField {
            table: T::ID,
            record: TID::record_id(@id),
            column: ID,
            data: M::serialize_member_inline(field),
        }
            .emit_event();
    }

    fn insert_fields<R, impl RE: RecordFieldsEvent<R, T>, +Drop<R>>(record: R) {
        RE::emit_record_fields(@record);
    }

    fn inserts_field<
        const ID: felt252,
        RFS,
        impl M: MemberTrait<Self::T::Record, Self::T, ID>,
        impl RF: RecordsField<ID, RFS, Self::T, M>,
    >(
        id_fields: RFS,
    ) {
        let records_data = RF::records_field_datas(id_fields);
        InsertsField { table: T::ID, column: ID, records_data }.emit_event();
    }
    fn inserts_fields<RS, impl RE: RecordsFieldsEvent<RS, T>, +Drop<RS>>(records: RS) {
        RE::emit_records_fields(records);
    }
    fn delete_record<K, impl TID: RecordId<K, Self::T>, +Drop<K>>(id: K) {
        DeleteRecord { table: T::ID, record: TID::record_id(@id) }.emit_event();
    }
    fn delete_records<KS, impl TID: RecordIds<KS, Self::T>, +Drop<KS>>(ids: KS) {
        DeleteRecords { table: T::ID, records: TID::record_ids(ids) }.emit_event();
    }
    fn delete_field<
        K, C, impl TID: RecordId<K, Self::T>, impl CID: ColumnId<C, Self::T>, +Drop<K>, +Drop<C>,
    >(
        id: K, column: C,
    ) {
        DeleteField { table: T::ID, record: TID::record_id(@id), column: CID::column_id(@column) }
            .emit_event();
    }

    fn delete_fields<
        K,
        CS,
        impl TID: RecordId<K, Self::T>,
        impl CID: ColumnIds<CS, Self::T>,
        +Drop<K>,
        +Drop<CS>,
    >(
        id: K, columns: CS,
    ) {
        DeleteFields {
            table: T::ID, record: TID::record_id(@id), columns: CID::columns_ids(columns),
        }
            .emit_event();
    }
    fn deletes_field<
        KS, C, impl TID: RecordIds<KS, Self::T>, impl CID: ColumnId<C, Self::T>, +Drop<C>,
    >(
        ids: KS, column: C,
    ) {
        DeletesField {
            table: T::ID, records: TID::record_ids(ids), column: CID::column_id(@column),
        }
            .emit_event();
    }
    fn deletes_fields<
        KS,
        CS,
        impl TID: RecordIds<KS, Self::T>,
        impl CID: ColumnIds<CS, Self::T>,
        +Drop<KS>,
        +Drop<CS>,
    >(
        ids: KS, columns: CS,
    ) {
        DeletesFields {
            table: T::ID, records: TID::record_ids(ids), columns: CID::columns_ids(columns),
        }
            .emit_event();
    }
}
