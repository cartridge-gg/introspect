use starknet_types_core::felt::Felt;

use crate::{CairoDeserializer, DecodeResult};

pub trait Transcode<In, Out> {
    fn transcode(&self, input: &mut In, output: &mut Out) -> DecodeResult<()>;
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
