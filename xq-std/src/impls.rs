use crate::{constants::*, traits::*, types::*, prims::*};

#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, collections::*, string::String, vec::Vec};
use serde_json::json;
#[cfg(not(feature = "std"))]
use core::{marker, mem::MaybeUninit, slice};
#[cfg(feature = "std")]
use std::{collections::*, marker, mem::MaybeUninit, slice};

use serde::{Deserialize, Serialize};

impl InitContext for InitContractContext{

    fn owner(&self) -> Address{
        let mut bytes: MaybeUninit<[u8; 20]> = MaybeUninit::uninit();
        let ptr = bytes.as_mut_ptr();
        let address = unsafe {
            get_owner(ptr as *mut u8);
            bytes.assume_init()
        };
        address
    }

    fn invoker(&self) -> Address{
        let mut bytes: MaybeUninit<[u8; 20]> = MaybeUninit::uninit();
        let ptr = bytes.as_mut_ptr();
        let address = unsafe {
            get_invoker(ptr as *mut u8);
            bytes.assume_init()
        };
        address
    }

    fn sender(&self) -> Address{
        let mut bytes: MaybeUninit<[u8; 20]> = MaybeUninit::uninit();
        let ptr = bytes.as_mut_ptr();
        let address = unsafe {
            get_sender(ptr as *mut u8);
            bytes.assume_init()
        };
        address
    }
    fn self_address(&self) -> Address{
        let mut bytes: MaybeUninit<[u8; 20]> = MaybeUninit::uninit();
        let ptr = bytes.as_mut_ptr();
        let address = unsafe {
            get_self_address(ptr as *mut u8);
            bytes.assume_init()
        };
        address
    }
    fn self_balance(&self) -> u64{
        unsafe { 
            get_self_balance()
         }
    }

    fn go(self) ->i32 {
        123
    }

    //where T: for<'a >+ serde::Deserialize<'a>
    //
    fn paramteter<T>(self)-> T
    where T: serde::de::DeserializeOwned
    {

        //let mut bytes: MaybeUninit<[u8; 1024]> = MaybeUninit::uninit();

        // unsafe {
        //     get_parameter(bytes.as_mut_ptr()  as *mut u8, 1024);
        // }

        //test
        //let mut bytes:[u8;1024] = [0;1024];
        let val = json!({"name":"200", "age": 111, "sex":"ok!"});
        let val = serde_json::to_vec(&val).unwrap();
        //bytes.write(val.as_slice() as [u8; 1024]);
        
        // let par = unsafe {
        //     bytes.assume_init()
        // }; 
        // let par = val.clone();
        // let p: T =   serde_json::from_slice(&par).unwrap();
        let par = val.clone();
        let p =   serde_json::from_slice(&par).unwrap();
        p
        
    }

    fn error_set(err:String){
        let err_len = err.len();
        let ptr = err.as_ptr();
        unsafe { 
            error_set(ptr, err_len as u32);
         }
        println!("error_set {}",err);
    }
    fn error_get()->String{
        format!("error_get")
    }

    fn return_data_set(data:String){
        let data_len = data.len();
        let ptr = data.as_ptr();
        unsafe { 
            return_data_set(ptr, data_len as u32);
         }
        println!("return data set {}",data);
    }
    fn return_data_get()->String{
        format!("return data get")
    }

    fn state_get()->String{
        //state_get() -> u32;
        String::new()
    }
    fn state_set(state:String){
        //state_set(ptr: *mut u8, length: u32) -> u32;
        todo!()
    }
    fn store_get(mut key:String)->String{
        let key_len = key.len();
        let ptr = key.as_mut_ptr();
        unsafe { 
            store_get(ptr, key_len as u32);
         }
        format!("store get: {:?}",key)
    }
    fn store_set(mut key:String, mut value:String)-> bool{
        let key_len = key.len();
        let kptr = key.as_mut_ptr();
        let value_len = value.len();
        let vptr = value.as_mut_ptr();
        let _a =unsafe { 
            store_set(kptr, key_len as u32, vptr, value_len as u32)
         };
        println!("store_set({}=>{})",key, value);
        true
    }

}

impl ExecContext for ExecContractContext{

    // fn invoker(&self) -> [u8; 8]{
    //     let mut bytes: MaybeUninit<[u8; 8]> = MaybeUninit::uninit();
    //     let ptr = bytes.as_mut_ptr();
    //     let address = unsafe {
    //         get_init_origin(ptr as *mut u8);
    //         bytes.assume_init()
    //     };
    //     address
    // }
    // fn self_address(&self) -> [u8; 8]{
    //     let mut bytes: MaybeUninit<[u8; 8]> = MaybeUninit::uninit();
    //     let ptr = bytes.as_mut_ptr();
    //     let address = unsafe {
    //         get_init_origin(ptr as *mut u8);
    //         bytes.assume_init()
    //     };
    //     address
    // }
    // fn self_balance(&self) -> u64{
    //     unsafe { get_receive_self_balance() }
    // }
    // fn sender(&self) -> [u8; 8]{
    //     let mut bytes: MaybeUninit<[u8; 8]> = MaybeUninit::uninit();
    //     let ptr = bytes.as_mut_ptr();
    //     let address = unsafe {
    //         get_init_origin(ptr as *mut u8);
    //         bytes.assume_init()
    //     };
    //     address
    // }
    // fn owner(&self) -> [u8; 8]{
    //     let mut bytes: MaybeUninit<[u8; 8]> = MaybeUninit::uninit();
    //     let ptr = bytes.as_mut_ptr();
    //     let address = unsafe {
    //         get_init_origin(ptr as *mut u8);
    //         bytes.assume_init()
    //     };
    //     address
    // }

    fn go(self) ->i32 {
        123
    }
}