use cgg_utils::{CollectionSplit, SnapForward};
use core::integer::u512;
use core::metaprogramming::TypeEqual;
use core::nullable::{FromNullableResult, match_nullable};
use core::num::traits::{Pow, Zero};
use starknet::{ClassHash, ContractAddress, EthAddress};

pub const SHIFT_31B: felt252 = 256_u256.pow(31).try_into().unwrap();
///                         b76543210
pub const B31_1: felt252 = 0b00000001 * SHIFT_31B;
pub const B31_2: felt252 = 0b00000010 * SHIFT_31B;
pub const B31_3: felt252 = 0b00000011 * SHIFT_31B;
pub const B31_1_U256: u256 = (0b00000001 * 256_u256.pow(31));
pub const B31_2_U256: u256 = (0b00000010 * 256_u256.pow(31));
pub const SHIFT_30B: felt252 = 256_u256.pow(30).try_into().unwrap();
pub const SHIFT_30B_U256: u256 = 256_u256.pow(30);
pub const B30_MASK: u256 = 256_u256.pow(30) - 1;

pub const B31_PARTIAL_1B: felt252 = B31_1 + SHIFT_30B;
pub const B31_PARTIAL_2B: felt252 = B31_1 + (SHIFT_30B * 2);
pub const B31_PARTIAL_3B: felt252 = B31_1 + (SHIFT_30B * 3);
pub const B31_PARTIAL_4B: felt252 = B31_1 + (SHIFT_30B * 4);
pub const B31_PARTIAL_5B: felt252 = B31_1 + (SHIFT_30B * 5);
pub const B31_PARTIAL_6B: felt252 = B31_1 + (SHIFT_30B * 6);
pub const B31_PARTIAL_7B: felt252 = B31_1 + (SHIFT_30B * 7);
pub const B31_PARTIAL_8B: felt252 = B31_1 + (SHIFT_30B * 8);
pub const B31_PARTIAL_9B: felt252 = B31_1 + (SHIFT_30B * 9);
pub const B31_PARTIAL_10B: felt252 = B31_1 + (SHIFT_30B * 10);
pub const B31_PARTIAL_11B: felt252 = B31_1 + (SHIFT_30B * 11);
pub const B31_PARTIAL_12B: felt252 = B31_1 + (SHIFT_30B * 12);
pub const B31_PARTIAL_13B: felt252 = B31_1 + (SHIFT_30B * 13);
pub const B31_PARTIAL_14B: felt252 = B31_1 + (SHIFT_30B * 14);
pub const B31_PARTIAL_15B: felt252 = B31_1 + (SHIFT_30B * 15);
pub const B31_PARTIAL_16B: felt252 = B31_1 + (SHIFT_30B * 16);

pub const fn partial_terminator<const SIZE: u8>() -> felt252 {
    B31_3 + (SHIFT_30B * SIZE.into())
}

impl SizeAdd of Add<Option<usize>> {
    const fn add(lhs: Option<usize>, rhs: Option<usize>) -> Option<usize> {
        match (lhs, rhs) {
            (Some(s), Some(o)) => Some(s + o),
            _ => None,
        }
    }
}

impl SizeMul of Mul<Option<usize>> {
    const fn mul(lhs: Option<usize>, rhs: Option<usize>) -> Option<usize> {
        match (lhs, rhs) {
            (Some(s), Some(o)) => Some(s * o),
            _ => None,
        }
    }
}

#[generate_trait]
pub impl SizeImpl of SizeTrait {
    const fn mul_checked(self: Option<u32>, other: u32) -> Option<u32> {
        match self {
            Some(s) => Some(s * other),
            None => None,
        }
    }
    const fn increment(self: Option<u32>) -> Option<u32> {
        self.add_checked(1)
    }
    const fn add_checked(self: Option<u32>, other: u32) -> Option<u32> {
        match self {
            Some(s) => Some(s + other),
            None => None,
        }
    }
    const fn matches(self: Option<u32>, other: Option<u32>) -> Option<u32> {
        match (self, other) {
            (Some(s), Some(o)) => match s == o {
                true => Some(s),
                false => None,
            },
            _ => None,
        }
    }
}

pub trait ISerde<T> {
    // const SIZE_HINT: Option<usize>;
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
    fn ideserialize(ref serialized: Span<felt252>) -> Option<T>;
}

pub trait ISerdeConst {
    // const SIZE_HINT: Option<usize>;
    fn iserialize(ref output: Array<felt252>);
}

impl ISerdeNone of ISerdeConst {
    // const SIZE_HINT: Option<usize> = None;
    fn iserialize(ref output: Array<felt252>) {
        output.append(0)
    }
}

impl FeltIntoBool of Into<felt252, bool> {
    fn into(self: felt252) -> bool {
        self != 0
    }
}

#[inline]
pub fn iserialize_keyed_type<T, +ISerde<T>>(id: felt252, value: @T, ref output: Array<felt252>) {
    output.append(id);
    ISerde::<T>::iserialize(value, ref output);
}

pub mod into_felt252 {
    pub impl ISerdeImpl<T, +Copy<T>, +Into<T, felt252>, +TryInto<felt252, T>> of super::ISerde<T> {
        // const SIZE_HINT: Option<usize> = Some(1);
        fn iserialize(self: @T, ref output: Array<felt252>) {
            output.append((*self).into());
        }

        fn ideserialize(ref serialized: Span<felt252>) -> Option<T> {
            Some((*serialized.pop_front()?).try_into()?)
        }
    }
}

pub mod empty {
    pub impl ISerdeImpl<T, +Default<T>> of super::ISerde<T> {
        // const SIZE_HINT: Option<usize> = Some(0);
        fn iserialize(self: @T, ref output: Array<felt252>) {}
        fn ideserialize(ref serialized: Span<felt252>) -> Option<T> {
            Some(Default::default())
        }
    }
}


pub impl Felt252ISerde of ISerde<felt252> {
    // const SIZE_HINT: Option<usize> = Some(1);
    fn iserialize(self: @felt252, ref output: Array<felt252>) {
        output.append(*self);
    }
    fn ideserialize(ref serialized: Span<felt252>) -> Option<felt252> {
        Some(*serialized.pop_front()?)
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
pub impl I8ISerde = into_felt252::ISerdeImpl<i8>;
pub impl I16ISerde = into_felt252::ISerdeImpl<i16>;
pub impl I32ISerde = into_felt252::ISerdeImpl<i32>;
pub impl I64ISerde = into_felt252::ISerdeImpl<i64>;
pub impl I128ISerde = into_felt252::ISerdeImpl<i128>;
pub impl ClassHashISerde = into_felt252::ISerdeImpl<ClassHash>;
pub impl ContractAddressISerde = into_felt252::ISerdeImpl<ContractAddress>;
pub impl EthAddressISerde = into_felt252::ISerdeImpl<EthAddress>;

pub impl U256ISerde of ISerde<u256> {
    // const SIZE_HINT: Option<usize> = Some(2);
    fn iserialize(self: @u256, ref output: Array<felt252>) {
        output.append((*self.low).into());
        output.append((*self.high).into());
    }
    fn ideserialize(ref serialized: Span<felt252>) -> Option<u256> {
        let low: u128 = (*serialized.pop_front()?).try_into()?;
        let high: u128 = (*serialized.pop_front()?).try_into()?;
        Some(u256 { low, high })
    }
}

pub impl U512ISerde of ISerde<u512> {
    // const SIZE_HINT: Option<usize> = Some(4);
    fn iserialize(self: @u512, ref output: Array<felt252>) {
        output.append((*self.limb0).into());
        output.append((*self.limb1).into());
        output.append((*self.limb2).into());
        output.append((*self.limb3).into());
    }
    fn ideserialize(ref serialized: Span<felt252>) -> Option<u512> {
        let limb0: u128 = (*serialized.pop_front()?).try_into()?;
        let limb1: u128 = (*serialized.pop_front()?).try_into()?;
        let limb2: u128 = (*serialized.pop_front()?).try_into()?;
        let limb3: u128 = (*serialized.pop_front()?).try_into()?;
        Some(u512 { limb0, limb1, limb2, limb3 })
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
                return data.pop_front().unwrap() + B31_2;
            }
            output.append(data.pop_front().unwrap());
        }
        word + B31_3 + (SHIFT_30B * size)
    }

    fn ideserialize_and_last(ref serialized: Span<felt252>) -> Option<(Array<felt252>, felt252)> {
        let mut data: Array<felt252> = Default::default();
        let value = loop {
            let value = *serialized.pop_front()?;
            match value.into() >= B31_2_U256 {
                true => { break value - B31_2; },
                false => data.append(value),
            }
        };
        Some((data, value))
    }

    fn ideserialize_from_parts(mut rest: Array<felt252>, last: felt252) -> Option<ByteArray> {
        let last_u256 = last.into();
        let (pending_word, pending_word_len) = if last_u256 >= B31_1_U256 {
            let len = ((last_u256 - B31_1_U256) / SHIFT_30B_U256).try_into()?;
            ((last_u256 & B30_MASK).try_into()?, len)
        } else {
            rest.append(last);
            (0_felt252, 0_felt252)
        };

        let mut data = Default::default();
        (rest, pending_word, pending_word_len).serialize(ref data);
        let mut span = data.span();
        Serde::deserialize(ref span)
    }
}


/// Minimum felt bytes encoding:
/// 0 bit in the 31st byte indicates if its a full 31 bytes (0 = full, 1 = partial)
/// 1 bit in the 31st byte indicates if there are more felts to come (0 = more, 1 = last):
/// In a partial byte, the size is stored in the 30th byte (0-30)
pub impl ByteArrayISerde of ISerde<ByteArray> {
    // const SIZE_HINT: Option<usize> = None;
    fn iserialize(self: @ByteArray, ref output: Array<felt252>) {
        output.append(self.iserialize_and_last(ref output));
    }
    fn ideserialize(ref serialized: Span<felt252>) -> Option<ByteArray> {
        let (mut rest, last) = ISerdeByteArray::ideserialize_and_last(ref serialized)?;
        ISerdeByteArray::ideserialize_from_parts(rest, last)
    }
}

pub impl OptionTISerde<T, impl S: ISerde<T>> of ISerde<Option<T>> {
    // const SIZE_HINT: Option<usize> = S::SIZE_HINT.add_checked(1);
    fn iserialize(self: @Option<T>, ref output: Array<felt252>) {
        match self {
            Option::Some(value) => {
                output.append(1);
                S::iserialize(value, ref output);
            },
            Option::None => { output.append(0); },
        }
    }

    fn ideserialize(ref serialized: Span<felt252>) -> Option<Option<T>> {
        match *serialized.pop_front()? {
            0 => Some(Option::None),
            1 => Some(Option::Some(S::ideserialize(ref serialized)?)),
            _ => None,
        }
    }
}

pub impl ResultISerde<
    Ok, Err, impl SOk: ISerde<Ok>, impl SErr: ISerde<Err>,
> of ISerde<Result<Ok, Err>> {
    // const SIZE_HINT: Option<usize> = SOk::SIZE_HINT.matches(SErr::SIZE_HINT).increment();
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

    fn ideserialize(ref serialized: Span<felt252>) -> Option<Result<Ok, Err>> {
        match *serialized.pop_front()? {
            0 => Some(Result::Ok(SOk::ideserialize(ref serialized)?)),
            1 => Some(Result::Err(SErr::ideserialize(ref serialized)?)),
            _ => None,
        }
    }
}

pub impl FromNullableResultTISerde<T, impl S: ISerde<T>> of ISerde<FromNullableResult<T>> {
    // const SIZE_HINT: Option<usize> = S::SIZE_HINT.increment();
    fn iserialize(self: @FromNullableResult<T>, ref output: Array<felt252>) {
        match self {
            FromNullableResult::NotNull(value) => {
                output.append(1);
                BoxTISerde::<T, S>::iserialize(value, ref output);
            },
            FromNullableResult::Null => { output.append(0); },
        }
    }
    fn ideserialize(ref serialized: Span<felt252>) -> Option<FromNullableResult<T>> {
        match *serialized.pop_front()? {
            0 => Some(FromNullableResult::Null),
            1 => Some(
                FromNullableResult::NotNull(BoxTISerde::<T, S>::ideserialize(ref serialized)?),
            ),
            _ => None,
        }
    }
}

pub impl NullableTISerde<T, impl S: ISerde<T>> of ISerde<Nullable<T>> {
    // const SIZE_HINT: Option<usize> = S::SIZE_HINT.increment();
    fn iserialize(self: @Nullable<T>, ref output: Array<felt252>) {
        match match_nullable(self.as_snapshot()) {
            FromNullableResult::NotNull(value) => {
                output.append(1);
                S::iserialize(value.deref(), ref output);
            },
            FromNullableResult::Null => { output.append(0); },
        }
    }
    fn ideserialize(ref serialized: Span<felt252>) -> Option<Nullable<T>> {
        match *serialized.pop_front()? {
            0 => Some(Default::default()),
            1 => Some(NullableTrait::new(S::ideserialize(ref serialized)?)),
            _ => None,
        }
    }
}

pub impl ArrayTISerde<T, impl S: ISerde<T>, +Drop<T>> of ISerde<Array<T>> {
    // const SIZE_HINT: Option<usize> = None;
    fn iserialize(self: @Array<T>, ref output: Array<felt252>) {
        output.append(self.len().into());
        for item in self {
            S::iserialize(item, ref output);
        }
    }
    fn ideserialize(ref serialized: Span<felt252>) -> Option<Array<T>> {
        let len: usize = (*serialized.pop_front()?).try_into()?;
        let mut array: Array<T> = Default::default();
        for _ in 0..len {
            array.append(S::ideserialize(ref serialized)?);
        }
        Some(array)
    }
}

pub impl SpanTISerde<T, impl S: ISerde<T>, +Drop<T>> of ISerde<Span<T>> {
    // const SIZE_HINT: Option<usize> = None;
    fn iserialize(self: @Span<T>, ref output: Array<felt252>) {
        output.append(self.len().into());
        for item in self {
            S::iserialize(item, ref output);
        }
    }
    fn ideserialize(ref serialized: Span<felt252>) -> Option<Span<T>> {
        Some(ArrayTISerde::ideserialize(ref serialized)?.span())
    }
}


pub impl BoxTISerde<T, impl S: ISerde<T>> of ISerde<Box<T>> {
    // const SIZE_HINT: Option<usize> = S::SIZE_HINT;
    fn iserialize(self: @Box<T>, ref output: Array<felt252>) {
        S::iserialize(self.as_snapshot().unbox(), ref output);
    }
    fn ideserialize(ref serialized: Span<felt252>) -> Option<Box<T>> {
        Some(BoxTrait::new(S::ideserialize(ref serialized)?))
    }
}

pub trait ISerializeTuple<T> {
    // const SIZE_HINT: Option<usize>;
    fn iserialize_tuple(self: T, ref output: Array<felt252>);
}

pub trait IDeserializeTuple<T> {
    fn ideserialize_tuple(ref serialized: Span<felt252>) -> Option<T>;
}

pub impl ISerializeTuple1<T0, impl S0: ISerde<T0>, +Drop<T0>> of ISerializeTuple<(T0,)> {
    // const SIZE_HINT: Option<usize> = S0::SIZE_HINT;
    fn iserialize_tuple(self: (T0,), ref output: Array<felt252>) {
        let (val0,) = self;
        S0::iserialize(@val0, ref output);
    }
}

pub impl IDeserializeTuple1<T, impl S: ISerde<T>, +Drop<T>> of IDeserializeTuple<(T,)> {
    fn ideserialize_tuple(ref serialized: Span<felt252>) -> Option<(T,)> {
        Some((S::ideserialize(ref serialized)?,))
    }
}

impl ISerdeTupleNext<
    T,
    impl CS: CollectionSplit<T>,
    impl IH: ISerde<CS::Head>,
    impl IR: ISerializeTuple<CS::Rest>,
    +Drop<CS::Rest>,
    +Drop<CS::Head>,
> of ISerializeTuple<T> {
    // const SIZE_HINT: Option<usize> = IH::SIZE_HINT + IR::SIZE_HINT;
    fn iserialize_tuple(self: T, ref output: Array<felt252>) {
        let (head, rest) = CS::split_head(self);
        ISerde::<CS::Head>::iserialize(@head, ref output);
        ISerializeTuple::<CS::Rest>::iserialize_tuple(rest, ref output);
    }
}


impl IDeserializeTupleNext<
    T,
    impl CS: CollectionSplit<T>,
    impl SH: ISerde<CS::Head>,
    impl SR: IDeserializeTuple<CS::Rest>,
    +Drop<CS::Rest>,
    +Drop<CS::Head>,
> of IDeserializeTuple<T> {
    fn ideserialize_tuple(ref serialized: Span<felt252>) -> Option<T> {
        let head = SH::ideserialize(ref serialized)?;
        let rest = SR::ideserialize_tuple(ref serialized)?;
        Some(CS::reconstruct(head, rest))
    }
}

pub trait IDeserializeFixedArray<T> {
    fn ideserialize_fixed_array(ref serialized: Span<felt252>) -> Option<T>;
}


impl IDeserializeFixedArray1<T, +ISerde<T>, +Drop<T>> of IDeserializeFixedArray<[T; 1]> {
    fn ideserialize_fixed_array(ref serialized: Span<felt252>) -> Option<[T; 1]> {
        Some([ISerde::<T>::ideserialize(ref serialized)?])
    }
}

impl IDeserializeFixedArrayNext<
    T,
    impl CS: CollectionSplit<T>,
    +IDeserializeFixedArray<CS::Rest>,
    +ISerde<CS::Head>,
    +Drop<CS::Rest>,
    +Drop<CS::Head>,
> of IDeserializeFixedArray<T> {
    fn ideserialize_fixed_array(ref serialized: Span<felt252>) -> Option<T> {
        let head = ISerde::<CS::Head>::ideserialize(ref serialized)?;
        let rest = IDeserializeFixedArray::<CS::Rest>::ideserialize_fixed_array(ref serialized)?;
        Some(CS::reconstruct(head, rest))
    }
}

impl TupleISerde<
    T,
    impl SF: SnapForward<T>,
    impl S: ISerializeTuple<SF::SnapForward>,
    impl D: IDeserializeTuple<T>,
> of ISerde<T> {
    // const SIZE_HINT: Option<usize> = S::SIZE_HINT;
    fn iserialize(self: @T, ref output: Array<felt252>) {
        S::iserialize_tuple(self.snap_forward(), ref output);
    }
    fn ideserialize(ref serialized: Span<felt252>) -> Option<T> {
        D::ideserialize_tuple(ref serialized)
    }
}


pub impl FixedArrayTNISerde<
    T,
    const SIZE: usize,
    impl S: ISerde<T>,
    +IDeserializeFixedArray<[T; SIZE]>,
    +Drop<T>,
    -TypeEqual<[T; SIZE], [T; 0]>,
> of ISerde<[T; SIZE]> {
    // const SIZE_HINT: Option<usize> = S::SIZE_HINT.mul_checked(SIZE);
    fn iserialize(self: @[T; SIZE], ref output: Array<felt252>) {
        for item in self {
            S::iserialize(item, ref output);
        }
    }
    fn ideserialize(ref serialized: Span<felt252>) -> Option<[T; SIZE]> {
        IDeserializeFixedArray::ideserialize_fixed_array(ref serialized)
    }
}


impl SSISerde<T, +ISerde<T>, +Drop<T>> of ISerde<@T> {
    // const SIZE_HINT: Option<usize> = ISerde::<T>::SIZE_HINT;
    fn iserialize(self: @@T, ref output: Array<felt252>) {
        ISerde::<T>::iserialize(*self, ref output);
    }
    fn ideserialize(ref serialized: Span<felt252>) -> Option<@T> {
        Some(@ISerde::<T>::ideserialize(ref serialized)?)
    }
}

