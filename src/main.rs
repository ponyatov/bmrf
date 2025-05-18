#![allow(unused_imports)]
#![cfg_attr(not(target_os = "linux"), no_std)]
#![cfg_attr(not(target_os = "linux"), no_main)]
// #![cfg_attr(feature = "raw", lang_items)]
#![feature(lang_items)]

#[cfg(feature = "linux")]
pub mod linux;
#[cfg(feature = "linux")]
use crate::linux::*;

#[cfg(feature = "uefi")]
pub mod uefi64;
#[cfg(feature = "uefi")]
use crate::uefi64::*;
#[cfg(feature = "uefi")]
use uefi::prelude::*;

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

#[cfg_attr(feature = "cortexM", entry)]
#[allow(dead_code)]
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
#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

pub mod cmd;
