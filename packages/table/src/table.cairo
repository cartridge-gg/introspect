use introspect_events::EmitEvent;
use introspect_events::database::{
    CreateTableWithColumns, DeleteField, DeleteFields, DeleteRecord, DeleteRecords, DeletesField,
    DeletesFields, InsertField, InsertFieldGroup, InsertFields, InsertRecord, InsertRecords,
    InsertsField, InsertsFieldGroup,
};
use introspect_types::{
    Attribute, ColumnDef, IdData, PrimaryDef, PrimaryTrait, PrimaryTypeDef, TypeDef,
};

pub fn multi_key_primary_def() -> PrimaryDef {
    PrimaryDef { name: "__id", type_def: PrimaryTypeDef::Felt252, attributes: [].span() }
}

pub fn default_primary_def() -> PrimaryDef {
    PrimaryDef { name: "id", type_def: PrimaryTypeDef::Felt252, attributes: [].span() }
}


pub trait AToSpanTrait<C, T> {
    fn to_span(self: C) -> Span<T>;
}


impl AToSpanImpl<C, T, +ToSpanTrait<C, T>, +Drop<C>> of AToSpanTrait<C, T> {
    fn to_span(self: C) -> Span<T> {
        self.span()
    }
}

impl ASPanToSpan<T> of AToSpanTrait<Span<T>, T> {
    fn to_span(self: Span<T>) -> Span<T> {
        self
    }
}

pub trait ColumnTrait<T> {
    fn column_id(self: @T) -> felt252;
}

pub trait TableKey<T, const COLUMNS: usize> {
    type Key;
    const KEY_IDS: [felt252; COLUMNS];
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
    type Field;
    fn columns() -> Span<ColumnDef>;
    fn child_defs() -> Array<(felt252, TypeDef)>;
}

pub trait TableId<K, impl T: Table> {
    fn id(self: @K) -> felt252;
}

pub trait TableData<V, impl T: Table> {
    fn data(self: @V) -> Span<felt252>;
}

pub trait TableIdData<R, impl T: Table> {
    fn record_tuple(self: @R) -> (felt252, Span<felt252>);
    fn record_id_data(self: @R) -> IdData {
        Self::record_tuple(self).into()
    }
    fn records_id_data<RS, +AToSpanTrait<RS, R>, +Drop<RS>>(
        records: RS,
    ) -> Span<
        IdData,
    > {
        records.to_span().into_iter().map(|r| Self::record_id_data(r)).collect::<Array<_>>().span()
    }
}


impl TableSSIdImpl<K, impl T: Table, impl TDT: TableId<K>> of TableId<@K, T> {
    fn id(self: @@K) -> felt252 {
        TDT::id(*self)
    }
}

impl TableDataSSImpl<V, impl T: Table, impl TDT: TableData<V>> of TableData<@V, T> {
    fn data(self: @@V) -> Span<felt252> {
        TDT::data(*self)
    }
}


impl TableIdDataSSImpl<R, impl T: Table, impl TID: TableIdData<R, T>> of TableIdData<@R, T> {
    fn record_tuple(self: @@R) -> (felt252, Span<felt252>) {
        TID::record_tuple(*self)
    }
}

pub impl IdDataTupleImpl<
    K, V, impl T: Table, impl TID: TableId<K, T>, impl TV: TableData<V, T>,
> of TableIdData<(K, V), T> {
    fn record_tuple(self: @(K, V)) -> (felt252, Span<felt252>) {
        let (key, value) = self;
        (TID::id(key), TV::data(value))
    }
}


pub trait FieldsTrait<F, const SIZE: usize, impl T: Table> {
    const GROUP_ID: felt252;
    const FIELD_IDS: [felt252; SIZE];
    fn column_ids() -> Span<felt252>;
    fn record_data(self: @F) -> Span<felt252>;
}


pub trait TableKeySpanTrait<K, impl T: Table> {
    fn serialize_keys(self: @K, ref keys: Array<felt252>);
}

pub trait KeySpanDataSpanTrait<R, impl T: Table> {
    fn serialize_keys(self: @R, ref keys: Array<felt252>);
    fn serialize_values(self: @R, ref values: Array<felt252>);
}

// impl KeySpanDataSpanSSImpl<
//     R, impl T: Table, impl KS: KeySpanDataSpanTrait<R, T>,
// > of KeySpanDataSpanTrait<@R, T> {
//     fn serialize_keys(self: @R, ref keys: Array<felt252>) {
//         KS::serialize_keys(self, ref keys);
//     }
//     fn serialize_values(self: @R, ref values: Array<felt252>) {
//         KS::serialize_values(self, ref values);
//     }
// }

pub impl KeySpanToPrimaryTableIdDataImpl<
    R,
    impl T: Table,
    impl S: KeySpanDataSpanTrait<R>,
    impl KT: KeySpanToPrimary<T>,
    -KeyToPrimary<T>,
    +PrimaryTrait<T::Primary>,
    +Drop<T::Primary>,
> of TableIdData<R, T> {
    fn record_tuple(self: @R) -> (felt252, Span<felt252>) {
        let mut data: Array<felt252> = Default::default();
        S::serialize_keys(self, ref data);
        let id = KT::key_span_to_primary(data.span()).to_felt252();
        S::serialize_values(self, ref data);
        (id, data.span())
    }
}


pub trait KeySpanToPrimary<impl T: Table> {
    fn key_span_to_primary(self: Span<felt252>) -> T::Primary;
}

pub trait KeyToPrimary<impl T: Table> {
    type Key;
    fn key_to_primary(self: Self::Key) -> T::Primary;
}


// impl KeyedColumnGroupTableIdDataImpl<
//     G,
//     impl T: Table,
//     +KeySpanToPrimary<T>,
//     -KeyToPrimary<T>,
//     +Drop<T::Primary>,

//     +PrimaryTrait<T::Primary>,
// > of TableIdDataTrait<G, T> {
//     fn record_tuple(self: @G) -> (felt252, Span<felt252>) {
//         let (keys, data ) =
//         let id = introspect_types::ISerde::iserialize_inline(self.key_1);
//         let mut data: Array<felt252> = Default::default();
//         introspect_types::ISerde::iserialize(self.key_2, ref data);
//         introspect_types::ISerde::iserialize(self.name, ref data);
//         (id, data.span())
//     }
// }

fn primary_or_key_to_ids<K, impl T: Table, +TableId<K, T>, +Drop<K>>(
    keys: Span<K>,
) -> Span<felt252> {
    keys.into_iter().map(|k| k.id()).collect::<Array<_>>().span()
}

trait ColumnIdsTrait<CS> {
    fn columns_ids(self: CS) -> Span<felt252>;
}
impl ColumnIdsImpl<C, CS, +AToSpanTrait<CS, C>, +ColumnTrait<C>, +Drop<CS>> of ColumnIdsTrait<CS> {
    fn columns_ids(self: CS) -> Span<felt252> {
        let span: Span<C> = self.to_span();
        span.into_iter().map(|c| c.column_id()).collect::<Array<_>>().span()
    }
}

impl TablePrimaryIdImpl<impl T: Table, +PrimaryTrait<T::Primary>> of TableId<T::Primary, T> {
    fn id(self: @T::Primary) -> felt252 {
        self.to_felt252()
    }
}

pub impl TableKeyIdImpl<
    K,
    impl T: Table,
    +TableKeySpanTrait<K, T>,
    +KeySpanToPrimary<T>,
    +Drop<K>,
    -KeyToPrimary<T>,
    +PrimaryTrait<T::Primary>,
    +Drop<T::Primary>,
> of TableId<K, T> {
    fn id(self: @K) -> felt252 {
        let mut data: Array<felt252> = Default::default();
        TableKeySpanTrait::<K, T>::serialize_keys(self, ref data);
        KeySpanToPrimary::<T>::key_span_to_primary(data.span()).to_felt252()
    }
}


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

pub trait TableHasColumnsTrait<F, impl T: Table> {}

impl SSTableHasColumnsImpl<
    F, impl T: Table, +TableHasColumnsTrait<F, T>,
> of TableHasColumnsTrait<@F, T> {}


pub trait KeyTrait<T, K, const SIZE: usize, const KEYS: [felt252; SIZE]> {}

pub trait RecordTrait<R, impl T: Table> {}

impl RecordImpl<impl T: Table> of RecordTrait<T::Record, T> {}
impl RecordSSImpl<impl T: Table> of RecordTrait<@T::Record, T> {}


pub trait RecordableEvent<R, impl T: Table> {
    fn emit_recordable(self: R);
    fn emit_recordables<RS, +AToSpanTrait<RS, R>, +Drop<RS>>(records: RS);
}


pub trait MemberTrait<impl T: Table, M, const ID: felt252> {
    fn serialize_member(self: @M, ref data: Array<felt252>);
    fn serialize_member_inline(
        self: @M,
    ) -> Span<
        felt252,
    > {
        let mut data: Array<felt252> = Default::default();
        Self::serialize_member(self, ref data);
        data.span()
    }
    // fn serialize_key_and_value(
//     self: @M, ref keys: Array<felt252>, ref values: Array<felt252>,
// ) {
//     match Self::Key {
//         true => Self::serialize_member(self, ref keys),
//         false => Self::serialize_member(self, ref values),
//     }
// }
// fn serialize_key(
//     self: @M, ref data: Array<felt252>,
// ) {
//     if Self::Key {
//         Self::serialize_member(self, ref data);
//     }
// }
// fn serialize_value(
//     self: @M, ref data: Array<felt252>,
// ) {
//     if !Self::Key {
//         Self::serialize_member(self, ref data);
//     }
// }
}

impl MemberSSImpl<
    impl T: Table, M, const ID: felt252, impl SM: MemberTrait<T, M, ID>,
> of MemberTrait<T, @M, ID> {
    fn serialize_member(self: @@M, ref data: Array<felt252>) {
        SM::serialize_member(*self, ref data);
    }
}


impl MemberTableDataImpl<
    impl T: Table, M, const ID: felt252, const Key: bool, impl MT: MemberTrait<T, M, ID>,
> of TableData<M, T> {
    fn data(self: @M) -> Span<felt252> {
        MT::serialize_member_inline(self)
    }
}

pub impl EmitRecordableRecordImpl<
    R, impl T: Table, impl IDD: TableIdData<R, T>, +Drop<R>, +RecordTrait<R>,
> of RecordableEvent<R, T> {
    fn emit_recordable(self: R) {
        let id_data = IDD::record_id_data(@self);
        InsertRecord { table: T::ID, record: id_data.id, data: id_data.data }.emit_event();
    }
    fn emit_recordables<RS, +AToSpanTrait<RS, R>, +Drop<RS>>(records: RS) {
        let records_data = IDD::records_id_data(records.to_span());
        InsertRecords { table: T::ID, records_data }.emit_event();
    }
}


pub impl ColumnGroupRecordable<
    R,
    const SIZE: usize,
    impl T: Table,
    impl G: ColumnGroupTrait<R, SIZE>,
    impl IDD: TableIdData<R, T>,
    +Drop<R>,
> of RecordableEvent<R, T> {
    fn emit_recordable(self: R) {
        let (record, data) = IDD::record_tuple(@self);
        InsertFieldGroup { table: T::ID, record, group: G::GROUP_ID, data }.emit_event();
    }
    fn emit_recordables<RS, +AToSpanTrait<RS, R>, +Drop<RS>>(records: RS) {
        let records_data = IDD::records_id_data(records.to_span());
        InsertsFieldGroup { table: T::ID, group: G::GROUP_ID, records_data }.emit_event();
    }
}

pub impl ColumnGroupRecordableTupleImpl<
    K,
    V,
    const SIZE: usize,
    impl T: Table,
    impl G: ColumnGroupTrait<V, SIZE>,
    impl IDD: TableIdData<(K, V), T>,
    +Drop<K>,
    +Drop<V>,
> of RecordableEvent<(K, V), T> {
    fn emit_recordable(self: (K, V)) {
        let (record, data) = IDD::record_tuple(@self);
        InsertFieldGroup { table: T::ID, record, group: G::GROUP_ID, data }.emit_event();
    }
    fn emit_recordables<RS, +AToSpanTrait<RS, (K, V)>, +Drop<RS>>(records: RS) {
        let records_data = IDD::records_id_data(records.to_span());
        InsertsFieldGroup { table: T::ID, group: G::GROUP_ID, records_data }.emit_event();
    }
}


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
    fn insert<R, impl RE: RecordableEvent<R, Self::T>>(record: R);
    fn inserts<RS, R, +AToSpanTrait<RS, R>, impl RE: RecordableEvent<R, Self::T>, +Drop<RS>>(
        records: RS,
    );
    fn insert_field<
        const ID: felt252,
        K,
        F,
        +TableId<K, Self::T>,
        +MemberTrait<Self::T, F, ID>,
        +Drop<K>,
        +Drop<F>,
    >(
        id: K, field: F,
    );
    fn insert_fields<K, +TableId<K, Self::T>, +Drop<K>>(id: K, fields: Span<Self::Field>);
    fn inserts_field<
        const ID: felt252,
        FS,
        K,
        F,
        +TableId<K, Self::T>,
        +MemberTrait<Self::T, F, ID>,
        +AToSpanTrait<FS, (K, F)>,
        +Drop<K>,
        +Drop<F>,
        +Drop<FS>,
    >(
        id_fields: FS,
    );
    fn delete_record<K, +TableId<K, Self::T>, +Drop<K>>(id: K);
    fn delete_records<K, +TableId<K, Self::T>, +Drop<K>>(ids: Span<K>);
    fn delete_field<K, +TableId<K, Self::T>, +Drop<K>>(id: K, column: Self::Column);
    fn delete_fields<K, CS, +TableId<K, Self::T>, +ColumnIdsTrait<CS>, +Drop<K>, +Drop<CS>>(
        id: K, columns: CS,
    );
    fn deletes_field<K, +TableId<K, Self::T>, +Drop<K>>(ids: Span<K>, column: Self::Column);
    fn deletes_fields<K, CS, +TableId<K, Self::T>, +ColumnIdsTrait<CS>, +Drop<K>, +Drop<CS>>(
        ids: Span<K>, columns: CS,
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

pub impl ITableImpl<impl T: Table, +ColumnTrait<T::Column>, +Drop<T::Column>> of ITable {
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
    fn insert<R, impl RE: RecordableEvent<R, T>>(record: R) {
        RE::emit_recordable(record);
    }
    fn inserts<RS, R, +AToSpanTrait<RS, R>, impl RE: RecordableEvent<R, T>, +Drop<RS>>(
        records: RS,
    ) {
        RE::emit_recordables(records);
    }
    fn insert_field<
        const ID: felt252,
        K,
        F,
        +TableId<K, Self::T>,
        impl M: MemberTrait<Self::T, F, ID>,
        +Drop<K>,
        +Drop<F>,
    >(
        id: K, field: F,
    ) {
        InsertField {
            table: T::ID, record: id.id(), column: ID, data: M::serialize_member_inline(@field),
        }
            .emit_event();
    }

    fn insert_fields<K, +TableId<K, Self::T>, +Drop<K>>(id: K, fields: Span<Self::Field>) {
        let columns = fields.into_iter().map(|f| f.column_id()).collect::<Array<_>>().span();
        let mut data: Array<felt252> = Default::default();
        for field in fields {
            data.append_span(field.data());
        }
        InsertFields { table: T::ID, record: id.id(), columns, data: data.span() }.emit_event();
    }

    fn inserts_field<
        const ID: felt252,
        FS,
        K,
        F,
        impl TID: TableId<K, Self::T>,
        impl MT: MemberTrait<Self::T, F, ID>,
        +AToSpanTrait<FS, (K, F)>,
        +Drop<K>,
        +Drop<F>,
        +Drop<FS>,
    >(
        id_fields: FS,
    ) {
        let records_data = IdDataTupleImpl::<
            K, F, T, TID, MemberTableDataImpl<T, F, ID>,
        >::records_id_data(id_fields);
        // let records_data = TableIdDataTrait::<(K, F), T>::records_id_data(id_fields);
        InsertsField { table: T::ID, column: ID, records_data }.emit_event();
    }

    fn delete_record<K, +TableId<K, Self::T>, +Drop<K>>(id: K) {
        DeleteRecord { table: T::ID, record: id.id() }.emit_event();
    }
    fn delete_records<K, +TableId<K, Self::T>, +Drop<K>>(ids: Span<K>) {
        DeleteRecords { table: T::ID, records: primary_or_key_to_ids(ids) }.emit_event();
    }
    fn delete_field<K, +TableId<K, Self::T>, +Drop<K>>(id: K, column: T::Column) {
        DeleteField { table: T::ID, record: id.id(), column: column.column_id() }.emit_event();
    }

    fn delete_fields<K, CS, +TableId<K, Self::T>, +ColumnIdsTrait<CS>, +Drop<K>, +Drop<CS>>(
        id: K, columns: CS,
    ) {
        DeleteFields { table: T::ID, record: id.id(), columns: columns.columns_ids() }.emit_event();
    }
    fn deletes_field<K, +TableId<K, Self::T>, +Drop<K>>(ids: Span<K>, column: T::Column) {
        DeletesField {
            table: T::ID, records: primary_or_key_to_ids(ids), column: column.column_id(),
        }
            .emit_event();
    }
    fn deletes_fields<K, CS, +TableId<K, Self::T>, +ColumnIdsTrait<CS>, +Drop<K>, +Drop<CS>>(
        ids: Span<K>, columns: CS,
    ) {
        DeletesFields {
            table: T::ID, records: primary_or_key_to_ids(ids), columns: columns.columns_ids(),
        }
            .emit_event();
    }
}
