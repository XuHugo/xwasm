use std::{
    collections::HashMap,
    sync::{Arc, OnceLock, RwLock},
    time::Duration,
};

use crate::{
    hostfunc::host_func_init,
    types::{Context, WasmTransactionOutput},
};
use anyhow::anyhow;
use types::transaction::{
    AbortInfo, ExecutionStatus, TransactionOutput as RawTransactionOutput, TransactionStatus,
};
use wasmtime::Val::I64;
use wasmtime::*;

fn runtime_engine() -> &'static Engine {
    static ENGINE: OnceLock<Engine> = OnceLock::new();

    ENGINE.get_or_init(|| {
        let engine = Engine::new(Config::new().epoch_interruption(true))
            .expect("failed to initialize Wasmtime engine");
        let epoch_engine = engine.clone();
        std::thread::Builder::new()
            .name("xwasm-epoch-ticker".to_string())
            .spawn(move || loop {
                std::thread::sleep(Duration::from_millis(10));
                epoch_engine.increment_epoch();
            })
            .expect("failed to start Wasmtime epoch ticker");
        engine
    })
}

fn cached_module(binary: &Arc<[u8]>) -> Result<Module> {
    static MODULES: OnceLock<RwLock<HashMap<usize, (Arc<[u8]>, Module)>>> = OnceLock::new();

    let engine = runtime_engine();
    let cache_key = Arc::as_ptr(binary) as *const u8 as usize;
    let modules = MODULES.get_or_init(|| RwLock::new(HashMap::new()));

    if let Some(module) = modules
        .read()
        .expect("module cache read lock poisoned")
        .get(&cache_key)
        .filter(|(cached_binary, _)| Arc::ptr_eq(cached_binary, binary))
        .map(|(_, module)| module.clone())
    {
        return Ok(module);
    }

    let module = unsafe { Module::deserialize(engine, binary.as_ref())? };
    let mut cache = modules.write().expect("module cache write lock poisoned");
    if let Some(existing) = cache
        .get(&cache_key)
        .filter(|(cached_binary, _)| Arc::ptr_eq(cached_binary, binary))
        .map(|(_, module)| module.clone())
    {
        return Ok(existing);
    }

    cache.insert(cache_key, (Arc::clone(binary), module.clone()));
    Ok(module)
}
// execute contract
pub fn execute(
    func_name: &str,
    context: Context,
    binary: &Arc<[u8]>,
    amount: u64,
) -> Result<WasmTransactionOutput> {
    let engine = runtime_engine();
    let module = cached_module(binary)?;
    let linker = host_func_init(engine)?;

    let mut store = Store::new(engine, context);
    store.set_epoch_deadline(100);
    let instance = linker.instantiate(&mut store, &module)?;

    let run = match instance.get_func(&mut store, func_name) {
        Some(r) => r,
        None => return Err(anyhow!("can't find {} func from contract!", func_name)),
    };
    let mut r = [wasmtime::Val::null()];
    let _ = match run.call(&mut store, &[I64(amount as i64)], &mut r) {
        Ok(_) => (),
        Err(trap) => {
            println!("call {} error: {:?}", func_name, trap);
            return Err(trap);
        }
    };
    let used_gas = store.data().gas_limit - store.data().gas_counter;
    let ret = Some(r[0].clone());

    //return Ok(WasmError::ExecuteFail);
    if let Some(Val::I32(n)) = ret {
        if store.data().gas_outof {
            return Ok(WasmTransactionOutput(RawTransactionOutput {
                data: "".to_string(),
                write_set: Default::default(),
                events: vec![],
                gas_used: used_gas,
                status: TransactionStatus::Keep(ExecutionStatus::OutOfGas),
            }));
        };

        if n == 1 {
            Ok(WasmTransactionOutput(RawTransactionOutput {
                data: store.data().output_data.clone(),
                write_set: store.data().writesets.clone(),
                events: store.data().event.clone(),
                gas_used: used_gas,
                status: TransactionStatus::Keep(ExecutionStatus::Success),
            }))
        } else {
            Ok(WasmTransactionOutput(RawTransactionOutput {
                data: "".to_string(),
                write_set: Default::default(),
                events: vec![],
                gas_used: used_gas,
                status: TransactionStatus::Keep(ExecutionStatus::MoveAbort {
                    location: 0,
                    code: 0,
                    info: Some(AbortInfo {
                        reason_name: "Wasm execution fail".to_string(),
                        description: "Wasm execution fail".to_string(),
                    }),
                }),
            }))
        }
    } else {
        Ok(WasmTransactionOutput(RawTransactionOutput {
            data: "".to_string(),
            write_set: Default::default(),
            events: vec![],
            gas_used: used_gas,
            status: TransactionStatus::Keep(ExecutionStatus::MoveAbort {
                location: 0,
                code: 0,
                info: Some(AbortInfo {
                    reason_name: "Wasm return error".to_string(),
                    description: "Wasm return error".to_string(),
                }),
            }),
        }))
    }
}
