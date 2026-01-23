use core::fmt::Debug;
use core_ext::CollectionSplit;
pub use snforge_std::fuzzable::{Fuzzable, generate_arg};

pub trait Fuzzy<T, +Debug<T>, +Drop<T>> {
    fn generate() -> T;
    fn generate_boxed() -> Box<T> {
        BoxTrait::new(Self::generate())
    }
    fn generate_array(
        length: u32,
    ) -> Array<T> {
        (0..length).into_iter().map(|_a| Self::generate()).collect::<Array<_>>()
    }
    fn generate_span(length: u32) -> Span<T> {
        Self::generate_array(length).span()
    }
    fn generate_array_lt(
        max_length: u32,
    ) -> Array<T> {
        let length = generate_arg(0, max_length);
        Self::generate_array(length)
    }
    fn generate_span_lt(max_length: u32) -> Span<T> {
        Self::generate_array_lt(max_length).span()
    }
    fn generate_fixed_array<
        const N: u32, +Serde<Span<T>>, +Serde<[T; N]>,
    >() -> [
        T
    ; N] {
        let mut output: Array<felt252> = Default::default();
        Self::generate_span(N).serialize(ref output);
        output.pop_front().unwrap();
        let mut span = output.span();
        Serde::<[T; N]>::deserialize(ref span).unwrap()
    }
}

pub impl FuzzyImpl<T, impl F: Fuzzable<T>, +Drop<T>, +Debug<T>> of Fuzzy<T> {
    fn generate() -> T {
        F::generate()
    }
}

pub trait FuzzableMaxDepth<T, +Drop<T>> {
    fn generate(depth_rem: u32) -> T;
    fn generate_boxed(depth_rem: u32) -> Box<T> {
        BoxTrait::new(Self::generate(depth_rem))
    }
    fn generate_span(
        depth_rem: u32, length: u32,
    ) -> Span<T> {
        Self::generate_array(depth_rem, length).span()
    }
    fn generate_array(
        depth_rem: u32, length: u32,
    ) -> Array<
        T,
    > {
        (0..length).into_iter().map(|_a| Self::generate(depth_rem)).collect::<Array<_>>()
    }
    fn generate_array_lt(
        depth_rem: u32, max_length: u32,
    ) -> Array<
        T,
    > {
        let length = generate_arg(0, max_length);
        Self::generate_array(depth_rem, length)
    }
    fn generate_span_lt(
        depth_rem: u32, max_length: u32,
    ) -> Span<T> {
        Self::generate_array_lt(depth_rem, max_length).span()
    }
}

impl FuzzableCollectionImpl<T, +Debug<T>, +FuzzableCollection<T>> of Fuzzable<T> {
    fn generate() -> T {
        FuzzableCollection::<T>::generate_collection()
    }
    fn blank() -> T {
        FuzzableCollection::<T>::blank_collection()
    }
}

pub trait FuzzableCollection<T> {
    fn generate_collection() -> T;
    fn blank_collection() -> T;
}


impl FuzzableTupleSize1Impl<
    T, impl DB: Debug<T>, impl FuzzyElem: Fuzzable<T, DB>, +Drop<T>,
> of FuzzableCollection<(T,)> {
    fn generate_collection() -> (T,) {
        (FuzzyElem::generate(),)
    }
    fn blank_collection() -> (T,) {
        (FuzzyElem::generate(),)
    }
}

impl FuzzableFixedArraySize1Impl<
    T, impl DB: Debug<T>, impl FuzzyElem: Fuzzable<T, DB>, +Drop<T>,
> of FuzzableCollection<[T; 1]> {
    fn generate_collection() -> [T; 1] {
        [FuzzyElem::generate()]
    }
    fn blank_collection() -> [T; 1] {
        [FuzzyElem::generate()]
    }
}


impl _FuzzableCollectionImpl<
    T,
    impl CS: CollectionSplit<T>,
    impl DH: Debug<CS::Head>,
    +Drop<CS::Head>,
    +Drop<CS::Rest>,
    impl FuzzyHead: Fuzzable<CS::Head, DH>,
    impl FuzzyRest: FuzzableCollection<CS::Rest>,
> of FuzzableCollection<T> {
    fn generate_collection() -> T {
        let head = FuzzyHead::generate();
        let rest = FuzzyRest::generate_collection();
        CS::reconstruct(head, rest)
    }
    fn blank_collection() -> T {
        let head = FuzzyHead::blank();
        let rest = FuzzyRest::blank_collection();
        CS::reconstruct(head, rest)
    }
}

pub trait FuzzableMaxDepthNode<T> {
    fn leaf() -> T;
    fn node(depth_rem: u32) -> T;
}

pub impl FuzzableMaxDepthNodeImpl<
    T, impl F: FuzzableMaxDepthNode<T>, +Drop<T>,
> of FuzzableMaxDepth<T> {
    fn generate(depth_rem: u32) -> T {
        match depth_rem {
            0 => F::leaf(),
            _ => F::node(depth_rem - 1),
        }
    }
}


pub mod attribute;
pub mod database;
pub mod id_data;
pub mod primary;
pub mod schema;
pub mod type_def;
pub use attribute::FuzzableAttribute;
pub use id_data::IdDataFuzzable;
pub use primary::{PrimaryDefFuzzable, PrimaryTypeDefFuzzable};
pub use schema::FuzzableExtColumnDef;
pub use type_def::{TypeDefFuzzable, TypeDefFuzzableToDepth};
