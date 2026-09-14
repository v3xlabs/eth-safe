use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(from = "u8")]
pub enum Operation {
    Call,
    DelegateCall,
    Unknown(u8),
}

impl From<u8> for Operation {
    fn from(raw: u8) -> Self {
        match raw {
            0 => Self::Call,
            1 => Self::DelegateCall,
            _ => Self::Unknown(raw),
        }
    }
}
