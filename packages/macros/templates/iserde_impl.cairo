pub impl ISerde{{name}}Impl of super::ISerde<{{item}}> {
    fn iserialize(self: @{{item}}, ref output: Array<felt252>) {
        {{BoxTySerdeImpl}}
    }
}