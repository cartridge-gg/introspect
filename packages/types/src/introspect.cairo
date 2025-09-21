trait Introspect<T> {
    fn introspect() -> Ty;
    fn schemas() -> Array<(felt252, Ty)>;
}

