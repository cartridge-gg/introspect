use starknet::core::types::Felt;

trait TryFromFelt: Sized {
    type Error;
    fn try_from_felt(value: Felt) -> Result<Self, Self::Error>;
}

impl TryFromFelt for u64 {
    type Error = &'static str;
    fn try_from_felt(value: Felt) -> Result<Self, Self::Error> {
        match value.to_be_digits() {
            [0, 0, 0, v] => Ok(v),
            _ => Err("Invalid Felt value for u64"),
        }
    }
}

impl TryFromFelt for bool {
    type Error = &'static str;
    fn try_from_felt(value: Felt) -> Result<Self, Self::Error> {
        match value.to_be_digits() {
            [0, 0, 0, 0] => Ok(false),
            [0, 0, 0, 1] => Ok(true),
            _ => Err("Invalid Felt value for bool"),
        }
    }
}

impl TryFromFelt for u8 {
    type Error = &'static str;
    fn try_from_felt(value: Felt) -> Result<Self, Self::Error> {
        match value.to_be_digits() {
            [0, 0, 0, v] if v <= u8::MAX as u64 => Ok(v as u8),
            _ => Err("Invalid Felt value for u8"),
        }
    }
}

impl TryFromFelt for u16 {
    type Error = &'static str;
    fn try_from_felt(value: Felt) -> Result<Self, Self::Error> {
        match value.to_be_digits() {
            [0, 0, 0, v] if v <= u16::MAX as u64 => Ok(v as u16),
            _ => Err("Invalid Felt value for u16"),
        }
    }
}

impl TryFromFelt for u32 {
    type Error = &'static str;
    fn try_from_felt(value: Felt) -> Result<Self, Self::Error> {
        match value.to_be_digits() {
            [0, 0, 0, v] if v <= u32::MAX as u64 => Ok(v as u32),
            _ => Err("Invalid Felt value for u32"),
        }
    }
}

impl TryFromFelt for u128 {
    type Error = &'static str;
    fn try_from_felt(value: Felt) -> Result<Self, Self::Error> {
        match value.to_be_digits() {
            [0, 0, high, low] => Ok(((high as u128) << 64) | (low as u128)),
            _ => Err("Invalid Felt value for u128"),
        }
    }
}
