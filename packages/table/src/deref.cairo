pub trait Snapable<T, S> {
    const fn snapshot(self: T) -> @S nopanic;
}

// pub trait ToSnapped<T, S> {
//     const fn to_snapshot(self: T) -> S nopanic;
// }

// impl ToSnappedImpl<T, +IsSnapped<T>, +Copy<T>> of ToSnapped<T, T> {
//     const fn to_snapshot(self: T) -> T nopanic {
//         self
//     }
// }

// impl ToSnappedSS<T, +Copy<T>, +IsSnapped<T>> of ToSnapped<@T, T> {
//     const fn to_snapshot(self: @T) -> T nopanic {
//         *self
//     }
// }

pub trait IsSnapped<T, +Copy<T>> {}
impl IsSnappedImpl<T> of IsSnapped<@T> {}

// pub trait TupleSnappable<T, S> {
//     const fn snapshot_forward(self: T) -> @S;
// }

impl TSnapable<T, +Drop<T>> of Snapable<T, T> {
    const fn snapshot(self: T) -> @T nopanic {
        @self
    }
}

impl TSSSnapable<T> of Snapable<@T, T> {
    const fn snapshot(self: @T) -> @T nopanic {
        self
    }
}

impl TSSSSnapable<T, impl SS: Snapable<@T, T>> of Snapable<@@T, T> {
    const fn snapshot(self: @@T) -> @T nopanic {
        SS::snapshot(*self)
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
// impl TupleSnappable<
//     T, impl TS: TupleSnapForward<T>, +Drop<T>, +Drop<TS::SnapForward>,
// > of Snapable<T, TS::SnapForward> {
//     const fn snapshot(self: T) -> @TS::SnapForward {
//         @TS::snap_forward(@self)
//     }
// }

// impl TupleSnappableSS<
//     T, impl TS: TupleSnapForward<T>, +Drop<T>, +Drop<TS::SnapForward>,
// > of Snapable<@T, TS::SnapForward> {
//     const fn snapshot(self: @T) -> @TS::SnapForward {
//         @TS::snap_forward(self)
//     }
// }

// impl SSTupleSnappable<
//     T, impl TS: TupleSnapForward<T>, +Drop<T>, +Drop<TS::SnapForward>,
// > of Snapable<@T, TS::SnapForward> {
//     const fn snapshot(self: @T) -> @TS::SnapForward nopanic {
//         @TS::snap_forward(self)
//     }
// }

// impl TupleSnappableTupleSize1<E0> of TupleSnapForward<(E0,)> {
//     type SnapForward = (@E0,);
//     #[inline(always)]
//     const fn snap_forward(self: @(E0,)) -> (@E0,) nopanic {
//         let (e0,) = self;
//         (e0,)
//     }
// }

// impl TupleSnappableTupleSize2SS<E0, E1> of TupleSnapForward<(@E0, @E1)> {
//     type SnapForward = (@E0, @E1);
//     #[inline(always)]
//     const fn snap_forward(self: @(@E0, @E1)) -> (@E0, @E1) nopanic {
//         let (e0, e1) = self;
//         (*e0, *e1)
//     }
// }

// impl TupleSnappableTupleSize3SS<E0, E1, E2> of TupleSnapForward<(@E0, @E1, @E2)> {
//     type SnapForward = (@E0, @E1, @E2);
//     #[inline(always)]
//     const fn snap_forward(self: @(@E0, @E1, @E2)) -> (@E0, @E1, @E2) nopanic {
//         let (e0, e1, e2) = self;
//         (*e0, *e1, *e2)
//     }
// }

// impl TupleSnappableTupleSize4SS<E0, E1, E2, E3> of TupleSnapForward<(@E0, @E1, @E2, @E3)> {
//     type SnapForward = (@E0, @E1, @E2, @E3);
//     #[inline(always)]
//     const fn snap_forward(self: @(@E0, @E1, @E2, @E3)) -> (@E0, @E1, @E2, @E3) nopanic {
//         let (e0, e1, e2, e3) = self;
//         (*e0, *e1, *e2, *e3)
//     }
// }

// impl TupleSnappableTupleSize5SS<
//     E0, E1, E2, E3, E4,
// > of TupleSnapForward<(@E0, @E1, @E2, @E3, @E4)> {
//     type SnapForward = (@E0, @E1, @E2, @E3, @E4);
//     #[inline(always)]
//     const fn snap_forward(self: @(@E0, @E1, @E2, @E3, @E4)) -> (@E0, @E1, @E2, @E3, @E4) nopanic
//     {
//         let (e0, e1, e2, e3, e4) = self;
//         (*e0, *e1, *e2, *e3, *e4)
//     }
// }

// impl TupleSnappableTupleSize6SS<
//     E0, E1, E2, E3, E4, E5,
// > of TupleSnapForward<(@E0, @E1, @E2, @E3, @E4, @E5)> {
//     type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5);
//     #[inline(always)]
//     const fn snap_forward(
//         self: @(@E0, @E1, @E2, @E3, @E4, @E5),
//     ) -> (@E0, @E1, @E2, @E3, @E4, @E5) nopanic {
//         let (e0, e1, e2, e3, e4, e5) = self;
//         (*e0, *e1, *e2, *e3, *e4, *e5)
//     }
// }

// impl TupleSnappableTupleSize7SS<
//     E0, E1, E2, E3, E4, E5, E6,
// > of TupleSnapForward<(@E0, @E1, @E2, @E3, @E4, @E5, @E6)> {
//     type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5, @E6);
//     #[inline(always)]
//     const fn snap_forward(
//         self: @(@E0, @E1, @E2, @E3, @E4, @E5, @E6),
//     ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6) nopanic {
//         let (e0, e1, e2, e3, e4, e5, e6) = self;
//         (*e0, *e1, *e2, *e3, *e4, *e5, *e6)
//     }
// }

// impl TupleSnappableTupleSize8SS<
//     E0, E1, E2, E3, E4, E5, E6, E7,
// > of TupleSnapForward<(@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7)> {
//     type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7);
//     #[inline(always)]
//     const fn snap_forward(
//         self: @(@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7),
//     ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7) nopanic {
//         let (e0, e1, e2, e3, e4, e5, e6, e7) = self;
//         (*e0, *e1, *e2, *e3, *e4, *e5, *e6, *e7)
//     }
// }

// impl TupleSnappableTupleSize9SS<
//     E0, E1, E2, E3, E4, E5, E6, E7, E8,
// > of TupleSnapForward<(@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8)> {
//     type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8);
//     #[inline(always)]
//     const fn snap_forward(
//         self: @(@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8),
//     ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8) nopanic {
//         let (e0, e1, e2, e3, e4, e5, e6, e7, e8) = self;
//         (*e0, *e1, *e2, *e3, *e4, *e5, *e6, *e7, *e8)
//     }
// }

// impl TupleSnappableTupleSize10SS<
//     E0, E1, E2, E3, E4, E5, E6, E7, E8, E9,
// > of TupleSnapForward<(@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9)> {
//     type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9);
//     #[inline(always)]
//     const fn snap_forward(
//         self: @(@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9),
//     ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9) nopanic {
//         let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9) = self;
//         (*e0, *e1, *e2, *e3, *e4, *e5, *e6, *e7, *e8, *e9)
//     }
// }

// impl TupleSnappableTupleSize11SS<
//     E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10,
// > of TupleSnapForward<(@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10)> {
//     type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10);
//     #[inline(always)]
//     const fn snap_forward(
//         self: @(@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10),
//     ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10) nopanic {
//         let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10) = self;
//         (*e0, *e1, *e2, *e3, *e4, *e5, *e6, *e7, *e8, *e9, *e10)
//     }
// }

// impl TupleSnappableTupleSize12SS<
//     E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11,
// > of TupleSnapForward<(@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11)> {
//     type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11);
//     #[inline(always)]
//     const fn snap_forward(
//         self: @(@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11),
//     ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11) nopanic {
//         let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11) = self;
//         (*e0, *e1, *e2, *e3, *e4, *e5, *e6, *e7, *e8, *e9, *e10, *e11)
//     }
// }

// impl TupleSnappableTupleSize13SS<
//     E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12,
// > of TupleSnapForward<(@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12)> {
//     type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12);
//     #[inline(always)]
//     const fn snap_forward(
//         self: @(@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12),
//     ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12) nopanic {
//         let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12) = self;
//         (*e0, *e1, *e2, *e3, *e4, *e5, *e6, *e7, *e8, *e9, *e10, *e11, *e12)
//     }
// }

// impl TupleSnappableTupleSize14SS<
//     E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13,
// > of TupleSnapForward<(@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12, @E13)>
// {
//     type SnapForward = (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12,
//     @E13);
//     #[inline(always)]
//     const fn snap_forward(
//         self: @(@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12, @E13),
//     ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12, @E13) nopanic {
//         let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13) = self;
//         (*e0, *e1, *e2, *e3, *e4, *e5, *e6, *e7, *e8, *e9, *e10, *e11, *e12, *e13)
//     }
// }

// impl TupleSnappableTupleSize15SS<
//     E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14,
// > of TupleSnapForward<
//     (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12, @E13, @E14),
// > {
//     type SnapForward = (
//         @E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12, @E13, @E14,
//     );
//     #[inline(always)]
//     const fn snap_forward(
//         self: @(@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12, @E13, @E14),
//     ) -> (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12, @E13, @E14) nopanic
//     {
//         let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14) = self;
//         (*e0, *e1, *e2, *e3, *e4, *e5, *e6, *e7, *e8, *e9, *e10, *e11, *e12, *e13, *e14)
//     }
// }

// impl TupleSnappableTupleSize16SS<
//     E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15,
// > of TupleSnapForward<
//     (@E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12, @E13, @E14, @E15),
// > {
//     type SnapForward = (
//         @E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12, @E13, @E14, @E15,
//     );
//     #[inline(always)]
//     const fn snap_forward(
//         self: @(
//             @E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12, @E13, @E14, @E15,
//         ),
//     ) -> (
//         @E0, @E1, @E2, @E3, @E4, @E5, @E6, @E7, @E8, @E9, @E10, @E11, @E12, @E13, @E14, @E15,
//     ) nopanic {
//         let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15) = self;
//         (*e0, *e1, *e2, *e3, *e4, *e5, *e6, *e7, *e8, *e9, *e10, *e11, *e12, *e13, *e14, *e15)
//     }
// }
// trait SpanSnappable<T> {
//     fn span_snap(self: Span<T>) -> @Span<T>;
// }
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


