use core::poseidon::hades_permutation;

// pub trait ConstFelts<const SIZE: u32> {
//     const DATA: [felt252; SIZE];
//     fn append(ref output: Array<felt252>);
// }

// pub trait AppendFixed<const SIZE: u32, const DATA: [felt252; SIZE]> {
//     fn append_fixed<impl ToSpan: ToSpanTrait<[felt252; SIZE], felt252>>(ref output:
//     Array<felt252>);
// }

pub fn append_const_felts<
    const DATA: [felt252; SIZE],
    const SIZE: u32,
    impl ToSpan: ToSpanTrait<[felt252; SIZE], felt252>,
>(
    ref output: Array<felt252>,
) {
    output.append_span(ToSpan::span(@DATA));
}


// impl ConstFeltsImpl<
//     const SIZE: u32,
//     const DATA: [felt252; SIZE],
//     impl ToSpan: ToSpanTrait<[felt252; SIZE], felt252>,
// > of ConstFelts<SIZE> {
//     const DATA: [felt252; SIZE] = DATA;
//     fn append(ref output: Array<felt252>) {
//         output.append_span(ToSpan::span(@Self::DATA));
//     }
// }

pub impl SpanDefault<T, +Drop<T>> of Default<Span<T>> {
    fn default() -> Span<T> {
        ArrayTrait::new().span()
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

#[cfg(test)]
mod tests {
    use core::hash::HashStateTrait;
    use core::poseidon::{PoseidonTrait, poseidon_hash_span};
    use super::*;

    #[test]
    fn test_poseidon_hash_single() {
        let value = 'salt';
        let result = poseidon_hash_single(value);
        let expected_result = poseidon_hash_span([value].span());
        let other_expected_result = PoseidonTrait::new().update(value).finalize();
        assert_eq!(result, expected_result);
        assert_eq!(result, other_expected_result);
    }

    #[test]
    fn test_poseidon_hash_two() {
        let value_1 = 'salt';
        let value_2 = 'beef';
        let result = poseidon_hash_two(value_1, value_2);
        let expected_result = poseidon_hash_span([value_1, value_2].span());
        let other_expected_result = PoseidonTrait::new().update(value_1).update(value_2).finalize();
        assert_eq!(result, expected_result);
        assert_eq!(result, other_expected_result);
    }

    #[test]
    fn test_poseidon_hash_three() {
        let value_1 = 'salt';
        let value_2 = 'beef';
        let value_3 = 'hash';
        let result = poseidon_hash_three(value_1, value_2, value_3);
        let expected_result = poseidon_hash_span([value_1, value_2, value_3].span());
        let other_expected_result = PoseidonTrait::new()
            .update(value_1)
            .update(value_2)
            .update(value_3)
            .finalize();
        assert_eq!(result, expected_result);
        assert_eq!(result, other_expected_result);
    }
}
