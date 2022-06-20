#![cfg_attr(not(feature = "std"), no_std, feature(alloc_error_handler, core_intrinsics))]

#[cfg(not(feature = "std"))]
pub extern crate alloc;

#[cfg(not(feature = "std"))]
#[alloc_error_handler]
fn on_oom(_layout: alloc::alloc::Layout) -> !{
    #[cfg(target_arch = "wasm32")]
        unsafe {
        core::arch::wasm32::unreachable()
    }
    #[cfg(not(target_arch = "wasm32"))]
    loop{}
}

#[cfg(feature = "std")]
pub use std::process::abort as trap;

#[cfg(all(not(feature = "std"), target_arch = "wasm32"))]
#[inline(always)]
pub fn trap() -> !{ unsafe {core::arch::wasm32::unreachable()}}

#[cfg(all(not(feature = "std"), not(target_arch = "wasm32")))]
#[inline(always)]
pub fn trap() -> ! {core::intrinsics::abort()}

#[cfg(not(feature = "std"))]
#[panic_handler]
fn abort_panic(_info: &core::panic::PanicInfo) -> !{
    #[cfg(target_arch = "wasm32")]
    unsafe {
        core::arch::wasm32::unreachable()
    }
    #[cfg(not(target_arch = "wasm32"))]
    loop {}
}

#[cfg(not(feature = "std"))]
pub use alloc::{collections, string, string::String, string::ToString, vec, vec::Vec};

#[cfg(not(feature = "std"))]
pub use core::{convert, marker, mem, num, result::*};

#[cfg(feature = "std")]
pub(crate) use std::vec;

#[cfg(feature = "std")]
pub use std::{collections, convert, marker, mem, num, string::String, vec::Vec};

pub mod constants;
mod impls;
mod prims;
mod traits;
mod types;
pub use impls::*;
pub use traits::*;
pub use types::*;

extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


