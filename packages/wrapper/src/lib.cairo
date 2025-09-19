#[derive(Copy, Drop, Default)]
pub enum Wrapper<T> {
    #[default]
    value: Box<T>,
}

impl ClassHashPartialEq<T, +PartialEq<T>> of PartialEq<Wrapper<T>> {
    #[inline]
    fn eq(lhs: @Wrapper<T>, rhs: @Wrapper<T>) -> bool {
        match (lhs, rhs) {
            (Wrapper::value(l), Wrapper::value(r)) => PartialEq::<
                T,
            >::eq(l.as_snapshot().unbox(), r.as_snapshot().unbox()),
        }
    }
}
const EMPTY_WRAPPER_ERROR: felt252 = 'Wrapper cannot not be empty';

impl WrapperSerdeImpl<T, +Serde<T>> of Serde<Wrapper<T>> {
    fn serialize(self: @Wrapper<T>, ref output: Array<felt252>) {
        match self {
            Wrapper::value(t) => { Serde::<T>::serialize(t.as_snapshot().unbox(), ref output); },
        }
    }

    fn deserialize(ref serialized: Span<felt252>) -> Option<Wrapper<T>> {
        match Serde::<T>::deserialize(ref serialized) {
            Option::Some(t) => Option::Some(Wrapper::value(BoxTrait::new(t))),
            Option::None => Option::None,
        }
    }
}

impl DerefWrapper<T> of Deref<Wrapper<T>> {
    type Target = T;
    fn deref(self: Wrapper<T>) -> T {
        match self {
            Wrapper::value(t) => t.unbox(),
        }
    }
}

impl SWrapIntoT<T> of Into<@Wrapper<T>, @T> {
    fn into(self: @Wrapper<T>) -> @T {
        match self {
            Wrapper::value(t) => t.as_snapshot().unbox(),
        }
    }
}

impl WrapIntoT<T> of Into<Wrapper<T>, T> {
    fn into(self: Wrapper<T>) -> T {
        match self {
            Wrapper::value(t) => t.unbox(),
        }
    }
}

impl TIntoWrap<T> of Into<T, Wrapper<T>> {
    fn into(self: T) -> Wrapper<T> {
        Wrapper::value(BoxTrait::new(self))
    }
}
