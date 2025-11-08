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

pub impl U256ISerde of ISerde<u256> {
    fn iserialize(self: @u256, ref output: Array<felt252>) {
        self.low.iserialize(ref output);
        self.high.iserialize(ref output);
    }
}

pub impl FixedArrayTNISerde<
    T, const SIZE: usize, +ISerde<T>, -TypeEqual<[T; SIZE], [T; 0]>,
> of ISerde<[T; SIZE]> {
    fn iserialize(self: @[T; SIZE], ref output: Array<felt252>) {
        for item in ToSpanTrait::<[T; SIZE], T>::span(self) {
            item.iserialize(ref output);
        }
    }
}


pub impl OptionTISerde<T, impl S: ISerde<T>> of ISerde<Option<T>> {
    fn iserialize(self: @Option<T>, ref output: Array<felt252>) {
        match self {
            Option::Some(value) => {
                output.append(0);
                S::iserialize(value, ref output);
            },
            Option::None => { output.append(1); },
        }
    }
}

pub impl ResultISerde<
    Ok, Err, impl SOk: ISerde<Ok>, impl SErr: ISerde<Err>,
> of ISerde<Result<Ok, Err>> {
    fn iserialize(self: @Result<Ok, Err>, ref output: Array<felt252>) {
        match self {
            Result::Ok(value) => {
                output.append(0);
                SOk::iserialize(value, ref output)
            },
            Result::Err(err) => {
                output.append(1);
                SErr::iserialize(err, ref output)
            },
        }
    }
}

pub impl ArrayTISerde<T, impl S: ISerde<T>> of ISerde<Array<T>> {
    fn iserialize(self: @Array<T>, ref output: Array<felt252>) {
        output.append(self.len().into());
        for item in self {
            S::iserialize(item, ref output);
        }
    }
}

pub impl SpanTISerde<T, impl S: ISerde<T>> of ISerde<Span<T>> {
    fn iserialize(self: @Span<T>, ref output: Array<felt252>) {
        output.append(self.len().into());
        for item in self {
            S::iserialize(item, ref output);
        }
    }
}

pub impl Tuple1ISerde<T0, impl S: ISerde<T0>> of ISerde<(T0,)> {
    fn iserialize(self: @(T0,), ref output: Array<felt252>) {
        let (val) = self;
        S::iserialize(val, ref output);
    }
}

pub impl Tuple2ISerde<T0, T1, impl S0: ISerde<T0>, impl S1: ISerde<T1>> of ISerde<(T0, T1)> {
    fn iserialize(self: @(T0, T1), ref output: Array<felt252>) {
        let (val0, val1) = self;
        S0::iserialize(val0, ref output);
        S1::iserialize(val1, ref output);
    }
}

pub impl Tuple3ISerde<
    T0, T1, T2, impl S0: ISerde<T0>, impl S1: ISerde<T1>, impl S2: ISerde<T2>,
> of ISerde<(T0, T1, T2)> {
    fn iserialize(self: @(T0, T1, T2), ref output: Array<felt252>) {
        let (val0, val1, val2) = self;
        S0::iserialize(val0, ref output);
        S1::iserialize(val1, ref output);
        S2::iserialize(val2, ref output);
    }
}

pub impl Tuple4ISerde<
    T0,
    T1,
    T2,
    T3,
    impl S0: ISerde<T0>,
    impl S1: ISerde<T1>,
    impl S2: ISerde<T2>,
    impl S3: ISerde<T3>,
> of ISerde<(T0, T1, T2, T3)> {
    fn iserialize(self: @(T0, T1, T2, T3), ref output: Array<felt252>) {
        let (val0, val1, val2, val3) = self;
        S0::iserialize(val0, ref output);
        S1::iserialize(val1, ref output);
        S2::iserialize(val2, ref output);
        S3::iserialize(val3, ref output);
    }
}


pub impl Tuple5ISerde<
    T0,
    T1,
    T2,
    T3,
    T4,
    impl S0: ISerde<T0>,
    impl S1: ISerde<T1>,
    impl S2: ISerde<T2>,
    impl S3: ISerde<T3>,
    impl S4: ISerde<T4>,
> of ISerde<(T0, T1, T2, T3, T4)> {
    fn iserialize(self: @(T0, T1, T2, T3, T4), ref output: Array<felt252>) {
        let (val0, val1, val2, val3, val4) = self;
        S0::iserialize(val0, ref output);
        S1::iserialize(val1, ref output);
        S2::iserialize(val2, ref output);
        S3::iserialize(val3, ref output);
        S4::iserialize(val4, ref output);
    }
}

pub impl Tuple6ISerde<
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    impl S0: ISerde<T0>,
    impl S1: ISerde<T1>,
    impl S2: ISerde<T2>,
    impl S3: ISerde<T3>,
    impl S4: ISerde<T4>,
    impl S5: ISerde<T5>,
> of ISerde<(T0, T1, T2, T3, T4, T5)> {
    fn iserialize(self: @(T0, T1, T2, T3, T4, T5), ref output: Array<felt252>) {
        let (val0, val1, val2, val3, val4, val5) = self;
        S0::iserialize(val0, ref output);
        S1::iserialize(val1, ref output);
        S2::iserialize(val2, ref output);
        S3::iserialize(val3, ref output);
        S4::iserialize(val4, ref output);
        S5::iserialize(val5, ref output);
    }
}

pub impl Tuple7ISerde<
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    impl S0: ISerde<T0>,
    impl S1: ISerde<T1>,
    impl S2: ISerde<T2>,
    impl S3: ISerde<T3>,
    impl S4: ISerde<T4>,
    impl S5: ISerde<T5>,
    impl S6: ISerde<T6>,
> of ISerde<(T0, T1, T2, T3, T4, T5, T6)> {
    fn iserialize(self: @(T0, T1, T2, T3, T4, T5, T6), ref output: Array<felt252>) {
        let (val0, val1, val2, val3, val4, val5, val6) = self;
        S0::iserialize(val0, ref output);
        S1::iserialize(val1, ref output);
        S2::iserialize(val2, ref output);
        S3::iserialize(val3, ref output);
        S4::iserialize(val4, ref output);
        S5::iserialize(val5, ref output);
        S6::iserialize(val6, ref output);
    }
}


pub impl Tuple8ISerde<
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    impl S0: ISerde<T0>,
    impl S1: ISerde<T1>,
    impl S2: ISerde<T2>,
    impl S3: ISerde<T3>,
    impl S4: ISerde<T4>,
    impl S5: ISerde<T5>,
    impl S6: ISerde<T6>,
    impl S7: ISerde<T7>,
> of ISerde<(T0, T1, T2, T3, T4, T5, T6, T7)> {
    fn iserialize(self: @(T0, T1, T2, T3, T4, T5, T6, T7), ref output: Array<felt252>) {
        let (val0, val1, val2, val3, val4, val5, val6, val7) = self;
        S0::iserialize(val0, ref output);
        S1::iserialize(val1, ref output);
        S2::iserialize(val2, ref output);
        S3::iserialize(val3, ref output);
        S4::iserialize(val4, ref output);
        S5::iserialize(val5, ref output);
        S6::iserialize(val6, ref output);
        S7::iserialize(val7, ref output);
    }
}


pub impl Tuple9ISerde<
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    impl S0: ISerde<T0>,
    impl S1: ISerde<T1>,
    impl S2: ISerde<T2>,
    impl S3: ISerde<T3>,
    impl S4: ISerde<T4>,
    impl S5: ISerde<T5>,
    impl S6: ISerde<T6>,
    impl S7: ISerde<T7>,
    impl S8: ISerde<T8>,
> of ISerde<(T0, T1, T2, T3, T4, T5, T6, T7, T8)> {
    fn iserialize(self: @(T0, T1, T2, T3, T4, T5, T6, T7, T8), ref output: Array<felt252>) {
        let (val0, val1, val2, val3, val4, val5, val6, val7, val8) = self;
        S0::iserialize(val0, ref output);
        S1::iserialize(val1, ref output);
        S2::iserialize(val2, ref output);
        S3::iserialize(val3, ref output);
        S4::iserialize(val4, ref output);
        S5::iserialize(val5, ref output);
        S6::iserialize(val6, ref output);
        S7::iserialize(val7, ref output);
        S8::iserialize(val8, ref output);
    }
}


pub impl Tuple10ISerde<
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    impl S0: ISerde<T0>,
    impl S1: ISerde<T1>,
    impl S2: ISerde<T2>,
    impl S3: ISerde<T3>,
    impl S4: ISerde<T4>,
    impl S5: ISerde<T5>,
    impl S6: ISerde<T6>,
    impl S7: ISerde<T7>,
    impl S8: ISerde<T8>,
    impl S9: ISerde<T9>,
> of ISerde<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)> {
    fn iserialize(self: @(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), ref output: Array<felt252>) {
        let (val0, val1, val2, val3, val4, val5, val6, val7, val8, val9) = self;
        S0::iserialize(val0, ref output);
        S1::iserialize(val1, ref output);
        S2::iserialize(val2, ref output);
        S3::iserialize(val3, ref output);
        S4::iserialize(val4, ref output);
        S5::iserialize(val5, ref output);
        S6::iserialize(val6, ref output);
        S7::iserialize(val7, ref output);
        S8::iserialize(val8, ref output);
        S9::iserialize(val9, ref output);
    }
}
