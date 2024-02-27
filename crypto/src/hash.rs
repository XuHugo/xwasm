
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct HashValue {
    hash: [u8; HashValue::LENGTH],
}

impl HashValue {
    pub const LENGTH: usize = 32;
}