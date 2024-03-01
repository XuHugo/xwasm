use crate::{constants::*, prims::*, traits::*, types::*};

#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, collections::*, string::String, vec::Vec};

#[cfg(not(feature = "std"))]
use core::{marker, mem::MaybeUninit, slice};
#[cfg(feature = "std")]
use std::mem::MaybeUninit;
use std::println;

impl Context for ContractContext {
    fn owner(&self) -> Address {
        let mut bytes: MaybeUninit<[u8; ADDRESS_NUM_LEN]> = MaybeUninit::uninit();
        let ptr = bytes.as_mut_ptr();
        let address = unsafe {
            get_owner(ptr as *mut u8);
            bytes.assume_init()
        };
        Address(address)
    }

    fn invoker(&self) -> Address {
        let mut bytes: MaybeUninit<[u8; ADDRESS_NUM_LEN]> = MaybeUninit::uninit();
        let ptr = bytes.as_mut_ptr();
        let address = unsafe {
            get_invoker(ptr as *mut u8);
            bytes.assume_init()
        };
        Address(address)
    }

    fn self_address(&self) -> Address {
        let mut bytes: MaybeUninit<[u8; ADDRESS_NUM_LEN]> = MaybeUninit::uninit();
        let ptr = bytes.as_mut_ptr();
        let address = unsafe {
            get_contract_address(ptr as *mut u8);
            bytes.assume_init()
        };
        Address(address)
    }

    fn self_balance(&self) -> u64 {
        unsafe { get_contract_balance() }
    }

    //where T: for<'a >+ serde::Deserialize<'a>
    //
    fn paramteter<T>(&self) -> T
    where
        T: serde::de::DeserializeOwned,
    {
        //let mut bytes: MaybeUninit<[u8; 1024]> = MaybeUninit::uninit();
        // unsafe {
        //     get_parameter(bytes.as_mut_ptr()  as *mut u8, 1024);
        // }
        println!("paramteter start:");
        let mut p = [0; MAX_PREALLOCATED_CAPACITY];
        let ret = unsafe { get_parameter(p.as_mut_ptr()) };
        let p = match serde_json::from_slice(&p[..ret as usize]) {
            Ok(o) => o,
            Err(err) => {
                panic!("paramteter error: {}", err);
            }
        };
        p
    }

    fn error(&self, err: String) {
        let _err_len = err.len();
        let _ptr = err.as_ptr();
        // unsafe {
        //     set_error(ptr, err_len as u32);
        //  }
        println!("error_set {}", err);
    }

    fn return_data(&self, data: String) {
        let data_len = data.len();
        let ptr = data.as_ptr();
        unsafe {
            set_return_data(ptr, data_len as u32);
        }
        println!("return data set {}", data);
    }

    fn store_get<T>(&self, mut key: String) -> T
    where
        T: serde::de::DeserializeOwned,
    {
        let mut p = [0; MAX_PREALLOCATED_CAPACITY];
        let key_len = key.len();
        let ptr = key.as_mut_ptr();
        let ret = unsafe { get_store(ptr, key_len as u32, p.as_mut_ptr()) };
        let p = serde_json::from_slice(&p[..ret as usize]).unwrap();
        p
    }
    fn store_set(&self, mut key: String, mut value: String) -> bool {
        let key_len = key.len();
        let kptr = key.as_mut_ptr();
        let value_len = value.len();
        let vptr = value.as_mut_ptr();
        let _a = unsafe { set_store(kptr, key_len as u32, vptr, value_len as u32) };
        true
    }

    fn event(&self, event: String) {
        let len = event.len();
        let ptr = event.as_ptr();
        unsafe {
            set_event(ptr, len as u32);
        }
        println!("event: {}", event);
    }

    fn transfer(&self, addr: Address, amount: u64) -> bool {
        let address = addr.0.as_ptr();
        let ret = unsafe { transfer(address, amount) };
        if ret == 0 {
            false
        } else {
            true
        }
    }
    fn call(&self, addr: Address, amount: u64, func: String, arg: String) -> bool {
        let address = addr.0.as_ptr();
        let parameter = arg.as_ptr();
        let len = arg.len();
        let func_name = func.as_ptr();
        let func_name_len = func.len();
        let ret = unsafe {
            call(
                address,
                amount,
                func_name,
                func_name_len as u32,
                parameter,
                len as u32,
            )
        };
        if ret == 0 {
            false
        } else {
            true
        }
    }

    fn block_time(&self) -> u64 {
        unsafe { get_block_time() }
    }
    fn block_number(&self) -> u64 {
        unsafe { get_block_height() }
    }
    fn tx_hash(&self) -> String {
        let mut h = [0; 64];
        let _ret = unsafe { get_block_hash(h.as_mut_ptr()) };
        String::from_utf8(h.to_vec()).unwrap()
    }
}
