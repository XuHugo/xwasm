
use wasmtime::{
    Engine, Module, Store, Val::{I64, I32}, Config
};
use anyhow::{bail, anyhow};
use crate::{types::{VMResult, ContractResult, Context, ContractError}, traits::HostFunc};
use crate::hostfunc::host_func_init;

// init contract
pub fn init_wasm(context: Context) -> VMResult<ContractResult> {

    let engine = Engine::new(Config::new().interruptable(true))
        .map_err(|e|ContractError::Internal(e.to_string()))?;
    let module = Module::from_binary(&engine, &context.get_code())
        .map_err(|e|ContractError::Internal(e.to_string()))?;
    let linker = host_func_init(&engine)?;

    let mut store = Store::new(&engine, context.clone() );
    let interrupt_handle = store.interrupt_handle()
        .map_err(|e|ContractError::Internal(e.to_string()))?;
    let instance = linker.instantiate(&mut store, &module)
        .map_err(|e|ContractError::Internal(e.to_string()))?;

    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(1));
        interrupt_handle.interrupt();
    });

    let run = match instance.get_func(&mut store, &context.get_func_name()){
        Some(r) => r,
        None => return Err(ContractError::InvalidCode(100))
    };

    let ret = match run.call(&mut store, &[I64(context.get_amount() as i64)]){
        Ok(r) => {
            Some(r[0].clone())
        },
        Err(trap) => {
            return Err(ContractError::Internal(trap.to_string()))
        },
    };

    if let Some(I32(n)) = ret {
        if n == 0 {
            Ok(ContractResult {
                event: store.data().get_event().map_err(|e|ContractError::Internal(e.to_string()))?,
                data: store.data().get_return_data().map_err(|e|ContractError::Internal(e.to_string()))?,
            })
        } else {
            Err(ContractError::InvalidCode(n))
        }
    } else {
        Err(ContractError::InvalidCode(100))
    }
}

// call contract
pub fn call_wasm(context: Context) -> VMResult<ContractResult> {

    let engine = Engine::new(Config::new().interruptable(true))
        .map_err(|e|ContractError::Internal(e.to_string()))?;
    let module = Module::from_binary(&engine, &context.get_code())
        .map_err(|e|ContractError::Internal(e.to_string()))?;
    let linker = host_func_init(&engine)?;

    let mut store = Store::new(&engine, context.clone() );
    let interrupt_handle = store.interrupt_handle()
        .map_err(|e|ContractError::Internal(e.to_string()))?;
    let instance = linker.instantiate(&mut store, &module)
        .map_err(|e|ContractError::Internal(e.to_string()))?;

    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(1));
        interrupt_handle.interrupt();
    });

    let run = match instance.get_func(&mut store, &context.get_func_name())
    {
        Some(r) => r,
        None => return Err(ContractError::InvalidCode(100))
    };

    let ret = match run.call(&mut store, &[I64(context.get_amount() as i64)]){
        Ok(r) => {
            Some(r[0].clone())
        },
        Err(trap) => {
            return Err(ContractError::Internal(trap.to_string()))
        },
    };

    if let Some(I32(n)) = ret {
        if n == 0 {
            Ok(ContractResult {
                event: store.data().get_event().map_err(|e|ContractError::Internal(e.to_string()))?,
                data: store.data().get_return_data().map_err(|e|ContractError::Internal(e.to_string()))?,
            })
        } else {
            Err(ContractError::InvalidCode(n))
        }
    } else {
        Err(ContractError::InvalidCode(100))
    }
}