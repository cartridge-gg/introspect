pub impl Gen{{name}}ISerdeImpl{{impl_params}} of introspect::ISerde<{{full_name}}> {
    fn iserialize(self: @{{full_name}}, ref output: Array<felt252>) {
        {{body}}
    }
}