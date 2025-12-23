use introspect_macros::attribute::IAttribute;

pub enum KeyType {
    None,
    Primary(Key),
    Custom(Vec<Key>),
}

pub struct Key {
    pub name: String,
    pub c_type: String,
}

pub struct Column {
    pub id: String,
    pub attributes: Vec<IAttribute>,
    pub name: String,
    pub c_type: String,
}

pub struct Primary {
    pub name: String,
    pub c_type: String,
}

pub struct Table {
    pub name: String,
    pub key: KeyType,
    pub columns: Vec<Column>,
    pub attributes: Vec<IAttribute>,
    pub table_impl: String,
    pub columns_mod: String,
    pub column_enum: String,
    pub meta_impl: String,
    pub primary_impl: String,
    pub columns_impl: String,
}

pub struct Member {
    pub name: String,
    pub c_type: String,
    pub key: bool,
}

impl Member {
    pub fn is_primary(&self) -> bool {
        matches!(
            self.c_type.as_str(),
            "felt252"
                | "bool"
                | "u8"
                | "u16"
                | "u32"
                | "u64"
                | "u128"
                | "i8"
                | "i16"
                | "i32"
                | "i64"
                | "i128"
                | "bytes31"
                | "ClassHash"
                | "ContractAddress"
                | "EthAddress"
                | "StorageAddress"
                | "StorageBaseAddress"
        )
    }
    pub fn to_key(&self) -> Key {
        Key {
            name: self.name.clone(),
            c_type: self.c_type.clone(),
        }
    }
}

fn get_key_type(members: &[Member]) -> KeyType {
    let keys: Vec<&Member> = members.into_iter().filter(|m| m.key).collect();
    match keys.len() {
        0 => KeyType::None,
        1 => {
            if keys[0].is_primary() {
                KeyType::Primary(keys[0].to_key())
            } else {
                KeyType::Custom(vec![keys[0].to_key()])
            }
        }
        _ => KeyType::Custom(keys.into_iter().map(Member::to_key).collect()),
    }
}
