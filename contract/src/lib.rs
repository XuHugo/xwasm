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
    let num = ctx.block_number();
    //let a:Param = ctx.paramteter();
    Ok(ret)
}
