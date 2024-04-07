use crate::{address::AccountAddress, rwset::WriteSet};
use anyhow::{ensure, format_err, Error, Result};
use serde::{Deserialize, Serialize};

pub type Version = u64;

#[derive(Clone)]
pub struct Transaction {
    pub sender: AccountAddress,
    pub dest: AccountAddress,
    pub contract_name: String,
    pub func_name: String,
    pub gas_limit: u64,
    pub gas: bool,
    pub code: Vec<u8>,
    pub param: String,
    pub amount: u64,
}

#[derive(Clone, Debug)]
pub struct TransactionOutput {
    pub data: String,
    pub write_set: WriteSet,
    pub events: Vec<String>,
    pub gas_used: u64,
    pub status: TransactionStatus,
}

impl TransactionOutput {
    pub fn status(&self) -> TransactionStatus {
        self.status.clone()
    }

    pub fn write_set(&self) -> WriteSet {
        self.write_set.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TransactionStatus {
    /// Discard the transaction output
    Discard(i64),

    /// Keep the transaction output
    Keep(ExecutionStatus),

    /// Retry the transaction, e.g., after a reconfiguration
    Retry,
}

impl TransactionStatus {
    pub fn status(&self) -> Result<ExecutionStatus, i64> {
        match self {
            TransactionStatus::Keep(status) => Ok(status.clone()),
            TransactionStatus::Discard(code) => Err(*code),
            TransactionStatus::Retry => Err(0),
        }
    }

    pub fn is_discarded(&self) -> bool {
        match self {
            TransactionStatus::Discard(_) => true,
            TransactionStatus::Keep(_) => false,
            TransactionStatus::Retry => true,
        }
    }

    pub fn as_kept_status(&self) -> Result<ExecutionStatus> {
        match self {
            TransactionStatus::Keep(s) => Ok(s.clone()),
            _ => Err(format_err!("Not Keep.")),
        }
    }
}

// impl From<VMStatus> for TransactionStatus {
//     fn from(vm_status: VMStatus) -> Self {
//         let status_code = vm_status.status_code();
//         match vm_status.keep_or_discard() {
//             Ok(recorded) => match recorded {
//                 KeptVMStatus::MiscellaneousError => {
//                     TransactionStatus::Keep(ExecutionStatus::MiscellaneousError(Some(status_code)))
//                 }
//                 _ => TransactionStatus::Keep(recorded.into()),
//             },
//             Err(code) => TransactionStatus::Discard(code),
//         }
//     }
// }

impl From<ExecutionStatus> for TransactionStatus {
    fn from(txn_execution_status: ExecutionStatus) -> Self {
        TransactionStatus::Keep(txn_execution_status)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ExecutionStatus {
    Success,
    OutOfGas,
    MoveAbort {
        location: u64,
        code: u64,
        info: Option<AbortInfo>,
    },
    ExecutionFailure {
        location: u64,
        function: u16,
        code_offset: u16,
    },
    MiscellaneousError(Option<i64>),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct AbortInfo {
    pub reason_name: String,
    pub description: String,
}
