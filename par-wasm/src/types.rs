use std::fmt::Display;

use crate::{
    vm::{self, execute},
    VM,
};
use parallel::{
    executor::{BlockExecutor, MVHashMapView, RAYON_EXEC_POOL},
    state::StateView,
    task::{ExecutionStatus, ExecutorTask, ModulePath, Transaction, TransactionOutput},
    txn_last_input_output::ReadDescriptor,
};
use statedb::STATE_DB;
use types::{
    address::AccountAddress,
    rwset::{StateKey, WriteOp, WriteSet},
    transaction::{
        Transaction as RawTransaction, TransactionOutput as RawTransactionOutput, TransactionStatus,
    },
};

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
//#[derive(Default)]
pub struct Context<'a> {
    pub(crate) func_name: &'a String,
    pub(crate) param: &'a String,
    pub(crate) invoker: AccountAddress,
    pub(crate) owner: AccountAddress,
    pub(crate) self_address: AccountAddress,
    pub(crate) self_balance: u64,
    pub(crate) output_data: String,
    pub(crate) event: Vec<String>,
    pub(crate) metadata: &'a Metadata,
    //gas
    pub(crate) gas: bool,
    pub(crate) gas_counter: u64,
    pub(crate) gas_limit: u64,
    pub(crate) gas_outof: bool,
    //parallel
    pub(crate) writesets: WriteSet,
    pub(crate) mvhashview: &'a MVHashMapView<'a, StateKey, WriteOp>,
}

impl<'a> Context<'a> {
    // pub fn new() -> Self {
    //     Default::default()
    // }
    pub fn init(
        func_name: &'a String,
        param: &'a String,
        invoker: AccountAddress,
        owner: AccountAddress,
        self_address: AccountAddress,
        self_balance: u64,
        metadata: &'a Metadata,
        gas: bool,
        gas_limit: u64,
        mvhashview: &'a MVHashMapView<StateKey, WriteOp>,
    ) -> Self {
        Context {
            func_name,
            param,
            invoker,
            owner,
            self_address,
            self_balance,
            output_data: String::new(),
            event: Vec::new(),
            metadata,

            gas,
            gas_counter: 0,
            gas_limit,
            gas_outof: false,

            writesets: Default::default(),
            mvhashview,
        }
    }
}

pub struct PreprocessedTransaction {
    pub txn: RawTransaction,

    pub owner: AccountAddress,
    pub self_address: AccountAddress,
    pub self_balance: u64,
    pub metadata: Metadata,
    pub gas: bool,
    pub gas_limit: u64,
}

impl Transaction for PreprocessedTransaction {
    type Key = StateKey;
    type Value = WriteOp;
}

#[derive(Clone, Debug)]
pub struct WasmTransactionOutput(pub RawTransactionOutput);

impl WasmTransactionOutput {
    pub fn status(&self) -> TransactionStatus {
        self.0.status.clone()
    }

    pub fn write_set(&self) -> WriteSet {
        self.0.write_set.clone()
    }
}

impl TransactionOutput for WasmTransactionOutput {
    type T = PreprocessedTransaction;

    fn get_writes(&self) -> Vec<(StateKey, WriteOp)> {
        self.0
            .write_set
            .iter()
            .map(|(key, op)| (key.clone(), op.clone()))
            .collect()
    }

    fn get_deltas(&self) -> Vec<(StateKey, u8)> {
        todo!()
    }

    /// Execution output for transactions that comes after SkipRest signal.
    fn skip_output() -> Self {
        Self(RawTransactionOutput {
            data: String::new(),
            write_set: WriteSet::default(),
            events: vec![],
            gas_used: 0,
            status: TransactionStatus::Retry,
        })
    }
}

pub(crate) struct WasmExecutorTask<'a, S> {
    _view: &'a S,
}

impl<'a, S: 'a + StateView> ExecutorTask for WasmExecutorTask<'a, S> {
    type T = PreprocessedTransaction;
    type Output = WasmTransactionOutput;
    type Error = i64;
    type Argument = &'a S;

    fn init(argument: &'a S) -> Self {
        WasmExecutorTask { _view: argument }
    }

    fn execute_transaction(
        &self,
        view: &MVHashMapView<StateKey, WriteOp>,
        txn: &PreprocessedTransaction,
    ) -> ExecutionStatus<Self::Output, Self::Error> {
        let context: Context = Context::init(
            &txn.txn.func_name,
            &txn.txn.param,
            txn.txn.sender,
            txn.owner,
            txn.self_address,
            txn.self_balance,
            &txn.metadata,
            txn.gas,
            txn.gas_limit,
            view,
        );
        // let func_name = match txn.kind {
        //     ExecKind::Init => format!("init_{}", &txn.contract_name),
        //     ExecKind::Call => format!("{}.{}", &txn.contract_name, &txn.func_name),
        // };

        match execute(&txn.txn.func_name, context, &txn.txn.code, txn.txn.amount) {
            Ok(output) => ExecutionStatus::Success(output),
            Err(_e) => ExecutionStatus::Abort(0),
        }
    }

    fn execute_transaction2(
        &self,
        view: &std::collections::BTreeMap<
            <Self::T as Transaction>::Key,
            <Self::T as Transaction>::Value,
        >,
        txn: &Self::T,
        txn_idx: usize,
    ) -> parallel::task::ExecutionStatus<Self::Output, Self::Error> {
        todo!()
    }
}

pub struct WasmtimeRuntime;

impl VM for WasmtimeRuntime {
    fn execute_transaction(
        func_name: &str,
        context: Context,
        binary: &[u8],
        amount: u64,
    ) -> anyhow::Result<WasmResult> {
        //let ret = vm::execute(func_name, context, binary, amount);
        //ret
        todo!()
    }
    fn parallel_execute_transactions<S: StateView>(
        transactions: Vec<PreprocessedTransaction>,
        state_view: &S,
        concurrency_level: usize,
    ) -> Result<Vec<WasmTransactionOutput>, i64> {
        let executor =
            BlockExecutor::<PreprocessedTransaction, WasmExecutorTask<S>>::new(concurrency_level);

        let mut ret = executor
            .execute_transactions_parallel(state_view, &transactions)
            .map(|(results)| results.0);

        // Explicit async drop. Happens here because we can't currently move to
        // BlockExecutor due to the Module publishing fallback. TODO: fix after
        // module publishing fallback is removed.
        RAYON_EXEC_POOL.spawn(move || {
            // Explicit async drops.
            drop(transactions);
        });

        // match ret {
        //     Ok(outputs) => Ok(outputs.into_iter().map(|op| op.0.output).collect()),
        //     Err(err) => Err(err),
        // }
        Err(0)
    }
}
