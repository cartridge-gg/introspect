use core::fmt::Debug;
use snforge_std::fuzzable::{Fuzzable, generate_arg};

pub trait FuzzableExt<T, +Debug<T>> {
    fn generate_boxed() -> Box<T>;
    fn generate_array(length: u32) -> Array<T>;
    fn generate_span(length: u32) -> Span<T>;
    fn generate_array_lt(max_length: u32) -> Array<T>;
    fn generate_span_lt(max_length: u32) -> Span<T>;
}

pub impl FuzzableExtImpl<T, +Fuzzable<T>, +Drop<T>, +Debug<T>> of FuzzableExt<T> {
    fn generate_boxed() -> Box<T> {
        BoxTrait::new(Fuzzable::generate())
    }
    fn generate_array(length: u32) -> Array<T> {
        (0..length).into_iter().map(|_a| Fuzzable::generate()).collect::<Array<_>>()
    }
    fn generate_span(length: u32) -> Span<T> {
        Self::generate_array(length).span()
    }
    fn generate_array_lt(max_length: u32) -> Array<T> {
        let length = generate_arg(0, max_length);
        Self::generate_array(length)
    }
    fn generate_span_lt(max_length: u32) -> Span<T> {
        Self::generate_array_lt(max_length).span()
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
