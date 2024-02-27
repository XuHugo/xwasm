use std::string::String;
use crate::constants::ADDRESS_NUM_LEN;


pub struct  Address(pub [u8; ADDRESS_NUM_LEN]);

impl From<String> for Address{
    fn from(address: String) -> Self{
        let mut addr:[u8; ADDRESS_NUM_LEN] = [0; ADDRESS_NUM_LEN];
        addr.clone_from_slice(&address.as_bytes()[0..ADDRESS_NUM_LEN]);
        Address(addr)
    }
}



#[derive(Clone, Copy)]
pub struct ContractContext;


pub struct ContractMeta;


