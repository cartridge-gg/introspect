pub trait ConstData {
    const SIZE: u32;
    const DATA: [felt252; Self::SIZE];
    fn serialize_bytes(ref self: Array<felt252>) {
        self.append_span(Self::DATA.span())
    }
}
