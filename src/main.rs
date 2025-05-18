#![allow(unused_imports)]
#![cfg_attr(not(target_os = "linux"), no_std)]
#![cfg_attr(not(target_os = "linux"), no_main)]

#[cfg(feature = "linux")]
pub mod linux;
#[cfg(feature = "linux")]
use crate::linux::args;

#[cfg(feature = "cortexM")]
pub mod cortex;
#[cfg(feature = "cortexM")]
use crate::cortex::args;

#[cfg(feature = "cortexM")]
use cortex_m_rt::entry;


pub fn main() {
    args();
    #[cfg(feature = "cortexM")]
    loop {
        // your code goes here
    }
}
