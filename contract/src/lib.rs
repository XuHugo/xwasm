//#![cfg_attr(not(feature = "std"), no_std)]
#![no_std]
extern crate alloc;
//extern crate xq_derive;
use alloc::string::{ToString, String};
use serde_derive::{Serialize, Deserialize};
//#[macro_use]
use xq_derive::{init,call, Output};
use xq_std::Context;
use serde::{Serialize};
use core::result::Result;


#[derive(Serialize, Deserialize, Debug)]
//#[state(contract="xq")]
struct Param{
    name: String,
    age: u64,
    sex: String,
}

#[derive(Serialize, Deserialize, Debug)]
//#[state(contract="xq")]
struct State{
    name: String,
    age: u64,
    number: u64,
}


#[derive(Serialize, Debug)]
struct RetValue{
    name: String,
    age: u64,
    sex: String,
}

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

#[init(contract="xq", payable)]
fn init<C:Context + Copy>(ctx: C, amoun3:u64, )->CResult<RetValue>{
    let a:Param = ctx.paramteter();
    // //let ret = RetValue{name:"xx".to_string(), age:22, sex:"man".to_string()};
    let x = ContractError::ParseParams.to_string();
    Err(ContractError::ParseParams)
}
#[call(contract="xq", function="abc")]
fn rcv<C:Context + Copy>(ctx: C,)->CResult<RetValue>{
    let a:Param = ctx.paramteter();
    let ret = RetValue{name:a.name, age:a.age, sex:a.sex};
    //let a:Param = ctx.paramteter();
    Ok(ret)
}

