pub impl Gen{{name}}ISerdeImpl{{impl_params}} of {{i_path}}::ISerde<{{full_name}}> {
    fn iserialize(self: @{{full_name}}, ref output: Array<felt252>) {
        {{serialize_body}}
    }
    fn ideserialize(ref serialized: Span<felt252>) -> Option<{{full_name}}> {
        {{deserialize_body}}
    }
}