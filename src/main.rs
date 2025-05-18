#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![cfg_attr(not(target_os = "linux"), no_std)]
#![cfg_attr(not(target_os = "linux"), no_main)]
// see https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html
#![allow(internal_features)]
#![cfg_attr(feature = "raw", feature(lang_items))]

#[cfg(feature = "linux")]
pub mod linux;
#[cfg(feature = "linux")]
use crate::linux::*;

#[cfg(feature = "uefi")]
pub mod uefi64;
#[cfg(feature = "uefi")]
use crate::uefi64::*;

#[cfg(feature = "bare")]
pub mod bare;
#[cfg(feature = "bare")]
use crate::bare::*;

#[cfg(feature = "raw")]
pub mod raw;
#[cfg(feature = "raw")]
use crate::raw::*;

#[cfg(feature = "cortexM")]
pub mod cortex;
#[cfg(feature = "cortexM")]
use crate::cortex::*;

#[cfg(feature = "cortexM")]
use cortex_m_rt::entry;

#[allow(dead_code)]
#[cfg_attr(feature = "cortexM", entry)]
fn main() -> ! {
    init();
    args();
    loop {
        tick();
    }
}

// see https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html
#[cfg(feature = "raw")]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[cfg(feature = "raw")]
use core::panic::PanicInfo;
#[cfg(feature = "raw")]
use core::sync::atomic;
#[cfg(feature = "raw")]
use core::sync::atomic::Ordering;

#[cfg(feature = "raw")]
#[unsafe(no_mangle)]
/// The name **must be** `_start`, otherwise the compiler throws away all code as unused.
/// The name can be changed by passing a different entry symbol as linker argument.
fn _start() -> ! {
    loop {}
}

#[cfg(feature = "raw")]
#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

pub mod cmd;
