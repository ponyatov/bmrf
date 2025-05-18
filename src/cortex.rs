#![cfg(feature = "cortexM")]
#![allow(unused_imports)]
#![allow(non_upper_case_globals)]

use crate::cmd::*;

// pick a panicking behavior
// use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

pub const argc: u8 = 2;
pub const argv: [&str; 2] = ["", "lib/bmrf.ini"];
pub const ini: &str = argv[1];
pub const src: &str = include_str!("../lib/bmrf.ini");

pub fn args() {
    hprintln!("args:");

    hprintln!("\t#{:?} {:?} -> {:?}", argc, argv, ini);
    hprintln!("\n{:?}\n", src);
}

pub fn init() {
    hprintln!("init:");
    nop();
}

pub fn tick() {
    hprintln!("tick:");
    halt();
}

pub fn nop() {
    hprintln!("\tnop\n");
}

pub fn halt() {
    hprintln!("\thalt\n");
    debug::exit(debug::EXIT_SUCCESS);
}
