#![allow(unused_imports)]
#![cfg(feature = "cortexM")]
#![no_std]
#![cfg(feature = "cortexM")]
#![no_main]

// #[cfg(feature = "linux")]
// pub mod linux;
// #[cfg(feature = "linux")]
// use crate::linux::args;

// #[cfg(feature = "linux")]
fn main() {
    args();
}

#[cfg(feature = "cortexM")]
pub mod cortex;
#[cfg(feature = "cortexM")]
use crate::cortex::args;

#[cfg(feature = "cortexM")]
use cortex_m::asm;
#[cfg(feature = "cortexM")]
use cortex_m_rt::entry;

#[cfg(feature = "cortexM")]
use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

#[cfg(feature = "cortexM")]
#[allow(dead_code)]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code
    args();
    loop {
        // your code goes here
    }
}
