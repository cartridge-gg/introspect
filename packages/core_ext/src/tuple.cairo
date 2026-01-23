use crate::{ToSnapshotBase, ToSnapshotOf};

pub trait TupleSnapForward<T> {
    type SnapForward;
    #[inline(always)]
    const fn snap_forward(self: @T) -> Self::SnapForward nopanic;
}

pub trait TupleSnapForwardTo<From, To> {
    #[inline(always)]
    const fn snap_forward(self: @From) -> To nopanic;
}

impl TupleSnapForwardSnapshot<T, impl TS: TupleSnapForward<T>> of TupleSnapForward<@T> {
    type SnapForward = TS::SnapForward;
    #[inline(always)]
    const fn snap_forward(self: @@T) -> Self::SnapForward nopanic {
        (*self).snap_forward()
    }
}

impl TupleSnapForwardSize0 of TupleSnapForward<()> {
    type SnapForward = ();
    #[inline(always)]
    const fn snap_forward(self: @()) nopanic {
        ()
    }
}

impl TupleSnapForwardSize1<E0, impl TS: ToSnapshotBase<@E0>> of TupleSnapForward<(E0,)> {
    type SnapForward = (@TS::Base,);
    #[inline(always)]
    const fn snap_forward(self: @(E0,)) -> Self::SnapForward nopanic {
        let (e0,) = self;
        (TS::to_snapshot(e0),)
    }
}

impl TupleSnapForwardSize2<
    E0, E1, impl TS0: ToSnapshotBase<@E0>, impl TS1: ToSnapshotBase<@E1>,
> of TupleSnapForward<(E0, E1)> {
    type SnapForward = (@TS0::Base, @TS1::Base);
    #[inline(always)]
    const fn snap_forward(self: @(E0, E1)) -> Self::SnapForward nopanic {
        let (e0, e1) = self;
        (TS0::to_snapshot(e0), TS1::to_snapshot(e1))
    }
}

impl TupleSnapForwardSize3<
    E0,
    E1,
    E2,
    impl TS0: ToSnapshotBase<@E0>,
    impl TS1: ToSnapshotBase<@E1>,
    impl TS2: ToSnapshotBase<@E2>,
> of TupleSnapForward<(E0, E1, E2)> {
    type SnapForward = (@TS0::Base, @TS1::Base, @TS2::Base);
    #[inline(always)]
    const fn snap_forward(self: @(E0, E1, E2)) -> Self::SnapForward nopanic {
        let (e0, e1, e2) = self;
        (TS0::to_snapshot(e0), TS1::to_snapshot(e1), TS2::to_snapshot(e2))
    }
}

impl TupleSnapForwardSize4<
    E0,
    E1,
    E2,
    E3,
    impl TS0: ToSnapshotBase<@E0>,
    impl TS1: ToSnapshotBase<@E1>,
    impl TS2: ToSnapshotBase<@E2>,
    impl TS3: ToSnapshotBase<@E3>,
> of TupleSnapForward<(E0, E1, E2, E3)> {
    type SnapForward = (@TS0::Base, @TS1::Base, @TS2::Base, @TS3::Base);
    #[inline(always)]
    const fn snap_forward(self: @(E0, E1, E2, E3)) -> Self::SnapForward nopanic {
        let (e0, e1, e2, e3) = self;
        (TS0::to_snapshot(e0), TS1::to_snapshot(e1), TS2::to_snapshot(e2), TS3::to_snapshot(e3))
    }
}

impl TupleSnapForwardSize5<
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
> of TupleSnapForward<(E0, E1, E2, E3, E4)> {
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

impl TupleSnapForwardSize6<
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
> of TupleSnapForward<(E0, E1, E2, E3, E4, E5)> {
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

impl TupleSnapForwardSize7<
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
> of TupleSnapForward<(E0, E1, E2, E3, E4, E5, E6)> {
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

impl TupleSnapForwardSize8<
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
> of TupleSnapForward<(E0, E1, E2, E3, E4, E5, E6, E7)> {
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

impl TupleSnapForwardSize9<
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
> of TupleSnapForward<(E0, E1, E2, E3, E4, E5, E6, E7, E8)> {
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

impl TupleSnapForwardSize10<
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
> of TupleSnapForward<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9)> {
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

impl TupleSnapForwardSize11<
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
> of TupleSnapForward<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10)> {
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

impl TupleSnapForwardSize12<
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
> of TupleSnapForward<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11)> {
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

impl TupleSnapForwardSize13<
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
> of TupleSnapForward<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12)> {
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

impl TupleSnapForwardSize14<
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
> of TupleSnapForward<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13)> {
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

impl TupleSnapForwardSize15<
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
> of TupleSnapForward<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14)> {
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

impl TupleSnapForwardSize16<
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
> of TupleSnapForward<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15)> {
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

impl TupleSnapshotSnapForwardTo<
    T, S, impl TS: TupleSnapForwardTo<T, S>,
> of TupleSnapForwardTo<@T, S> {
    #[inline(always)]
    const fn snap_forward(self: @@T) -> S nopanic {
        TS::snap_forward(*self)
    }
}

impl TupleSnapForwardToSize0 of TupleSnapForwardTo<(), ()> {
    #[inline(always)]
    const fn snap_forward(self: @()) nopanic {
        ()
    }
}

impl TupleSnapForwardToSize1<
    E0, S0, impl SS0: ToSnapshotOf<@E0, S0>,
> of TupleSnapForwardTo<(E0,), (@S0,)> {
    #[inline(always)]
    const fn snap_forward(self: @(E0,)) -> (@S0,) nopanic {
        let (e0,) = self;
        (SS0::to_snapshot(e0),)
    }
}

impl TupleSnapForwardToSize2<
    E0, E1, S0, S1, impl SS0: ToSnapshotOf<@E0, S0>, impl SS1: ToSnapshotOf<@E1, S1>,
> of TupleSnapForwardTo<(E0, E1), (@S0, @S1)> {
    #[inline(always)]
    const fn snap_forward(self: @(E0, E1)) -> (@S0, @S1) nopanic {
        let (e0, e1) = self;
        (SS0::to_snapshot(e0), SS1::to_snapshot(e1))
    }
}


impl TupleSnapForwardToSize3<
    E0,
    E1,
    E2,
    S0,
    S1,
    S2,
    impl SS0: ToSnapshotOf<@E0, S0>,
    impl SS1: ToSnapshotOf<@E1, S1>,
    impl SS2: ToSnapshotOf<@E2, S2>,
> of TupleSnapForwardTo<(E0, E1, E2), (@S0, @S1, @S2)> {
    #[inline(always)]
    const fn snap_forward(self: @(E0, E1, E2)) -> (@S0, @S1, @S2) nopanic {
        let (e0, e1, e2) = self;
        (SS0::to_snapshot(e0), SS1::to_snapshot(e1), SS2::to_snapshot(e2))
    }
}

impl TupleSnapForwardToSize4<
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
> of TupleSnapForwardTo<(E0, E1, E2, E3), (@S0, @S1, @S2, @S3)> {
    #[inline(always)]
    const fn snap_forward(self: @(E0, E1, E2, E3)) -> (@S0, @S1, @S2, @S3) nopanic {
        let (e0, e1, e2, e3) = self;
        (SS0::to_snapshot(e0), SS1::to_snapshot(e1), SS2::to_snapshot(e2), SS3::to_snapshot(e3))
    }
}

impl TupleSnapForwardToSize5<
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
> of TupleSnapForwardTo<(E0, E1, E2, E3, E4), (@S0, @S1, @S2, @S3, @S4)> {
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

impl TupleSnapForwardToSize6<
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
> of TupleSnapForwardTo<(E0, E1, E2, E3, E4, E5), (@S0, @S1, @S2, @S3, @S4, @S5)> {
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

impl TupleSnapForwardToSize7<
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
> of TupleSnapForwardTo<(E0, E1, E2, E3, E4, E5, E6), (@S0, @S1, @S2, @S3, @S4, @S5, @S6)> {
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

impl TupleSnapForwardToSize8<
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
> of TupleSnapForwardTo<
    (E0, E1, E2, E3, E4, E5, E6, E7), (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7),
> {
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

impl TupleSnapForwardToSize9<
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
> of TupleSnapForwardTo<
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

impl TupleSnapForwardToSize10<
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
> of TupleSnapForwardTo<
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

impl TupleSnapForwardToSize11<
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
> of TupleSnapForwardTo<
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

impl TupleSnapForwardToSize12<
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
> of TupleSnapForwardTo<
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

impl TupleSnapForwardToSize13<
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
> of TupleSnapForwardTo<
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

impl TupleSnapForwardToSize14<
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
> of TupleSnapForwardTo<
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

impl TupleSnapForwardToSize15<
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
> of TupleSnapForwardTo<
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

impl TupleSnapForwardToSize16<
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
> of TupleSnapForwardTo<
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

