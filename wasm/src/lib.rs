pub mod wasmtime_vm;
pub mod types;

use crate::types::{Context, WasmResult};

pub trait VM{
    fn execute(func_name: &str, context: Context, binary: &[u8], amount: u64) -> anyhow::Result<WasmResult>;
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
