#[derive(Drop, Serde, PartialEq, Debug)]
pub struct Attribute {
    pub name: ByteArray,
    pub data: Option<ByteArray>,
}
