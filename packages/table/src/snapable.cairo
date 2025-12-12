pub trait Snapable<T, S> {
    fn snapshot(self: T) -> @S;
}


impl TSnapable<T, +Drop<T>> of Snapable<T, T> {
    fn snapshot(self: T) -> @T {
        @self
    }
}

impl TSSSnapable<T> of Snapable<@T, T> {
    fn snapshot(self: @T) -> @T {
        self
    }
}

impl TSSSSnapable<T, impl SS: Snapable<@T, T>> of Snapable<@@T, T> {
    fn snapshot(self: @@T) -> @T {
        SS::snapshot(*self)
    }
}
