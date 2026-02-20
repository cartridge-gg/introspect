use std::io;

use crate::{CairoDeserializer, DecodeError};
use primitive_types::{U256, U512};
use serde::Serializer;
use serde_json::Serializer as JsonSerializer;
use starknet_types_core::felt::Felt;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum TranscodeError<D, S> {
    #[error("Deserialize error: {0}")]
    Deserialize(D),
    #[error("Serialize error: {0}")]
    Serialize(S),
}

impl<D, S> TranscodeError<D, S> {
    pub fn de(err: D) -> Self {
        Self::Deserialize(err)
    }
    pub fn se(err: S) -> Self {
        Self::Serialize(err)
    }
}

pub trait TranscodeResult<T, D, S> {
    fn map_de(self) -> Result<T, TranscodeError<D, S>>;
    fn and_then_tc<U, F>(self, op: F) -> Result<(), TranscodeError<D, S>>
    where
        F: FnOnce(T) -> Result<U, S>;
}

pub trait TranscodeSerializeResult<T, D, S> {
    fn map_se(self) -> Result<T, TranscodeError<D, S>>;
}

impl<D, S, T> TranscodeResult<T, D, S> for Result<T, D> {
    fn map_de(self) -> Result<T, TranscodeError<D, S>> {
        self.map_err(TranscodeError::Deserialize)
    }
    fn and_then_tc<U, F>(self, f: F) -> Result<(), TranscodeError<D, S>>
    where
        F: FnOnce(T) -> Result<U, S>,
    {
        f(self.map_de()?).map_err(TranscodeError::se)?;
        Ok(())
    }
}

impl<D, S, T> TranscodeSerializeResult<T, D, S> for Result<T, S> {
    fn map_se(self) -> Result<T, TranscodeError<D, S>> {
        self.map_err(TranscodeError::se)
    }
}

pub trait Transcode<In, Out> {
    type SerializeError;
    type DeserializeError;
    fn transcode(
        &self,
        input: &mut In,
        output: &mut Out,
    ) -> Result<(), TranscodeError<Self::DeserializeError, Self::SerializeError>>;
    fn transcode_complete(
        &self,
        input: &mut In,
    ) -> Result<Out, TranscodeError<Self::DeserializeError, Self::SerializeError>>
    where
        Out: Default,
    {
        let mut output = Default::default();
        self.transcode(input, &mut output)?;
        Ok(output)
    }
}

pub trait CairoWrite {
    type Error;
    type Ok;
    fn write_byte(&mut self, byte: u8) -> Result<Self::Ok, Self::Error>;
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<Self::Ok, Self::Error>;
    fn write_variable_bytes(&mut self, bytes: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.write_bytes(&bytes.len().to_be_bytes())?;
        self.write_bytes(bytes)
    }
    fn write_felt(&mut self, felt: Felt) -> Result<Self::Ok, Self::Error>;
}

impl CairoWrite for Vec<u8> {
    type Error = ();
    type Ok = ();
    #[inline]
    fn write_byte(&mut self, byte: u8) -> Result<Self::Ok, Self::Error> {
        self.push(byte);
        Ok(())
    }

    #[inline]
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.extend_from_slice(bytes);
        Ok(())
    }

    #[inline]
    fn write_felt(&mut self, felt: Felt) -> Result<Self::Ok, Self::Error> {
        self.extend_from_slice(&felt.to_bytes_be());
        Ok(())
    }
}

pub struct IoCairoWrite<W: io::Write> {
    inner: W,
}

impl<W: io::Write> IoCairoWrite<W> {
    #[inline]
    pub fn new(inner: W) -> Self {
        Self { inner }
    }

    #[inline]
    pub fn into_inner(self) -> W {
        self.inner
    }
}

impl<W: io::Write> CairoWrite for IoCairoWrite<W> {
    type Error = io::Error;
    type Ok = ();

    #[inline]
    fn write_byte(&mut self, byte: u8) -> Result<Self::Ok, Self::Error> {
        self.inner.write_all(&[byte])
    }

    #[inline]
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.inner.write_all(bytes)
    }

    #[inline]
    fn write_felt(&mut self, felt: Felt) -> Result<Self::Ok, Self::Error> {
        self.inner.write_all(&felt.to_bytes_be())
    }
}

pub trait TranscodeWriter<In> {
    type DeserializeError;
    type SerializeError;
    fn transcode_bytes<const N: usize>(
        &mut self,
        input: &mut In,
    ) -> Result<(), TranscodeError<Self::DeserializeError, Self::SerializeError>>;
    fn transcode_felt(
        &mut self,
        input: &mut In,
    ) -> Result<(), TranscodeError<Self::DeserializeError, Self::SerializeError>>;
}
impl<Out, In> TranscodeWriter<In> for Out
where
    Self: CairoWrite,
    In: CairoDeserializer,
{
    type DeserializeError = DecodeError;
    type SerializeError = <Self as CairoWrite>::Error;

    fn transcode_bytes<const N: usize>(
        &mut self,
        input: &mut In,
    ) -> Result<(), TranscodeError<Self::DeserializeError, Self::SerializeError>> {
        input
            .next_bytes::<N>()
            .and_then_tc(|b| self.write_bytes(&b))?;
        Ok(())
    }
    fn transcode_felt(
        &mut self,
        input: &mut In,
    ) -> Result<(), TranscodeError<Self::DeserializeError, Self::SerializeError>> {
        input.next_felt().and_then_tc(|f| self.write_felt(f))?;
        Ok(())
    }
}

pub trait CairoSerializer: Serializer {
    fn serialize_byte_string(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(value)
    }
    fn serialize_felt(self, value: [u8; 32]) -> Result<Self::Ok, Self::Error> {
        self.serialize_byte_string(&value)
    }
    fn serialize_eth_address(self, value: [u8; 20]) -> Result<Self::Ok, Self::Error> {
        self.serialize_byte_string(&value)
    }
    fn serialize_u256(self, value: U256) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&format!("{value}"))
    }
    fn serialize_u512(self, value: U512) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&format!("{value}"))
    }
}

impl<'a, W, F> CairoSerializer for &'a mut JsonSerializer<W, F>
where
    W: io::Write,
    F: serde_json::ser::Formatter,
    &'a mut JsonSerializer<W, F>: Serializer,
{
    fn serialize_byte_string(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&format!("\\x{}", hex::encode(value)))
    }
}
