use std::fmt::Display;

use crate::{vm, VM};
use statedb::STATE_DB;

pub const ADDRESS_SIZE: usize = 20;
pub const GAS_SCALE: u64 = 10;
pub const GAS_ENV_FUNC_BASE: u64 = 20 * GAS_SCALE;
pub const GAS_INIT_FUNC_BASE: u64 = 200 * GAS_SCALE;

#[derive(Default, Clone, Copy)]
pub struct Address(pub [u8; ADDRESS_SIZE]);

impl std::ops::Deref for Address {
    type Target = [u8; ADDRESS_SIZE];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for Address {
    fn from(address: String) -> Self {
        let mut addr: [u8; ADDRESS_SIZE] = [0; ADDRESS_SIZE];
        addr.clone_from_slice(&address.as_bytes()[0..ADDRESS_SIZE]);
        Address(addr)
    }
}

impl From<&str> for Address {
    fn from(address: &str) -> Self {
        let mut addr: [u8; ADDRESS_SIZE] = [0; ADDRESS_SIZE];
        addr.clone_from_slice(&address.as_bytes()[0..ADDRESS_SIZE]);
        Address(addr)
    }
}

#[derive(Default)]
pub struct Amount(u64);

#[derive(Default, Clone)]
pub struct Metadata {
    pub block_time: u64,
    pub block_height: u64,
    pub tx_hash: String,
}

impl Metadata {
    pub fn transfer(&self, _address: Address, _amountt: u64) -> Result<(), WasmError> {
        Ok(())
    }

    pub fn call(
        &self,
        _address: Address,
        _amount: u64,
        _func: String,
        _arg: String,
    ) -> Result<(), WasmError> {
        Ok(())
    }

    pub fn set_store(&self, key: &[u8], value: &[u8]) -> Result<(), WasmError> {
        match STATE_DB.clone().lock().put(key, value) {
            Ok(_) => Ok(()),
            Err(_) => return Err(WasmError::HostFuncErr),
        }
    }

    pub fn get_store(&self, key: &[u8]) -> Result<Vec<u8>, WasmError> {
        match STATE_DB.clone().lock().get(key) {
            Ok(v) => Ok(v),
            Err(_) => return Err(WasmError::HostFuncErr),
        }
    }
}

#[derive(Debug)]
pub enum WasmResult {
    Success {
        used_gas: u64,
        event: Vec<String>,
        data: String,
    },
    Reject {
        code: WasmError,
        reason: String,
        used_gas: u64,
    },
}

#[derive(Debug)]
pub enum WasmError {
    CodeNotFound = 1000,
    InvalidModule,
    CantDeserializeWasm,
    InvalidMemory,
    InvalidHeapPages,
    OutOfGas,
    ExecuteFail,
    InvalidReturn,
    Instantiation,
    KeyNotFound,
    HostFuncErr,
}

impl Display for WasmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WasmError::CodeNotFound => write!(f, "{}", Self::CodeNotFound),
            WasmError::InvalidModule => write!(f, "{}", Self::InvalidModule),
            WasmError::CantDeserializeWasm => write!(f, "{}", Self::CantDeserializeWasm),
            WasmError::InvalidMemory => write!(f, "{}", Self::InvalidMemory),
            WasmError::InvalidHeapPages => write!(f, "{}", Self::InvalidHeapPages),
            WasmError::Instantiation => write!(f, "{}", Self::Instantiation),
            WasmError::OutOfGas => write!(f, "{}", Self::OutOfGas),
            WasmError::ExecuteFail => write!(f, "{}", Self::ExecuteFail),
            WasmError::InvalidReturn => write!(f, "{}", Self::InvalidReturn),
            WasmError::KeyNotFound => write!(f, "{}", Self::KeyNotFound),
            WasmError::HostFuncErr => write!(f, "{}", Self::HostFuncErr),
        }
    }
}

impl std::error::Error for WasmError {}

#[allow(dead_code)]
#[derive(Default)]
pub struct Context {
    pub(crate) func_name: String,
    pub(crate) state: String,
    pub(crate) param: String,
    pub(crate) invoker: Address,
    pub(crate) owner: Address,
    pub(crate) self_address: Address,
    pub(crate) event: Vec<String>,
    pub(crate) self_balance: u64,
    pub(crate) output_data: String,
    pub(crate) metadata: Metadata,
    pub(crate) gas: bool,
    pub(crate) gas_counter: u64,
    pub(crate) gas_limit: u64,
    pub(crate) gas_outof: bool,
}

impl Context {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn init(
        func_name: String,
        state: String,
        param: String,
        invoker: Address,
        owner: Address,
        self_address: Address,
        self_balance: u64,
        metadata: Metadata,
        gas: bool,
        gas_limit: u64,
    ) -> Self {
        Context {
            func_name,
            state,
            param,
            invoker,
            owner,
            self_address,
            event: Vec::new(),
            self_balance,
            output_data: String::new(),
            metadata,
            gas,
            gas_counter: 0,
            gas_limit,
            gas_outof: false,
        }
    }
}

// pub struct Executor{
//     pub context: Context,
//     pub db: KVDB,
// }

pub struct WasmtimeRuntime;

impl VM for WasmtimeRuntime {
    fn execute(
        func_name: &str,
        context: Context,
        binary: &[u8],
        amount: u64,
    ) -> anyhow::Result<WasmResult> {
        let ret = vm::execute(func_name, context, binary, amount);
        ret
    }
}
