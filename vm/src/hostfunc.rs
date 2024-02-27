
use std::convert::TryInto;

use wasmtime::{
    Engine, Linker, Extern, Caller
};
use crate::types::{Context, VMResult};
use crate::traits::HostFunc;

pub fn host_func_init(engine: &Engine) -> VMResult<Linker<Context>>{
    let mut linker = Linker::new(&engine);

    linker.func_wrap("xq", "get_invoker", |mut caller: Caller<'_, Context>, value: i32| {

        let invoker = caller.data_mut().get_invoker();

        let mem = match caller.get_export("memory"){
            Some(Extern::Memory(mem)) => mem,
            _ => return (),
        };
        let  data = mem.write(caller, value as usize, &invoker).unwrap();
        data
    }).unwrap();

    linker.func_wrap("xq", "get_owner", |mut caller: Caller<'_, Context>, value: i32| {

        let owner = caller.data_mut().get_owner();

        let mem = match caller.get_export("memory"){
            Some(Extern::Memory(mem)) => mem,
            _ => return (),
        };
        let  data = mem.write(caller, value as usize, &owner).unwrap();
        data
    }).unwrap();

    linker.func_wrap("xq", "get_contract_address", |mut caller: Caller<'_, Context>, value: i32| {

        let addr = caller.data_mut().get_contract_address();

        let mem = match caller.get_export("memory"){
            Some(Extern::Memory(mem)) => mem,
            _ => return (),
        };
        let  data = mem.write(caller, value as usize, &addr).unwrap();
        data
    }).unwrap();

    linker.func_wrap("xq", "get_contract_balance", |mut caller: Caller<'_, Context>|-> u64 {
        let balance = caller.data_mut().get_contract_balance();
        balance
    }).unwrap();

    linker.func_wrap("xq", "get_parameter", |mut caller: Caller<'_, Context>, value: i32|-> i32 {

        let s = match caller.data_mut().get_parameter(){
            Some(p)=>p,
            None=> return -1,
        };

        let mem = match caller.get_export("memory"){
            Some(Extern::Memory(mem)) => mem,
            _ => return 0,
        };
        let  _data = mem.write(caller, value as usize, &s.as_bytes());
        0
    }).unwrap();

    linker.func_wrap("xq", "call", |mut caller: Caller<'_, Context>, 
        address:i32, 
        amount:i64, 
        func_name:i32, 
        func_name_len:i32, 
        parameter:i32, 
        parameter_len:i32, |-> i32 {

        let mem = match caller.get_export("memory") {
            Some(Extern::Memory(mem)) => mem,
            _ => return -1i32,
        };

        let  data = mem.data(&caller)
            .get(address as u32 as usize..)
            .and_then(|arr| arr.get(..32 as u32 as usize));
        let  addr = match data {
            Some(s) => s,
            None => return 0,
        };
        let addrs = match addr[..].try_into(){
            Ok(r)=>r,
            Err(e)=> return -1,
        };

        let  data = mem.data(&caller)
            .get(func_name as u32 as usize..)
            .and_then(|arr| arr.get(..func_name_len as u32 as usize));
        let  name = match data {
            Some(s) => s,
            None => return -1i32,
        };
        let name = match std::str::from_utf8(name) {
            Ok(v) => v.to_string(),
            Err(_) => return -107i32,
        };

        let  data = mem.data(&caller)
            .get(parameter as u32 as usize..)
            .and_then(|arr| arr.get(..parameter_len as u32 as usize));
        
        let  param = match data {
            Some(s) => s,
            None => return -1i32,
        };
        
        let param = match std::str::from_utf8(param) {
            Ok(v) => v.to_string(),
            Err(_) => return -107i32,
        };
        let ret = caller.data_mut().call(
            addrs,
            amount as u64,
            name,
            param,
        ).unwrap();
        0
    }).unwrap();

    linker.func_wrap("xq", "transfer", |mut caller: Caller<'_, Context>,start:i32, amount:i64|-> i32 {
        let mem = match caller.get_export("memory") {
            Some(Extern::Memory(mem)) => mem,
            _ => return 3i32,
        };

        let  data = mem.data(&caller)
            .get(start as u32 as usize..)
            .and_then(|arr| arr.get(..32 as u32 as usize));
        let  address = match data {
            Some(s) => s,
            None => return 0,
        };
        let addrs = match address[..].try_into(){
            Ok(r)=>r,
            Err(e)=> return -1,
        };
        let ret = caller.data_mut().transfer(addrs, amount as u64).unwrap();
        0
    }).unwrap();


    linker.func_wrap("xq", "set_store",  |mut caller: Caller<'_, Context>, 
        key: i32, 
        klen: i32, 
        value: i32, 
        vlen: i32| ->i32 {
        let mem = match caller.get_export("memory") {
            Some(Extern::Memory(mem)) => mem,
            _ => return 3i32,
        };
        let  data = mem.data(&caller)
            .get(key as u32 as usize..)
            .and_then(|arr| arr.get(..klen as u32 as usize));
        let  k = match data {
            Some(s) => s,
            None => return 0,
        };

        let  data = mem.data(&caller)
            .get(value as u32 as usize..)
            .and_then(|arr| arr.get(..vlen as u32 as usize));
        let  v = match data {
            Some(s) => s,
            None => return 0,
        };

        let res = caller.data().set_store(k.to_vec(), v.to_vec()).unwrap();
        0
    } ).unwrap();

    linker.func_wrap("xq", "get_store", |mut caller: Caller<'_, Context>, key: i32, klength: i32, value: i32|-> i32 {
        let buffer:Vec<u8> = Vec::new();
        let mem = match caller.get_export("memory"){
            Some(Extern::Memory(mem)) => mem,
            _ => return 0,
        };
        let  data = mem.data(&caller)
            .get(key as u32 as usize..)
            .and_then(|arr| arr.get(..klength as u32 as usize));
        let  k = match data {
            Some(s) => s,
            None => return 0,
        };
        let v = caller.data().get_store(k.to_vec()).unwrap();
        let  data = mem.write(caller, value as usize, &v);
        0 as i32
    }).unwrap();

    linker.func_wrap("xq", "get_block_time", |mut caller: Caller<'_, Context> |-> i64 {

        let t = caller.data_mut().get_block_time();
        t as i64
    }).unwrap();

    linker.func_wrap("xq", "get_block_height", |mut caller: Caller<'_, Context> |-> i64 {

        let t = caller.data_mut().get_block_height();
        t as i64
    }).unwrap();

    linker.func_wrap("xq", "get_block_hash", |mut caller: Caller<'_, Context>, value: i32|-> i64 {
        let mem = match caller.get_export("memory"){
            Some(Extern::Memory(mem)) => mem,
            _ => return 0,
        };
        let t = caller.data_mut().get_block_hash();
        let  data = mem.write(caller, value as usize, &t.as_bytes()).unwrap();
        0 as i64
    }).unwrap();

    linker.func_wrap("xq", "set_event", |mut caller: Caller<'_, Context>, start: i32, length: i32 |-> i32 {
        let mem = match caller.get_export("memory") {
            Some(Extern::Memory(mem)) => mem,
            _ => return -1i32,
        };
        let  data = mem.data(&caller)
            .get(start as u32 as usize..)
            .and_then(|arr| arr.get(..length as u32 as usize));
        let  event = match data {
            Some(s) => s,
            None => return -1i32,
        };

        let res = caller.data().set_event(event.to_vec()).unwrap();
        0
    }).unwrap();

    linker.func_wrap("xq", "set_return_data", |mut caller: Caller<'_, Context>, start: i32, length: i32 |-> i32 {

        let mem = match caller.get_export("memory") {
            Some(Extern::Memory(mem)) => mem,
            _ => return -1i32,
        };
        let  data = mem.data(&caller)
            .get(start as u32 as usize..)
            .and_then(|arr| arr.get(..length as u32 as usize));
        let  rdata = match data {
            Some(s) => s,
            None => return -1i32,
        };

        let res = caller.data().set_return_data(rdata.to_vec()).unwrap();
        0
    }).unwrap();

    Ok(linker)
}

