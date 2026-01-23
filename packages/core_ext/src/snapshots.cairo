pub trait Snapshot<T> {}
pub trait Owned<T> {}
pub trait NestedSnapshot<T> {}
pub trait SingleSnapshot<T> {}

pub trait SnapshotOf<T, S> {}
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

pub trait AsSnapshot<From, Snapshot> {
    const fn as_snapshot(self: From) -> Snapshot nopanic;
}

pub trait ToSnapshotOf<From, Snapshot> {
    const fn to_snapshot(self: From) -> @Snapshot nopanic;
}

pub trait ToSnapshotBase<T> {
    type Base;
    const fn to_snapshot(self: T) -> @Self::Base nopanic;
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

impl AsSnapshot2Impl<T, +Owned<T>> of AsSnapshot<@@T, @T> {
    const fn as_snapshot(self: @@T) -> @T nopanic {
        *self
    }
}

impl AsSnapshot3Impl<T, +Owned<T>> of AsSnapshot<@@@T, @T> {
    const fn as_snapshot(self: @@@T) -> @T nopanic {
        **self
    }
}


// impl AsSnapshotMultiImpl<
//     T, impl BT: BaseType<T>, impl SS: AsSnapshot<@T, @BT::Base>, +Owned<BT::Base>,
// > of AsSnapshot<@@T, @BT::Base> {
//     const fn as_snapshot(self: @@T) -> @BT::Base nopanic {
//         SS::as_snapshot(*self)
//     }
// }

impl ToSnapShotBaseOwned<T, +Drop<T>, +Owned<T>> of ToSnapshotBase<T> {
    type Base = T;
    const fn to_snapshot(self: T) -> @T nopanic {
        @self
    }
}

impl ToSnapshotBaseSnapshot<T, +Owned<T>> of ToSnapshotBase<@T> {
    type Base = T;
    const fn to_snapshot(self: @T) -> @T nopanic {
        self
    }
}

impl ToSnapshotBaseNested<T, impl SS: ToSnapshotBase<@T>, +Owned<SS::Base>> of ToSnapshotBase<@@T> {
    type Base = SS::Base;
    const fn to_snapshot(self: @@T) -> @SS::Base nopanic {
        SS::to_snapshot(*self)
    }
}

impl ToSnapshotOfOwned<T, +Drop<T>, +Owned<T>> of ToSnapshotOf<T, T> {
    const fn to_snapshot(self: T) -> @T nopanic {
        @self
    }
}

impl ToSnapshotOfSnapshot<T, +Owned<T>> of ToSnapshotOf<@T, T> {
    const fn to_snapshot(self: @T) -> @T nopanic {
        self
    }
}

// impl ToSnapshot2Impl<T, +Owned<T>> of ToSnapshotOf<@@T, T> {
//     const fn to_snapshot(self: @@T) -> @T nopanic {
//         *self
//     }
// }

// impl ToSnapshot3Impl<T, +Owned<T>> of ToSnapshotOf<@@@T, T> {
//     const fn to_snapshot(self: @@@T) -> @T nopanic {
//         **self
//     }
// }

impl ToSnapshotNestedImpl<T, impl TS: ToSnapshotBase<T>> of ToSnapshotOf<@T, TS::Base> {
    const fn to_snapshot(self: @T) -> @TS::Base nopanic {
        TS::to_snapshot(*self)
    }
}
// impl ToSnapshotNestedImpl<T, S, impl SS: ToSnapshotOf<@T, S>, +Owned<S>> of ToSnapshotOf<@@T, S>
// {
//     const fn to_snapshot(self: @@T) -> @S nopanic {
//         SS::to_snapshot(*self)
//     }
// }
mod test_mod {
    use super::{ToSnapshotBase, ToSnapshotOf};
    fn test() {
        let a = ToSnapshotBase::to_snapshot(1);
        let b = ToSnapshotBase::to_snapshot(@1);
        let c = ToSnapshotBase::to_snapshot(@@1);
        let d = ToSnapshotBase::to_snapshot(@@@@@@1);
        let a = ToSnapshotOf::to_snapshot(1);
        let b = ToSnapshotOf::to_snapshot(@1);
        let c = ToSnapshotOf::to_snapshot(@@@@@@1);
        // let d = ToSnapshotOf::to_snapshot(@@@@@@1);
    }
}

