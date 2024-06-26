use crate::{
    hostfunc::host_func_init,
    types::{Context, WasmError, WasmResult, WasmTransactionOutput},
};
use anyhow::anyhow;
use parallel::task::ModulePath;
use types::transaction::{
    AbortInfo, ExecutionStatus, TransactionOutput as RawTransactionOutput, TransactionStatus,
};
use wasmtime::Val::I64;
use wasmtime::*;

// execute contract
pub fn execute(
    func_name: &str,
    context: Context,
    binary: &[u8],
    amount: u64,
) -> Result<WasmTransactionOutput> {
    let engine = Engine::new(Config::new().epoch_interruption(true))?;
    let module = unsafe { Module::deserialize(&engine, binary)? };
    //let module = Module::from_binary(&engine, binary)?;
    let linker = host_func_init(&engine)?;

    let mut store = Store::new(&engine, context);
    store.set_epoch_deadline(1);
    let instance = linker.instantiate(&mut store, &module)?;

    let engine_clone = engine.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(1));
        engine_clone.increment_epoch();
    });

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
