use crate::{constants::*, traits::*, types::*, prims::*};

#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, collections::*, string::String, vec::Vec};
use serde_json::json;
#[cfg(not(feature = "std"))]
use core::{marker, mem::MaybeUninit, slice};
#[cfg(feature = "std")]
use std::{collections::*, marker, mem::MaybeUninit, slice};

use serde::{Deserialize, Serialize};

impl Context for ContractContext{

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

    //where T: for<'a >+ serde::Deserialize<'a>
    //
    fn paramteter<T>(&self)-> T
    where T: serde::de::DeserializeOwned
    {
        //let mut bytes: MaybeUninit<[u8; 1024]> = MaybeUninit::uninit();
        // unsafe {
        //     get_parameter(bytes.as_mut_ptr()  as *mut u8, 1024);
        // }

        let mut p = [0;1024];
        let _ret = unsafe { 
            get_parameter(p.as_mut_ptr(), 1024)
         };
         let p =   serde_json::from_slice(&p).unwrap();
        // //test
        // //let mut bytes:[u8;1024] = [0;1024];
        // let val = json!({"name":"200", "age": 111, "sex":"ok!"});
        // let val = serde_json::to_vec(&val).unwrap();
        // //bytes.write(val.as_slice() as [u8; 1024]);
        
        // // let par = unsafe {
        // //     bytes.assume_init()
        // // }; 
        // // let par = val.clone();
        // // let p: T =   serde_json::from_slice(&par).unwrap();
        // let par = val.clone();
        // let p =   serde_json::from_slice(&par).unwrap();
        p
        
    }

    fn error(&self, err:String){
        let err_len = err.len();
        let ptr = err.as_ptr();
        unsafe { 
            set_error(ptr, err_len as u32);
         }
        println!("error_set {}",err);
    }

    fn return_data(&self, data:String){
        let data_len = data.len();
        let ptr = data.as_ptr();
        unsafe { 
            set_return_data(ptr, data_len as u32);
         }
        println!("return data set {}",data);
    }

    fn store_get(&self, mut key:String)->String{
        let key_len = key.len();
        let ptr = key.as_mut_ptr();
        unsafe { 
            get_store(ptr, key_len as u32);
         }
        format!("store get: {:?}",key)
    }
    fn store_set(&self, mut key:String, mut value:String)-> bool{
        let key_len = key.len();
        let kptr = key.as_mut_ptr();
        let value_len = value.len();
        let vptr = value.as_mut_ptr();
        let _a =unsafe { 
            set_store(kptr, key_len as u32, vptr, value_len as u32)
         };
        println!("store_set({}=>{})",key, value);
        true
    }

    fn event(&self, event:String){
        let len = event.len();
        let ptr = event.as_ptr();
        unsafe { 
            set_event(ptr, len as u32);
         }
        println!("event: {}",event);
    }

    fn transfer(&self, addr:Address, amount:u64)->bool{
        let address = addr.as_ptr();
        let ret = unsafe { 
            _transfer(address, amount)
        };
        if ret == 0{
            false
        }else{
            true
        }
    }
    fn call(&self, addr:Address, amount:u64, arg:String)->bool{
        let address = addr.as_ptr();
        let parameter = arg.as_ptr();
        let len = arg.len();
        let ret = unsafe { 
            _call(address, amount, parameter, len as u32)
        };
        if ret == 0{
            false
        }else{
            true
        }
    }

    fn block_time(&self)->u64{
        unsafe { 
            get_block_time()
         }
    }
    fn block_number(&self)->u64{
        unsafe { 
            get_block_number()
         }
    }
    fn tx_hash(&self) -> String{
        let mut h = [0;64];
        let _ret = unsafe { 
            get_tx_hash(h.as_mut_ptr())
         };
         String::from_utf8(h.to_vec()).unwrap()
    }

}
