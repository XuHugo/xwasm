//#![cfg_attr(not(feature = "std"), no_std)]
use serde_derive::{Serialize, Deserialize};
use xq_derive::{init,call, Output};
use xq_std::*;
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
fn init<C:Context + Copy>(ctx: C, amoun3:u64)->CResult<RetValue>{
    println!("this is init ! amount:{:?}", amoun3);
    let a:Param = ctx.paramteter();
    println!("this is init !parameter{:?}", a);
    //let ret = RetValue{name:"xx".to_string(), age:22, sex:"man".to_string()};
    let x = ContractError::ParseParams.to_string();
    println!("this is init enum string{:?}", x);
    Err(ContractError::ParseParams)
}
#[call(contract="xq", func="abc")]
fn rcv<C:Context + Copy>(ctx: C,)->CResult<RetValue>{
    println!("this is call ! amount:{:?}", 0);

    let a:Param = ctx.paramteter();
    println!("this is call !parameter{:?}", a);
    let ret = RetValue{name:"xx".to_string(), age:22, sex:"man".to_string()};
    Ok(ret)
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
