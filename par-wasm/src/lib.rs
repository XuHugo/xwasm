mod hostfunc;
pub mod types;
pub mod vm;
use crate::types::{Context, WasmResult};

pub trait VM {
    fn execute_transaction(
        func_name: &str,
        context: Context,
        binary: &[u8],
        amount: u64,
    ) -> anyhow::Result<WasmResult>;

    pub fn parallel_execute_transactions<S: StateView>(
        transactions: Vec<PreprocessedTransaction>,
        state_view: &S,
        concurrency_level: usize,
    ) -> Result<Vec<GeeTransactionOutput>, VMStatus> {
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
