impl PlayerRecordId of introspect_table::RecordId<felt252, PlayerTable> {
    fn record_id(self: @felt252) -> felt252 {
        *self
    }
}
