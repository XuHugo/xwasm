use crate::types::{Address, Context, ADDRESS_SIZE, GAS_ENV_FUNC_BASE};
use anyhow::Result;
use parallel::task::ModulePath;
use types::{
    address::AccountAddress,
    rwset::{AccessPath, StateKey, WriteOp},
};
use wasmtime::{Caller, Engine, Extern, Linker};

// env function for wasm
pub fn host_func_init(engine: &Engine) -> Result<Linker<Context>> {
    let mut linker = Linker::new(&engine);

    linker.func_wrap(
        "xq",
        "get_owner",
        |mut caller: Caller<'_, Context>, ptr: i32| {
            charge_gas(&mut caller, GAS_ENV_FUNC_BASE);
            //let owner:&[u8] = caller.data_mut().owner.deref();
            let addr = caller.data_mut().owner; //
            let owner = &addr.0;

            let mem = match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => mem,
                _ => anyhow::bail!("get_owner:failed to find memory"),
            };
            match mem.write(caller, ptr as usize, owner) {
                Ok(_) => (),
                Err(_) => anyhow::bail!("get_owner:MemoryAccessError"),
            };
            Ok(())
        },
    )?;

    linker.func_wrap(
        "xq",
        "get_invoker",
        |mut caller: Caller<'_, Context>, ptr: i32| {
            charge_gas(&mut caller, GAS_ENV_FUNC_BASE);
            let invoker = caller.data_mut().invoker;

            let mem = match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => mem,
                _ => anyhow::bail!("get_invoker:failed to find memory"),
            };
            match mem.write(caller, ptr as usize, &invoker.0) {
                Ok(_) => (),
                Err(_) => anyhow::bail!("get_invoker:MemoryAccessError"),
            };
            Ok(())
        },
    )?;

    linker.func_wrap(
        "xq",
        "get_contract_address",
        |mut caller: Caller<'_, Context>, ptr: i32| {
            charge_gas(&mut caller, GAS_ENV_FUNC_BASE);
            let contract_addr = caller.data_mut().self_address;

            let mem = match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => mem,
                _ => anyhow::bail!("get_contract_address:failed to find memory"),
            };
            match mem.write(caller, ptr as usize, &contract_addr.0) {
                Ok(_) => (),
                Err(_) => anyhow::bail!("get_contract_address:MemoryAccessError"),
            };
            Ok(())
        },
    )?;

    linker.func_wrap(
        "xq",
        "get_contract_balance",
        |mut caller: Caller<'_, Context>| -> i64 {
            charge_gas(&mut caller, GAS_ENV_FUNC_BASE);
            let t = caller.data_mut().self_balance;
            t as i64
        },
    )?;

    linker.func_wrap(
        "xq",
        "get_parameter",
        |mut caller: Caller<'_, Context>, ptr: i32| -> i32 {
            charge_gas(&mut caller, GAS_ENV_FUNC_BASE);

            if caller.data().param.len() > 4096 {
                //MAX_PREALLOCATED_CAPACITY
                return -103i32;
            }
            //let param:&[u8] = &caller.data_mut().param.as_bytes();
            let args = caller.data_mut().param.clone();
            let param: &[u8] = &args.as_bytes();

            let mem = match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => mem,
                _ => return -103i32,
            };

            match mem.write(caller, ptr as usize, param) {
                Ok(_) => (),
                Err(_e) => return -103i32,
            };
            param.len() as i32
        },
    )?;

    linker.func_wrap(
        "xq",
        "get_block_time",
        |mut caller: Caller<'_, Context>| -> u64 {
            charge_gas(&mut caller, GAS_ENV_FUNC_BASE);
            caller.data().metadata.block_time
        },
    )?;

    linker.func_wrap(
        "xq",
        "get_block_height",
        |mut caller: Caller<'_, Context>| -> u64 {
            charge_gas(&mut caller, GAS_ENV_FUNC_BASE);
            caller.data_mut().metadata.block_height
        },
    )?;

    linker.func_wrap(
        "xq",
        "get_block_hash",
        |mut caller: Caller<'_, Context>, ptr: i32| -> i32 {
            charge_gas(&mut caller, GAS_ENV_FUNC_BASE);
            let tx: String = caller.data_mut().metadata.tx_hash.clone();
            let mem = match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => mem,
                _ => return -106i32,
            };
            match mem.write(caller, ptr as usize, &tx.as_bytes()) {
                Ok(_) => (),
                Err(_e) => return -103i32,
            };
            tx.as_bytes().len() as i32
        },
    )?;

    linker.func_wrap(
        "xq",
        "set_event",
        |mut caller: Caller<'_, Context>, start: i32, length: i32| -> i32 {
            charge_gas(&mut caller, GAS_ENV_FUNC_BASE);
            if length <= 512 {
                //MAX_LOG_SIZE

                let mem = match caller.get_export("memory") {
                    Some(Extern::Memory(mem)) => mem,
                    _ => return -100i32,
                };
                let data = mem
                    .data(&caller)
                    .get(start as u32 as usize..)
                    .and_then(|arr| arr.get(..length as u32 as usize));
                let event = match data {
                    Some(e) => match String::from_utf8(e.to_vec()) {
                        Ok(o) => o,
                        Err(_) => return -102i32,
                    },
                    None => return -101i32,
                };

                caller.data_mut().event.push(event);
                length
            } else {
                -102i32
            }
        },
    )?;

    linker.func_wrap(
        "xq",
        "set_return_data",
        |mut caller: Caller<'_, Context>, start: i32, len: i32| -> i32 {
            charge_gas(&mut caller, GAS_ENV_FUNC_BASE);

            let mem = match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => mem,
                _ => return -104i32,
            };
            let data = mem
                .data(&caller)
                .get(start as u32 as usize..)
                .and_then(|arr| arr.get(..len as u32 as usize));
            let output = match data {
                Some(e) => match String::from_utf8(e.to_vec()) {
                    Ok(o) => o,
                    Err(_) => return -102i32,
                },
                None => return -105i32,
            };

            caller.data_mut().output_data = output;
            len as i32
        },
    )?;

    linker.func_wrap(
        "xq",
        "set_store",
        |mut caller: Caller<'_, Context>, kstart: i32, klen: u32, vstart: i32, vlen: u32| -> i32 {
            charge_gas(&mut caller, GAS_ENV_FUNC_BASE);
            let mem = match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => mem,
                _ => return -106i32,
            };
            let key = match mem
                .data(&caller)
                .get(kstart as u32 as usize..)
                .and_then(|arr| arr.get(..klen as usize))
            {
                Some(s) => s.to_vec(),
                None => return -107i32,
            };

            let value = match mem
                .data(&caller)
                .get(vstart as u32 as usize..)
                .and_then(|arr| arr.get(..vlen as usize))
            {
                Some(s) => s.to_vec(),
                None => return -107i32,
            };

            let address = String::from(&caller.data().self_address);
            let addr = match AccountAddress::from_hex_literal(&address) {
                Ok(addr) => addr,
                Err(e) => return -107i32,
            };

            let in_key = StateKey::AccessPath(AccessPath::new(addr, key));
            let op = WriteOp::Modification(value);
            let _wr = caller.data_mut().writesets.insert((in_key, op));

            // match caller.data().metadata.set_store(key, value) {
            //     Ok(_) => (),
            //     Err(_e) => return -108i32,
            // };
            0
        },
    )?;

    linker.func_wrap(
        "xq",
        "get_store",
        |mut caller: Caller<'_, Context>, start: i32, length: u32, vstart: i32| -> i32 {
            charge_gas(&mut caller, GAS_ENV_FUNC_BASE);

            let mem = match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => mem,
                _ => return -106i32,
            };
            let key = match mem
                .data(&caller)
                .get(start as u32 as usize..)
                .and_then(|arr| arr.get(..length as usize))
            {
                Some(s) => s.to_vec(),
                None => return -107i32,
            };

            let address = String::from(&caller.data().self_address);
            let addr = match AccountAddress::from_hex_literal(&address) {
                Ok(addr) => addr,
                Err(e) => return -107i32,
            };

            let in_key = StateKey::AccessPath(AccessPath::new(addr, key.clone()));
            let value = match caller.data().mvhashview.read(&in_key) {
                parallel::executor::ReadResult::Value(v) => {
                    let c = match v.as_ref() {
                        WriteOp::Creation(value) => value.clone(),
                        WriteOp::Modification(value) => value.clone(),
                        WriteOp::Deletion => return -108i32,
                    };
                    c
                }
                parallel::executor::ReadResult::U128(_) => return -108i32,
                parallel::executor::ReadResult::Unresolved(_) => return -108i32,
                parallel::executor::ReadResult::None => {
                    //todo: push to captured_reads
                    let value: Vec<u8> = match caller.data().metadata.get_store(&key) {
                        Ok(k) => k,
                        Err(_e) => return -108i32,
                    };
                    value
                }
            };

            match mem.write(caller, vstart as usize, &value.as_slice()) {
                Ok(_) => (),
                Err(_e) => return -108i32,
            };

            value.len() as i32
        },
    )?;

    linker.func_wrap(
        "xq",
        "call",
        |mut caller: Caller<'_, Context>,
         addr_index: i32,
         amount: u64,
         func_start: u32,
         func_len: u32,
         par_start: u32,
         par_len: u32|
         -> i32 {
            charge_gas(&mut caller, GAS_ENV_FUNC_BASE);
            let mem = match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => mem,
                _ => return -1i32,
            };

            let address = match mem
                .data(&caller)
                .get(addr_index as u32 as usize..)
                .and_then(|arr| arr.get(..ADDRESS_SIZE))
            {
                Some(e) => match String::from_utf8(e.to_vec()) {
                    Ok(o) => o,
                    Err(_) => return -102i32,
                },
                None => return 0,
            };

            let func = match mem
                .data(&caller)
                .get(func_start as usize..)
                .and_then(|arr| arr.get(..func_len as usize))
            {
                Some(e) => match String::from_utf8(e.to_vec()) {
                    Ok(o) => o,
                    Err(_) => return -102i32,
                },
                None => return 0,
            };

            let arg = match mem
                .data(&caller)
                .get(par_start as usize..)
                .and_then(|arr| arr.get(..par_len as usize))
            {
                Some(e) => match String::from_utf8(e.to_vec()) {
                    Ok(o) => o,
                    Err(_) => return -102i32,
                },
                None => return 0,
            };

            match caller
                .data_mut()
                .metadata
                .call(Address::from(address), amount, func, arg)
            {
                Ok(_) => (),
                Err(_e) => return -108i32,
            };
            0
        },
    )?;

    linker.func_wrap(
        "xq",
        "transfer",
        |mut caller: Caller<'_, Context>, start: i32, amount: u64| -> i32 {
            charge_gas(&mut caller, GAS_ENV_FUNC_BASE);
            let mem = match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => mem,
                _ => return 3i32,
            };

            let address = match mem
                .data(&caller)
                .get(start as u32 as usize..)
                .and_then(|arr| arr.get(..ADDRESS_SIZE as u32 as usize))
            {
                Some(e) => match String::from_utf8(e.to_vec()) {
                    Ok(o) => o,
                    Err(_) => return -102i32,
                },
                None => return 0,
            };
            match caller
                .data_mut()
                .metadata
                .transfer(Address::from(address), amount)
            {
                Ok(_) => (),
                Err(_e) => return -108i32,
            };
            0
        },
    )?;

    linker.func_wrap(
        "gas",
        "gas",
        |mut caller: Caller<'_, Context>, amount: u32| {
            if caller.data_mut().gas_outof {
                return ();
            }
            let prev = caller.data_mut().gas_counter;

            let ret = match prev.checked_add(amount as u64) {
                None => false,
                Some(val) if val > caller.data_mut().gas_limit => false,
                Some(_) => {
                    caller.data_mut().gas_counter = prev + amount as u64;
                    true
                }
            };

            return if ret {
                ()
            } else {
                caller.data_mut().gas_outof = true;
                ()
            };
        },
    )?;

    linker.func_wrap(
        "xq",
        "Debug",
        |mut caller: Caller<'_, Context>, ptr: i32, len: i32| {
            let mem = match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => mem,
                _ => return println!("debug get mem err"),
            };

            let msg = match mem
                .data(&caller)
                .get(ptr as u32 as usize..)
                .and_then(|arr| arr.get(..len as u32 as usize))
            {
                Some(e) => match String::from_utf8(e.to_vec()) {
                    Ok(o) => o,
                    Err(_) => return (),
                },
                None => return (),
            };
            println!("WASM Debug:{:?}", msg);
        },
    )?;

    Ok(linker)
}

fn charge_gas(caller: &mut Caller<'_, Context>, amount: u64) {
    if !caller.data().gas {
        return ();
    }
    if caller.data_mut().gas_outof {
        return ();
    }
    let prev = caller.data_mut().gas_counter;
    let ret = match prev.checked_add(amount) {
        // gas charge overflow protection
        None => false,
        Some(val) if val > caller.data_mut().gas_limit => false,
        Some(_) => {
            caller.data_mut().gas_counter = prev + amount;
            true
        }
    };

    return if ret {
        ()
    } else {
        caller.data_mut().gas_outof = true;
        ()
    };
}
