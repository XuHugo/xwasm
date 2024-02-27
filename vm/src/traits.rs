
use crate::types::{Address, ContractResult, VMResult, Context};
use anyhow::Result;

pub trait HostFunc{
    fn get_func_name(&self) -> String;
    fn get_amount(&self) -> u64;
    fn get_code(&self) -> Vec<u8>;
    fn get_invoker(&mut self)->Address;
    fn get_owner(&mut self)->Address;

    fn get_contract_address(&mut self)->Address;
    fn get_contract_balance(&mut self) -> u64;

    fn get_parameter(&mut self) -> Option<String>;
    fn transfer(&mut self, address: Address, amount: u64) -> Result<()>;
    fn call(
        &mut self,
        address:Address,
        amount: u64,
        func_name: String,
        parameter: String,
    ) -> Result<()>;

    fn get_store(&self, key: Vec<u8>) -> Result<Vec<u8>>;
    fn set_store(&self, key: Vec<u8>, value: Vec<u8>) -> Result<()>;

    fn get_block_time(&mut self) -> u64;
    fn get_block_hash(&mut self) -> String;
    fn get_block_height(&mut self) -> u64;

    fn set_event(&self, event: Vec<u8>) -> Result<()>;
    fn set_return_data(&self, data: Vec<u8>) -> Result<()>;
    fn get_event(&self) -> Result<Vec<u8>>;
    fn get_return_data(&self) -> Result<Vec<u8>>;
}

pub trait VM{
    fn init(ctx:Context) -> VMResult<ContractResult>;
    fn call(ctx:Context) -> VMResult<ContractResult>;
}