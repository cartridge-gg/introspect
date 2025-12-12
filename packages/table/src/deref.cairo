pub trait Snapable<T, S> {
    const fn snapshot(self: T) -> @S;
}


impl TSnapable<T, +Drop<T>> of Snapable<T, T> {
    const fn snapshot(self: T) -> @T {
        @self
    }
}

impl TSSSnapable<T> of Snapable<@T, T> {
    const fn snapshot(self: @T) -> @T {
        self
    }
}

impl TSSSSnapable<T, impl SS: Snapable<@T, T>> of Snapable<@@T, T> {
    const fn snapshot(self: @@T) -> @T {
        SS::snapshot(*self)
    }
}


pub trait Spannable<C, T> {
    fn to_span(self: C) -> Span<T>;
}


// impl SPannableSS<T, E, impl SP: Spannable<@T, E>, impl SS:Snapable<@T, T> +Drop<T>> of
// Spannable<T, E> {
//     fn to_span(self: T) -> Span<E> {
//         // SS::to_span(@self)
//     }
// }

impl SpannableImpl<C, T, +ToSpanTrait<C, T>, +Drop<C>> of Spannable<C, T> {
    fn to_span(self: C) -> Span<T> {
        self.span()
    }
}

impl SpannableRefImpl<C, T, +ToSpanTrait<C, T>> of Spannable<@C, T> {
    fn to_span(self: @C) -> Span<T> {
        self.span()
    }
}


impl SpannableSpan<T> of Spannable<Span<T>, T> {
    fn to_span(self: Span<T>) -> Span<T> {
        self
    }
}

impl SSSpannableSpan<T> of Spannable<@Span<T>, T> {
    fn to_span(self: @Span<T>) -> Span<T> {
        *self
    }
}
// impl AFixedArrayToSpan<
//     T, const N: usize, impl TS: ToSpanTrait<[T; N], T>,
// > of Spannable<@[T; N], T> {
//     fn to_span(self: @[T; N]) -> Span<T> {
//         TS::span(self)
//     }
// }

// impl SSPanableImpl<S, T, impl TS: ToSpanTrait<S, T>> of Spannable<@S, T> {
//     fn to_span(self: @S) -> Span<T> {
//         TS::span(self)
//     }
// }

// pub trait Iterable<T, S, impl II: IntoIterator<T>, +TypeEqual<II::Iterator::Item, S>> {
//     type IntoIter;
//     impl Iterator: Iterator<Self::IntoIter>;
//     fn into_iter(self: T) -> Self::IntoIter;
// }

// impl IterableImpl<
//     T, S, impl II: IntoIterator<T>, ,
// > of Iterable<T, S> {
//     type IntoIter = II::IntoIter;
//     fn into_iter(self: T) -> Self::IntoIter {
//         II::into_iter(self)
//     }

// }


