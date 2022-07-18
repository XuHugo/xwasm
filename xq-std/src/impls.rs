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
            //get_init_origin(ptr as *mut u8);
            bytes.assume_init()
        };
        address
    }

    fn invoker(&self) -> Address{
        let mut bytes: MaybeUninit<[u8; 20]> = MaybeUninit::uninit();
        let ptr = bytes.as_mut_ptr();
        let address = unsafe {
            //get_init_origin(ptr as *mut u8);
            bytes.assume_init()
        };
        address
    }

    fn sender(&self) -> Address{
        let mut bytes: MaybeUninit<[u8; 20]> = MaybeUninit::uninit();
        let ptr = bytes.as_mut_ptr();
        let address = unsafe {
            //get_init_origin(ptr as *mut u8);
            bytes.assume_init()
        };
        address
    }
    fn self_address(&self) -> Address{
        let mut bytes: MaybeUninit<[u8; 20]> = MaybeUninit::uninit();
        let ptr = bytes.as_mut_ptr();
        let address = unsafe {
            //get_init_origin(ptr as *mut u8);
            bytes.assume_init()
        };
        address
    }
    fn self_balance(&self) -> u64{
        unsafe { 
            //get_receive_self_balance()
         }
         0
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

    fn error_set(self, err_code:i32){
        todo!()
    }
    fn error_get(self){
        todo!()
    }

    fn state_get()->String{
        //state_get() -> u32;
        String::new()
    }
    fn state_set(state:String){
        //state_set(ptr: *mut u8, length: u32) -> u32;
        todo!()
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