#![allow(unused_imports)]
#![cfg_attr(not(target_os = "linux"), no_std)]
#![cfg_attr(not(target_os = "linux"), no_main)]

#[cfg(feature = "linux")]
pub mod linux;
#[cfg(feature = "linux")]
use crate::linux::*;

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

pub mod cmd;
