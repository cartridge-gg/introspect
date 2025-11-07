use core::metaprogramming::TypeEqual;
use starknet::{ClassHash, ContractAddress};

pub trait ISerde<T> {
    fn iserialize(self: @T, ref output: Array<felt252>);
}

pub mod into_felt252 {
    pub impl ISerdeImpl<T, +Copy<T>, +Into<T, felt252>> of super::ISerde<T> {
        fn iserialize(self: @T, ref output: Array<felt252>) {
            output.append((*self).into());
        }
    }
}

pub mod empty {
    pub impl ISerdeImpl<T> of super::ISerde<T> {
        fn iserialize(self: @T, ref output: Array<felt252>) {}
    }
}

pub impl Felt252ISerde of ISerde<felt252> {
    fn iserialize(self: @felt252, ref output: Array<felt252>) {
        output.append(*self);
    }
}

pub impl FixedArrayT0ISerde<T> = empty::ISerdeImpl<[T; 0]>;
pub impl EmptyTupleISerde = empty::ISerdeImpl<()>;

pub impl BoolISerde = into_felt252::ISerdeImpl<bool>;
pub impl U8ISerde = into_felt252::ISerdeImpl<u8>;
pub impl U16ISerde = into_felt252::ISerdeImpl<u16>;
pub impl U32ISerde = into_felt252::ISerdeImpl<u32>;
pub impl U64ISerde = into_felt252::ISerdeImpl<u64>;
pub impl U128ISerde = into_felt252::ISerdeImpl<u128>;
pub impl ClassHashISerde = into_felt252::ISerdeImpl<ClassHash>;
pub impl ContractAddressISerde = into_felt252::ISerdeImpl<ContractAddress>;
pub impl EthAddressISerde = into_felt252::ISerdeImpl<starknet::EthAddress>;

pub impl FixedArrayTNISerde<
    T, const SIZE: usize, +ISerde<T>, -TypeEqual<[T; SIZE], [T; 0]>,
> of ISerde<[T; SIZE]> {
    fn iserialize(self: @[T; SIZE], ref output: Array<felt252>) {
        for item in ToSpanTrait::<[T; SIZE], T>::span(self) {
            item.iserialize(ref output);
        }
    }
}


pub impl OptionTISerde<T, +ISerde<T>> of ISerde<Option<T>> {
    fn iserialize(self: @Option<T>, ref output: Array<felt252>) {
        match self {
            Option::Some(value) => {
                output.append(1);
                value.iserialize(ref output)
            },
            Option::None => { output.append(0); },
        }
    }
}

pub impl ResultISerde<T, E, +ISerde<T>, +ISerde<E>> of ISerde<Result<T, E>> {
    fn iserialize(self: @Result<T, E>, ref output: Array<felt252>) {
        match self {
            Result::Ok(value) => {
                output.append(0);
                value.iserialize(ref output)
            },
            Result::Err(err) => {
                output.append(1);
                err.iserialize(ref output)
            },
        }
    }
}

pub impl ArrayTISerde<T, +ISerde<T>> of ISerde<Array<T>> {
    fn iserialize(self: @Array<T>, ref output: Array<felt252>) {
        output.append(self.len().into());
        for item in self {
            item.iserialize(ref output);
        }
    }
}

pub impl SpanTISerde<T, +ISerde<T>> of ISerde<Span<T>> {
    fn iserialize(self: @Span<T>, ref output: Array<felt252>) {
        output.append(self.len().into());
        for item in self {
            item.iserialize(ref output);
        }
    }
}

pub impl Tuple1ISerde<T0, +ISerde<T0>> of ISerde<(T0,)> {
    fn iserialize(self: @(T0,), ref output: Array<felt252>) {
        let (val) = self;
        val.iserialize(ref output);
    }
}

pub impl Tuple1SSISerde<T0, +ISerde<T0>> of ISerde<(@T0,)> {
    fn iserialize(self: @(@T0,), ref output: Array<felt252>) {
        let (val) = self;
        val.iserialize(ref output);
    }
}

pub impl Tuple2ISerde<T0, T1, +ISerde<T0>, +ISerde<T1>> of ISerde<(T0, T1)> {
    fn iserialize(self: @(T0, T1), ref output: Array<felt252>) {
        let (val0, val1) = self;
        val0.iserialize(ref output);
        val1.iserialize(ref output);
    }
}

pub impl Tuple2SSISerde<T0, T1, +ISerde<T0>, +ISerde<T1>> of ISerde<(@T0, @T1)> {
    fn iserialize(self: @(@T0, @T1), ref output: Array<felt252>) {
        let (val0, val1) = self;
        val0.iserialize(ref output);
        val1.iserialize(ref output);
    }
}


pub impl Tuple3ISerde<T0, T1, T2, +ISerde<T0>, +ISerde<T1>, +ISerde<T2>> of ISerde<(T0, T1, T2)> {
    fn iserialize(self: @(T0, T1, T2), ref output: Array<felt252>) {
        let (val0, val1, val2) = self;
        val0.iserialize(ref output);
        val1.iserialize(ref output);
        val2.iserialize(ref output);
    }
}

pub impl Tuple3SSISerde<
    T0, T1, T2, +ISerde<T0>, +ISerde<T1>, +ISerde<T2>,
> of ISerde<(@T0, @T1, @T2)> {
    fn iserialize(self: @(@T0, @T1, @T2), ref output: Array<felt252>) {
        let (val0, val1, val2) = self;
        val0.iserialize(ref output);
        val1.iserialize(ref output);
        val2.iserialize(ref output);
    }
}

pub impl Tuple4ISerde<
    T0, T1, T2, T3, +ISerde<T0>, +ISerde<T1>, +ISerde<T2>, +ISerde<T3>,
> of ISerde<(T0, T1, T2, T3)> {
    fn iserialize(self: @(T0, T1, T2, T3), ref output: Array<felt252>) {
        let (val0, val1, val2, val3) = self;
        val0.iserialize(ref output);
        val1.iserialize(ref output);
        val2.iserialize(ref output);
        val3.iserialize(ref output);
    }
}

pub impl Tuple4SSISerde<
    T0, T1, T2, T3, +ISerde<T0>, +ISerde<T1>, +ISerde<T2>, +ISerde<T3>,
> of ISerde<(@T0, @T1, @T2, @T3)> {
    fn iserialize(self: @(@T0, @T1, @T2, @T3), ref output: Array<felt252>) {
        let (val0, val1, val2, val3) = self;
        val0.iserialize(ref output);
        val1.iserialize(ref output);
        val2.iserialize(ref output);
        val3.iserialize(ref output);
    }
}


pub impl Tuple5ISerde<
    T0, T1, T2, T3, T4, +ISerde<T0>, +ISerde<T1>, +ISerde<T2>, +ISerde<T3>, +ISerde<T4>,
> of ISerde<(T0, T1, T2, T3, T4)> {
    fn iserialize(self: @(T0, T1, T2, T3, T4), ref output: Array<felt252>) {
        let (val0, val1, val2, val3, val4) = self;
        val0.iserialize(ref output);
        val1.iserialize(ref output);
        val2.iserialize(ref output);
        val3.iserialize(ref output);
        val4.iserialize(ref output);
    }
}

