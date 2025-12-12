use core::metaprogramming::TypeEqual;
use introspect_events::EmitEvent;
use introspect_events::database::{
    CreateTableWithColumns, DeleteField, DeleteFields, DeleteRecord, DeleteRecords, DeletesField,
    DeletesFields, InsertField, InsertFieldGroup, InsertFields, InsertRecord, InsertRecords,
    InsertsField, InsertsFieldGroup,
};
use introspect_types::{
    Attribute, ColumnDef, ISerde, IdData, PrimaryDef, PrimaryTrait, PrimaryTypeDef, TypeDef,
};
use crate::{Snapable, Spannable};

pub fn multi_key_primary_def() -> PrimaryDef {
    PrimaryDef { name: "__id", type_def: PrimaryTypeDef::Felt252, attributes: [].span() }
}

pub fn default_primary_def() -> PrimaryDef {
    PrimaryDef { name: "id", type_def: PrimaryTypeDef::Felt252, attributes: [].span() }
}


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
    type Column;
    fn columns() -> Span<ColumnDef>;
    fn child_defs() -> Array<(felt252, TypeDef)>;
}

pub trait KeySpanToPrimary<T, impl T: Table> {
    fn key_span_to_primary(self: Span<felt252>) -> T::Primary;
}

pub trait KeyToPrimary<T, impl T: Table> {
    type Key;
    fn key_to_primary(self: Self::Key) -> T::Primary;
}

pub trait RecordId<K, impl T: Table> {
    fn record_id(self: @K) -> felt252;
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

pub trait RecordIdDatas<RS, impl T: Table> {
    fn records_id_data(self: @RS) -> Span<IdData>;
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

impl RecordIdDatasImpl<
    RS,
    R,
    impl T: Table,
    impl TID: RecordIdData<R, T>,
    +IntoIterator<@RS>,
    +TypeEqual<IntoIterator::<@RS>::Iterator::Item, @R>,
    +Drop<IntoIterator::<@RS>::IntoIter>,
> of RecordIdDatas<RS, T> {
    fn records_id_data(self: @RS) -> Span<IdData> {
        self.into_iter().map(|r| TID::record_id_data(r)).collect::<Array<_>>().span()
    }
}

impl TableIdDataSSImpl<R, impl T: Table, impl TID: RecordIdData<R, T>> of RecordIdData<@R, T> {
    fn record_tuple(self: @@R) -> (felt252, Span<felt252>) {
        TID::record_tuple(*self)
    }
}

pub impl IdDataTupleImpl<
    KV,
    K,
    V,
    impl T: Table,
    impl TID: RecordId<K, T>,
    impl TV: RecordData<V, T>,
    impl SS: Snapable<@KV, (K, V)>,
> of RecordIdData<KV, T> {
    fn record_tuple(self: @KV) -> (felt252, Span<felt252>) {
        let (key, value) = SS::snapshot(self);
        (TID::record_id(key), TV::record_data(value))
    }
}

pub trait TableKeySpanTrait<R, K, impl T: Table> {
    fn serialize_keys(self: @K, ref keys: Array<felt252>);
}

pub trait KeySpanDataSpanTrait<R, impl T: Table> {
    fn serialize_keys(self: @R, ref keys: Array<felt252>);
    fn serialize_values(self: @R, ref values: Array<felt252>);
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


pub impl KeySpanToPrimaryTableIdDataImpl<
    R,
    impl T: Table,
    impl S: KeySpanDataSpanTrait<R, T>,
    impl KT: KeySpanToPrimary<T::Record, T>,
    -KeyToPrimary<T::Record, T>,
    +PrimaryTrait<T::Primary>,
    +Drop<T::Primary>,
> of RecordIdData<R, T> {
    fn record_tuple(self: @R) -> (felt252, Span<felt252>) {
        let mut data: Array<felt252> = Default::default();
        S::serialize_keys(self, ref data);
        let id = KT::key_span_to_primary(data.span()).to_felt252();
        S::serialize_values(self, ref data);
        (id, data.span())
    }
}


pub impl TableKeyIdImpl<
    K,
    impl T: Table,
    +TableKeySpanTrait<T::Record, K, T>,
    +KeySpanToPrimary<T::Record, T>,
    +Drop<K>,
    -KeyToPrimary<T::Record, T>,
    +PrimaryTrait<T::Primary>,
    +Drop<T::Primary>,
> of RecordId<K, T> {
    fn record_id(self: @K) -> felt252 {
        let mut data: Array<felt252> = Default::default();
        TableKeySpanTrait::<T::Record, K, T>::serialize_keys(self, ref data);
        KeySpanToPrimary::<T::Record, T>::key_span_to_primary(data.span()).to_felt252()
    }
}

trait KeyedMember<KM, impl T: Table> {
    fn keyed_member_id_data(self: KM) -> IdData;
}


// trait RecordsMember<R, KMS, impl T: Table> {
//     fn record_id_member_datas(self: @KMS) -> Span<IdData>;
// }

// pub impl RecordsMemberImpl<
//     const ID: felt252,
//     KMS,
//     K,
//     M,
//     impl T: Table,
//     impl RID: RecordId<K, T>,
//     +MemberTrait<T::Record, T, M, ID>,
//     +Spannable<KMS, (K, M)>,
// > of RecordsMember<T::Record, KMS, T> {
//     fn record_id_member_datas(self: @KMS) -> Span<IdData> {
//         RecordIdDatasImpl::<T, (K, M), KMS, _, II, TE>::records_id_data(self)
//     }
// }

pub trait ColumnGroupTrait<C, const SIZE: usize> {
    const GROUP_ID: felt252;
    const COLUMN_IDS: [felt252; SIZE];
}

impl ColumnGroupSSImpl<
    C, const SIZE: usize, +ColumnGroupTrait<C, SIZE>,
> of ColumnGroupTrait<@C, SIZE> {
    const GROUP_ID: felt252 = ColumnGroupTrait::<C>::GROUP_ID;
    const COLUMN_IDS: [felt252; SIZE] = ColumnGroupTrait::<C>::COLUMN_IDS;
}

// pub trait KeyTrait<T, K, const SIZE: usize, const KEYS: [felt252; SIZE]> {}

pub trait RecordTrait<R, impl T: Table> {}

impl RecordImpl<impl T: Table> of RecordTrait<T::Record, T> {}
impl RecordSSImpl<impl T: Table> of RecordTrait<@T::Record, T> {}


pub trait RecordableEvent<R, impl T: Table> {
    fn emit_recordable(self: @R);
    fn emit_recordables<RS, +Spannable<RS, R>, +Drop<RS>>(records: RS);
}


// pub trait TableMember<impl T: Table, const ID: felt252> {
//     type Type;
//     fn serialize_member(self: @Self::Type, ref data: Array<felt252>);
// }

pub trait MemberTrait<R, impl T: Table, const ID: felt252> {
    type Type;
    fn serialize_member(self: @Self::Type, ref data: Array<felt252>);
    fn serialize_member_inline(
        self: @Self::Type,
    ) -> Span<
        felt252,
    > {
        let mut data: Array<felt252> = Default::default();
        Self::serialize_member(self, ref data);
        data.span()
    }
}

pub trait TypedMember<impl T: Table, const ID: felt252, M> {
    fn serialize_member(self: @M, ref data: Array<felt252>);
}


impl TypedMemberImpl<
    impl T: Table, const ID: felt252, impl M: MemberTrait<T::Record, T, ID>,
> of TypedMember<T, ID, M::Type> {
    fn serialize_member(self: @M::Type, ref data: Array<felt252>) {
        M::serialize_member(self, ref data);
    }
}

// impl MemberTraitImpl<
//     impl T: Table, const ID: felt252, M, +TypedMember<T, ID, M>, +ISerde<M>,
// > of MemberTrait<T::Record, T, ID> {
//     type Type = M;
//     fn serialize_member(self: @M, ref data: Array<felt252>) {
//         // TM::serialize_member(self, ref data);
//         introspect_types::serde::ISerde::iserialize(self, ref data);
//     }
// }

// impl TableMemberImpl<
//     impl T: Table, const ID: felt252, M, impl TM: TableMember<T, ID, M>,
// > of MemberTrait<T::Record, T, ID> {
//     fn serialize_member(self: @M, ref data: Array<felt252>) {
//         TM::serialize_member(self, ref data);
//     }
// }

// impl TableMemberSSImpl<
//     impl T: Table, const ID: felt252, M, impl TM: TableMember<T, ID, M>,
// > of MemberTrait<T::Record, T, @M, ID> {
//     fn serialize_member(self: @@M, ref data: Array<felt252>) {
//         TM::serialize_member(*self, ref data);
//     }
// }
pub mod iserde_table_member {
    use introspect_types::ISerde;
    pub impl Impl<
        impl T: super::Table, const ID: felt252, M, +ISerde<M>,
    > of super::MemberTrait<T::Record, T, ID> {
        type Type = M;
        fn serialize_member(self: @Self::Type, ref data: Array<felt252>) {
            ISerde::iserialize(self, ref data);
        }
    }
}

// impl MemberSSImpl<
//     R, impl T: Table, M, const ID: felt252, impl SM: MemberTrait<R, T, M, ID>,

// > of MemberTrait<R, T, @M, ID> {
//     fn serialize_member(self: @@M, ref data: Array<felt252>) {
//         SM::serialize_member(*self, ref data);
//     }
// }

impl MemberTableDataImpl<
    const ID: felt252,
    impl T: Table,
    M,
    impl MT: MemberTrait<T::Record, T, ID>,
    impl S: Snapable<@M, MT::Type>,
> of RecordData<M, T> {
    fn record_data(self: @M) -> Span<felt252> {
        MT::serialize_member_inline(S::snapshot(self))
    }
}

pub impl EmitRecordableRecordImpl<
    R, impl T: Table, impl IDD: RecordIdData<R, T>, +Drop<R>, +RecordTrait<R>,
> of RecordableEvent<R, T> {
    fn emit_recordable(self: @R) {
        let id_data = IDD::record_id_data(self);
        InsertRecord { table: T::ID, record: id_data.id, data: id_data.data }.emit_event();
    }
    fn emit_recordables<RS, +Spannable<RS, R>, +Drop<RS>>(records: RS) {
        let records_data = RecordIdDatasImpl::<
            Span<R>, R, T, IDD,
        >::records_id_data(@records.to_span());
        InsertRecords { table: T::ID, records_data }.emit_event();
    }
}


pub impl ColumnGroupRecordable<
    R,
    const SIZE: usize,
    impl T: Table,
    impl G: ColumnGroupTrait<R, SIZE>,
    impl IDD: RecordIdData<R, T>,
    impl SS: Snapable<@R, R>,
    +Drop<R>,
> of RecordableEvent<R, T> {
    fn emit_recordable(self: @R) {
        let (record, data) = IDD::record_tuple(SS::snapshot(self));
        InsertFieldGroup { table: T::ID, record, group: G::GROUP_ID, data }.emit_event();
    }
    fn emit_recordables<RS, +Spannable<RS, R>, +Drop<RS>>(records: RS) {
        let records_data = RecordIdDatasImpl::<
            Span<R>, R, T, IDD,
        >::records_id_data(@records.to_span());
        InsertsFieldGroup { table: T::ID, group: G::GROUP_ID, records_data }.emit_event();
    }
}

// pub impl ColumnGroupRecordableTupleImpl<
//     KV,
//     K,
//     V,
//     const SIZE: usize,
//     impl T: Table,
//     impl G: ColumnGroupTrait<V, SIZE>,
//     impl IDD: TableIdData<(K, V), T>,
//     +Drop<K>,
//     +Drop<V>,
// > of RecordableEvent<KV, T> {
//     fn emit_recordable(self: @KV) {
//         let (record, data) = IDD::record_tuple(@self);
//         InsertFieldGroup { table: T::ID, record, group: G::GROUP_ID, data }.emit_event();
//     }
//     fn emit_recordables<RS, +Spannable<RS, (K, V)>, +Drop<RS>>(records: RS) {
//         let records_data = IDD::records_id_data(records.to_span());
//         InsertsFieldGroup { table: T::ID, group: G::GROUP_ID, records_data }.emit_event();
//     }
// }

pub trait Table {
    type Primary;
    type Column;
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
    type Column;
    const ID: felt252;
    fn register_table();
    fn insert<R, +RecordableEvent<R, Self::T>, +Snapable<@R, R>, +Drop<R>>(record: R);
    fn inserts<RS, R, +Spannable<RS, R>, +RecordableEvent<R, Self::T>, +Drop<RS>>(records: RS);
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
    // fn insert_fields<K, +TableId<K, Self::T>, +Drop<K>>(id: K, fields: Span<Self::Field>);
    fn inserts_field<
        const ID: felt252,
        KMS,
        K,
        F,
        +RecordId<K, Self::T>,
        impl M: MemberTrait<Self::T::Record, Self::T, ID>,
        +Spannable<KMS, (K, F)>,
        +Snapable<@F, M::Type>,
        +Drop<KMS>,
    >(
        id_fields: KMS,
    );
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
    type Column = Columns::Column;
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

pub impl ITableImpl<impl T: Table, +ColumnId<T::Column>, +Drop<T::Column>> of ITable {
    type Column = T::Column;
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
    fn insert<R, impl RE: RecordableEvent<R, T>, impl SS: Snapable<@R, R>, +Drop<R>>(record: R) {
        RE::emit_recordable(SS::snapshot(@record));
    }
    fn inserts<RS, R, +Spannable<RS, R>, impl RE: RecordableEvent<R, T>, +Drop<RS>>(records: RS) {
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
            data: M::serialize_member_inline(field.snapshot()),
        }
            .emit_event();
    }

    // fn insert_fields<K, +TableId<K, Self::T>, +Drop<K>>(id: K, fields: Span<Self::Field>) {
    //     let columns = fields.into_iter().map(|f| f.column_id()).collect::<Array<_>>().span();
    //     let mut data: Array<felt252> = Default::default();
    //     for field in fields {
    //         data.append_span(field.data());
    //     }
    //     InsertFields { table: T::ID, record: id.id(), columns, data: data.span() }.emit_event();
    // }

    fn inserts_field<
        const ID: felt252,
        KMS,
        K,
        F,
        KF,
        +RecordId<K, Self::T>,
        impl M: MemberTrait<Self::T::Record, Self::T, ID>,
        +Spannable<KMS, KF>,
        +Snapable<@KF, (K, F)>,
        +Snapable<@F, M::Type>,
        +Drop<KMS>,
    >(
        id_fields: KMS,
    ) {
        let records_data = RecordIdDatasImpl::<
            Span<KF>, KF, T,
        >::records_id_data(@id_fields.to_span());
        InsertsField { table: T::ID, column: ID, records_data }.emit_event();
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
