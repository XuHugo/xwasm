use crate::types::Address;
//use serde::{Deserialize, Serialize};

pub trait InitContext{

    fn owner(&self) -> Address;
    fn invoker(&self) -> Address;
    fn sender(&self) -> Address;
    fn self_address(&self) -> Address;
    fn self_balance(&self) -> u64;
    
    fn go(self)->i32;
    fn paramteter<T:>(self)-> T where T: serde::de::DeserializeOwned;

    fn state_get()->String;
    fn state_set(state:String);

    fn error_set(err:String);
    fn error_get()->String;

    fn return_data_set(data:String);
    fn return_data_get()->String;

    fn store_get(key: String)->String;
    fn store_set(key: String, value:String)->bool;
}

pub trait ExecContext{

    // fn invoker(&self) -> [u8; 8];
    // fn self_address(&self) -> [u8; 8];
    // fn self_balance(&self) -> u64;
    // fn sender(&self) -> [u8; 8];
    // fn owner(&self) -> [u8; 8];

    fn go(self)->i32;
}

