/// A trait for extending a collection from the front.
pub(crate) trait CollectionExtendFront<T, E> {
    /// The type of the resulting collection.
    type Result;
    /// Creates a new collection from the `value` collection with `element` in front of it.
    fn extend_front(value: T, element: E) -> Self::Result nopanic;
}


impl TupleExtendFrontTupleSize0<E> of CollectionExtendFront<(), E> {
    type Result = (E,);
    fn extend_front(value: (), element: E) -> (E,) nopanic {
        (element,)
    }
}

impl TupleExtendFrontTupleSize1<E0, E> of CollectionExtendFront<(E0,), E> {
    type Result = (E, E0);
    fn extend_front(value: (E0,), element: E) -> (E, E0) nopanic {
        let (e0,) = value;
        (element, e0)
    }
}

impl TupleExtendFrontTupleSize2<E0, E1, E> of CollectionExtendFront<(E0, E1), E> {
    type Result = (E, E0, E1);
    fn extend_front(value: (E0, E1), element: E) -> (E, E0, E1) nopanic {
        let (e0, e1) = value;
        (element, e0, e1)
    }
}

impl TupleExtendFrontTupleSize3<E0, E1, E2, E> of CollectionExtendFront<(E0, E1, E2), E> {
    type Result = (E, E0, E1, E2);
    fn extend_front(value: (E0, E1, E2), element: E) -> (E, E0, E1, E2) nopanic {
        let (e0, e1, e2) = value;
        (element, e0, e1, e2)
    }
}

impl TupleExtendFrontTupleSize4<E0, E1, E2, E3, E> of CollectionExtendFront<(E0, E1, E2, E3), E> {
    type Result = (E, E0, E1, E2, E3);
    fn extend_front(value: (E0, E1, E2, E3), element: E) -> (E, E0, E1, E2, E3) nopanic {
        let (e0, e1, e2, e3) = value;
        (element, e0, e1, e2, e3)
    }
}

impl TupleExtendFrontTupleSize5<
    E0, E1, E2, E3, E4, E,
> of CollectionExtendFront<(E0, E1, E2, E3, E4), E> {
    type Result = (E, E0, E1, E2, E3, E4);
    fn extend_front(value: (E0, E1, E2, E3, E4), element: E) -> (E, E0, E1, E2, E3, E4) nopanic {
        let (e0, e1, e2, e3, e4) = value;
        (element, e0, e1, e2, e3, e4)
    }
}

impl TupleExtendFrontTupleSize6<
    E0, E1, E2, E3, E4, E5, E,
> of CollectionExtendFront<(E0, E1, E2, E3, E4, E5), E> {
    type Result = (E, E0, E1, E2, E3, E4, E5);
    fn extend_front(
        value: (E0, E1, E2, E3, E4, E5), element: E,
    ) -> (E, E0, E1, E2, E3, E4, E5) nopanic {
        let (e0, e1, e2, e3, e4, e5) = value;
        (element, e0, e1, e2, e3, e4, e5)
    }
}

impl TupleExtendFrontTupleSize7<
    E0, E1, E2, E3, E4, E5, E6, E,
> of CollectionExtendFront<(E0, E1, E2, E3, E4, E5, E6), E> {
    type Result = (E, E0, E1, E2, E3, E4, E5, E6);
    fn extend_front(
        value: (E0, E1, E2, E3, E4, E5, E6), element: E,
    ) -> (E, E0, E1, E2, E3, E4, E5, E6) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6) = value;
        (element, e0, e1, e2, e3, e4, e5, e6)
    }
}

impl TupleExtendFrontTupleSize8<
    E0, E1, E2, E3, E4, E5, E6, E7, E,
> of CollectionExtendFront<(E0, E1, E2, E3, E4, E5, E6, E7), E> {
    type Result = (E, E0, E1, E2, E3, E4, E5, E6, E7);
    fn extend_front(
        value: (E0, E1, E2, E3, E4, E5, E6, E7), element: E,
    ) -> (E, E0, E1, E2, E3, E4, E5, E6, E7) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7) = value;
        (element, e0, e1, e2, e3, e4, e5, e6, e7)
    }
}

impl TupleExtendFrontTupleSize9<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E,
> of CollectionExtendFront<(E0, E1, E2, E3, E4, E5, E6, E7, E8), E> {
    type Result = (E, E0, E1, E2, E3, E4, E5, E6, E7, E8);
    fn extend_front(
        value: (E0, E1, E2, E3, E4, E5, E6, E7, E8), element: E,
    ) -> (E, E0, E1, E2, E3, E4, E5, E6, E7, E8) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8) = value;
        (element, e0, e1, e2, e3, e4, e5, e6, e7, e8)
    }
}

impl TupleExtendFrontTupleSize10<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E,
> of CollectionExtendFront<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9), E> {
    type Result = (E, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9);
    fn extend_front(
        value: (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9), element: E,
    ) -> (E, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9) = value;
        (element, e0, e1, e2, e3, e4, e5, e6, e7, e8, e9)
    }
}

impl TupleExtendFrontTupleSize11<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E,
> of CollectionExtendFront<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10), E> {
    type Result = (E, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10);
    fn extend_front(
        value: (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10), element: E,
    ) -> (E, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10) = value;
        (element, e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10)
    }
}

impl TupleExtendFrontTupleSize12<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E,
> of CollectionExtendFront<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11), E> {
    type Result = (E, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11);
    fn extend_front(
        value: (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11), element: E,
    ) -> (E, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11) = value;
        (element, e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11)
    }
}

impl TupleExtendFrontTupleSize13<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E,
> of CollectionExtendFront<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12), E> {
    type Result = (E, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12);
    fn extend_front(
        value: (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12), element: E,
    ) -> (E, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12) = value;
        (element, e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12)
    }
}

impl TupleExtendFrontTupleSize14<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E,
> of CollectionExtendFront<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13), E> {
    type Result = (E, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13);
    fn extend_front(
        value: (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13), element: E,
    ) -> (E, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13) = value;
        (element, e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13)
    }
}

impl TupleExtendFrontTupleSize15<
    E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E,
> of CollectionExtendFront<(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14), E> {
    type Result = (E, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14);
    fn extend_front(
        value: (E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14), element: E,
    ) -> (E, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14) nopanic {
        let (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14) = value;
        (element, e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14)
    }
}


impl TupleExtendFrontFixedSizedArraySize0<T> of CollectionExtendFront<[T; 0], T> {
    type Result = [T; 1];
    fn extend_front(value: [T; 0], element: T) -> [T; 1] nopanic {
        let [] = value;
        [element]
    }
}

impl TupleExtendFrontFixedSizedArraySize1<T> of CollectionExtendFront<[T; 1], T> {
    type Result = [T; 2];
    fn extend_front(value: [T; 1], element: T) -> [T; 2] nopanic {
        let [e0] = value;
        [element, e0]
    }
}

impl TupleExtendFrontFixedSizedArraySize2<T> of CollectionExtendFront<[T; 2], T> {
    type Result = [T; 3];
    fn extend_front(value: [T; 2], element: T) -> [T; 3] nopanic {
        let [e0, e1] = value;
        [element, e0, e1]
    }
}

impl TupleExtendFrontFixedSizedArraySize3<T> of CollectionExtendFront<[T; 3], T> {
    type Result = [T; 4];
    fn extend_front(value: [T; 3], element: T) -> [T; 4] nopanic {
        let [e0, e1, e2] = value;
        [element, e0, e1, e2]
    }
}

impl TupleExtendFrontFixedSizedArraySize4<T> of CollectionExtendFront<[T; 4], T> {
    type Result = [T; 5];
    fn extend_front(value: [T; 4], element: T) -> [T; 5] nopanic {
        let [e0, e1, e2, e3] = value;
        [element, e0, e1, e2, e3]
    }
}

impl TupleExtendFrontFixedSizedArraySize5<T> of CollectionExtendFront<[T; 5], T> {
    type Result = [T; 6];
    fn extend_front(value: [T; 5], element: T) -> [T; 6] nopanic {
        let [e0, e1, e2, e3, e4] = value;
        [element, e0, e1, e2, e3, e4]
    }
}

impl TupleExtendFrontFixedSizedArraySize6<T> of CollectionExtendFront<[T; 6], T> {
    type Result = [T; 7];
    fn extend_front(value: [T; 6], element: T) -> [T; 7] nopanic {
        let [e0, e1, e2, e3, e4, e5] = value;
        [element, e0, e1, e2, e3, e4, e5]
    }
}

impl TupleExtendFrontFixedSizedArraySize7<T> of CollectionExtendFront<[T; 7], T> {
    type Result = [T; 8];
    fn extend_front(value: [T; 7], element: T) -> [T; 8] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6] = value;
        [element, e0, e1, e2, e3, e4, e5, e6]
    }
}

impl TupleExtendFrontFixedSizedArraySize8<T> of CollectionExtendFront<[T; 8], T> {
    type Result = [T; 9];
    fn extend_front(value: [T; 8], element: T) -> [T; 9] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7] = value;
        [element, e0, e1, e2, e3, e4, e5, e6, e7]
    }
}

impl TupleExtendFrontFixedSizedArraySize9<T> of CollectionExtendFront<[T; 9], T> {
    type Result = [T; 10];
    fn extend_front(value: [T; 9], element: T) -> [T; 10] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8] = value;
        [element, e0, e1, e2, e3, e4, e5, e6, e7, e8]
    }
}

impl TupleExtendFrontFixedSizedArraySize10<T> of CollectionExtendFront<[T; 10], T> {
    type Result = [T; 11];
    fn extend_front(value: [T; 10], element: T) -> [T; 11] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9] = value;
        [element, e0, e1, e2, e3, e4, e5, e6, e7, e8, e9]
    }
}

impl TupleExtendFrontFixedSizedArraySize11<T> of CollectionExtendFront<[T; 11], T> {
    type Result = [T; 12];
    fn extend_front(value: [T; 11], element: T) -> [T; 12] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10] = value;
        [element, e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10]
    }
}

impl TupleExtendFrontFixedSizedArraySize12<T> of CollectionExtendFront<[T; 12], T> {
    type Result = [T; 13];
    fn extend_front(value: [T; 12], element: T) -> [T; 13] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11] = value;
        [element, e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11]
    }
}

impl TupleExtendFrontFixedSizedArraySize13<T> of CollectionExtendFront<[T; 13], T> {
    type Result = [T; 14];
    fn extend_front(value: [T; 13], element: T) -> [T; 14] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12] = value;
        [element, e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12]
    }
}

impl TupleExtendFrontFixedSizedArraySize14<T> of CollectionExtendFront<[T; 14], T> {
    type Result = [T; 15];
    fn extend_front(value: [T; 14], element: T) -> [T; 15] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13] = value;
        [element, e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13]
    }
}

impl TupleExtendFrontFixedSizedArraySize15<T> of CollectionExtendFront<[T; 15], T> {
    type Result = [T; 16];
    fn extend_front(value: [T; 15], element: T) -> [T; 16] nopanic {
        let [e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14] = value;
        [element, e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14]
    }
}
