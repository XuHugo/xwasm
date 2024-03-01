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
