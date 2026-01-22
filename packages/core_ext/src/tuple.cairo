use crate::ToSnapshot;

pub trait TupleSnappable<T, S> {
    #[inline(always)]
    const fn snap_tuple(self: @T) -> S nopanic;
}

// impl TupleSnappableEw<T> of TupleSnappable<T, @T> {
//     #[inline(always)]
//     const fn snap_tuple(self: @T) -> @T nopanic {
//         self
//     }
// }

impl TupleSnappableSS<T, S, impl TS: TupleSnappable<T, S>> of TupleSnappable<@T, S> {
    #[inline(always)]
    const fn snap_tuple(self: @@T) -> S nopanic {
        (*self).snap_tuple()
    }
}

impl TupleSnappableTupleSize0 of TupleSnappable<(), ()> {
    #[inline(always)]
    const fn snap_tuple(self: @()) nopanic {
        ()
    }
}

impl TupleSnappableTupleSize1<E0, S0, +ToSnapshot<@E0, S0>> of TupleSnappable<(E0,), (@S0,)> {
    #[inline(always)]
    const fn snap_tuple(self: @(E0,)) -> (@S0,) nopanic {
        let (e0,) = self;
        (e0.to_snapshot(),)
    }
}

impl TupleSnappableTupleSize2<
    E0, E1, S0, S1, +ToSnapshot<@E0, S0>, +ToSnapshot<@E1, S1>,
> of TupleSnappable<(E0, E1), (@S0, @S1)> {
    #[inline(always)]
    const fn snap_tuple(self: @(E0, E1)) -> (@S0, @S1) nopanic {
        let (e0, e1) = self;
        (e0.to_snapshot(), e1.to_snapshot())
    }
}

impl TupleSnappableTupleSize3<
    E0, E1, E2, S0, S1, S2, +ToSnapshot<@E0, S0>, +ToSnapshot<@E1, S1>, +ToSnapshot<@E2, S2>,
> of TupleSnappable<(E0, E1, E2), (@S0, @S1, @S2)> {
    #[inline(always)]
    const fn snap_tuple(self: @(E0, E1, E2)) -> (@S0, @S1, @S2) nopanic {
        let (e0, e1, e2) = self;
        (e0.to_snapshot(), e1.to_snapshot(), e2.to_snapshot())
    }
}

impl TupleSnappableTupleSize4<
    E0,
    E1,
    E2,
    E3,
    S0,
    S1,
    S2,
    S3,
    +ToSnapshot<@E0, S0>,
    +ToSnapshot<@E1, S1>,
    +ToSnapshot<@E2, S2>,
    +ToSnapshot<@E3, S3>,
> of TupleSnappable<(E0, E1, E2, E3), (@S0, @S1, @S2, @S3)> {
    #[inline(always)]
    const fn snap_tuple(self: @(E0, E1, E2, E3)) -> (@S0, @S1, @S2, @S3) nopanic {
        let (e0, e1, e2, e3) = self;
        (e0.to_snapshot(), e1.to_snapshot(), e2.to_snapshot(), e3.to_snapshot())
    }
}

impl TupleSnappableTupleSize5<
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
    +ToSnapshot<@E0, S0>,
    +ToSnapshot<@E1, S1>,
    +ToSnapshot<@E2, S2>,
    +ToSnapshot<@E3, S3>,
    +ToSnapshot<@E4, S4>,
> of TupleSnappable<(E0, E1, E2, E3, E4), (@S0, @S1, @S2, @S3, @S4)> {
    #[inline(always)]
    const fn snap_tuple(self: @(E0, E1, E2, E3, E4)) -> (@S0, @S1, @S2, @S3, @S4) nopanic {
        let (e0, e1, e2, e3, e4) = self;
        (e0.to_snapshot(), e1.to_snapshot(), e2.to_snapshot(), e3.to_snapshot(), e4.to_snapshot())
    }
}

impl TupleSnappableTupleSize6<
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
    +ToSnapshot<@E0, S0>,
    +ToSnapshot<@E1, S1>,
    +ToSnapshot<@E2, S2>,
    +ToSnapshot<@E3, S3>,
    +ToSnapshot<@E4, S4>,
    +ToSnapshot<@E5, S5>,
> of TupleSnappable<(E0, E1, E2, E3, E4, E5), (@S0, @S1, @S2, @S3, @S4, @S5)> {
    #[inline(always)]
    const fn snap_tuple(self: @(E0, E1, E2, E3, E4, E5)) -> (@S0, @S1, @S2, @S3, @S4, @S5) nopanic {
        let (e0, e1, e2, e3, e4, e5) = self;
        (
            e0.to_snapshot(),
            e1.to_snapshot(),
            e2.to_snapshot(),
            e3.to_snapshot(),
            e4.to_snapshot(),
            e5.to_snapshot(),
        )
    }
}

impl TupleSnappableTupleSize7<
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
    +ToSnapshot<@E0, S0>,
    +ToSnapshot<@E1, S1>,
    +ToSnapshot<@E2, S2>,
    +ToSnapshot<@E3, S3>,
    +ToSnapshot<@E4, S4>,
    +ToSnapshot<@E5, S5>,
    +ToSnapshot<@E6, S6>,
> of TupleSnappable<(E0, E1, E2, E3, E4, E5, E6), (@S0, @S1, @S2, @S3, @S4, @S5, @S6)> {
    #[inline(always)]
    const fn snap_tuple(
        self: @(E0, E1, E2, E3, E4, E5, E6),
    ) -> (@S0, @S1, @S2, @S3, @S4, @S5, @S6) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6) = self;
        (
            e0.to_snapshot(),
            e1.to_snapshot(),
            e2.to_snapshot(),
            e3.to_snapshot(),
            e4.to_snapshot(),
            e5.to_snapshot(),
            e6.to_snapshot(),
        )
    }
}

impl TupleSnappableTupleSize8<
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
    +ToSnapshot<@E0, S0>,
    +ToSnapshot<@E1, S1>,
    +ToSnapshot<@E2, S2>,
    +ToSnapshot<@E3, S3>,
    +ToSnapshot<@E4, S4>,
    +ToSnapshot<@E5, S5>,
    +ToSnapshot<@E6, S6>,
    +ToSnapshot<@E7, S7>,
> of TupleSnappable<(E0, E1, E2, E3, E4, E5, E6, E7), (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7)> {
    #[inline(always)]
    const fn snap_tuple(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7),
    ) -> (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7) = self;
        (
            e0.to_snapshot(),
            e1.to_snapshot(),
            e2.to_snapshot(),
            e3.to_snapshot(),
            e4.to_snapshot(),
            e5.to_snapshot(),
            e6.to_snapshot(),
            e7.to_snapshot(),
        )
    }
}

impl TupleSnappableTupleSize9<
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
    +ToSnapshot<@E0, S0>,
    +ToSnapshot<@E1, S1>,
    +ToSnapshot<@E2, S2>,
    +ToSnapshot<@E3, S3>,
    +ToSnapshot<@E4, S4>,
    +ToSnapshot<@E5, S5>,
    +ToSnapshot<@E6, S6>,
    +ToSnapshot<@E7, S7>,
    +ToSnapshot<@E8, S8>,
> of TupleSnappable<
    (E0, E1, E2, E3, E4, E5, E6, E7, E8), (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8),
> {
    #[inline(always)]
    const fn snap_tuple(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8),
    ) -> (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8) = self;
        (
            e0.to_snapshot(),
            e1.to_snapshot(),
            e2.to_snapshot(),
            e3.to_snapshot(),
            e4.to_snapshot(),
            e5.to_snapshot(),
            e6.to_snapshot(),
            e7.to_snapshot(),
            e8.to_snapshot(),
        )
    }
}

impl TupleSnappableTupleSize10<
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
    +ToSnapshot<@E0, S0>,
    +ToSnapshot<@E1, S1>,
    +ToSnapshot<@E2, S2>,
    +ToSnapshot<@E3, S3>,
    +ToSnapshot<@E4, S4>,
    +ToSnapshot<@E5, S5>,
    +ToSnapshot<@E6, S6>,
    +ToSnapshot<@E7, S7>,
    +ToSnapshot<@E8, S8>,
    +ToSnapshot<@E9, S9>,
> of TupleSnappable<
    (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9), (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9),
> {
    #[inline(always)]
    const fn snap_tuple(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9),
    ) -> (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9) = self;
        (
            e0.to_snapshot(),
            e1.to_snapshot(),
            e2.to_snapshot(),
            e3.to_snapshot(),
            e4.to_snapshot(),
            e5.to_snapshot(),
            e6.to_snapshot(),
            e7.to_snapshot(),
            e8.to_snapshot(),
            e9.to_snapshot(),
        )
    }
}

impl TupleSnappableTupleSize11<
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
    +ToSnapshot<@E0, S0>,
    +ToSnapshot<@E1, S1>,
    +ToSnapshot<@E2, S2>,
    +ToSnapshot<@E3, S3>,
    +ToSnapshot<@E4, S4>,
    +ToSnapshot<@E5, S5>,
    +ToSnapshot<@E6, S6>,
    +ToSnapshot<@E7, S7>,
    +ToSnapshot<@E8, S8>,
    +ToSnapshot<@E9, S9>,
    +ToSnapshot<@E10, S10>,
> of TupleSnappable<
    (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10),
    (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9, @S10),
> {
    #[inline(always)]
    const fn snap_tuple(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10),
    ) -> (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9, @S10) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10) = self;
        (
            e0.to_snapshot(),
            e1.to_snapshot(),
            e2.to_snapshot(),
            e3.to_snapshot(),
            e4.to_snapshot(),
            e5.to_snapshot(),
            e6.to_snapshot(),
            e7.to_snapshot(),
            e8.to_snapshot(),
            e9.to_snapshot(),
            e10.to_snapshot(),
        )
    }
}

impl TupleSnappableTupleSize12<
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
    +ToSnapshot<@E0, S0>,
    +ToSnapshot<@E1, S1>,
    +ToSnapshot<@E2, S2>,
    +ToSnapshot<@E3, S3>,
    +ToSnapshot<@E4, S4>,
    +ToSnapshot<@E5, S5>,
    +ToSnapshot<@E6, S6>,
    +ToSnapshot<@E7, S7>,
    +ToSnapshot<@E8, S8>,
    +ToSnapshot<@E9, S9>,
    +ToSnapshot<@E10, S10>,
    +ToSnapshot<@E11, S11>,
> of TupleSnappable<
    (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11),
    (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9, @S10, @S11),
> {
    #[inline(always)]
    const fn snap_tuple(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11),
    ) -> (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9, @S10, @S11) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11) = self;
        (
            e0.to_snapshot(),
            e1.to_snapshot(),
            e2.to_snapshot(),
            e3.to_snapshot(),
            e4.to_snapshot(),
            e5.to_snapshot(),
            e6.to_snapshot(),
            e7.to_snapshot(),
            e8.to_snapshot(),
            e9.to_snapshot(),
            e10.to_snapshot(),
            e11.to_snapshot(),
        )
    }
}

impl TupleSnappableTupleSize13<
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
    +ToSnapshot<@E0, S0>,
    +ToSnapshot<@E1, S1>,
    +ToSnapshot<@E2, S2>,
    +ToSnapshot<@E3, S3>,
    +ToSnapshot<@E4, S4>,
    +ToSnapshot<@E5, S5>,
    +ToSnapshot<@E6, S6>,
    +ToSnapshot<@E7, S7>,
    +ToSnapshot<@E8, S8>,
    +ToSnapshot<@E9, S9>,
    +ToSnapshot<@E10, S10>,
    +ToSnapshot<@E11, S11>,
    +ToSnapshot<@E12, S12>,
> of TupleSnappable<
    (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12),
    (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9, @S10, @S11, @S12),
> {
    #[inline(always)]
    const fn snap_tuple(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12),
    ) -> (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9, @S10, @S11, @S12) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12) = self;
        (
            e0.to_snapshot(),
            e1.to_snapshot(),
            e2.to_snapshot(),
            e3.to_snapshot(),
            e4.to_snapshot(),
            e5.to_snapshot(),
            e6.to_snapshot(),
            e7.to_snapshot(),
            e8.to_snapshot(),
            e9.to_snapshot(),
            e10.to_snapshot(),
            e11.to_snapshot(),
            e12.to_snapshot(),
        )
    }
}

impl TupleSnappableTupleSize14<
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
    +ToSnapshot<@E0, S0>,
    +ToSnapshot<@E1, S1>,
    +ToSnapshot<@E2, S2>,
    +ToSnapshot<@E3, S3>,
    +ToSnapshot<@E4, S4>,
    +ToSnapshot<@E5, S5>,
    +ToSnapshot<@E6, S6>,
    +ToSnapshot<@E7, S7>,
    +ToSnapshot<@E8, S8>,
    +ToSnapshot<@E9, S9>,
    +ToSnapshot<@E10, S10>,
    +ToSnapshot<@E11, S11>,
    +ToSnapshot<@E12, S12>,
    +ToSnapshot<@E13, S13>,
> of TupleSnappable<
    (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13),
    (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9, @S10, @S11, @S12, @S13),
> {
    #[inline(always)]
    const fn snap_tuple(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13),
    ) -> (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9, @S10, @S11, @S12, @S13) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13) = self;
        (
            e0.to_snapshot(),
            e1.to_snapshot(),
            e2.to_snapshot(),
            e3.to_snapshot(),
            e4.to_snapshot(),
            e5.to_snapshot(),
            e6.to_snapshot(),
            e7.to_snapshot(),
            e8.to_snapshot(),
            e9.to_snapshot(),
            e10.to_snapshot(),
            e11.to_snapshot(),
            e12.to_snapshot(),
            e13.to_snapshot(),
        )
    }
}

impl TupleSnappableTupleSize15<
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
    +ToSnapshot<@E0, S0>,
    +ToSnapshot<@E1, S1>,
    +ToSnapshot<@E2, S2>,
    +ToSnapshot<@E3, S3>,
    +ToSnapshot<@E4, S4>,
    +ToSnapshot<@E5, S5>,
    +ToSnapshot<@E6, S6>,
    +ToSnapshot<@E7, S7>,
    +ToSnapshot<@E8, S8>,
    +ToSnapshot<@E9, S9>,
    +ToSnapshot<@E10, S10>,
    +ToSnapshot<@E11, S11>,
    +ToSnapshot<@E12, S12>,
    +ToSnapshot<@E13, S13>,
    +ToSnapshot<@E14, S14>,
> of TupleSnappable<
    (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14),
    (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9, @S10, @S11, @S12, @S13, @S14),
> {
    #[inline(always)]
    const fn snap_tuple(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14),
    ) -> (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9, @S10, @S11, @S12, @S13, @S14) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14) = self;
        (
            e0.to_snapshot(),
            e1.to_snapshot(),
            e2.to_snapshot(),
            e3.to_snapshot(),
            e4.to_snapshot(),
            e5.to_snapshot(),
            e6.to_snapshot(),
            e7.to_snapshot(),
            e8.to_snapshot(),
            e9.to_snapshot(),
            e10.to_snapshot(),
            e11.to_snapshot(),
            e12.to_snapshot(),
            e13.to_snapshot(),
            e14.to_snapshot(),
        )
    }
}

impl TupleSnappableTupleSize16<
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
    +ToSnapshot<@E0, S0>,
    +ToSnapshot<@E1, S1>,
    +ToSnapshot<@E2, S2>,
    +ToSnapshot<@E3, S3>,
    +ToSnapshot<@E4, S4>,
    +ToSnapshot<@E5, S5>,
    +ToSnapshot<@E6, S6>,
    +ToSnapshot<@E7, S7>,
    +ToSnapshot<@E8, S8>,
    +ToSnapshot<@E9, S9>,
    +ToSnapshot<@E10, S10>,
    +ToSnapshot<@E11, S11>,
    +ToSnapshot<@E12, S12>,
    +ToSnapshot<@E13, S13>,
    +ToSnapshot<@E14, S14>,
    +ToSnapshot<@E15, S15>,
> of TupleSnappable<
    (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15),
    (@S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9, @S10, @S11, @S12, @S13, @S14, @S15),
> {
    #[inline(always)]
    const fn snap_tuple(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15),
    ) -> (
        @S0, @S1, @S2, @S3, @S4, @S5, @S6, @S7, @S8, @S9, @S10, @S11, @S12, @S13, @S14, @S15,
    ) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15) = self;
        (
            e0.to_snapshot(),
            e1.to_snapshot(),
            e2.to_snapshot(),
            e3.to_snapshot(),
            e4.to_snapshot(),
            e5.to_snapshot(),
            e6.to_snapshot(),
            e7.to_snapshot(),
            e8.to_snapshot(),
            e9.to_snapshot(),
            e10.to_snapshot(),
            e11.to_snapshot(),
            e12.to_snapshot(),
            e13.to_snapshot(),
            e14.to_snapshot(),
            e15.to_snapshot(),
        )
    }
}

pub trait TupleSnapForward<T> {
    type SnapForward;
    fn snap_forward(self: @T) -> Self::SnapForward nopanic;
}

impl TupleSnapForwardTupleSize0 of TupleSnapForward<()> {
    type SnapForward = ();
    fn snap_forward(self: @()) nopanic {
        ()
    }
}

impl TupleSnapForwardTupleSize1<E0> of TupleSnapForward<(E0,)> {
    type SnapForward = (@E0,);
    fn snap_forward(self: @(E0,)) -> (@E0,) nopanic {
        let (e0,) = self;
        (e0,)
    }
}

impl TupleSnapForwardTupleSize2<E0, E1> of TupleSnapForward<(E0, E1)> {
    type SnapForward = (@E0, @E1);
    fn snap_forward(self: @(E0, E1)) -> (@E0, @E1) nopanic {
        let (e0, e1) = self;
        (e0, e1)
    }
}

impl TupleSnapForwardTupleSize3<E0, E1, E2> of TupleSnapForward<(E0, E1, E2)> {
    type SnapForward = (@E0, @E1, @E2);
    fn snap_forward(self: @(E0, E1, E2)) -> (@E0, @E1, @E2) nopanic {
        let (e0, e1, e2) = self;
        (e0, e1, e2)
    }
}

impl TupleSnapForwardTupleSize4<E0, E1, E2, E3> of TupleSnapForward<(E0, E1, E2, E3)> {
    type SnapForward = (@E0, @E1, @E2, @E3);
    fn snap_forward(self: @(E0, E1, E2, E3)) -> (@E0, @E1, @E2, @E3) nopanic {
        let (e0, e1, e2, e3) = self;
        (e0, e1, e2, e3)
    }
}

impl TupleSnapForwardTupleSize5<E0, E1, E2, E3, E4> of TupleSnapForward<(E0, E1, E2, E3, E4)> {
    type SnapForward = (@E0, @E1, @E2, @E3, @E4);
    fn snap_forward(self: @(E0, E1, E2, E3, E4)) -> (@E0, @E1, @E2, @E3, @E4) nopanic {
        let (e0, e1, e2, e3, e4) = self;
        (e0, e1, e2, e3, e4)
    }
}

impl TupleSnapForwardTupleSize6<
    E0, E1, E2, E3, E4, E5,
> of TupleSnapForward<(E0, E1, E2, E3, E4, E5)> {
    type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5);
    fn snap_forward(self: @(E0, E1, E2, E3, E4, E5)) -> (@E0, @E1, @E2, @E3, @E4, @E5) nopanic {
        let (e0, e1, e2, e3, e4, e5) = self;
        (e0, e1, e2, e3, e4, e5)
    }
}

impl TupleSnapForwardTupleSize7<
    E0, E1, E2, E3, E4, E5, E6,
> of TupleSnapForward<(E0, E1, E2, E3, E4, E5, E6)> {
    type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5, @E6);
    fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6),
    ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6) = self;
        (e0, e1, e2, e3, e4, e5, e6)
    }
}

impl TupleSnapForwardTupleSize8<
    E0, E1, E2, E3, E4, E5, E6, E7,
> of TupleSnapForward<(E0, E1, E2, E3, E4, E5, E6, E7)> {
    type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7);
    fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7),
    ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7) = self;
        (e0, e1, e2, e3, e4, e5, e6, e7)
    }
}

impl TupleSnapForwardTupleSize9<
    E0, E1, E2, E3, E4, E5, E6, E7, E8,
> of TupleSnapForward<(E0, E1, E2, E3, E4, E5, E6, E7, E8)> {
    type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8);
    fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8),
    ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8) = self;
        (e0, e1, e2, e3, e4, e5, e6, e7, e8)
    }
}

impl TupleSnapForwardTupleSize10<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9,
> of TupleSnapForward<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9)> {
    type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9);
    fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9),
    ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9) = self;
        (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9)
    }
}

impl TupleSnapForwardTupleSize11<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10,
> of TupleSnapForward<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10)> {
    type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10);
    fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10),
    ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10) = self;
        (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10)
    }
}

impl TupleSnapForwardTupleSize12<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11,
> of TupleSnapForward<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11)> {
    type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11);
    fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11),
    ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11) = self;
        (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11)
    }
}

impl TupleSnapForwardTupleSize13<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12,
> of TupleSnapForward<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12)> {
    type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12);
    fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12),
    ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12) = self;
        (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12)
    }
}

impl TupleSnapForwardTupleSize14<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13,
> of TupleSnapForward<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13)> {
    type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12, @E13);
    fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13),
    ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12, @E13) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13) = self;
        (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13)
    }
}

impl TupleSnapForwardTupleSize15<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14,
> of TupleSnapForward<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14)> {
    type SnapForward = (
        @E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12, @E13, @E14,
    );
    fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14),
    ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12, @E13, @E14) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14) = self;
        (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14)
    }
}

impl TupleSnapForwardTupleSize16<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15,
> of TupleSnapForward<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15)> {
    type SnapForward = (
        @E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12, @E13, @E14, @E15,
    );
    fn snap_forward(
        self: @(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15),
    ) -> (
        @E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12, @E13, @E14, @E15,
    ) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15) = self;
        (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15)
    }
}
