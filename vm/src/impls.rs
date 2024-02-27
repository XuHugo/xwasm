
use crate::types::{Address, Context, Wasm_vm, VMResult, ContractResult};
use crate::traits::{HostFunc, VM};
use crate::wasm::{init_wasm, call_wasm};
use anyhow::Result;

impl HostFunc for Context{
    fn get_func_name(&self) -> String{
        self.func_name.clone()
    }

    fn get_amount(&self) -> u64{
        self.amount
    }

    fn get_code(&self) -> Vec<u8>{
        self.code.clone()
    }

    fn get_invoker(&mut self)->Address{
        self.invoker
    }
    fn get_owner(&mut self)->Address{
        self.owner
    }

    fn get_contract_address(&mut self)->Address{
        self.contract_address
    }
    fn get_contract_balance(&mut self) -> u64{
        self.contract_balance
    }

    fn get_parameter(&mut self) -> Option<String>{
        self.parameter.clone()
    }
    fn transfer(&mut self, address: Address, amount: u64) -> Result<()>{
        Ok(())
    }
    fn call(
        &mut self,
        address: Address,
        amount: u64,
        func_name: String,
        parameter: String,
    ) -> Result<()>{
        Ok(())
    }

    fn get_store(&self, key: Vec<u8>) -> Result<Vec<u8>>{
        Ok(vec![1,2,3])
    }
    fn set_store(&self, key: Vec<u8>, value: Vec<u8>) -> Result<()>{
        Ok(())
    }

    fn get_block_time(&mut self) -> u64{
        self.metadata.block_time
    }
    fn get_block_hash(&mut self) -> String{
        self.metadata.block_hash.clone()
    }
    fn get_block_height(&mut self) -> u64{
        self.metadata.block_height
    }

    fn set_event(&self, event: Vec<u8>) -> Result<()>{
        Ok(())
    }
    fn set_return_data(&self, data: Vec<u8>) -> Result<()>{
        Ok(())
    }
    fn get_event(&self) -> Result<Vec<u8>>{
        Ok(Vec::new())
    }
    fn get_return_data(&self) -> Result<Vec<u8>>{
        Ok(Vec::new())
    }
}

impl VM for Wasm_vm{
    fn init(ctx:Context) -> VMResult<ContractResult> {
        
        init_wasm(ctx)
    }

    fn call(ctx:Context) -> VMResult<ContractResult> {
        call_wasm(ctx)
    }
}