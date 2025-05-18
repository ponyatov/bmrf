#![no_std]
#![no_main]

#[cfg(feature = "linux")]
pub mod linux;
#[cfg(feature = "linux")]
use crate::linux::args;

#[cfg(feature = "cortex")]
pub mod cortex;
#[cfg(feature = "cortex")]
use crate::cortex::args;

#[cfg(feature = "linux")]
fn main() {
    args();
}

#[cfg(feature = "cortex")]
use cortex_m::asm;
#[cfg(feature = "cortex")]
use cortex_m_rt::entry;

#[cfg(feature = "cortex")]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    loop {
        // your code goes here
    }
}
