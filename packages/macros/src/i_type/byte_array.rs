use cairo_syntax_parser::CairoWriteSlice;
use std::fmt::{Result as FmtResult, Write};

pub trait CWriteIBytes {
    fn to_serialized_bytes<W: Write>(&self, buf: &mut W) -> FmtResult;
    fn to_const_byte_array<W: Write>(&self, buf: &mut W, name: &str) -> FmtResult;
}

pub fn bytes_to_byte_array_felts(bytes: &[u8]) -> (&[[u8; 31]], &[u8]) {
    let (full, partial) = bytes.as_chunks::<31>();
    if partial.is_empty() {
        let last = full.len() - 1;
        (&full[..last], &full[last])
    } else {
        (full, partial)
    }
}

pub fn string_to_felts(s: &str) -> (&[[u8; 31]], &[u8]) {
    bytes_to_byte_array_felts(s.as_bytes())
}

impl CWriteIBytes for String {
    fn to_serialized_bytes<W: Write>(&self, buf: &mut W) -> FmtResult {
        let (felts, final_felt) = string_to_felts(self);
        felts.cwrite_terminated(buf, ',')?;
        write_terminal_byte31(buf, final_felt)
    }
    fn to_const_byte_array<W: Write>(&self, buf: &mut W, name: &str) -> FmtResult {
        let size = self.len().div_ceil(31);
        write!(buf, "const {name}: [felt252; {size}] = [")?;
        self.to_serialized_bytes(buf)?;
        buf.write_str("];\n")
    }
}

pub fn write_terminal_byte31<W: Write>(buf: &mut W, bytes: &[u8]) -> FmtResult {
    if bytes.len() == 31 {
        write!(buf, "0x02")?;
    } else {
        write!(buf, "0x03{:02x}", bytes.len())?;
        (0..30 - bytes.len())
            .map(|_| buf.write_str("00"))
            .collect::<FmtResult>()?;
    }
    bytes
        .iter()
        .map(|b| write!(buf, "{b:02x}"))
        .collect::<FmtResult>()
}

pub fn byte_array_felt_len(string: &str) -> u32 {
    (string.len() as u32).div_ceil(31)
}
