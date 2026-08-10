//#![cfg_attr(not(feature = "std"), no_std)]
#![no_std]
extern crate alloc;
//extern crate xq_derive;
use alloc::string::{String, ToString};
use serde_derive::{Deserialize, Serialize};
//#[macro_use]
use core::result::Result;

use xq_derive::{call, init, Output};
use xq_std::Context;

#[derive(Serialize, Deserialize, Debug)]
struct Param {
    name: String,
    age: u64,
    sex: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct State {
    name: String,
    age: u64,
    number: u64,
}

#[derive(Serialize, Debug)]
struct RetValue {
    name: String,
    age: u64,
    sex: String,
}

#[allow(dead_code)]
#[derive(Debug, Output)]
enum ContractError {
    ParseParams,
    OnlyAccounts,
    MoreThanAllowed,
    LogFull,
    InsufficientFunds,
    LogMalformed,
}

type CResult<T> = Result<T, ContractError>;

#[init(contract = "xq", payable)]
fn init<C: Context + Copy>(ctx: C, _amoun3: u64) -> CResult<RetValue> {
    let a: Param = ctx.paramteter();
    let ret = RetValue {
        name: a.name,
        age: a.age,
        sex: a.sex,
    };
    let _x = ContractError::ParseParams.to_string();
    //Err(ContractError::ParseParams)
    Ok(ret)
}
#[call(contract = "xq", function = "abc")]
fn rcv<C: Context + Copy>(ctx: C) -> CResult<RetValue> {
    let a: Param = ctx.paramteter();
    let ret = RetValue {
        name: a.name,
        age: a.age,
        sex: a.sex,
    };
    let state = serde_json::to_string(&ret).unwrap();
    ctx.store_set("last_call".to_string(), state);
    let num = ctx.block_number();
    //let a:Param = ctx.paramteter();
    Ok(ret)
}
#[call(contract = "xq", function = "store")]
fn store_by_name<C: Context + Copy>(ctx: C) -> CResult<RetValue> {
    let a: Param = ctx.paramteter();
    let key = a.name.clone();
    let ret = RetValue {
        name: a.name,
        age: a.age,
        sex: a.sex,
    };
    let state = serde_json::to_string(&ret).unwrap();
    ctx.store_set(key, state);
    Ok(ret)
}
#[call(contract = "xq", function = "increment")]
fn increment<C: Context + Copy>(ctx: C) -> CResult<u64> {
    let current: u64 = ctx.store_get("counter".to_string());
    let next = current + 1;
    ctx.store_set("counter".to_string(), next.to_string());
    Ok(next)
}
#[call(contract = "xq", function = "compute")]
fn compute<C: Context + Copy>(ctx: C) -> CResult<u64> {
    let a: Param = ctx.paramteter();
    let mut value = a.age ^ 0x9e37_79b9_7f4a_7c15;
    for index in 0..a.age {
        value = value
            .wrapping_mul(0xbf58_476d_1ce4_e5b9)
            .rotate_left((index & 63) as u32)
            ^ index;
    }
    Ok(value)
}
