use serde_derive::{Serialize, Deserialize};
use serde_json::json;
//#![cfg_attr(not(feature = "std"), no_std)]
use xq_derive::{init,call, Output};
use xq_std::*;
use xq_wasm::*;
use wasm::{*, types::{WasmtimeRuntime, Context as WasmCtx, Address, Metadata}};
//use serde::{Serialize};
//use serde_json::Result;
use core::result::Result;
use std::{
    fs,
};

// #[derive(Serialize, Deserialize, Debug)]
// //#[state(contract="xq")]
// struct Param{
//     name: String,
//     age: u64,
//     sex: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// //#[state(contract="xq")]
// struct State{
//     name: String,
//     age: u64,
//     number: u64,
// }


// #[derive(Serialize, Debug)]
// struct RetValue{
//     name: String,
//     age: u64,
//     sex: String,
// }

// #[derive(Debug, Output)]
// enum ContractError {
//     ParseParams,
//     OnlyAccounts,
//     MoreThanAllowed,
//     LogFull,
//     InsufficientFunds,
//     LogMalformed,
    
// }

// // impl core::fmt::Display for ContractError
// // {
// //     fn fmt(&self, f :& mut core::fmt::Formatter<'_>) -> core::fmt::Result
// //     {
// //         match self{
// //             ContractError::ParseParams => core::write!(f, "{}", ContractError::ParseParams.to), 
// //             ContractError::OnlyAccounts=> core::write!(f, "{}", ContractError::OnlyAccounts), 
// //             ContractError::MoreThanAllowed => core::write!(f, "{}", ContractError::MoreThanAllowed), 
// //             ContractError::LogFull => core::write!(f, "{}", ContractError::LogFull), 
// //             ContractError::InsufficientFunds => core::write!(f, "{}", ContractError::InsufficientFunds), 
// //             ContractError::LogMalformed => core::write!(f, "{}", ContractError::LogMalformed),
// //         }
// //     }
// // }

// type CResult<T> = Result<T, ContractError>;

// #[init(contract="xq", payable)]
// fn init<C:Context + Copy>(ctx: C, amoun3:u64)->CResult<RetValue>{
//     //println!("this is init ! amount:{:?}", amoun3);
//     //let a:Param = ctx.paramteter();
//     //println!("this is init !parameter{:?}", a);
//     //let ret = RetValue{name:"xx".to_string(), age:22, sex:"man".to_string()};
//     //let x = ContractError::ParseParams.to_string();
//     //println!("this is init enum string{:?}", x);
//     Err(ContractError::ParseParams)
// }
// #[call(contract="xq", func="abc")]
// fn rcv<C:Context + Copy>(ctx: C)->CResult<RetValue>{
//     //println!("this is call ! amount:{:?}", 0);
//     //let a:Param = ctx.paramteter();
//     //println!("this is call !parameter{:?}", a);
//     let ret = RetValue{name:"xx".to_string(), age:22, sex:"man".to_string()};
//     Ok(ret)
// }


fn main() {
    println!("xq, hello!");
    //derive_t();
    //wasm_t();
    vm_t();
}

fn vm_t(){
    let modules = fs::read("./xq_test/contract.wasm").unwrap();
    let binary = modules.as_slice();

    // let engine = wasmtime::Engine::default();
    // let binary = match engine.precompile_module(wasm_bytes) {
    //     Ok(b) => b,
    //     Err(e) => {
    //         return
    //     },
    // };
    // struct Param{
    //     name: String,
    //     age: u64,
    //     sex: String,
    // }

    let val = json!({"name":"200", "age": 111, "sex":"ok!"});
    let val = val.to_string();

    let wtrt = wasm::types::WasmtimeRuntime;
    //let func_name = "call_abc"; init_xq
    let func_name = "call_abc";
    let amount:u64 = 0;
    let context = WasmCtx::init(
        String::from("call_abc"),
        String::from("state"),
        //String::from("param"),
        val,
        Address([0; 20]),
        Address([0; 20]),
        Address([1; 20]),
        0,
        Metadata{
            block_time: 0,
            block_height: 1,  
            tx_hash: String::from("tx_hash"),  
        },
        false,
        0,
    );
    let ret = match WasmtimeRuntime::execute(func_name, context, &binary, amount){
        Ok(o) =>println!("{:?}",o),
        Err(e) => println!("{}",e),
    };
}

// fn derive_t(){
//     let x = ContractError::ParseParams.to_string();
//     println!("this is macrofn enum string{:?}", x);
//     //let state = State{name:"xx".to_string(), age:34, number:30};
//     let i = init_xq(5);
//     //let c = call_abc(0);
//     //println!("init:{}   call:{}",i,0);
//     let data = r#"
//     {
//         "name": "XuQiang",
//         "age": 18,
//         "sex": "male" 
//     }"#;

//     let p: Param = serde_json::from_str(data).unwrap();
//     println!("Please call {} at the number {}", p.name, p.sex);
// }

// fn wasm_t(){
//     let code = fs::read("./xq_test/hw_rust.wasm").unwrap();
//     //let geeco_buf = geeco.as_slice();
//     match parse::DecodeBinary(code){
//         Ok(m)=>{
//             println!("{:#?}",m.magic);
//             println!("{:#?}",m.version);
//             println!("{:#?}",m.custom);
//             println!("{:#?}",m.types);
//             println!("{:#?}",m.import);
//             println!("{:#?}",m.function);
//             println!("{:#?}",m.table);
//             println!("{:#?}",m.memory);
//             println!("{:#?}",m.global);
//             println!("{:#?}",m.export);
//             println!("{:#?}",m.start);
//             //println!("{:?}",m.element);
//             //println!("{:?}",m.code);
//             //println!("{:?}",m.data);
//             match m.invoke_export("main"){
//                 Ok(_o)=>{
//                     println!("invoke ok!!");
//                 },
//                 Err(_e)=>{
//                     println!("invoke error!!");
//                 },
//             };
//         },
//         Err(_e)=>println!("parse module error!"),
//     };
// }
