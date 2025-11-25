use core::integer::u512;
use core::metaprogramming::TypeEqual;
use core::nullable::{FromNullableResult, match_nullable};
use core::num::traits::{Pow, Zero};
use starknet::{ClassHash, ContractAddress};
use crate::Attribute;

const SHIFT_31B: felt252 = 256_u256.pow(31).try_into().unwrap();
///                         b76543210
pub const B31_1: felt252 = 0b00000001 * SHIFT_31B;
pub const B31_2: felt252 = 0b00000010 * SHIFT_31B;
pub const B31_3: felt252 = 0b00000011 * SHIFT_31B;
pub const B31_4: felt252 = 0b00000100 * SHIFT_31B;
pub const SHIFT_30B: felt252 = 256_u256.pow(30).try_into().unwrap();

pub trait ISerde<T> {
    fn iserialize(self: @T, ref output: Array<felt252>);
    fn iserialize_inline(
        self: @T,
    ) -> Span<
        felt252,
    > {
        let mut data: Array<felt252> = Default::default();
        Self::iserialize(self, ref data);
        data.span()
    }
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

pub impl Byte31ISerde = into_felt252::ISerdeImpl<bytes31>;
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
        output.append((*self.low).into());
        output.append((*self.high).into());
    }
}

pub impl U512ISerde of ISerde<u512> {
    fn iserialize(self: @u512, ref output: Array<felt252>) {
        output.append((*self.limb0).into());
        output.append((*self.limb1).into());
        output.append((*self.limb2).into());
        output.append((*self.limb3).into());
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


pub impl AttributeISerde of ISerde<Attribute> {
    fn iserialize(self: @Attribute, ref output: Array<felt252>) {
        match self.data {
            Option::Some(data) => {
                output.append(self.name.iserialize_and_last(ref output) + B31_4);
                data.iserialize(ref output);
            },
            Option::None => { self.name.iserialize(ref output); },
        }
    }
}


#[generate_trait]
pub impl ByteArrayISerdeImpl of ISerdeByteArray {
    fn iserialize_and_last(self: @ByteArray, ref output: Array<felt252>) -> felt252 {
        let mut data: Array<felt252> = Default::default();
        self.serialize(ref data);
        let full_felts: u32 = data.pop_front().unwrap().try_into().unwrap();
        let mut data_span = data.span();
        let [word, size] = data_span.multi_pop_back::<2>().unwrap().unbox();
        if full_felts.is_non_zero() {
            for _ in 0..(full_felts - 1) {
                output.append(data.pop_front().unwrap());
            }
            if size.is_zero() {
                return data.pop_front().unwrap() + B31_1;
            }
            output.append(data.pop_front().unwrap());
        }
        word + B31_3 + (SHIFT_30B * size)
    }
}


/// Minimum felt bytes encoding:
/// 0 bit in the 31st byte indicates if its a full 31 bytes (0 = full, 1 = partial)
/// 1 bit in the 31st byte indicates if there are more felts to come (0 = more, 1 = last):
/// In a partial byte, the size is stored in the 30th byte (0-30)
pub impl ByteArrayISerde of ISerde<ByteArray> {
    fn iserialize(self: @ByteArray, ref output: Array<felt252>) {
        let last_felt = self.iserialize_and_last(ref output);
        output.append(last_felt);
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

pub impl FromNullableResultTISerde<T, impl S: ISerde<T>> of ISerde<FromNullableResult<T>> {
    fn iserialize(self: @FromNullableResult<T>, ref output: Array<felt252>) {
        match self {
            FromNullableResult::NotNull(value) => {
                output.append(1);
                BoxTISerde::<T, S>::iserialize(value, ref output);
            },
            FromNullableResult::Null => { output.append(0); },
        }
    }
}

pub impl NullableTISerde<T, impl S: ISerde<T>> of ISerde<Nullable<T>> {
    fn iserialize(self: @Nullable<T>, ref output: Array<felt252>) {
        match match_nullable(self.as_snapshot()) {
            FromNullableResult::NotNull(value) => {
                output.append(1);
                S::iserialize(value.deref(), ref output);
            },
            FromNullableResult::Null => { output.append(0); },
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


pub impl BoxTISerde<T, impl S: ISerde<T>> of ISerde<Box<T>> {
    fn iserialize(self: @Box<T>, ref output: Array<felt252>) {
        S::iserialize(self.as_snapshot().unbox(), ref output);
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

