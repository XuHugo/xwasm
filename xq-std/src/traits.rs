use crate::types::Address;
use serde::{Deserialize, Serialize};

pub trait InitContext{

    fn owner(&self) -> Address;
    fn invoker(&self) -> Address;
    fn sender(&self) -> Address;
    fn self_address(&self) -> Address;
    fn self_balance(&self) -> u64;
    
    fn go(self)->i32;
    fn paramteter<T:>(self)-> T where T: serde::de::DeserializeOwned;

    fn error_set(self, err_code:i32);
    fn error_get(self);
}

pub trait ExecContext{

    // fn invoker(&self) -> [u8; 8];
    // fn self_address(&self) -> [u8; 8];
    // fn self_balance(&self) -> u64;
    // fn sender(&self) -> [u8; 8];
    // fn owner(&self) -> [u8; 8];

    fn go(self)->i32;
}

