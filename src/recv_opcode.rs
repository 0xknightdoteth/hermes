#[derive(Debug, PartialEq)]
pub enum RecvOpcode {
    Ping,
}

impl RecvOpcode {
    pub fn from_u16(value: u16) -> Option<RecvOpcode> {
        match value {
            1 => Some(RecvOpcode::Ping),
            _ => None,
        }
    }
}
