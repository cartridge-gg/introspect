use crate::AttributeCallType;

pub struct IntrospectManager {
    pub macro_mode: AttributeCallType,
    pub derives: Vec<String>,
}
