use crate::decode_error::DecodeResultTrait;
use crate::{ByteArray, CairoDeserializer, DecodeError, DecodeResult};
use starknet_types_core::felt::Felt;
use std::ops::Deref;
use std::sync::Arc;
pub trait ByteSource {
    fn next(&mut self) -> DecodeResult<u8>;
    fn nexts<const N: usize>(&mut self) -> DecodeResult<[u8; N]> {
        let mut bytes = [0; N];
        if N > 0 {
            bytes[0] = self.next()?;
            for byte in &mut bytes[1..] {
                *byte = self.next().raise_eof()?;
            }
        }
        Ok(bytes)
    }
    fn position(&self) -> usize;
}

impl<S: ByteSource + ?Sized> ByteSource for &mut S {
    #[inline]
    fn next(&mut self) -> DecodeResult<u8> {
        (**self).next()
    }

    #[inline]
    fn nexts<const N: usize>(&mut self) -> DecodeResult<[u8; N]> {
        (**self).nexts::<N>()
    }

    #[inline]
    fn position(&self) -> usize {
        (**self).position()
    }
}

pub struct DerefBytesSource<B: Deref<Target = [u8]>> {
    bytes: B,
    pos: usize,
}

impl<B: Deref<Target = [u8]>> DerefBytesSource<B> {
    #[inline]
    pub fn new(bytes: B) -> Self {
        Self { bytes, pos: 0 }
    }

    #[inline]
    pub fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.pos)
    }
}

impl<B: Deref<Target = [u8]>> ByteSource for DerefBytesSource<B> {
    #[inline]
    fn next(&mut self) -> DecodeResult<u8> {
        let b = *self.bytes.get(self.pos).ok_or(DecodeError::Eof)?;
        self.pos += 1;
        Ok(b)
    }

    #[inline]
    fn nexts<const N: usize>(&mut self) -> DecodeResult<[u8; N]> {
        let mut bytes = [0; N];
        if N > 0 {
            let remaining = self.remaining();
            if remaining == 0 {
                return Err(DecodeError::Eof);
            } else if remaining < N {
                return Err(DecodeError::UnexpectedEof);
            } else {
                bytes.copy_from_slice(&self.bytes[self.pos..self.pos + N]);
            }
        }
        self.pos += N;
        Ok(bytes)
    }

    #[inline]
    fn position(&self) -> usize {
        self.pos
    }
}

pub type SliceBytesSource<'a> = DerefBytesSource<&'a [u8]>;
pub type VecBytesSource = DerefBytesSource<Vec<u8>>;
pub type ArcBytesSource = DerefBytesSource<Arc<[u8]>>;

pub trait IntoByteSource {
    type Source: ByteSource;
    fn into_source(self) -> Self::Source;
}

impl<'a> IntoByteSource for &'a [u8] {
    type Source = DerefBytesSource<&'a [u8]>;
    #[inline]
    fn into_source(self) -> Self::Source {
        DerefBytesSource::new(self)
    }
}

impl IntoByteSource for Vec<u8> {
    type Source = DerefBytesSource<Vec<u8>>;
    #[inline]
    fn into_source(self) -> Self::Source {
        DerefBytesSource::new(self)
    }
}

impl IntoByteSource for Arc<[u8]> {
    type Source = DerefBytesSource<Arc<[u8]>>;
    #[inline]
    fn into_source(self) -> Self::Source {
        DerefBytesSource::new(self)
    }
}

impl<'a, S: ByteSource + ?Sized> IntoByteSource for &'a mut S {
    type Source = &'a mut S;
    #[inline]
    fn into_source(self) -> Self::Source {
        self
    }
}

impl<B: ByteSource> CairoDeserializer for B {
    fn next_felt(&mut self) -> DecodeResult<Felt> {
        Ok(Felt::from_bytes_be(&self.nexts::<32>()?))
    }
    fn next_byte(&mut self) -> DecodeResult<u8> {
        self.next()
    }
    fn next_bytes<const N: usize>(&mut self) -> DecodeResult<[u8; N]> {
        self.nexts::<N>()
    }
    fn next_byte_array(&mut self) -> DecodeResult<ByteArray>
    where
        Self: Sized,
    {
        self.next_byte_array_bytes().map(Into::into)
    }
    fn next_byte_array_bytes(&mut self) -> DecodeResult<Vec<u8>>
    where
        Self: Sized,
    {
        let len = self.next_u32()? as usize;
        let mut bytes = Vec::with_capacity(len);
        for _ in 0..len {
            bytes.push(self.next_byte().raise_eof()?);
        }
        Ok(bytes)
    }
    fn next_u256(&mut self) -> DecodeResult<primitive_types::U256> {
        Ok(primitive_types::U256::from_big_endian(&self.nexts::<32>()?))
    }
    fn next_u512(&mut self) -> DecodeResult<primitive_types::U512> {
        Ok(primitive_types::U512::from_big_endian(&self.nexts::<64>()?))
    }
    fn next_option_is_some(&mut self) -> DecodeResult<bool> {
        self.next_bool_tag("Option")
    }
}
