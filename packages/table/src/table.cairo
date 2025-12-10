use core::metaprogramming::TypeEqual;
use introspect_events::EmitEvent;
use introspect_events::database::{
    AddColumn, AddColumns, CreateFieldGroup, CreateIndex, CreateTable, CreateTableFromClassHash,
    CreateTableWithColumns, DeleteField, DeleteFieldGroup, DeleteFieldGroups, DeleteFields,
    DeleteRecord, DeleteRecords, DeletesField, DeletesFieldGroup, DeletesFieldGroups, DeletesFields,
    DropColumn, DropColumns, DropIndex, DropTable, InsertField, InsertFieldGroup, InsertFieldGroups,
    InsertFields, InsertRecord, InsertRecords, InsertsField, InsertsFieldGroup, InsertsFieldGroups,
    InsertsFields, RenameColumn, RenameColumns, RenamePrimary, RenameTable, RetypeColumn,
    RetypeColumns, RetypePrimary,
};
use introspect_types::{
    Attribute, ColumnDef, ISerde, IdData, PrimaryDef, PrimaryTrait, PrimaryTypeDef, TypeDef,
};

pub fn default_table_primary_def() -> PrimaryDef {
    PrimaryDef { name: "__id", type_def: PrimaryTypeDef::Felt252, attributes: [].span() }
}

pub trait FieldTrait<T> {
    fn column_id(self: @T) -> felt252;
    fn data(self: @T) -> Span<felt252>;
}
pub trait FieldsTrait<F, impl T: Table> {
    fn column_id(self: @F) -> felt252;
    fn id_datas(self: @F) -> Span<IdData>;
}

pub trait ColumnTrait<T> {
    fn column_id(self: @T) -> felt252;
}

pub trait TableColumns<T> {
    type Column;
    type Field;
    fn columns() -> Span<ColumnDef>;
    fn child_defs() -> Array<(felt252, TypeDef)>;
    fn record_data(self: @T) -> Span<felt252>;
}

pub trait TableKey<T, const COLUMNS: usize> {
    type Key;
    type SKey;
    const KEY_IDS: [felt252; COLUMNS];
}

pub trait TablePrimary<T> {
    type Primary;
    type Key;
    fn primary_def() -> PrimaryDef {
        PrimaryDef { name: "__id", type_def: PrimaryTypeDef::Felt252, attributes: [].span() }
    }
    fn record_id(self: @T) -> felt252;
}

pub trait TableMeta {
    const ID: felt252;
    fn name() -> ByteArray;
    fn attributes() -> Span<Attribute>;
}

pub trait TablePrimaryOrKey<K, impl T: Table> {
    fn to_id(self: @K) -> felt252;
}


fn primary_or_key_to_ids<K, impl T: Table, +TablePrimaryOrKey<K, T>, +Drop<K>>(
    keys: Span<K>,
) -> Span<felt252> {
    keys.into_iter().map(|k| k.to_id()).collect::<Array<_>>().span()
}


fn columns_to_ids<C, +ColumnTrait<C>, +Drop<C>, const SIZE: usize>(
    columns: [C; SIZE],
) -> Span<felt252> {
    let span = BoxTrait::new(@columns).span();
    span.into_iter().map(|c| c.column_id()).collect::<Array<_>>().span()
}

impl TablePrimaryImpl<
    impl T: Table, +PrimaryTrait<T::Primary>, +Drop<T::Primary>,
> of TablePrimaryOrKey<T::Primary, T> {
    fn to_id(self: @T::Primary) -> felt252 {
        self.to_felt252()
    }
}


impl TablePrimarySSImpl<K, impl T: Table, +TablePrimaryOrKey<K>> of TablePrimaryOrKey<@K, T> {
    fn to_id(self: @@K) -> felt252 {
        (*self).to_id()
    }
}


pub trait Table {
    type Record;
    type Primary;
    type Column;
    type Field;
    type Key;
    const ID: felt252;
    fn name() -> ByteArray;
    fn attributes() -> Span<Attribute>;
    fn primary() -> PrimaryDef;
    fn columns() -> Span<ColumnDef>;
    fn child_defs() -> Array<(felt252, TypeDef)>;
    fn record_tuple(self: @Self::Record) -> (felt252, Span<felt252>);
    fn record_id_data(self: @Self::Record) -> IdData {
        Self::record_tuple(self).into()
    }
    fn records_id_data(
        records: Span<@Self::Record>,
    ) -> Span<
        IdData,
    > {
        records.into_iter().map(|r| Self::record_id_data(*r)).collect::<Array<_>>().span()
    }
}


pub trait ITable {
    impl T: Table;
    type Record;
    type Column;
    type Field;
    type Key;
    const ID: felt252;
    fn register_table();
    fn insert_record(record: @Self::Record);
    fn insert_records(records: Span<@Self::Record>);
    fn insert_key_and_fields(key: @Self::Key, fields: Span<Self::Field>);
    fn insert_field<K, +TablePrimaryOrKey<K, Self::T>, +Drop<K>>(id: K, field: Self::Field);
    fn insert_fields<K, +TablePrimaryOrKey<K, Self::T>, +Drop<K>>(id: K, fields: Span<Self::Field>);
    fn inserts_field<F, +FieldsTrait<F, Self::T>, +Drop<F>>(id_fields: F);
    fn delete_record<K, +TablePrimaryOrKey<K, Self::T>, +Drop<K>>(id: K);
    fn delete_records<K, +TablePrimaryOrKey<K, Self::T>, +Drop<K>>(ids: Span<K>);
    fn delete_field<K, +TablePrimaryOrKey<K, Self::T>, +Drop<K>>(id: K, column: Self::Column);
    fn delete_fields<K, const SIZE: usize, +TablePrimaryOrKey<K, Self::T>, +Drop<K>>(
        id: K, columns: [Self::Column; SIZE],
    );
    fn deletes_field<K, +TablePrimaryOrKey<K, Self::T>, +Drop<K>>(
        ids: Span<K>, column: Self::Column,
    );
    fn deletes_fields<K, const SIZE: usize, +TablePrimaryOrKey<K, Self::T>, +Drop<K>>(
        ids: Span<K>, columns: [Self::Column; SIZE],
    );
}

pub impl TableImpl<
    Record,
    impl Meta: TableMeta,
    impl Primary: TablePrimary<Record>,
    impl Columns: TableColumns<Record>,
> of Table {
    type Record = Record;
    type Primary = Primary::Primary;
    type Key = Primary::Key;
    type Column = Columns::Column;
    type Field = Columns::Field;
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
    fn record_tuple(self: @Self::Record) -> (felt252, Span<felt252>) {
        (Primary::record_id(self), Columns::record_data(self))
    }
}


pub impl TableTupleImpl<
    K,
    T,
    impl Meta: TableMeta,
    impl Primary: TablePrimary<K>,
    impl Columns: TableColumns<T>,
    +PrimaryTrait<K>,
    +Drop<K>,
> of Table {
    type Record = (K, @T);
    type Primary = Primary::Primary;
    type Key = Primary::Key;
    type Column = Columns::Column;
    type Field = Columns::Field;
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
    fn record_tuple(self: @(K, @T)) -> (felt252, Span<felt252>) {
        let (key, record) = self;
        (key.to_felt252(), Columns::record_data(*record))
    }
}


pub fn field_to_id_data<K, T, impl Ta: Table, +TablePrimaryOrKey<K, Ta>, +ISerde<T>, +Drop<T>>(
    key_field: @(K, T),
) -> IdData {
    let (key, field) = key_field;
    (key.to_id(), field.iserialize_inline()).into()
}

pub fn fields_to_id_datas<K, T, impl Ta: Table, +TablePrimaryOrKey<K, Ta>, +ISerde<T>, +Drop<T>>(
    key_fields: @Span<(K, T)>,
) -> Span<IdData> {
    key_fields.into_iter().map(|kf| field_to_id_data::<K, T, Ta>(kf)).collect::<Array<_>>().span()
}

pub impl ITableImpl<
    impl T: Table,
    +FieldTrait<T::Field>,
    +ColumnTrait<T::Column>,
    +TablePrimaryOrKey<T::Key>,
    +Drop<T::Field>,
    +Drop<T::Column>,
> of ITable {
    type Record = T::Record;
    type Column = T::Column;
    type Field = T::Field;
    type Key = T::Key;
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
    fn insert_record(record: @T::Record) {
        let (record, data) = T::record_tuple(record);
        InsertRecord { table: T::ID, record, data }.emit_event();
    }
    fn insert_records(records: Span<@T::Record>) {
        InsertRecords { table: T::ID, records_data: T::records_id_data(records) }.emit_event();
    }
    fn insert_key_and_fields(key: @Self::Key, fields: Span<Self::Field>) {
        let columns = fields.into_iter().map(|f| f.column_id()).collect::<Array<_>>().span();
        let mut data: Array<felt252> = Default::default();
        for field in fields {
            data.append_span(field.data());
        }
        InsertFields { table: T::ID, record: key.to_id(), columns, data: data.span() }.emit_event();
    }
    fn insert_field<K, +TablePrimaryOrKey<K, Self::T>, +Drop<K>>(id: K, field: T::Field) {
        InsertField {
            table: T::ID, record: id.to_id(), column: field.column_id(), data: field.data(),
        }
            .emit_event();
    }

    fn insert_fields<K, +TablePrimaryOrKey<K, Self::T>, +Drop<K>>(
        id: K, fields: Span<Self::Field>,
    ) {
        let columns = fields.into_iter().map(|f| f.column_id()).collect::<Array<_>>().span();
        let mut data: Array<felt252> = Default::default();
        for field in fields {
            data.append_span(field.data());
        }
        InsertFields { table: T::ID, record: id.to_id(), columns, data: data.span() }.emit_event();
    }

    fn inserts_field<F, +FieldsTrait<F, Self::T>, +Drop<F>>(id_fields: F) {
        InsertsField {
            table: T::ID, column: id_fields.column_id(), records_data: id_fields.id_datas(),
        }
            .emit_event();
    }

    fn delete_record<K, +TablePrimaryOrKey<K, Self::T>, +Drop<K>>(id: K) {
        DeleteRecord { table: T::ID, record: id.to_id() }.emit_event();
    }
    fn delete_records<K, +TablePrimaryOrKey<K, Self::T>, +Drop<K>>(ids: Span<K>) {
        DeleteRecords { table: T::ID, records: primary_or_key_to_ids(ids) }.emit_event();
    }
    fn delete_field<K, +TablePrimaryOrKey<K, Self::T>, +Drop<K>>(id: K, column: T::Column) {
        DeleteField { table: T::ID, record: id.to_id(), column: column.column_id() }.emit_event();
    }

    fn delete_fields<K, const SIZE: usize, +TablePrimaryOrKey<K, Self::T>, +Drop<K>>(
        id: K, columns: [T::Column; SIZE],
    ) {
        DeleteFields { table: T::ID, record: id.to_id(), columns: columns_to_ids(columns) }
            .emit_event();
    }
    fn deletes_field<K, +TablePrimaryOrKey<K, Self::T>, +Drop<K>>(ids: Span<K>, column: T::Column) {
        DeletesField {
            table: T::ID, records: primary_or_key_to_ids(ids), column: column.column_id(),
        }
            .emit_event();
    }
    fn deletes_fields<K, const SIZE: usize, +TablePrimaryOrKey<K, Self::T>, +Drop<K>>(
        ids: Span<K>, columns: [T::Column; SIZE],
    ) {
        DeletesFields {
            table: T::ID, records: primary_or_key_to_ids(ids), columns: columns_to_ids(columns),
        }
            .emit_event();
    }
}

pub impl ITableTupleImpl<
    impl T: Table,
    +FieldTrait<T::Field>,
    +ColumnTrait<T::Column>,
    +TablePrimaryOrKey<T::Key>,
    +Drop<T::Field>,
    +Drop<T::Column>,
> of ITable {
    type Record = T::Record;
    type Column = T::Column;
    type Field = T::Field;
    type Key = T::Key;
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
    fn insert_record(record: @T::Record) {
        InsertRecord { table: T::ID, record: T::record_id(record), data: T::record_data(record) }
            .emit_event();
    }
    fn insert_records(records: Span<@T::Record>) {
        InsertRecords { table: T::ID, records_data: T::records_id_data(records) }.emit_event();
    }
    fn insert_field<K, +TablePrimaryOrKey<K, Self::T>, +Drop<K>>(id: K, field: T::Field) {
        InsertField {
            table: T::ID, record: id.to_id(), column: field.column_id(), data: field.data(),
        }
            .emit_event();
    }

    fn insert_fields<K, +TablePrimaryOrKey<K, Self::T>, +Drop<K>>(
        id: K, fields: Span<Self::Field>,
    ) {
        let columns = fields.into_iter().map(|f| f.column_id()).collect::<Array<_>>().span();
        let mut data: Array<felt252> = Default::default();
        for field in fields {
            data.append_span(field.data());
        }
        InsertFields { table: T::ID, record: id.to_id(), columns, data: data.span() }.emit_event();
    }

    fn inserts_field<F, +FieldsTrait<F, Self::T>, +Drop<F>>(id_fields: F) {
        InsertsField {
            table: T::ID, column: id_fields.column_id(), records_data: id_fields.id_datas(),
        }
            .emit_event();
    }

    fn delete_record<K, +TablePrimaryOrKey<K, Self::T>, +Drop<K>>(id: K) {
        DeleteRecord { table: T::ID, record: id.to_id() }.emit_event();
    }
    fn delete_records<K, +TablePrimaryOrKey<K, Self::T>, +Drop<K>>(ids: Span<K>) {
        DeleteRecords { table: T::ID, records: primary_or_key_to_ids(ids) }.emit_event();
    }
    fn delete_field<K, +TablePrimaryOrKey<K, Self::T>, +Drop<K>>(id: K, column: T::Column) {
        DeleteField { table: T::ID, record: id.to_id(), column: column.column_id() }.emit_event();
    }

    fn delete_fields<K, const SIZE: usize, +TablePrimaryOrKey<K, Self::T>, +Drop<K>>(
        id: K, columns: [T::Column; SIZE],
    ) {
        DeleteFields { table: T::ID, record: id.to_id(), columns: columns_to_ids(columns) }
            .emit_event();
    }
    fn deletes_field<K, +TablePrimaryOrKey<K, Self::T>, +Drop<K>>(ids: Span<K>, column: T::Column) {
        DeletesField {
            table: T::ID, records: primary_or_key_to_ids(ids), column: column.column_id(),
        }
            .emit_event();
    }
    fn deletes_fields<K, const SIZE: usize, +TablePrimaryOrKey<K, Self::T>, +Drop<K>>(
        ids: Span<K>, columns: [T::Column; SIZE],
    ) {
        DeletesFields {
            table: T::ID, records: primary_or_key_to_ids(ids), columns: columns_to_ids(columns),
        }
            .emit_event();
    }
}
