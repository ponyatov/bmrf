#![cfg(feature = "linux")]

use crate::cmd::*;

pub fn args() {
    println!("args:");
    // \ args
    let argv: Vec<String> = std::env::args().collect();
    let argc = argv.len();
    let ini = if argc > 1 { &argv[1] } else { "lib/ini.ini" };
    let src = std::fs::read_to_string(ini).unwrap();
    println!("\t#{:?} {:?} -> {:?}", argc, argv, ini);
    // / args
    println!("\n{:?}\n", src);
}

pub fn init() {
    println!("init:");
    nop();
}

pub fn tick() {
    println!("tick:");
    halt();
}

pub fn nop() {
    eprintln!("\tnop\n");
}

pub fn halt() {
    eprintln!("\thalt\n");
    std::process::exit(0);
}
