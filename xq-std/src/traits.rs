use crate::types::Address;

pub trait Context{
    fn owner(&self) -> Address;
    fn invoker(&self) -> Address;
    fn sender(&self) -> Address;

    fn self_address(&self) -> Address;
    fn self_balance(&self) -> u64;
    
    fn paramteter<T:>(&self)-> T where T: serde::de::DeserializeOwned;
    fn store_get(&self, key: String)->String;
    fn store_set(&self, key: String, value:String)->bool;
    fn error(&self, err:String);
    fn event(&self, event:String);
    fn return_data(&self, data:String);
    fn transfer(&self, addr:Address, amount:u64)->bool;
    fn call(&self, addr:Address, amount:u64, arg:String)->bool;

    fn block_time(&self)->u64;
    fn block_number(&self)->u64;
    fn tx_hash(&self)->String;
}


