#[cfg(any())]
mod legacy {
    use std::fs;

    use serde::{Deserialize, Serialize};
    use wasm::{
        types::{Address, Context, Metadata, WasmtimeRuntime},
        VM,
    };
    use wasmtime::Config;

    #[test]
    fn test_init_wasm_contract() {
        let modules = fs::read("./tests/wasmfile/contract.wasm").unwrap();
        let wasm_bytes = modules.as_slice();

        let engine = wasmtime::Engine::new(Config::new().epoch_interruption(true)).unwrap();
        let aot_bytes = match engine.precompile_module(wasm_bytes) {
            Ok(b) => b,
            Err(_e) => return,
        };

        let metadata = Metadata {
            block_time: 111,
            block_height: 222,
            tx_hash: String::from("txhash"),
        };

        #[derive(Serialize, Deserialize, Debug)]
        //#[state(contract="xq")]
        struct Param {
            name: String,
            age: u64,
            sex: String,
        }

        let a: Param = Param {
            name: String::from("xq"),
            age: 18,
            sex: String::from("man"),
        };
        let ctx = Context::init(
            String::from("init_xq"),
            String::from(""),
            String::from(serde_json::to_string(&a).unwrap()),
            Address::from("0xf6b02a2d47b84e845b7e3623355f04tbi0000002"),
            Address::from("0xf6b02a2d47b84e845b7e3623355f04tbi0000002"),
            Address::from("0xf6b02a2d47b84e845b7e3623355f04tbi0000002"),
            100,
            metadata,
            false,
            10000,
        );
        let ret = WasmtimeRuntime::execute("init_xq", ctx, &aot_bytes, 0);

        match ret {
            Ok(r) => println!("{:?}", r),
            Err(e) => println!("{:?}", e),
        }
    }

    #[test]
    fn test_call_wasm_contract() {
        let modules = fs::read("./tests/wasmfile/contract.wasm").unwrap();
        let wasm_bytes = modules.as_slice();

        let engine = wasmtime::Engine::new(Config::new().epoch_interruption(true)).unwrap();
        let aot_bytes = match engine.precompile_module(wasm_bytes) {
            Ok(b) => b,
            Err(_e) => return,
        };

        let metadata = Metadata {
            block_time: 111,
            block_height: 222,
            tx_hash: String::from("txhash"),
        };

        #[derive(Serialize, Deserialize, Debug)]
        //#[state(contract="xq")]
        struct Param {
            name: String,
            age: u64,
            sex: String,
        }

        let a: Param = Param {
            name: String::from("xq"),
            age: 10,
            sex: String::from("man"),
        };
        let ctx = Context::init(
            String::from("xq_abc"),
            String::from(""),
            String::from(serde_json::to_string(&a).unwrap()),
            Address::from("0xf6b02a2d47b84e845b7e3623355f04tbi0000002"),
            Address::from("0xf6b02a2d47b84e845b7e3623355f04tbi0000002"),
            Address::from("0xf6b02a2d47b84e845b7e3623355f04tbi0000002"),
            100,
            metadata,
            false,
            10000,
        );
        let ret = WasmtimeRuntime::execute("xq_abc", ctx, &aot_bytes, 0);

        match ret {
            Ok(r) => println!("{:?}", r),
            Err(e) => println!("{:?}", e),
        }
    }
}

use std::fs;

use parallel::state::{StateView, StateViewId};
use types::{
    address::AccountAddress,
    rwset::StateKey,
    transaction::{ExecutionStatus, Transaction, TransactionStatus},
};
use wasm::{
    types::{Metadata, PreprocessedTransaction, WasmtimeRuntime},
    VM,
};
use wasmtime::Config;

struct EmptyStateView;

impl StateView for EmptyStateView {
    fn id(&self) -> StateViewId {
        StateViewId::Miscellaneous
    }

    fn get_state_value(&self, _state_key: &StateKey) -> anyhow::Result<Option<Vec<u8>>> {
        Ok(None)
    }

    fn is_genesis(&self) -> bool {
        false
    }
}

#[test]
fn executes_edsl_init_with_current_block_api() {
    let wasm_path = std::env::var_os("XWASM_CONTRACT_WASM")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| "./tests/wasmfile/contract.wasm".into());
    let wasm_bytes = fs::read(wasm_path).unwrap();
    let engine = wasmtime::Engine::new(Config::new().epoch_interruption(true)).unwrap();
    let aot_bytes = engine.precompile_module(&wasm_bytes).unwrap();
    let sender = AccountAddress::new([1; AccountAddress::LENGTH]);
    let contract = AccountAddress::new([2; AccountAddress::LENGTH]);

    let outputs = WasmtimeRuntime::sequential_execute_transactions(
        vec![PreprocessedTransaction {
            txn: Transaction {
                sender,
                dest: contract,
                contract_name: "xq".to_string(),
                func_name: "init_xq".to_string(),
                gas_limit: 10_000,
                gas: false,
                code: aot_bytes.into(),
                param: serde_json::json!({
                    "name": "genesis",
                    "age": 1,
                    "sex": "man"
                })
                .to_string(),
                amount: 0,
            },
            owner: sender,
            self_address: contract,
            self_balance: 100,
            metadata: Metadata {
                block_time: 111,
                block_height: 222,
                tx_hash: "init-tx".to_string(),
            },
            gas: false,
            gas_limit: 10_000,
        }],
        &EmptyStateView,
    )
    .unwrap();

    assert_eq!(outputs.len(), 1);
    assert_eq!(
        outputs[0].status(),
        TransactionStatus::Keep(ExecutionStatus::Success)
    );
    let output: serde_json::Value = serde_json::from_str(&outputs[0].0.data).unwrap();
    assert_eq!(output["name"], "genesis");
    assert_eq!(output["age"], 1);
}
