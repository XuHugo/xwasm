use anyhow::{Error};
use std::{
    fs,
    sync::Arc,
};

/// Size of an account address when serialized in binary.
/// NB: This is different from the Base58 representation.
pub const ADDRESS_SIZE: usize = 20;
pub const GAS_SCALE: u64 = 10;
pub const GAS_ENV_FUNC_BASE: u64 = 20*GAS_SCALE;
pub const GAS_INIT_FUNC_BASE: u64 = 200*GAS_SCALE;

#[derive(Default)]
pub struct Address(pub [u8; ADDRESS_SIZE]);

#[derive(Default)]
pub struct Amount (pub xq: u64,)

#[derive(Default)]
pub struct Metadata {
    pub block_time: u64,
    pub block_height: u64,  
    pub tx_hash: String,  
}

#[derive(Debug)]
pub enum ContractResult {
    Success {
        gas:       u64,
        event:     Vec<String>,
        data:      String,
    },
    Reject {
        reason:           i32,
        remaining_energy: u64,
    },
    OutOfEnergy,
}

#[derive(Debug, derive_more::Display)]
pub enum ContractError {
    /// Code could not be read from the state.
    CodeNotFound,
    /// Wasm code failed validation.
    InvalidModule,
    /// Wasm code could not be deserialized.
    CantDeserializeWasm,
    /// The module does not export a linear memory named `memory`.
    InvalidMemory,
    /// The number of heap pages requested is disallowed by the module.
    InvalidHeapPages,
    /// Instantiation error.
    Instantiation(String),
    /// Other error happenend.
    Other(String),
}

impl std::error::Error for ContractError {}


#[derive(Clone, Default)]
pub struct Context {
    pub(crate) func_name: String,
    pub(crate) state: String,
    pub(crate) param: String,
    pub(crate) invoker:  Address,
    pub(crate) owner:   Address,
    pub(crate) self_address: Address,
    pub(crate) event: Vec<String>,
    pub(crate) self_balance: Amount,
    pub(crate) ret_data: String,
    pub(crate) metadata: Metadata,
    pub(crate) gas: bool,
    pub(crate) gas_counter: u64,
    pub(crate) gas_limit: u64,
    pub(crate) gas_outof: bool,
    
}

impl Context {
    pub fn new() -> Self {
        Context{
            default()
        }
    }
    pub fn init(
        func_name: String,
        state: String,
        param: String,
        invoker:  Address,
        owner:   Address,
        self_address: Address,
        self_balance: u64,
        metadata: Metadata,
        gas: bool,
        gas_limit: u64,
    ) -> Self{
        Context{
            func_name,
            state,
            param,
            invoker,
            owner,
            self_address,
            event: Vec::new(),
            self_balance,
            ret_data: String::new(),
            metadata,
            gas,
            gas_counter: 0,
            gas_limit,
            gas_outof: false,
        }
    }
}


pub struct Executor{
    pub context: Context,
    pub db: KVDB,
}
