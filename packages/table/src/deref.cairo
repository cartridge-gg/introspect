pub trait Snapshot<T> {}
pub trait Owned<T> {}
pub trait NestedSnapshot<T> {}
pub trait SingleSnapshot<T> {}

pub trait SnapshotOf<T, S> {
    fn test() {}
}
// pub trait IsOwnedOf<T, O> {}
pub trait EquivalentType<T, S> {}
pub trait BaseType<T> {
    type Base;
}

impl SnapshotImpl<T> of Snapshot<@T> {}
impl OwnedImpl<T, -Snapshot<T>> of Owned<T> {}
impl NestedSnapshotImpl<T> of NestedSnapshot<@@T> {}
impl SingleSnapshotImpl<T, -NestedSnapshot<T>> of SingleSnapshot<T> {}

impl SnapshotOfImpl<T> of SnapshotOf<@T, T> {}
impl SnapshotOfMultiImpl<T, S, +SnapshotOf<T, S>> of SnapshotOf<@T, S> {}

impl EquivalentTypeImpl<T> of EquivalentType<T, T> {}
impl EquivalentTypeRHSSingleSSImpl<
    T, S, +EquivalentType<T, S>, +SnapshotOf<@T, S>,
> of EquivalentType<@T, S> {}
impl EquivalentTypeLHSSingleSSImpl<
    T, S, +EquivalentType<T, S>, +SnapshotOf<@S, T>,
> of EquivalentType<T, @S> {}


impl BaseTypeImpl<T, +Owned<T>> of BaseType<T> {
    type Base = T;
}

impl BaseTypeMultiImpl<T, impl BT: BaseType<T>> of BaseType<@T> {
    type Base = BT::Base;
}

pub trait AsSnapshot<T, S> {
    const fn as_snapshot(self: T) -> S nopanic;
}

pub trait ToSnapshot<T, S> {
    const fn to_snapshot(self: T) -> @S nopanic;
}

impl AsSnapshotOwnedImpl<T, +Owned<T>, +Drop<T>> of AsSnapshot<T, @T> {
    const fn as_snapshot(self: T) -> @T nopanic {
        @self
    }
}

impl AsSnapshotSnapShotImpl<T, +Owned<T>> of AsSnapshot<@T, @T> {
    const fn as_snapshot(self: @T) -> @T nopanic {
        self
    }
}


impl AsSnapshotMultiImpl<
    T, impl BT: BaseType<T>, impl SS: AsSnapshot<@T, @BT::Base>, +Owned<BT::Base>,
> of AsSnapshot<@@T, @BT::Base> {
    const fn as_snapshot(self: @@T) -> @BT::Base nopanic {
        SS::as_snapshot(*self)
    }
}


impl ToSnapshotOwnedImpl<T, +Drop<T>, +Owned<T>> of ToSnapshot<T, T> {
    const fn to_snapshot(self: T) -> @T nopanic {
        @self
    }
}

impl ToSnapshotSnapshotImpl<T, +Owned<T>> of ToSnapshot<@T, T> {
    const fn to_snapshot(self: @T) -> @T nopanic {
        self
    }
}

impl ToSnapshotDoubleImpl<T> of ToSnapshot<@@T, T> {
    const fn to_snapshot(self: @@T) -> @T nopanic {
        *self
    }
}


pub trait Spannable<C, T> {
    fn to_span(self: C) -> Span<T>;
}


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

