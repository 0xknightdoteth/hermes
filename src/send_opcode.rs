#[derive(Debug, PartialEq)]
pub enum SendOpcode {
    Pong,
}

impl SendOpcode {
    pub fn to_u16(&self) -> u16 {
        match self {
            SendOpcode::Pong => 1,
        }
    }
}
