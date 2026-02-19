use std::io;

use crate::{CairoDeserializer, DecodeResult, EthAddress};
use primitive_types::{U256, U512};
use serde::Serializer;
use serde_json::Serializer as JsonSerializer;
use starknet_types_core::felt::Felt;

#[derive(Debug, thiserror::Error)]
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
    fn and_then_tc<U, F>(self, op: F) -> Result<U, TranscodeError<D, S>>
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
    fn and_then_tc<U, F>(self, f: F) -> Result<U, TranscodeError<D, S>>
    where
        F: FnOnce(T) -> Result<U, S>,
    {
        f(self.map_de()?).map_err(TranscodeError::se)
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
    type Ok;
    fn transcode(
        &self,
        input: &mut In,
        output: Out,
    ) -> Result<Self::Ok, TranscodeError<Self::DeserializeError, Self::SerializeError>>;
}

pub trait CairoWrite {
    fn write_byte(&mut self, byte: u8) -> DecodeResult<()>;
    fn write_bytes(&mut self, bytes: &[u8]) -> DecodeResult<()>;
    fn write_variable_bytes(&mut self, bytes: &[u8]) -> DecodeResult<()> {
        self.write_bytes(&bytes.len().to_be_bytes())?;
        self.write_bytes(bytes)
    }
    fn write_felt(&mut self, felt: Felt) -> DecodeResult<()>;
}

pub trait TranscodeWriter<In>
where
    Self: CairoWrite,
    In: CairoDeserializer,
{
    fn transcode_bytes<const N: usize>(&mut self, input: &mut In) -> DecodeResult<()> {
        self.write_bytes(&input.next_bytes::<N>()?)
    }
    fn transcode_felt(&mut self, input: &mut In) -> DecodeResult<()> {
        self.write_felt(input.next_felt()?)
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
