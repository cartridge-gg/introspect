pub trait CollectionSplit<T> {
    /// The type of the first element of the collection.
    type Head;
    /// The type of the rest of the collection.
    type Rest;
    /// Splits the collection into the head and the rest.
    fn split_head(self: T) -> (Self::Head, Self::Rest) nopanic;
    /// Reconstructs the collection from the head and the rest.
    fn reconstruct(head: Self::Head, rest: Self::Rest) -> T nopanic;
}


impl TupleSplitTupleSize1<E0> of CollectionSplit<(E0,)> {
    type Head = E0;
    type Rest = ();
    fn split_head(self: (E0,)) -> (E0, ()) nopanic {
        let (e0,) = self;
        (e0, ())
    }
    fn reconstruct(head: E0, rest: ()) -> (E0,) nopanic {
        (head,)
    }
}

impl TupleSplitTupleSize2<E0, E1> of CollectionSplit<(E0, E1)> {
    type Head = E0;
    type Rest = (E1,);
    fn split_head(self: (E0, E1)) -> (E0, (E1,)) nopanic {
        let (e0, e1) = self;
        (e0, (e1,))
    }
    fn reconstruct(head: E0, rest: (E1,)) -> (E0, E1) nopanic {
        let (e1,) = rest;
        (head, e1)
    }
}

impl TupleSplitTupleSize3<E0, E1, E2> of CollectionSplit<(E0, E1, E2)> {
    type Head = E0;
    type Rest = (E1, E2);
    fn split_head(self: (E0, E1, E2)) -> (E0, (E1, E2)) nopanic {
        let (e0, e1, e2) = self;
        (e0, (e1, e2))
    }
    fn reconstruct(head: E0, rest: (E1, E2)) -> (E0, E1, E2) nopanic {
        let (e1, e2) = rest;
        (head, e1, e2)
    }
}

impl TupleSplitTupleSize4<E0, E1, E2, E3> of CollectionSplit<(E0, E1, E2, E3)> {
    type Head = E0;
    type Rest = (E1, E2, E3);
    fn split_head(self: (E0, E1, E2, E3)) -> (E0, (E1, E2, E3)) nopanic {
        let (e0, e1, e2, e3) = self;
        (e0, (e1, e2, e3))
    }
    fn reconstruct(head: E0, rest: (E1, E2, E3)) -> (E0, E1, E2, E3) nopanic {
        let (e1, e2, e3) = rest;
        (head, e1, e2, e3)
    }
}

impl TupleSplitTupleSize5<E0, E1, E2, E3, E4> of CollectionSplit<(E0, E1, E2, E3, E4)> {
    type Head = E0;
    type Rest = (E1, E2, E3, E4);
    fn split_head(self: (E0, E1, E2, E3, E4)) -> (E0, (E1, E2, E3, E4)) nopanic {
        let (e0, e1, e2, e3, e4) = self;
        (e0, (e1, e2, e3, e4))
    }
    fn reconstruct(head: E0, rest: (E1, E2, E3, E4)) -> (E0, E1, E2, E3, E4) nopanic {
        let (e1, e2, e3, e4) = rest;
        (head, e1, e2, e3, e4)
    }
}

impl TupleSplitTupleSize6<E0, E1, E2, E3, E4, E5> of CollectionSplit<(E0, E1, E2, E3, E4, E5)> {
    type Head = E0;
    type Rest = (E1, E2, E3, E4, E5);
    fn split_head(self: (E0, E1, E2, E3, E4, E5)) -> (E0, (E1, E2, E3, E4, E5)) nopanic {
        let (e0, e1, e2, e3, e4, e5) = self;
        (e0, (e1, e2, e3, e4, e5))
    }
    fn reconstruct(head: E0, rest: (E1, E2, E3, E4, E5)) -> (E0, E1, E2, E3, E4, E5) nopanic {
        let (e1, e2, e3, e4, e5) = rest;
        (head, e1, e2, e3, e4, e5)
    }
}

impl TupleSplitTupleSize7<
    E0, E1, E2, E3, E4, E5, E6,
> of CollectionSplit<(E0, E1, E2, E3, E4, E5, E6)> {
    type Head = E0;
    type Rest = (E1, E2, E3, E4, E5, E6);
    fn split_head(self: (E0, E1, E2, E3, E4, E5, E6)) -> (E0, (E1, E2, E3, E4, E5, E6)) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6) = self;
        (e0, (e1, e2, e3, e4, e5, e6))
    }
    fn reconstruct(
        head: E0, rest: (E1, E2, E3, E4, E5, E6),
    ) -> (E0, E1, E2, E3, E4, E5, E6) nopanic {
        let (e1, e2, e3, e4, e5, e6) = rest;
        (head, e1, e2, e3, e4, e5, e6)
    }
}

impl TupleSplitTupleSize8<
    E0, E1, E2, E3, E4, E5, E6, E7,
> of CollectionSplit<(E0, E1, E2, E3, E4, E5, E6, E7)> {
    type Head = E0;
    type Rest = (E1, E2, E3, E4, E5, E6, E7);
    fn split_head(
        self: (E0, E1, E2, E3, E4, E5, E6, E7),
    ) -> (E0, (E1, E2, E3, E4, E5, E6, E7)) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7) = self;
        (e0, (e1, e2, e3, e4, e5, e6, e7))
    }
    fn reconstruct(
        head: E0, rest: (E1, E2, E3, E4, E5, E6, E7),
    ) -> (E0, E1, E2, E3, E4, E5, E6, E7) nopanic {
        let (e1, e2, e3, e4, e5, e6, e7) = rest;
        (head, e1, e2, e3, e4, e5, e6, e7)
    }
}

impl TupleSplitTupleSize9<
    E0, E1, E2, E3, E4, E5, E6, E7, E8,
> of CollectionSplit<(E0, E1, E2, E3, E4, E5, E6, E7, E8)> {
    type Head = E0;
    type Rest = (E1, E2, E3, E4, E5, E6, E7, E8);
    fn split_head(
        self: (E0, E1, E2, E3, E4, E5, E6, E7, E8),
    ) -> (E0, (E1, E2, E3, E4, E5, E6, E7, E8)) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8) = self;
        (e0, (e1, e2, e3, e4, e5, e6, e7, e8))
    }
    fn reconstruct(
        head: E0, rest: (E1, E2, E3, E4, E5, E6, E7, E8),
    ) -> (E0, E1, E2, E3, E4, E5, E6, E7, E8) nopanic {
        let (e1, e2, e3, e4, e5, e6, e7, e8) = rest;
        (head, e1, e2, e3, e4, e5, e6, e7, e8)
    }
}

impl TupleSplitTupleSize10<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9,
> of CollectionSplit<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9)> {
    type Head = E0;
    type Rest = (E1, E2, E3, E4, E5, E6, E7, E8, E9);
    fn split_head(
        self: (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9),
    ) -> (E0, (E1, E2, E3, E4, E5, E6, E7, E8, E9)) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9) = self;
        (e0, (e1, e2, e3, e4, e5, e6, e7, e8, e9))
    }
    fn reconstruct(
        head: E0, rest: (E1, E2, E3, E4, E5, E6, E7, E8, E9),
    ) -> (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9) nopanic {
        let (e1, e2, e3, e4, e5, e6, e7, e8, e9) = rest;
        (head, e1, e2, e3, e4, e5, e6, e7, e8, e9)
    }
}

impl TupleSplitTupleSize11<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10,
> of CollectionSplit<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10)> {
    type Head = E0;
    type Rest = (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10);
    fn split_head(
        self: (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10),
    ) -> (E0, (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10)) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10) = self;
        (e0, (e1, e2, e3, e4, e5, e6, e7, e8, e9, e10))
    }
    fn reconstruct(
        head: E0, rest: (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10),
    ) -> (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10) nopanic {
        let (e1, e2, e3, e4, e5, e6, e7, e8, e9, e10) = rest;
        (head, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10)
    }
}

impl TupleSplitTupleSize12<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11,
> of CollectionSplit<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11)> {
    type Head = E0;
    type Rest = (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11);
    fn split_head(
        self: (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11),
    ) -> (E0, (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11)) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11) = self;
        (e0, (e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11))
    }
    fn reconstruct(
        head: E0, rest: (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11),
    ) -> (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11) nopanic {
        let (e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11) = rest;
        (head, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11)
    }
}

impl TupleSplitTupleSize13<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12,
> of CollectionSplit<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12)> {
    type Head = E0;
    type Rest = (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12);
    fn split_head(
        self: (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12),
    ) -> (E0, (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12)) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12) = self;
        (e0, (e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12))
    }
    fn reconstruct(
        head: E0, rest: (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12),
    ) -> (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12) nopanic {
        let (e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12) = rest;
        (head, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12)
    }
}

impl TupleSplitTupleSize14<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13,
> of CollectionSplit<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13)> {
    type Head = E0;
    type Rest = (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13);
    fn split_head(
        self: (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13),
    ) -> (E0, (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13)) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13) = self;
        (e0, (e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13))
    }
    fn reconstruct(
        head: E0, rest: (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13),
    ) -> (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13) nopanic {
        let (e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13) = rest;
        (head, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13)
    }
}

impl TupleSplitTupleSize15<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14,
> of CollectionSplit<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14)> {
    type Head = E0;
    type Rest = (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14);
    fn split_head(
        self: (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14),
    ) -> (E0, (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14)) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14) = self;
        (e0, (e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14))
    }
    fn reconstruct(
        head: E0, rest: (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14),
    ) -> (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14) nopanic {
        let (e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14) = rest;
        (head, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14)
    }
}

impl TupleSplitTupleSize16<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15,
> of CollectionSplit<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15)> {
    type Head = E0;
    type Rest = (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15);
    fn split_head(
        self: (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15),
    ) -> (E0, (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15)) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15) = self;
        (e0, (e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15))
    }
    fn reconstruct(
        head: E0, rest: (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15),
    ) -> (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15) nopanic {
        let (e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15) = rest;
        (head, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15)
    }
}


impl TupleSplitFixedSizedArraySized1<T> of CollectionSplit<[T; 1]> {
    type Head = T;
    type Rest = [T; 0];
    fn split_head(self: [T; 1]) -> (T, [T; 0]) nopanic {
        let [e0] = self;
        (e0, [])
    }
    fn reconstruct(head: T, rest: [T; 0]) -> [T; 1] nopanic {
        let [] = rest;
        [head]
    }
}

impl TupleSplitFixedSizedArraySized2<T> of CollectionSplit<[T; 2]> {
    type Head = T;
    type Rest = [T; 1];
    fn split_head(self: [T; 2]) -> (T, [T; 1]) nopanic {
        let [e0, e1] = self;
        (e0, [e1])
    }
    fn reconstruct(head: T, rest: [T; 1]) -> [T; 2] nopanic {
        let [e1] = rest;
        [head, e1]
    }
}

impl TupleSplitFixedSizedArraySized3<T> of CollectionSplit<[T; 3]> {
    type Head = T;
    type Rest = [T; 2];
    fn split_head(self: [T; 3]) -> (T, [T; 2]) nopanic {
        let [e0, e1, e2] = self;
        (e0, [e1, e2])
    }
    fn reconstruct(head: T, rest: [T; 2]) -> [T; 3] nopanic {
        let [e1, e2] = rest;
        [head, e1, e2]
    }
}

impl TupleSplitFixedSizedArraySized4<T> of CollectionSplit<[T; 4]> {
    type Head = T;
    type Rest = [T; 3];
    fn split_head(self: [T; 4]) -> (T, [T; 3]) nopanic {
        let [e0, e1, e2, e3] = self;
        (e0, [e1, e2, e3])
    }
    fn reconstruct(head: T, rest: [T; 3]) -> [T; 4] nopanic {
        let [e1, e2, e3] = rest;
        [head, e1, e2, e3]
    }
}

impl TupleSplitFixedSizedArraySized5<T> of CollectionSplit<[T; 5]> {
    type Head = T;
    type Rest = [T; 4];
    fn split_head(self: [T; 5]) -> (T, [T; 4]) nopanic {
        let [e0, e1, e2, e3, e4] = self;
        (e0, [e1, e2, e3, e4])
    }
    fn reconstruct(head: T, rest: [T; 4]) -> [T; 5] nopanic {
        let [e1, e2, e3, e4] = rest;
        [head, e1, e2, e3, e4]
    }
}

impl TupleSplitFixedSizedArraySized6<T> of CollectionSplit<[T; 6]> {
    type Head = T;
    type Rest = [T; 5];
    fn split_head(self: [T; 6]) -> (T, [T; 5]) nopanic {
        let [e0, e1, e2, e3, e4, e5] = self;
        (e0, [e1, e2, e3, e4, e5])
    }
    fn reconstruct(head: T, rest: [T; 5]) -> [T; 6] nopanic {
        let [e1, e2, e3, e4, e5] = rest;
        [head, e1, e2, e3, e4, e5]
    }
}

impl TupleSplitFixedSizedArraySized7<T> of CollectionSplit<[T; 7]> {
    type Head = T;
    type Rest = [T; 6];
    fn split_head(self: [T; 7]) -> (T, [T; 6]) nopanic {
        let [e0, e1, e2, e3, e4, e5, e6] = self;
        (e0, [e1, e2, e3, e4, e5, e6])
    }
    fn reconstruct(head: T, rest: [T; 6]) -> [T; 7] nopanic {
        let [e1, e2, e3, e4, e5, e6] = rest;
        [head, e1, e2, e3, e4, e5, e6]
    }
}

impl TupleSplitFixedSizedArraySized8<T> of CollectionSplit<[T; 8]> {
    type Head = T;
    type Rest = [T; 7];
    fn split_head(self: [T; 8]) -> (T, [T; 7]) nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7] = self;
        (e0, [e1, e2, e3, e4, e5, e6, e7])
    }
    fn reconstruct(head: T, rest: [T; 7]) -> [T; 8] nopanic {
        let [e1, e2, e3, e4, e5, e6, e7] = rest;
        [head, e1, e2, e3, e4, e5, e6, e7]
    }
}

impl TupleSplitFixedSizedArraySized9<T> of CollectionSplit<[T; 9]> {
    type Head = T;
    type Rest = [T; 8];
    fn split_head(self: [T; 9]) -> (T, [T; 8]) nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8] = self;
        (e0, [e1, e2, e3, e4, e5, e6, e7, e8])
    }
    fn reconstruct(head: T, rest: [T; 8]) -> [T; 9] nopanic {
        let [e1, e2, e3, e4, e5, e6, e7, e8] = rest;
        [head, e1, e2, e3, e4, e5, e6, e7, e8]
    }
}

impl TupleSplitFixedSizedArraySized10<T> of CollectionSplit<[T; 10]> {
    type Head = T;
    type Rest = [T; 9];
    fn split_head(self: [T; 10]) -> (T, [T; 9]) nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9] = self;
        (e0, [e1, e2, e3, e4, e5, e6, e7, e8, e9])
    }
    fn reconstruct(head: T, rest: [T; 9]) -> [T; 10] nopanic {
        let [e1, e2, e3, e4, e5, e6, e7, e8, e9] = rest;
        [head, e1, e2, e3, e4, e5, e6, e7, e8, e9]
    }
}

impl TupleSplitFixedSizedArraySized11<T> of CollectionSplit<[T; 11]> {
    type Head = T;
    type Rest = [T; 10];
    fn split_head(self: [T; 11]) -> (T, [T; 10]) nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10] = self;
        (e0, [e1, e2, e3, e4, e5, e6, e7, e8, e9, e10])
    }
    fn reconstruct(head: T, rest: [T; 10]) -> [T; 11] nopanic {
        let [e1, e2, e3, e4, e5, e6, e7, e8, e9, e10] = rest;
        [head, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10]
    }
}

impl TupleSplitFixedSizedArraySized12<T> of CollectionSplit<[T; 12]> {
    type Head = T;
    type Rest = [T; 11];
    fn split_head(self: [T; 12]) -> (T, [T; 11]) nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11] = self;
        (e0, [e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11])
    }
    fn reconstruct(head: T, rest: [T; 11]) -> [T; 12] nopanic {
        let [e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11] = rest;
        [head, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11]
    }
}

impl TupleSplitFixedSizedArraySized13<T> of CollectionSplit<[T; 13]> {
    type Head = T;
    type Rest = [T; 12];
    fn split_head(self: [T; 13]) -> (T, [T; 12]) nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12] = self;
        (e0, [e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12])
    }
    fn reconstruct(head: T, rest: [T; 12]) -> [T; 13] nopanic {
        let [e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12] = rest;
        [head, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12]
    }
}

impl TupleSplitFixedSizedArraySized14<T> of CollectionSplit<[T; 14]> {
    type Head = T;
    type Rest = [T; 13];
    fn split_head(self: [T; 14]) -> (T, [T; 13]) nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13] = self;
        (e0, [e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13])
    }
    fn reconstruct(head: T, rest: [T; 13]) -> [T; 14] nopanic {
        let [e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13] = rest;
        [head, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13]
    }
}

impl TupleSplitFixedSizedArraySized15<T> of CollectionSplit<[T; 15]> {
    type Head = T;
    type Rest = [T; 14];
    fn split_head(self: [T; 15]) -> (T, [T; 14]) nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14] = self;
        (e0, [e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14])
    }
    fn reconstruct(head: T, rest: [T; 14]) -> [T; 15] nopanic {
        let [e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14] = rest;
        [head, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14]
    }
}

impl TupleSplitFixedSizedArraySized16<T> of CollectionSplit<[T; 16]> {
    type Head = T;
    type Rest = [T; 15];
    fn split_head(self: [T; 16]) -> (T, [T; 15]) nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15] = self;
        (e0, [e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15])
    }
    fn reconstruct(head: T, rest: [T; 15]) -> [T; 16] nopanic {
        let [e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15] = rest;
        [head, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15]
    }
}

