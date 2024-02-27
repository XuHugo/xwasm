
use serde::{Deserialize, Serialize};


#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountAddress([u8; AccountAddress::LENGTH]);

impl AccountAddress {
    pub const LENGTH: usize = 20;

}