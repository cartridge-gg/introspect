use core::poseidon::hades_permutation;

pub impl SpanDefault<T, +Drop<T>> of Default<Span<T>> {
    fn default() -> Span<T> {
        array![].span()
    }
}


pub fn poseidon_hash_single<T, +Into<T, felt252>, +Drop<T>>(value: T) -> felt252 {
    let (r, _, _) = hades_permutation(value.into(), 1, 0);
    r
}

pub fn poseidon_hash_two<T, S, +Into<T, felt252>, +Into<S, felt252>, +Drop<S>>(
    value_1: T, value_2: S,
) -> felt252 {
    let (s0, s1, s2) = hades_permutation(value_1.into(), value_2.into(), 0);
    let (r, _, _) = hades_permutation(s0 + 1, s1, s2);
    r
}

pub fn poseidon_hash_three<
    T, S, U, +Into<T, felt252>, +Into<S, felt252>, +Into<U, felt252>, +Drop<S>, +Drop<U>,
>(
    value_1: T, value_2: S, value_3: U,
) -> felt252 {
    let (s0, s1, s2) = hades_permutation(value_1.into(), value_2.into(), 0);
    let (r, _, _) = hades_permutation(s0 + value_3.into(), s1 + 1, s2);
    r
}
