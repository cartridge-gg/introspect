use crate::{ToSnapshotBase, ToSnapshotOf};

pub trait SnapForward<T> {
    type SnapForward;
    #[inline(always)]
    const fn snap_forward(self: @T) -> Self::SnapForward nopanic;
}

pub trait SnapForwardDeep<T> {
    type SnapForward;
    #[inline(always)]
    const fn snap_forward(self: @T) -> Self::SnapForward nopanic;
}


pub trait SnapForwardTo<From, To> {
    #[inline(always)]
    const fn snap_forward(self: @From) -> To nopanic;
}

impl TupleSize0SnapForward of SnapForward<()> {
    type SnapForward = ();
    const fn snap_forward(self: @()) nopanic {
        ()
    }
}

impl TupleSize1SnapForward<E0> of SnapForward<(E0,)> {
    type SnapForward = (@E0,);
    const fn snap_forward(self: @(E0,)) -> (@E0,) nopanic {
        let (e0,) = self;
        (e0,)
    }
}

impl TupleSize2SnapForward<E0, E1> of SnapForward<(E0, E1)> {
    type SnapForward = (@E0, @E1);
    const fn snap_forward(self: @(E0, E1)) -> (@E0, @E1) nopanic {
        let (e0, e1) = self;
        (e0, e1)
    }
}

impl TupleSize3SnapForward<E0, E1, E2> of SnapForward<(E0, E1, E2)> {
    type SnapForward = (@E0, @E1, @E2);
    const fn snap_forward(self: @(E0, E1, E2)) -> (@E0, @E1, @E2) nopanic {
        let (e0, e1, e2) = self;
        (e0, e1, e2)
    }
}

impl TupleSize4SnapForward<E0, E1, E2, E3> of SnapForward<(E0, E1, E2, E3)> {
    type SnapForward = (@E0, @E1, @E2, @E3);
    const fn snap_forward(self: @(E0, E1, E2, E3)) -> (@E0, @E1, @E2, @E3) nopanic {
        let (e0, e1, e2, e3) = self;
        (e0, e1, e2, e3)
    }
}

impl TupleSize5SnapForward<E0, E1, E2, E3, E4> of SnapForward<(E0, E1, E2, E3, E4)> {
    type SnapForward = (@E0, @E1, @E2, @E3, @E4);
    const fn snap_forward(self: @(E0, E1, E2, E3, E4)) -> (@E0, @E1, @E2, @E3, @E4) nopanic {
        let (e0, e1, e2, e3, e4) = self;
        (e0, e1, e2, e3, e4)
    }
}

impl TupleSize6SnapForward<E0, E1, E2, E3, E4, E5> of SnapForward<(E0, E1, E2, E3, E4, E5)> {
    type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5);
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5),
    ) -> (@E0, @E1, @E2, @E3, @E4, @E5) nopanic {
        let (e0, e1, e2, e3, e4, e5) = self;
        (e0, e1, e2, e3, e4, e5)
    }
}

impl TupleSize7SnapForward<
    E0, E1, E2, E3, E4, E5, E6,
> of SnapForward<(E0, E1, E2, E3, E4, E5, E6)> {
    type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5, @E6);
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6),
    ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6) = self;
        (e0, e1, e2, e3, e4, e5, e6)
    }
}

impl TupleSize8SnapForward<
    E0, E1, E2, E3, E4, E5, E6, E7,
> of SnapForward<(E0, E1, E2, E3, E4, E5, E6, E7)> {
    type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7);
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7),
    ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7) = self;
        (e0, e1, e2, e3, e4, e5, e6, e7)
    }
}

impl TupleSize9SnapForward<
    E0, E1, E2, E3, E4, E5, E6, E7, E8,
> of SnapForward<(E0, E1, E2, E3, E4, E5, E6, E7, E8)> {
    type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8);
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8),
    ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8) = self;
        (e0, e1, e2, e3, e4, e5, e6, e7, e8)
    }
}

impl TupleSize10SnapForward<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9,
> of SnapForward<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9)> {
    type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9);
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9),
    ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9) = self;
        (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9)
    }
}

impl TupleSize11SnapForward<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10,
> of SnapForward<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10)> {
    type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10);
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10),
    ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10) = self;
        (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10)
    }
}

impl TupleSize12SnapForward<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11,
> of SnapForward<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11)> {
    type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11);
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11),
    ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11) = self;
        (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11)
    }
}

impl TupleSize13SnapForward<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12,
> of SnapForward<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12)> {
    type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12);
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12),
    ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12) = self;
        (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12)
    }
}

impl TupleSize14SnapForward<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13,
> of SnapForward<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13)> {
    type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12, @E13);
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13),
    ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12, @E13) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13) = self;
        (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13)
    }
}

impl TupleSize15SnapForward<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14,
> of SnapForward<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14)> {
    type SnapForward = (
        @E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12, @E13, @E14,
    );
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14),
    ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12, @E13, @E14) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14) = self;
        (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14)
    }
}

impl TupleSize16SnapForward<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15,
> of SnapForward<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15)> {
    type SnapForward = (
        @E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12, @E13, @E14, @E15,
    );
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15),
    ) -> (
        @E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12, @E13, @E14, @E15,
    ) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15) = self;
        (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15)
    }
}

impl TupleSnapForwardSnapshot<T, impl TS: SnapForwardDeep<T>> of SnapForwardDeep<@T> {
    type SnapForward = TS::SnapForward;
    #[inline(always)]
    const fn snap_forward(self: @@T) -> Self::SnapForward nopanic {
        (*self).snap_forward()
    }
}

impl TupleSize0SnapForwardDeep of SnapForwardDeep<()> {
    type SnapForward = ();
    #[inline(always)]
    const fn snap_forward(self: @()) nopanic {
        ()
    }
}

impl TupleSize1SnapForwardDeep<E0, impl TS: ToSnapshotBase<@E0>> of SnapForwardDeep<(E0,)> {
    type SnapForward = (@TS::Base,);
    #[inline(always)]
    const fn snap_forward(self: @(E0,)) -> Self::SnapForward nopanic {
        let (e0,) = self;
        (TS::to_snapshot(e0),)
    }
}

impl TupleSize2SnapForwardDeep<
    E0, E1, impl TS0: ToSnapshotBase<@E0>, impl TS1: ToSnapshotBase<@E1>,
> of SnapForwardDeep<(E0, E1)> {
    type SnapForward = (@TS0::Base, @TS1::Base);
    #[inline(always)]
    const fn snap_forward(self: @(E0, E1)) -> Self::SnapForward nopanic {
        let (e0, e1) = self;
        (TS0::to_snapshot(e0), TS1::to_snapshot(e1))
    }
}

impl TupleSize3SnapForwardDeep<
    E0,
    E1,
    E2,
    impl TS0: ToSnapshotBase<@E0>,
    impl TS1: ToSnapshotBase<@E1>,
    impl TS2: ToSnapshotBase<@E2>,
> of SnapForwardDeep<(E0, E1, E2)> {
    type SnapForward = (@TS0::Base, @TS1::Base, @TS2::Base);
    #[inline(always)]
    const fn snap_forward(self: @(E0, E1, E2)) -> Self::SnapForward nopanic {
        let (e0, e1, e2) = self;
        (TS0::to_snapshot(e0), TS1::to_snapshot(e1), TS2::to_snapshot(e2))
    }
}

impl TupleSize4SnapForwardDeep<
    E0,
    E1,
    E2,
    E3,
    impl TS0: ToSnapshotBase<@E0>,
    impl TS1: ToSnapshotBase<@E1>,
    impl TS2: ToSnapshotBase<@E2>,
    impl TS3: ToSnapshotBase<@E3>,
> of SnapForwardDeep<(E0, E1, E2, E3)> {
    type SnapForward = (@TS0::Base, @TS1::Base, @TS2::Base, @TS3::Base);
    #[inline(always)]
    const fn snap_forward(self: @(E0, E1, E2, E3)) -> Self::SnapForward nopanic {
        let (e0, e1, e2, e3) = self;
        (TS0::to_snapshot(e0), TS1::to_snapshot(e1), TS2::to_snapshot(e2), TS3::to_snapshot(e3))
    }
}

impl TupleSize5SnapForwardDeep<
    E0,
    E1,
    E2,
    E3,
    E4,
    impl TS0: ToSnapshotBase<@E0>,
    impl TS1: ToSnapshotBase<@E1>,
    impl TS2: ToSnapshotBase<@E2>,
    impl TS3: ToSnapshotBase<@E3>,
    impl TS4: ToSnapshotBase<@E4>,
> of SnapForwardDeep<(E0, E1, E2, E3, E4)> {
    type SnapForward = (@TS0::Base, @TS1::Base, @TS2::Base, @TS3::Base, @TS4::Base);
    #[inline(always)]
    const fn snap_forward(self: @(E0, E1, E2, E3, E4)) -> Self::SnapForward nopanic {
        let (e0, e1, e2, e3, e4) = self;
        (
            TS0::to_snapshot(e0),
            TS1::to_snapshot(e1),
            TS2::to_snapshot(e2),
            TS3::to_snapshot(e3),
            TS4::to_snapshot(e4),
        )
    }
}

impl TupleSize6SnapForwardDeep<
    E0,
    E1,
    E2,
    E3,
    E4,
    E5,
    impl TS0: ToSnapshotBase<@E0>,
    impl TS1: ToSnapshotBase<@E1>,
    impl TS2: ToSnapshotBase<@E2>,
    impl TS3: ToSnapshotBase<@E3>,
    impl TS4: ToSnapshotBase<@E4>,
    impl TS5: ToSnapshotBase<@E5>,
> of SnapForwardDeep<(E0, E1, E2, E3, E4, E5)> {
    type SnapForward = (@TS0::Base, @TS1::Base, @TS2::Base, @TS3::Base, @TS4::Base, @TS5::Base);
    #[inline(always)]
    const fn snap_forward(self: @(E0, E1, E2, E3, E4, E5)) -> Self::SnapForward nopanic {
        let (e0, e1, e2, e3, e4, e5) = self;
        (
            TS0::to_snapshot(e0),
            TS1::to_snapshot(e1),
            TS2::to_snapshot(e2),
            TS3::to_snapshot(e3),
            TS4::to_snapshot(e4),
            TS5::to_snapshot(e5),
        )
    }
}

impl TupleSize7SnapForwardDeep<
    E0,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    impl TS0: ToSnapshotBase<@E0>,
    impl TS1: ToSnapshotBase<@E1>,
    impl TS2: ToSnapshotBase<@E2>,
    impl TS3: ToSnapshotBase<@E3>,
    impl TS4: ToSnapshotBase<@E4>,
    impl TS5: ToSnapshotBase<@E5>,
    impl TS6: ToSnapshotBase<@E6>,
> of SnapForwardDeep<(E0, E1, E2, E3, E4, E5, E6)> {
    type SnapForward = (
        @TS0::Base, @TS1::Base, @TS2::Base, @TS3::Base, @TS4::Base, @TS5::Base, @TS6::Base,
    );
    #[inline(always)]
    const fn snap_forward(self: @(E0, E1, E2, E3, E4, E5, E6)) -> Self::SnapForward nopanic {
        let (e0, e1, e2, e3, e4, e5, e6) = self;
        (
            TS0::to_snapshot(e0),
            TS1::to_snapshot(e1),
            TS2::to_snapshot(e2),
            TS3::to_snapshot(e3),
            TS4::to_snapshot(e4),
            TS5::to_snapshot(e5),
            TS6::to_snapshot(e6),
        )
    }
}

impl TupleSize8SnapForwardDeep<
    E0,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    impl TS0: ToSnapshotBase<@E0>,
    impl TS1: ToSnapshotBase<@E1>,
    impl TS2: ToSnapshotBase<@E2>,
    impl TS3: ToSnapshotBase<@E3>,
    impl TS4: ToSnapshotBase<@E4>,
    impl TS5: ToSnapshotBase<@E5>,
    impl TS6: ToSnapshotBase<@E6>,
    impl TS7: ToSnapshotBase<@E7>,
> of SnapForwardDeep<(E0, E1, E2, E3, E4, E5, E6, E7)> {
    type SnapForward = (
        @TS0::Base,
        @TS1::Base,
        @TS2::Base,
        @TS3::Base,
        @TS4::Base,
        @TS5::Base,
        @TS6::Base,
        @TS7::Base,
    );
    #[inline(always)]
    const fn snap_forward(self: @(E0, E1, E2, E3, E4, E5, E6, E7)) -> Self::SnapForward nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7) = self;
        (
            TS0::to_snapshot(e0),
            TS1::to_snapshot(e1),
            TS2::to_snapshot(e2),
            TS3::to_snapshot(e3),
            TS4::to_snapshot(e4),
            TS5::to_snapshot(e5),
            TS6::to_snapshot(e6),
            TS7::to_snapshot(e7),
        )
    }
}

impl TupleSize9SnapForwardDeep<
    E0,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    impl TS0: ToSnapshotBase<@E0>,
    impl TS1: ToSnapshotBase<@E1>,
    impl TS2: ToSnapshotBase<@E2>,
    impl TS3: ToSnapshotBase<@E3>,
    impl TS4: ToSnapshotBase<@E4>,
    impl TS5: ToSnapshotBase<@E5>,
    impl TS6: ToSnapshotBase<@E6>,
    impl TS7: ToSnapshotBase<@E7>,
    impl TS8: ToSnapshotBase<@E8>,
> of SnapForwardDeep<(E0, E1, E2, E3, E4, E5, E6, E7, E8)> {
    type SnapForward = (
        @TS0::Base,
        @TS1::Base,
        @TS2::Base,
        @TS3::Base,
        @TS4::Base,
        @TS5::Base,
        @TS6::Base,
        @TS7::Base,
        @TS8::Base,
    );
    #[inline(always)]
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8),
    ) -> Self::SnapForward nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8) = self;
        (
            TS0::to_snapshot(e0),
            TS1::to_snapshot(e1),
            TS2::to_snapshot(e2),
            TS3::to_snapshot(e3),
            TS4::to_snapshot(e4),
            TS5::to_snapshot(e5),
            TS6::to_snapshot(e6),
            TS7::to_snapshot(e7),
            TS8::to_snapshot(e8),
        )
    }
}

impl TupleSize10SnapForwardDeep<
    E0,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    E9,
    impl TS0: ToSnapshotBase<@E0>,
    impl TS1: ToSnapshotBase<@E1>,
    impl TS2: ToSnapshotBase<@E2>,
    impl TS3: ToSnapshotBase<@E3>,
    impl TS4: ToSnapshotBase<@E4>,
    impl TS5: ToSnapshotBase<@E5>,
    impl TS6: ToSnapshotBase<@E6>,
    impl TS7: ToSnapshotBase<@E7>,
    impl TS8: ToSnapshotBase<@E8>,
    impl TS9: ToSnapshotBase<@E9>,
> of SnapForwardDeep<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9)> {
    type SnapForward = (
        @TS0::Base,
        @TS1::Base,
        @TS2::Base,
        @TS3::Base,
        @TS4::Base,
        @TS5::Base,
        @TS6::Base,
        @TS7::Base,
        @TS8::Base,
        @TS9::Base,
    );
    #[inline(always)]
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9),
    ) -> Self::SnapForward nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9) = self;
        (
            TS0::to_snapshot(e0),
            TS1::to_snapshot(e1),
            TS2::to_snapshot(e2),
            TS3::to_snapshot(e3),
            TS4::to_snapshot(e4),
            TS5::to_snapshot(e5),
            TS6::to_snapshot(e6),
            TS7::to_snapshot(e7),
            TS8::to_snapshot(e8),
            TS9::to_snapshot(e9),
        )
    }
}

impl TupleSize11SnapForwardDeep<
    E0,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    E9,
    E10,
    impl TS0: ToSnapshotBase<@E0>,
    impl TS1: ToSnapshotBase<@E1>,
    impl TS2: ToSnapshotBase<@E2>,
    impl TS3: ToSnapshotBase<@E3>,
    impl TS4: ToSnapshotBase<@E4>,
    impl TS5: ToSnapshotBase<@E5>,
    impl TS6: ToSnapshotBase<@E6>,
    impl TS7: ToSnapshotBase<@E7>,
    impl TS8: ToSnapshotBase<@E8>,
    impl TS9: ToSnapshotBase<@E9>,
    impl TS10: ToSnapshotBase<@E10>,
> of SnapForwardDeep<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10)> {
    type SnapForward = (
        @TS0::Base,
        @TS1::Base,
        @TS2::Base,
        @TS3::Base,
        @TS4::Base,
        @TS5::Base,
        @TS6::Base,
        @TS7::Base,
        @TS8::Base,
        @TS9::Base,
        @TS10::Base,
    );
    #[inline(always)]
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10),
    ) -> Self::SnapForward nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10) = self;
        (
            TS0::to_snapshot(e0),
            TS1::to_snapshot(e1),
            TS2::to_snapshot(e2),
            TS3::to_snapshot(e3),
            TS4::to_snapshot(e4),
            TS5::to_snapshot(e5),
            TS6::to_snapshot(e6),
            TS7::to_snapshot(e7),
            TS8::to_snapshot(e8),
            TS9::to_snapshot(e9),
            TS10::to_snapshot(e10),
        )
    }
}

impl TupleSize12SnapForwardDeep<
    E0,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    E9,
    E10,
    E11,
    impl TS0: ToSnapshotBase<@E0>,
    impl TS1: ToSnapshotBase<@E1>,
    impl TS2: ToSnapshotBase<@E2>,
    impl TS3: ToSnapshotBase<@E3>,
    impl TS4: ToSnapshotBase<@E4>,
    impl TS5: ToSnapshotBase<@E5>,
    impl TS6: ToSnapshotBase<@E6>,
    impl TS7: ToSnapshotBase<@E7>,
    impl TS8: ToSnapshotBase<@E8>,
    impl TS9: ToSnapshotBase<@E9>,
    impl TS10: ToSnapshotBase<@E10>,
    impl TS11: ToSnapshotBase<@E11>,
> of SnapForwardDeep<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11)> {
    type SnapForward = (
        @TS0::Base,
        @TS1::Base,
        @TS2::Base,
        @TS3::Base,
        @TS4::Base,
        @TS5::Base,
        @TS6::Base,
        @TS7::Base,
        @TS8::Base,
        @TS9::Base,
        @TS10::Base,
        @TS11::Base,
    );
    #[inline(always)]
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11),
    ) -> Self::SnapForward nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11) = self;
        (
            TS0::to_snapshot(e0),
            TS1::to_snapshot(e1),
            TS2::to_snapshot(e2),
            TS3::to_snapshot(e3),
            TS4::to_snapshot(e4),
            TS5::to_snapshot(e5),
            TS6::to_snapshot(e6),
            TS7::to_snapshot(e7),
            TS8::to_snapshot(e8),
            TS9::to_snapshot(e9),
            TS10::to_snapshot(e10),
            TS11::to_snapshot(e11),
        )
    }
}

impl TupleSize13SnapForwardDeep<
    E0,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    E9,
    E10,
    E11,
    E12,
    impl TS0: ToSnapshotBase<@E0>,
    impl TS1: ToSnapshotBase<@E1>,
    impl TS2: ToSnapshotBase<@E2>,
    impl TS3: ToSnapshotBase<@E3>,
    impl TS4: ToSnapshotBase<@E4>,
    impl TS5: ToSnapshotBase<@E5>,
    impl TS6: ToSnapshotBase<@E6>,
    impl TS7: ToSnapshotBase<@E7>,
    impl TS8: ToSnapshotBase<@E8>,
    impl TS9: ToSnapshotBase<@E9>,
    impl TS10: ToSnapshotBase<@E10>,
    impl TS11: ToSnapshotBase<@E11>,
    impl TS12: ToSnapshotBase<@E12>,
> of SnapForwardDeep<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12)> {
    type SnapForward = (
        @TS0::Base,
        @TS1::Base,
        @TS2::Base,
        @TS3::Base,
        @TS4::Base,
        @TS5::Base,
        @TS6::Base,
        @TS7::Base,
        @TS8::Base,
        @TS9::Base,
        @TS10::Base,
        @TS11::Base,
        @TS12::Base,
    );
    #[inline(always)]
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12),
    ) -> Self::SnapForward nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12) = self;
        (
            TS0::to_snapshot(e0),
            TS1::to_snapshot(e1),
            TS2::to_snapshot(e2),
            TS3::to_snapshot(e3),
            TS4::to_snapshot(e4),
            TS5::to_snapshot(e5),
            TS6::to_snapshot(e6),
            TS7::to_snapshot(e7),
            TS8::to_snapshot(e8),
            TS9::to_snapshot(e9),
            TS10::to_snapshot(e10),
            TS11::to_snapshot(e11),
            TS12::to_snapshot(e12),
        )
    }
}

impl TupleSize14SnapForwardDeep<
    E0,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    E9,
    E10,
    E11,
    E12,
    E13,
    impl TS0: ToSnapshotBase<@E0>,
    impl TS1: ToSnapshotBase<@E1>,
    impl TS2: ToSnapshotBase<@E2>,
    impl TS3: ToSnapshotBase<@E3>,
    impl TS4: ToSnapshotBase<@E4>,
    impl TS5: ToSnapshotBase<@E5>,
    impl TS6: ToSnapshotBase<@E6>,
    impl TS7: ToSnapshotBase<@E7>,
    impl TS8: ToSnapshotBase<@E8>,
    impl TS9: ToSnapshotBase<@E9>,
    impl TS10: ToSnapshotBase<@E10>,
    impl TS11: ToSnapshotBase<@E11>,
    impl TS12: ToSnapshotBase<@E12>,
    impl TS13: ToSnapshotBase<@E13>,
> of SnapForwardDeep<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13)> {
    type SnapForward = (
        @TS0::Base,
        @TS1::Base,
        @TS2::Base,
        @TS3::Base,
        @TS4::Base,
        @TS5::Base,
        @TS6::Base,
        @TS7::Base,
        @TS8::Base,
        @TS9::Base,
        @TS10::Base,
        @TS11::Base,
        @TS12::Base,
        @TS13::Base,
    );
    #[inline(always)]
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13),
    ) -> Self::SnapForward nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13) = self;
        (
            TS0::to_snapshot(e0),
            TS1::to_snapshot(e1),
            TS2::to_snapshot(e2),
            TS3::to_snapshot(e3),
            TS4::to_snapshot(e4),
            TS5::to_snapshot(e5),
            TS6::to_snapshot(e6),
            TS7::to_snapshot(e7),
            TS8::to_snapshot(e8),
            TS9::to_snapshot(e9),
            TS10::to_snapshot(e10),
            TS11::to_snapshot(e11),
            TS12::to_snapshot(e12),
            TS13::to_snapshot(e13),
        )
    }
}

impl TupleSize15SnapForwardDeep<
    E0,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    E9,
    E10,
    E11,
    E12,
    E13,
    E14,
    impl TS0: ToSnapshotBase<@E0>,
    impl TS1: ToSnapshotBase<@E1>,
    impl TS2: ToSnapshotBase<@E2>,
    impl TS3: ToSnapshotBase<@E3>,
    impl TS4: ToSnapshotBase<@E4>,
    impl TS5: ToSnapshotBase<@E5>,
    impl TS6: ToSnapshotBase<@E6>,
    impl TS7: ToSnapshotBase<@E7>,
    impl TS8: ToSnapshotBase<@E8>,
    impl TS9: ToSnapshotBase<@E9>,
    impl TS10: ToSnapshotBase<@E10>,
    impl TS11: ToSnapshotBase<@E11>,
    impl TS12: ToSnapshotBase<@E12>,
    impl TS13: ToSnapshotBase<@E13>,
    impl TS14: ToSnapshotBase<@E14>,
> of SnapForwardDeep<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14)> {
    type SnapForward = (
        @TS0::Base,
        @TS1::Base,
        @TS2::Base,
        @TS3::Base,
        @TS4::Base,
        @TS5::Base,
        @TS6::Base,
        @TS7::Base,
        @TS8::Base,
        @TS9::Base,
        @TS10::Base,
        @TS11::Base,
        @TS12::Base,
        @TS13::Base,
        @TS14::Base,
    );
    #[inline(always)]
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14),
    ) -> Self::SnapForward nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14) = self;
        (
            TS0::to_snapshot(e0),
            TS1::to_snapshot(e1),
            TS2::to_snapshot(e2),
            TS3::to_snapshot(e3),
            TS4::to_snapshot(e4),
            TS5::to_snapshot(e5),
            TS6::to_snapshot(e6),
            TS7::to_snapshot(e7),
            TS8::to_snapshot(e8),
            TS9::to_snapshot(e9),
            TS10::to_snapshot(e10),
            TS11::to_snapshot(e11),
            TS12::to_snapshot(e12),
            TS13::to_snapshot(e13),
            TS14::to_snapshot(e14),
        )
    }
}

impl TupleSize16SnapForwardDeep<
    E0,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    E9,
    E10,
    E11,
    E12,
    E13,
    E14,
    E15,
    impl TS0: ToSnapshotBase<@E0>,
    impl TS1: ToSnapshotBase<@E1>,
    impl TS2: ToSnapshotBase<@E2>,
    impl TS3: ToSnapshotBase<@E3>,
    impl TS4: ToSnapshotBase<@E4>,
    impl TS5: ToSnapshotBase<@E5>,
    impl TS6: ToSnapshotBase<@E6>,
    impl TS7: ToSnapshotBase<@E7>,
    impl TS8: ToSnapshotBase<@E8>,
    impl TS9: ToSnapshotBase<@E9>,
    impl TS10: ToSnapshotBase<@E10>,
    impl TS11: ToSnapshotBase<@E11>,
    impl TS12: ToSnapshotBase<@E12>,
    impl TS13: ToSnapshotBase<@E13>,
    impl TS14: ToSnapshotBase<@E14>,
    impl TS15: ToSnapshotBase<@E15>,
> of SnapForwardDeep<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15)> {
    type SnapForward = (
        @TS0::Base,
        @TS1::Base,
        @TS2::Base,
        @TS3::Base,
        @TS4::Base,
        @TS5::Base,
        @TS6::Base,
        @TS7::Base,
        @TS8::Base,
        @TS9::Base,
        @TS10::Base,
        @TS11::Base,
        @TS12::Base,
        @TS13::Base,
        @TS14::Base,
        @TS15::Base,
    );
    #[inline(always)]
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15),
    ) -> Self::SnapForward nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15) = self;
        (
            TS0::to_snapshot(e0),
            TS1::to_snapshot(e1),
            TS2::to_snapshot(e2),
            TS3::to_snapshot(e3),
            TS4::to_snapshot(e4),
            TS5::to_snapshot(e5),
            TS6::to_snapshot(e6),
            TS7::to_snapshot(e7),
            TS8::to_snapshot(e8),
            TS9::to_snapshot(e9),
            TS10::to_snapshot(e10),
            TS11::to_snapshot(e11),
            TS12::to_snapshot(e12),
            TS13::to_snapshot(e13),
            TS14::to_snapshot(e14),
            TS15::to_snapshot(e15),
        )
    }
}

impl TupleSnapshotSnapForwardTo<T, S, impl TS: SnapForwardTo<T, S>> of SnapForwardTo<@T, S> {
    #[inline(always)]
    const fn snap_forward(self: @@T) -> S nopanic {
        TS::snap_forward(*self)
    }
}

impl TupleSize0SnapForwardTo of SnapForwardTo<(), ()> {
    #[inline(always)]
    const fn snap_forward(self: @()) nopanic {
        ()
    }
}

impl TupleSize1SnapForwardTo<
    E0, S0, impl SS0: ToSnapshotOf<@E0, S0>,
> of SnapForwardTo<(E0,), (@S0,)> {
    #[inline(always)]
    const fn snap_forward(self: @(E0,)) -> (@S0,) nopanic {
        let (e0,) = self;
        (SS0::to_snapshot(e0),)
    }
}

impl TupleSize2SnapForwardTo<
    E0, E1, S0, S1, impl SS0: ToSnapshotOf<@E0, S0>, impl SS1: ToSnapshotOf<@E1, S1>,
> of SnapForwardTo<(E0, E1), (@S0, @S1)> {
    #[inline(always)]
    const fn snap_forward(self: @(E0, E1)) -> (@S0, @S1) nopanic {
        let (e0, e1) = self;
        (SS0::to_snapshot(e0), SS1::to_snapshot(e1))
    }
}


impl TupleSize3SnapForwardTo<
    E0,
    E1,
    E2,
    S0,
    S1,
    S2,
    impl SS0: ToSnapshotOf<@E0, S0>,
    impl SS1: ToSnapshotOf<@E1, S1>,
    impl SS2: ToSnapshotOf<@E2, S2>,
> of SnapForwardTo<(E0, E1, E2), (@S0, @S1, @S2)> {
    #[inline(always)]
    const fn snap_forward(self: @(E0, E1, E2)) -> (@S0, @S1, @S2) nopanic {
        let (e0, e1, e2) = self;
        (SS0::to_snapshot(e0), SS1::to_snapshot(e1), SS2::to_snapshot(e2))
    }
}

impl TupleSize4SnapForwardTo<
    E0,
    E1,
    E2,
    E3,
    S0,
    S1,
    S2,
    S3,
    impl SS0: ToSnapshotOf<@E0, S0>,
    impl SS1: ToSnapshotOf<@E1, S1>,
    impl SS2: ToSnapshotOf<@E2, S2>,
    impl SS3: ToSnapshotOf<@E3, S3>,
> of SnapForwardTo<(E0, E1, E2, E3), (@S0, @S1, @S2, @S3)> {
    #[inline(always)]
    const fn snap_forward(self: @(E0, E1, E2, E3)) -> (@S0, @S1, @S2, @S3) nopanic {
        let (e0, e1, e2, e3) = self;
        (SS0::to_snapshot(e0), SS1::to_snapshot(e1), SS2::to_snapshot(e2), SS3::to_snapshot(e3))
    }
}

impl TupleSize5SnapForwardTo<
    E0,
    E1,
    E2,
    E3,
    E4,
    S0,
    S1,
    S2,
    S3,
    S4,
    impl SS0: ToSnapshotOf<@E0, S0>,
    impl SS1: ToSnapshotOf<@E1, S1>,
    impl SS2: ToSnapshotOf<@E2, S2>,
    impl SS3: ToSnapshotOf<@E3, S3>,
    impl SS4: ToSnapshotOf<@E4, S4>,
> of SnapForwardTo<(E0, E1, E2, E3, E4), (@S0, @S1, @S2, @S3, @S4)> {
    #[inline(always)]
    const fn snap_forward(self: @(E0, E1, E2, E3, E4)) -> (@S0, @S1, @S2, @S3, @S4) nopanic {
        let (e0, e1, e2, e3, e4) = self;
        (
            SS0::to_snapshot(e0),
            SS1::to_snapshot(e1),
            SS2::to_snapshot(e2),
            SS3::to_snapshot(e3),
            SS4::to_snapshot(e4),
        )
    }
}

impl TupleSize6SnapForwardTo<
    E0,
    E1,
    E2,
    E3,
    E4,
    E5,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    impl SS0: ToSnapshotOf<@E0, S0>,
    impl SS1: ToSnapshotOf<@E1, S1>,
    impl SS2: ToSnapshotOf<@E2, S2>,
    impl SS3: ToSnapshotOf<@E3, S3>,
    impl SS4: ToSnapshotOf<@E4, S4>,
    impl SS5: ToSnapshotOf<@E5, S5>,
> of SnapForwardTo<(E0, E1, E2, E3, E4, E5), (@S0, @S1, @S2, @S3, @S4, @S5)> {
    #[inline(always)]
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5),
    ) -> (@S0, @S1, @S2, @S3, @S4, @S5) nopanic {
        let (e0, e1, e2, e3, e4, e5) = self;
        (
            SS0::to_snapshot(e0),
            SS1::to_snapshot(e1),
            SS2::to_snapshot(e2),
            SS3::to_snapshot(e3),
            SS4::to_snapshot(e4),
            SS5::to_snapshot(e5),
        )
    }
}

impl TupleSize7SnapForwardTo<
    E0,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    impl SS0: ToSnapshotOf<@E0, S0>,
    impl SS1: ToSnapshotOf<@E1, S1>,
    impl SS2: ToSnapshotOf<@E2, S2>,
    impl SS3: ToSnapshotOf<@E3, S3>,
    impl SS4: ToSnapshotOf<@E4, S4>,
    impl SS5: ToSnapshotOf<@E5, S5>,
    impl SS6: ToSnapshotOf<@E6, S6>,
> of SnapForwardTo<(E0, E1, E2, E3, E4, E5, E6), (@S0, @S1, @S2, @S3, @S4, @S5, @S6)> {
    #[inline(always)]
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6),
    ) -> (@S0, @S1, @S2, @S3, @S4, @S5, @S6) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6) = self;
        (
            SS0::to_snapshot(e0),
            SS1::to_snapshot(e1),
            SS2::to_snapshot(e2),
            SS3::to_snapshot(e3),
            SS4::to_snapshot(e4),
            SS5::to_snapshot(e5),
            SS6::to_snapshot(e6),
        )
    }
}

impl TupleSize8SnapForwardTo<
    E0,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    impl SS0: ToSnapshotOf<@E0, S0>,
    impl SS1: ToSnapshotOf<@E1, S1>,
    impl SS2: ToSnapshotOf<@E2, S2>,
    impl SS3: ToSnapshotOf<@E3, S3>,
    impl SS4: ToSnapshotOf<@E4, S4>,
    impl SS5: ToSnapshotOf<@E5, S5>,
    impl SS6: ToSnapshotOf<@E6, S6>,
    impl SS7: ToSnapshotOf<@E7, S7>,
> of SnapForwardTo<(E0, E1, E2, E3, E4, E5, E6, E7), (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7)> {
    #[inline(always)]
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7),
    ) -> (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7) = self;
        (
            SS0::to_snapshot(e0),
            SS1::to_snapshot(e1),
            SS2::to_snapshot(e2),
            SS3::to_snapshot(e3),
            SS4::to_snapshot(e4),
            SS5::to_snapshot(e5),
            SS6::to_snapshot(e6),
            SS7::to_snapshot(e7),
        )
    }
}

impl TupleSize9SnapForwardTo<
    E0,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    impl SS0: ToSnapshotOf<@E0, S0>,
    impl SS1: ToSnapshotOf<@E1, S1>,
    impl SS2: ToSnapshotOf<@E2, S2>,
    impl SS3: ToSnapshotOf<@E3, S3>,
    impl SS4: ToSnapshotOf<@E4, S4>,
    impl SS5: ToSnapshotOf<@E5, S5>,
    impl SS6: ToSnapshotOf<@E6, S6>,
    impl SS7: ToSnapshotOf<@E7, S7>,
    impl SS8: ToSnapshotOf<@E8, S8>,
> of SnapForwardTo<
    (E0, E1, E2, E3, E4, E5, E6, E7, E8), (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8),
> {
    #[inline(always)]
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8),
    ) -> (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8) = self;
        (
            SS0::to_snapshot(e0),
            SS1::to_snapshot(e1),
            SS2::to_snapshot(e2),
            SS3::to_snapshot(e3),
            SS4::to_snapshot(e4),
            SS5::to_snapshot(e5),
            SS6::to_snapshot(e6),
            SS7::to_snapshot(e7),
            SS8::to_snapshot(e8),
        )
    }
}

impl TupleSize10SnapForwardTo<
    E0,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    E9,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    impl SS0: ToSnapshotOf<@E0, S0>,
    impl SS1: ToSnapshotOf<@E1, S1>,
    impl SS2: ToSnapshotOf<@E2, S2>,
    impl SS3: ToSnapshotOf<@E3, S3>,
    impl SS4: ToSnapshotOf<@E4, S4>,
    impl SS5: ToSnapshotOf<@E5, S5>,
    impl SS6: ToSnapshotOf<@E6, S6>,
    impl SS7: ToSnapshotOf<@E7, S7>,
    impl SS8: ToSnapshotOf<@E8, S8>,
    impl SS9: ToSnapshotOf<@E9, S9>,
> of SnapForwardTo<
    (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9), (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9),
> {
    #[inline(always)]
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9),
    ) -> (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9) = self;
        (
            SS0::to_snapshot(e0),
            SS1::to_snapshot(e1),
            SS2::to_snapshot(e2),
            SS3::to_snapshot(e3),
            SS4::to_snapshot(e4),
            SS5::to_snapshot(e5),
            SS6::to_snapshot(e6),
            SS7::to_snapshot(e7),
            SS8::to_snapshot(e8),
            SS9::to_snapshot(e9),
        )
    }
}

impl TupleSize11SnapForwardTo<
    E0,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    E9,
    E10,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    impl SS0: ToSnapshotOf<@E0, S0>,
    impl SS1: ToSnapshotOf<@E1, S1>,
    impl SS2: ToSnapshotOf<@E2, S2>,
    impl SS3: ToSnapshotOf<@E3, S3>,
    impl SS4: ToSnapshotOf<@E4, S4>,
    impl SS5: ToSnapshotOf<@E5, S5>,
    impl SS6: ToSnapshotOf<@E6, S6>,
    impl SS7: ToSnapshotOf<@E7, S7>,
    impl SS8: ToSnapshotOf<@E8, S8>,
    impl SS9: ToSnapshotOf<@E9, S9>,
    impl SS10: ToSnapshotOf<@E10, S10>,
> of SnapForwardTo<
    (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10),
    (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9, @S10),
> {
    #[inline(always)]
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10),
    ) -> (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9, @S10) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10) = self;
        (
            SS0::to_snapshot(e0),
            SS1::to_snapshot(e1),
            SS2::to_snapshot(e2),
            SS3::to_snapshot(e3),
            SS4::to_snapshot(e4),
            SS5::to_snapshot(e5),
            SS6::to_snapshot(e6),
            SS7::to_snapshot(e7),
            SS8::to_snapshot(e8),
            SS9::to_snapshot(e9),
            SS10::to_snapshot(e10),
        )
    }
}

impl TupleSize12SnapForwardTo<
    E0,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    E9,
    E10,
    E11,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    impl SS0: ToSnapshotOf<@E0, S0>,
    impl SS1: ToSnapshotOf<@E1, S1>,
    impl SS2: ToSnapshotOf<@E2, S2>,
    impl SS3: ToSnapshotOf<@E3, S3>,
    impl SS4: ToSnapshotOf<@E4, S4>,
    impl SS5: ToSnapshotOf<@E5, S5>,
    impl SS6: ToSnapshotOf<@E6, S6>,
    impl SS7: ToSnapshotOf<@E7, S7>,
    impl SS8: ToSnapshotOf<@E8, S8>,
    impl SS9: ToSnapshotOf<@E9, S9>,
    impl SS10: ToSnapshotOf<@E10, S10>,
    impl SS11: ToSnapshotOf<@E11, S11>,
> of SnapForwardTo<
    (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11),
    (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9, @S10, @S11),
> {
    #[inline(always)]
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11),
    ) -> (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9, @S10, @S11) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11) = self;
        (
            SS0::to_snapshot(e0),
            SS1::to_snapshot(e1),
            SS2::to_snapshot(e2),
            SS3::to_snapshot(e3),
            SS4::to_snapshot(e4),
            SS5::to_snapshot(e5),
            SS6::to_snapshot(e6),
            SS7::to_snapshot(e7),
            SS8::to_snapshot(e8),
            SS9::to_snapshot(e9),
            SS10::to_snapshot(e10),
            SS11::to_snapshot(e11),
        )
    }
}

impl TupleSize13SnapForwardTo<
    E0,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    E9,
    E10,
    E11,
    E12,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    S12,
    impl SS0: ToSnapshotOf<@E0, S0>,
    impl SS1: ToSnapshotOf<@E1, S1>,
    impl SS2: ToSnapshotOf<@E2, S2>,
    impl SS3: ToSnapshotOf<@E3, S3>,
    impl SS4: ToSnapshotOf<@E4, S4>,
    impl SS5: ToSnapshotOf<@E5, S5>,
    impl SS6: ToSnapshotOf<@E6, S6>,
    impl SS7: ToSnapshotOf<@E7, S7>,
    impl SS8: ToSnapshotOf<@E8, S8>,
    impl SS9: ToSnapshotOf<@E9, S9>,
    impl SS10: ToSnapshotOf<@E10, S10>,
    impl SS11: ToSnapshotOf<@E11, S11>,
    impl SS12: ToSnapshotOf<@E12, S12>,
> of SnapForwardTo<
    (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12),
    (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9, @S10, @S11, @S12),
> {
    #[inline(always)]
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12),
    ) -> (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9, @S10, @S11, @S12) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12) = self;
        (
            SS0::to_snapshot(e0),
            SS1::to_snapshot(e1),
            SS2::to_snapshot(e2),
            SS3::to_snapshot(e3),
            SS4::to_snapshot(e4),
            SS5::to_snapshot(e5),
            SS6::to_snapshot(e6),
            SS7::to_snapshot(e7),
            SS8::to_snapshot(e8),
            SS9::to_snapshot(e9),
            SS10::to_snapshot(e10),
            SS11::to_snapshot(e11),
            SS12::to_snapshot(e12),
        )
    }
}

impl TupleSize14SnapForwardTo<
    E0,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    E9,
    E10,
    E11,
    E12,
    E13,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    S12,
    S13,
    impl SS0: ToSnapshotOf<@E0, S0>,
    impl SS1: ToSnapshotOf<@E1, S1>,
    impl SS2: ToSnapshotOf<@E2, S2>,
    impl SS3: ToSnapshotOf<@E3, S3>,
    impl SS4: ToSnapshotOf<@E4, S4>,
    impl SS5: ToSnapshotOf<@E5, S5>,
    impl SS6: ToSnapshotOf<@E6, S6>,
    impl SS7: ToSnapshotOf<@E7, S7>,
    impl SS8: ToSnapshotOf<@E8, S8>,
    impl SS9: ToSnapshotOf<@E9, S9>,
    impl SS10: ToSnapshotOf<@E10, S10>,
    impl SS11: ToSnapshotOf<@E11, S11>,
    impl SS12: ToSnapshotOf<@E12, S12>,
    impl SS13: ToSnapshotOf<@E13, S13>,
> of SnapForwardTo<
    (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13),
    (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9, @S10, @S11, @S12, @S13),
> {
    #[inline(always)]
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13),
    ) -> (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9, @S10, @S11, @S12, @S13) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13) = self;
        (
            SS0::to_snapshot(e0),
            SS1::to_snapshot(e1),
            SS2::to_snapshot(e2),
            SS3::to_snapshot(e3),
            SS4::to_snapshot(e4),
            SS5::to_snapshot(e5),
            SS6::to_snapshot(e6),
            SS7::to_snapshot(e7),
            SS8::to_snapshot(e8),
            SS9::to_snapshot(e9),
            SS10::to_snapshot(e10),
            SS11::to_snapshot(e11),
            SS12::to_snapshot(e12),
            SS13::to_snapshot(e13),
        )
    }
}

impl TupleSize15SnapForwardTo<
    E0,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    E9,
    E10,
    E11,
    E12,
    E13,
    E14,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    S12,
    S13,
    S14,
    impl SS0: ToSnapshotOf<@E0, S0>,
    impl SS1: ToSnapshotOf<@E1, S1>,
    impl SS2: ToSnapshotOf<@E2, S2>,
    impl SS3: ToSnapshotOf<@E3, S3>,
    impl SS4: ToSnapshotOf<@E4, S4>,
    impl SS5: ToSnapshotOf<@E5, S5>,
    impl SS6: ToSnapshotOf<@E6, S6>,
    impl SS7: ToSnapshotOf<@E7, S7>,
    impl SS8: ToSnapshotOf<@E8, S8>,
    impl SS9: ToSnapshotOf<@E9, S9>,
    impl SS10: ToSnapshotOf<@E10, S10>,
    impl SS11: ToSnapshotOf<@E11, S11>,
    impl SS12: ToSnapshotOf<@E12, S12>,
    impl SS13: ToSnapshotOf<@E13, S13>,
    impl SS14: ToSnapshotOf<@E14, S14>,
> of SnapForwardTo<
    (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14),
    (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9, @S10, @S11, @S12, @S13, @S14),
> {
    #[inline(always)]
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14),
    ) -> (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9, @S10, @S11, @S12, @S13, @S14) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14) = self;
        (
            SS0::to_snapshot(e0),
            SS1::to_snapshot(e1),
            SS2::to_snapshot(e2),
            SS3::to_snapshot(e3),
            SS4::to_snapshot(e4),
            SS5::to_snapshot(e5),
            SS6::to_snapshot(e6),
            SS7::to_snapshot(e7),
            SS8::to_snapshot(e8),
            SS9::to_snapshot(e9),
            SS10::to_snapshot(e10),
            SS11::to_snapshot(e11),
            SS12::to_snapshot(e12),
            SS13::to_snapshot(e13),
            SS14::to_snapshot(e14),
        )
    }
}

impl TupleSize16SnapForwardTo<
    E0,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    E9,
    E10,
    E11,
    E12,
    E13,
    E14,
    E15,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    S12,
    S13,
    S14,
    S15,
    impl SS0: ToSnapshotOf<@E0, S0>,
    impl SS1: ToSnapshotOf<@E1, S1>,
    impl SS2: ToSnapshotOf<@E2, S2>,
    impl SS3: ToSnapshotOf<@E3, S3>,
    impl SS4: ToSnapshotOf<@E4, S4>,
    impl SS5: ToSnapshotOf<@E5, S5>,
    impl SS6: ToSnapshotOf<@E6, S6>,
    impl SS7: ToSnapshotOf<@E7, S7>,
    impl SS8: ToSnapshotOf<@E8, S8>,
    impl SS9: ToSnapshotOf<@E9, S9>,
    impl SS10: ToSnapshotOf<@E10, S10>,
    impl SS11: ToSnapshotOf<@E11, S11>,
    impl SS12: ToSnapshotOf<@E12, S12>,
    impl SS13: ToSnapshotOf<@E13, S13>,
    impl SS14: ToSnapshotOf<@E14, S14>,
    impl SS15: ToSnapshotOf<@E15, S15>,
> of SnapForwardTo<
    (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15),
    (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9, @S10, @S11, @S12, @S13, @S14, @S15),
> {
    #[inline(always)]
    const fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15),
    ) -> (
        @S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9, @S10, @S11, @S12, @S13, @S14, @S15,
    ) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15) = self;
        (
            SS0::to_snapshot(e0),
            SS1::to_snapshot(e1),
            SS2::to_snapshot(e2),
            SS3::to_snapshot(e3),
            SS4::to_snapshot(e4),
            SS5::to_snapshot(e5),
            SS6::to_snapshot(e6),
            SS7::to_snapshot(e7),
            SS8::to_snapshot(e8),
            SS9::to_snapshot(e9),
            SS10::to_snapshot(e10),
            SS11::to_snapshot(e11),
            SS12::to_snapshot(e12),
            SS13::to_snapshot(e13),
            SS14::to_snapshot(e14),
            SS15::to_snapshot(e15),
        )
    }
}

impl FixedSizedArraySize0SnapForward<T> of SnapForward<[T; 0]> {
    type SnapForward = [@T; 0];
    const fn snap_forward(self: @[T; 0]) -> [@T; 0] nopanic {
        []
    }
}

impl FixedSizedArraySize1SnapForward<T, impl SS: ToSnapshotBase<@T>> of SnapForward<[T; 1]> {
    type SnapForward = [@T; 1];
    const fn snap_forward(self: @[T; 1]) -> [@T; 1] nopanic {
        let [e0] = self;
        [e0]
    }
}

impl FixedSizedArraySize2SnapForward<T, impl SS: ToSnapshotBase<@T>> of SnapForward<[T; 2]> {
    type SnapForward = [@T; 2];
    const fn snap_forward(self: @[T; 2]) -> [@T; 2] nopanic {
        let [e0, e1] = self;
        [e0, e1]
    }
}

impl FixedSizedArraySize3SnapForward<T, impl SS: ToSnapshotBase<@T>> of SnapForward<[T; 3]> {
    type SnapForward = [@T; 3];
    const fn snap_forward(self: @[T; 3]) -> [@T; 3] nopanic {
        let [e0, e1, e2] = self;
        [e0, e1, e2]
    }
}

impl FixedSizedArraySize4SnapForward<T, impl SS: ToSnapshotBase<@T>> of SnapForward<[T; 4]> {
    type SnapForward = [@T; 4];
    const fn snap_forward(self: @[T; 4]) -> [@T; 4] nopanic {
        let [e0, e1, e2, e3] = self;
        [e0, e1, e2, e3]
    }
}

impl FixedSizedArraySize5SnapForward<T, impl SS: ToSnapshotBase<@T>> of SnapForward<[T; 5]> {
    type SnapForward = [@T; 5];
    const fn snap_forward(self: @[T; 5]) -> [@T; 5] nopanic {
        let [e0, e1, e2, e3, e4] = self;
        [e0, e1, e2, e3, e4]
    }
}

impl FixedSizedArraySize6SnapForward<T, impl SS: ToSnapshotBase<@T>> of SnapForward<[T; 6]> {
    type SnapForward = [@T; 6];
    const fn snap_forward(self: @[T; 6]) -> [@T; 6] nopanic {
        let [e0, e1, e2, e3, e4, e5] = self;
        [e0, e1, e2, e3, e4, e5]
    }
}

impl FixedSizedArraySize7SnapForward<T, impl SS: ToSnapshotBase<@T>> of SnapForward<[T; 7]> {
    type SnapForward = [@T; 7];
    const fn snap_forward(self: @[T; 7]) -> [@T; 7] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6] = self;
        [e0, e1, e2, e3, e4, e5, e6]
    }
}

impl FixedSizedArraySize8SnapForward<T, impl SS: ToSnapshotBase<@T>> of SnapForward<[T; 8]> {
    type SnapForward = [@T; 8];
    const fn snap_forward(self: @[T; 8]) -> [@T; 8] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7] = self;
        [e0, e1, e2, e3, e4, e5, e6, e7]
    }
}

impl FixedSizedArraySize9SnapForward<T, impl SS: ToSnapshotBase<@T>> of SnapForward<[T; 9]> {
    type SnapForward = [@T; 9];
    const fn snap_forward(self: @[T; 9]) -> [@T; 9] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8] = self;
        [e0, e1, e2, e3, e4, e5, e6, e7, e8]
    }
}

impl FixedSizedArraySize10SnapForward<T, impl SS: ToSnapshotBase<@T>> of SnapForward<[T; 10]> {
    type SnapForward = [@T; 10];
    const fn snap_forward(self: @[T; 10]) -> [@T; 10] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9] = self;
        [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9]
    }
}

impl FixedSizedArraySize11SnapForward<T, impl SS: ToSnapshotBase<@T>> of SnapForward<[T; 11]> {
    type SnapForward = [@T; 11];
    const fn snap_forward(self: @[T; 11]) -> [@T; 11] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10] = self;
        [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10]
    }
}

impl FixedSizedArraySize12SnapForward<T, impl SS: ToSnapshotBase<@T>> of SnapForward<[T; 12]> {
    type SnapForward = [@T; 12];
    const fn snap_forward(self: @[T; 12]) -> [@T; 12] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11] = self;
        [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11]
    }
}

impl FixedSizedArraySize13SnapForward<T, impl SS: ToSnapshotBase<@T>> of SnapForward<[T; 13]> {
    type SnapForward = [@T; 13];
    const fn snap_forward(self: @[T; 13]) -> [@T; 13] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12] = self;
        [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12]
    }
}

impl FixedSizedArraySize14SnapForward<T, impl SS: ToSnapshotBase<@T>> of SnapForward<[T; 14]> {
    type SnapForward = [@T; 14];
    const fn snap_forward(self: @[T; 14]) -> [@T; 14] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13] = self;
        [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13]
    }
}

impl FixedSizedArraySize15SnapForward<T, impl SS: ToSnapshotBase<@T>> of SnapForward<[T; 15]> {
    type SnapForward = [@T; 15];
    const fn snap_forward(self: @[T; 15]) -> [@T; 15] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14] = self;
        [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14]
    }
}

impl FixedSizedArraySize16SnapForward<T, impl SS: ToSnapshotBase<@T>> of SnapForward<[T; 16]> {
    type SnapForward = [@T; 16];
    const fn snap_forward(self: @[T; 16]) -> [@T; 16] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15] = self;
        [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15]
    }
}

impl FixedSizedArraySize0SnapForwardDeep<T> of SnapForwardDeep<[T; 0]> {
    type SnapForward = [@T; 0];
    const fn snap_forward(self: @[T; 0]) -> [@T; 0] nopanic {
        []
    }
}

impl FixedSizedArraySize1SnapForwardDeep<
    T, impl SS: ToSnapshotBase<@T>,
> of SnapForwardDeep<[T; 1]> {
    type SnapForward = [@SS::Base; 1];
    const fn snap_forward(self: @[T; 1]) -> [@SS::Base; 1] nopanic {
        let [e0] = self;
        [SS::to_snapshot(e0)]
    }
}

impl FixedSizedArraySize2SnapForwardDeep<
    T, impl SS: ToSnapshotBase<@T>,
> of SnapForwardDeep<[T; 2]> {
    type SnapForward = [@SS::Base; 2];
    const fn snap_forward(self: @[T; 2]) -> [@SS::Base; 2] nopanic {
        let [e0, e1] = self;
        [SS::to_snapshot(e0), SS::to_snapshot(e1)]
    }
}

impl FixedSizedArraySize3SnapForwardDeep<
    T, impl SS: ToSnapshotBase<@T>,
> of SnapForwardDeep<[T; 3]> {
    type SnapForward = [@SS::Base; 3];
    const fn snap_forward(self: @[T; 3]) -> [@SS::Base; 3] nopanic {
        let [e0, e1, e2] = self;
        [SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2)]
    }
}

impl FixedSizedArraySize4SnapForwardDeep<
    T, impl SS: ToSnapshotBase<@T>,
> of SnapForwardDeep<[T; 4]> {
    type SnapForward = [@SS::Base; 4];
    const fn snap_forward(self: @[T; 4]) -> [@SS::Base; 4] nopanic {
        let [e0, e1, e2, e3] = self;
        [SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2), SS::to_snapshot(e3)]
    }
}

impl FixedSizedArraySize5SnapForwardDeep<
    T, impl SS: ToSnapshotBase<@T>,
> of SnapForwardDeep<[T; 5]> {
    type SnapForward = [@SS::Base; 5];
    const fn snap_forward(self: @[T; 5]) -> [@SS::Base; 5] nopanic {
        let [e0, e1, e2, e3, e4] = self;
        [
            SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2), SS::to_snapshot(e3),
            SS::to_snapshot(e4),
        ]
    }
}

impl FixedSizedArraySize6SnapForwardDeep<
    T, impl SS: ToSnapshotBase<@T>,
> of SnapForwardDeep<[T; 6]> {
    type SnapForward = [@SS::Base; 6];
    const fn snap_forward(self: @[T; 6]) -> [@SS::Base; 6] nopanic {
        let [e0, e1, e2, e3, e4, e5] = self;
        [
            SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2), SS::to_snapshot(e3),
            SS::to_snapshot(e4), SS::to_snapshot(e5),
        ]
    }
}

impl FixedSizedArraySize7SnapForwardDeep<
    T, impl SS: ToSnapshotBase<@T>,
> of SnapForwardDeep<[T; 7]> {
    type SnapForward = [@SS::Base; 7];
    const fn snap_forward(self: @[T; 7]) -> [@SS::Base; 7] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6] = self;
        [
            SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2), SS::to_snapshot(e3),
            SS::to_snapshot(e4), SS::to_snapshot(e5), SS::to_snapshot(e6),
        ]
    }
}

impl FixedSizedArraySize8SnapForwardDeep<
    T, impl SS: ToSnapshotBase<@T>,
> of SnapForwardDeep<[T; 8]> {
    type SnapForward = [@SS::Base; 8];
    const fn snap_forward(self: @[T; 8]) -> [@SS::Base; 8] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7] = self;
        [
            SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2), SS::to_snapshot(e3),
            SS::to_snapshot(e4), SS::to_snapshot(e5), SS::to_snapshot(e6), SS::to_snapshot(e7),
        ]
    }
}

impl FixedSizedArraySize9SnapForwardDeep<
    T, impl SS: ToSnapshotBase<@T>,
> of SnapForwardDeep<[T; 9]> {
    type SnapForward = [@SS::Base; 9];
    const fn snap_forward(self: @[T; 9]) -> [@SS::Base; 9] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8] = self;
        [
            SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2), SS::to_snapshot(e3),
            SS::to_snapshot(e4), SS::to_snapshot(e5), SS::to_snapshot(e6), SS::to_snapshot(e7),
            SS::to_snapshot(e8),
        ]
    }
}

impl FixedSizedArraySize10SnapForwardDeep<
    T, impl SS: ToSnapshotBase<@T>,
> of SnapForwardDeep<[T; 10]> {
    type SnapForward = [@SS::Base; 10];
    const fn snap_forward(self: @[T; 10]) -> [@SS::Base; 10] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9] = self;
        [
            SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2), SS::to_snapshot(e3),
            SS::to_snapshot(e4), SS::to_snapshot(e5), SS::to_snapshot(e6), SS::to_snapshot(e7),
            SS::to_snapshot(e8), SS::to_snapshot(e9),
        ]
    }
}

impl FixedSizedArraySize11SnapForwardDeep<
    T, impl SS: ToSnapshotBase<@T>,
> of SnapForwardDeep<[T; 11]> {
    type SnapForward = [@SS::Base; 11];
    const fn snap_forward(self: @[T; 11]) -> [@SS::Base; 11] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10] = self;
        [
            SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2), SS::to_snapshot(e3),
            SS::to_snapshot(e4), SS::to_snapshot(e5), SS::to_snapshot(e6), SS::to_snapshot(e7),
            SS::to_snapshot(e8), SS::to_snapshot(e9), SS::to_snapshot(e10),
        ]
    }
}

impl FixedSizedArraySize12SnapForwardDeep<
    T, impl SS: ToSnapshotBase<@T>,
> of SnapForwardDeep<[T; 12]> {
    type SnapForward = [@SS::Base; 12];
    const fn snap_forward(self: @[T; 12]) -> [@SS::Base; 12] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11] = self;
        [
            SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2), SS::to_snapshot(e3),
            SS::to_snapshot(e4), SS::to_snapshot(e5), SS::to_snapshot(e6), SS::to_snapshot(e7),
            SS::to_snapshot(e8), SS::to_snapshot(e9), SS::to_snapshot(e10), SS::to_snapshot(e11),
        ]
    }
}

impl FixedSizedArraySize13SnapForwardDeep<
    T, impl SS: ToSnapshotBase<@T>,
> of SnapForwardDeep<[T; 13]> {
    type SnapForward = [@SS::Base; 13];
    const fn snap_forward(self: @[T; 13]) -> [@SS::Base; 13] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12] = self;
        [
            SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2), SS::to_snapshot(e3),
            SS::to_snapshot(e4), SS::to_snapshot(e5), SS::to_snapshot(e6), SS::to_snapshot(e7),
            SS::to_snapshot(e8), SS::to_snapshot(e9), SS::to_snapshot(e10), SS::to_snapshot(e11),
            SS::to_snapshot(e12),
        ]
    }
}

impl FixedSizedArraySize14SnapForwardDeep<
    T, impl SS: ToSnapshotBase<@T>,
> of SnapForwardDeep<[T; 14]> {
    type SnapForward = [@SS::Base; 14];
    const fn snap_forward(self: @[T; 14]) -> [@SS::Base; 14] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13] = self;
        [
            SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2), SS::to_snapshot(e3),
            SS::to_snapshot(e4), SS::to_snapshot(e5), SS::to_snapshot(e6), SS::to_snapshot(e7),
            SS::to_snapshot(e8), SS::to_snapshot(e9), SS::to_snapshot(e10), SS::to_snapshot(e11),
            SS::to_snapshot(e12), SS::to_snapshot(e13),
        ]
    }
}

impl FixedSizedArraySize15SnapForwardDeep<
    T, impl SS: ToSnapshotBase<@T>,
> of SnapForwardDeep<[T; 15]> {
    type SnapForward = [@SS::Base; 15];
    const fn snap_forward(self: @[T; 15]) -> [@SS::Base; 15] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14] = self;
        [
            SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2), SS::to_snapshot(e3),
            SS::to_snapshot(e4), SS::to_snapshot(e5), SS::to_snapshot(e6), SS::to_snapshot(e7),
            SS::to_snapshot(e8), SS::to_snapshot(e9), SS::to_snapshot(e10), SS::to_snapshot(e11),
            SS::to_snapshot(e12), SS::to_snapshot(e13), SS::to_snapshot(e14),
        ]
    }
}

impl FixedSizedArraySize16SnapForwardDeep<
    T, impl SS: ToSnapshotBase<@T>,
> of SnapForwardDeep<[T; 16]> {
    type SnapForward = [@SS::Base; 16];
    const fn snap_forward(self: @[T; 16]) -> [@SS::Base; 16] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15] = self;
        [
            SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2), SS::to_snapshot(e3),
            SS::to_snapshot(e4), SS::to_snapshot(e5), SS::to_snapshot(e6), SS::to_snapshot(e7),
            SS::to_snapshot(e8), SS::to_snapshot(e9), SS::to_snapshot(e10), SS::to_snapshot(e11),
            SS::to_snapshot(e12), SS::to_snapshot(e13), SS::to_snapshot(e14), SS::to_snapshot(e15),
        ]
    }
}


impl FixedSizedArraySize0SnapForwardTo<T, S> of SnapForwardTo<[T; 0], [@S; 0]> {
    #[inline(always)]
    const fn snap_forward(self: @[T; 0]) -> [@S; 0] nopanic {
        []
    }
}

impl FixedSizedArraySize1SnapForwardTo<
    T, S, impl SS: ToSnapshotOf<@T, S>,
> of SnapForwardTo<[T; 1], [@S; 1]> {
    #[inline(always)]
    const fn snap_forward(self: @[T; 1]) -> [@S; 1] nopanic {
        let [e0] = self;
        [SS::to_snapshot(e0)]
    }
}

impl FixedSizedArraySize2SnapForwardTo<
    T, S, impl SS: ToSnapshotOf<@T, S>,
> of SnapForwardTo<[T; 2], [@S; 2]> {
    #[inline(always)]
    const fn snap_forward(self: @[T; 2]) -> [@S; 2] nopanic {
        let [e0, e1] = self;
        [SS::to_snapshot(e0), SS::to_snapshot(e1)]
    }
}

impl FixedSizedArraySize3SnapForwardTo<
    T, S, impl SS: ToSnapshotOf<@T, S>,
> of SnapForwardTo<[T; 3], [@S; 3]> {
    #[inline(always)]
    const fn snap_forward(self: @[T; 3]) -> [@S; 3] nopanic {
        let [e0, e1, e2] = self;
        [SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2)]
    }
}

impl FixedSizedArraySize4SnapForwardTo<
    T, S, impl SS: ToSnapshotOf<@T, S>,
> of SnapForwardTo<[T; 4], [@S; 4]> {
    #[inline(always)]
    const fn snap_forward(self: @[T; 4]) -> [@S; 4] nopanic {
        let [e0, e1, e2, e3] = self;
        [SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2), SS::to_snapshot(e3)]
    }
}

impl FixedSizedArraySize5SnapForwardTo<
    T, S, impl SS: ToSnapshotOf<@T, S>,
> of SnapForwardTo<[T; 5], [@S; 5]> {
    #[inline(always)]
    const fn snap_forward(self: @[T; 5]) -> [@S; 5] nopanic {
        let [e0, e1, e2, e3, e4] = self;
        [
            SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2), SS::to_snapshot(e3),
            SS::to_snapshot(e4),
        ]
    }
}

impl FixedSizedArraySize6SnapForwardTo<
    T, S, impl SS: ToSnapshotOf<@T, S>,
> of SnapForwardTo<[T; 6], [@S; 6]> {
    #[inline(always)]
    const fn snap_forward(self: @[T; 6]) -> [@S; 6] nopanic {
        let [e0, e1, e2, e3, e4, e5] = self;
        [
            SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2), SS::to_snapshot(e3),
            SS::to_snapshot(e4), SS::to_snapshot(e5),
        ]
    }
}

impl FixedSizedArraySize7SnapForwardTo<
    T, S, impl SS: ToSnapshotOf<@T, S>,
> of SnapForwardTo<[T; 7], [@S; 7]> {
    #[inline(always)]
    const fn snap_forward(self: @[T; 7]) -> [@S; 7] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6] = self;
        [
            SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2), SS::to_snapshot(e3),
            SS::to_snapshot(e4), SS::to_snapshot(e5), SS::to_snapshot(e6),
        ]
    }
}

impl FixedSizedArraySize8SnapForwardTo<
    T, S, impl SS: ToSnapshotOf<@T, S>,
> of SnapForwardTo<[T; 8], [@S; 8]> {
    #[inline(always)]
    const fn snap_forward(self: @[T; 8]) -> [@S; 8] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7] = self;
        [
            SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2), SS::to_snapshot(e3),
            SS::to_snapshot(e4), SS::to_snapshot(e5), SS::to_snapshot(e6), SS::to_snapshot(e7),
        ]
    }
}

impl FixedSizedArraySize9SnapForwardTo<
    T, S, impl SS: ToSnapshotOf<@T, S>,
> of SnapForwardTo<[T; 9], [@S; 9]> {
    #[inline(always)]
    const fn snap_forward(self: @[T; 9]) -> [@S; 9] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8] = self;
        [
            SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2), SS::to_snapshot(e3),
            SS::to_snapshot(e4), SS::to_snapshot(e5), SS::to_snapshot(e6), SS::to_snapshot(e7),
            SS::to_snapshot(e8),
        ]
    }
}

impl FixedSizedArraySize10SnapForwardTo<
    T, S, impl SS: ToSnapshotOf<@T, S>,
> of SnapForwardTo<[T; 10], [@S; 10]> {
    #[inline(always)]
    const fn snap_forward(self: @[T; 10]) -> [@S; 10] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9] = self;
        [
            SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2), SS::to_snapshot(e3),
            SS::to_snapshot(e4), SS::to_snapshot(e5), SS::to_snapshot(e6), SS::to_snapshot(e7),
            SS::to_snapshot(e8), SS::to_snapshot(e9),
        ]
    }
}

impl FixedSizedArraySize11SnapForwardTo<
    T, S, impl SS: ToSnapshotOf<@T, S>,
> of SnapForwardTo<[T; 11], [@S; 11]> {
    #[inline(always)]
    const fn snap_forward(self: @[T; 11]) -> [@S; 11] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10] = self;
        [
            SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2), SS::to_snapshot(e3),
            SS::to_snapshot(e4), SS::to_snapshot(e5), SS::to_snapshot(e6), SS::to_snapshot(e7),
            SS::to_snapshot(e8), SS::to_snapshot(e9), SS::to_snapshot(e10),
        ]
    }
}

impl FixedSizedArraySize12SnapForwardTo<
    T, S, impl SS: ToSnapshotOf<@T, S>,
> of SnapForwardTo<[T; 12], [@S; 12]> {
    #[inline(always)]
    const fn snap_forward(self: @[T; 12]) -> [@S; 12] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11] = self;
        [
            SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2), SS::to_snapshot(e3),
            SS::to_snapshot(e4), SS::to_snapshot(e5), SS::to_snapshot(e6), SS::to_snapshot(e7),
            SS::to_snapshot(e8), SS::to_snapshot(e9), SS::to_snapshot(e10), SS::to_snapshot(e11),
        ]
    }
}

impl FixedSizedArraySize13SnapForwardTo<
    T, S, impl SS: ToSnapshotOf<@T, S>,
> of SnapForwardTo<[T; 13], [@S; 13]> {
    #[inline(always)]
    const fn snap_forward(self: @[T; 13]) -> [@S; 13] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12] = self;
        [
            SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2), SS::to_snapshot(e3),
            SS::to_snapshot(e4), SS::to_snapshot(e5), SS::to_snapshot(e6), SS::to_snapshot(e7),
            SS::to_snapshot(e8), SS::to_snapshot(e9), SS::to_snapshot(e10), SS::to_snapshot(e11),
            SS::to_snapshot(e12),
        ]
    }
}

impl FixedSizedArraySize14SnapForwardTo<
    T, S, impl SS: ToSnapshotOf<@T, S>,
> of SnapForwardTo<[T; 14], [@S; 14]> {
    #[inline(always)]
    const fn snap_forward(self: @[T; 14]) -> [@S; 14] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13] = self;
        [
            SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2), SS::to_snapshot(e3),
            SS::to_snapshot(e4), SS::to_snapshot(e5), SS::to_snapshot(e6), SS::to_snapshot(e7),
            SS::to_snapshot(e8), SS::to_snapshot(e9), SS::to_snapshot(e10), SS::to_snapshot(e11),
            SS::to_snapshot(e12), SS::to_snapshot(e13),
        ]
    }
}

impl FixedSizedArraySize15SnapForwardTo<
    T, S, impl SS: ToSnapshotOf<@T, S>,
> of SnapForwardTo<[T; 15], [@S; 15]> {
    #[inline(always)]
    const fn snap_forward(self: @[T; 15]) -> [@S; 15] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14] = self;
        [
            SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2), SS::to_snapshot(e3),
            SS::to_snapshot(e4), SS::to_snapshot(e5), SS::to_snapshot(e6), SS::to_snapshot(e7),
            SS::to_snapshot(e8), SS::to_snapshot(e9), SS::to_snapshot(e10), SS::to_snapshot(e11),
            SS::to_snapshot(e12), SS::to_snapshot(e13), SS::to_snapshot(e14),
        ]
    }
}

impl FixedSizedArraySize16SnapForwardTo<
    T, S, impl SS: ToSnapshotOf<@T, S>,
> of SnapForwardTo<[T; 16], [@S; 16]> {
    #[inline(always)]
    const fn snap_forward(self: @[T; 16]) -> [@S; 16] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15] = self;
        [
            SS::to_snapshot(e0), SS::to_snapshot(e1), SS::to_snapshot(e2), SS::to_snapshot(e3),
            SS::to_snapshot(e4), SS::to_snapshot(e5), SS::to_snapshot(e6), SS::to_snapshot(e7),
            SS::to_snapshot(e8), SS::to_snapshot(e9), SS::to_snapshot(e10), SS::to_snapshot(e11),
            SS::to_snapshot(e12), SS::to_snapshot(e13), SS::to_snapshot(e14), SS::to_snapshot(e15),
        ]
    }
}
