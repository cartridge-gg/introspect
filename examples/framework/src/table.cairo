use introspect::events::database::emitters::{
    emit_create_table_with_columns, emit_delete_field, emit_delete_fields, emit_delete_record,
    emit_delete_records, emit_deletes_field, emit_deletes_fields, emit_insert_field,
    emit_insert_fields, emit_insert_record, emit_insert_records, emit_inserts_field,
    emit_inserts_fields,
};
use introspect::events::emit_declare_type;
use introspect::{Attribute, IdData, IdDataTrait, PrimaryDef, Schema};



pub trait ITable<R, +Schema<R>, +IdDataTrait<R>, +Drop<R>> {
    const SELECTOR: felt252;
    fn name() -> ByteArray;
    fn primary() -> PrimaryDef;
    fn attributes() -> Span<Attribute>;
    fn register_table() {
        for (hash, child_def) in Schema::<R>::child_defs() {
            emit_declare_type(hash, child_def);
        }
        emit_create_table_with_columns(
            id: Self::SELECTOR,
            name: Self::name(),
            attributes: Self::attributes(),
            primary: Self::primary(),
            columns: Schema::<R>::columns(),
        );
    }
    fn insert_record(
        record: @R,
    ) {
        let (id, data) = record.id_data_tuple();
        emit_insert_record(Self::SELECTOR, id, data);
    }

    fn insert_records(
        records: Span<R>,
    ) {
        let mut id_datas: Array<IdData> = Default::default();
        for record in records {
            id_datas.append(record.id_data());
        }
        emit_insert_records(Self::SELECTOR, id_datas.span());
    }
    fn insert_field<const FIELD: felt252, T, +IdDataTrait<T>>(
        record: @T,
    ) {
        let (id, data) = record.id_data_tuple();
        emit_insert_field(Self::SELECTOR, FIELD, id, data);
    }

    fn insert_fields<const FIELDS: [felt252; N], const N: usize, T, +IdDataTrait<T>>(
        record: @T,
    ) {
        let (id, data) = record.id_data_tuple();
        emit_insert_fields(Self::SELECTOR, id, BoxTrait::new(@FIELDS).span(), data);
    }

    fn inserts_field<const FIELD: felt252, T, +IdDataTrait<T>>(
        records: Span<@T>,
    ) {
        let mut id_datas: Array<IdData> = Default::default();
        for record in records {
            id_datas.append(record.id_data());
        }
        emit_inserts_field(Self::SELECTOR, FIELD, id_datas.span());
    }

    fn inserts_fields<const FIELDS: [felt252; N], const N: usize, T, +IdDataTrait<T>>(
        records: Span<@T>,
    ) {
        let mut id_datas: Array<IdData> = Default::default();
        for record in records {
            id_datas.append(record.id_data());
        }
        emit_inserts_fields(Self::SELECTOR, BoxTrait::new(@FIELDS).span(), id_datas.span());
    }

    fn delete_record(record_id: felt252) {
        emit_delete_record(Self::SELECTOR, record_id);
    }

    fn delete_records(
        record_ids: Span<felt252>,
    ) {
        emit_delete_records(Self::SELECTOR, record_ids);
    }

    fn delete_field<const FIELD: felt252>(
        record_id: felt252,
    ) {
        emit_delete_field(Self::SELECTOR, record_id, FIELD);
    }

    fn delete_fields<const FIELDS: [felt252; N], const N: usize>(
        record_id: felt252,
    ) {
        emit_delete_fields(Self::SELECTOR, record_id, BoxTrait::new(@FIELDS).span());
    }

    fn deletes_field<const FIELD: felt252>(
        record_ids: Span<felt252>,
    ) {
        emit_deletes_field(Self::SELECTOR, FIELD, record_ids);
    }

    fn deletes_fields<const FIELDS: [felt252; N], const N: usize>(
        record_ids: Span<felt252>,
    ) {
        emit_deletes_fields(Self::SELECTOR, BoxTrait::new(@FIELDS).span(), record_ids);
    }
}

