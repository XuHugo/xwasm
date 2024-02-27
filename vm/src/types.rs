

pub type Address = [u8;32];

//context
#[derive(Clone)]
pub struct Context {
    pub func_name: String,
    pub parameter: Option<String>,
    pub amount: u64,
    pub code:Vec<u8>,

    pub returndata: String,
    pub event: Vec<String>,

    pub invoker:  Address,
    pub owner:  Address,
    pub contract_balance: u64,
    pub contract_address: Address,

    pub metadata: Metadata,
}

#[derive(Clone)]
pub struct Metadata{
    pub block_time: u64,
    pub block_height: u64,
    pub block_hash: String,
}


//Result && error 
#[derive(Debug)]
pub struct ContractResult {
    pub  data:Vec<u8>,
    pub event:Vec<u8>,
}

#[derive(Debug)]
pub enum ContractError {
    OutOfGas,
    
    InvalidCode(i32),
    
    Internal(String),
    
    Wasm(String),
    
    Reverted,
}

pub type VMResult<T> = Result<T, ContractError>;

//VM
pub struct Wasm_vm{

}