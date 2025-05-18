#![cfg(feature = "uefi")]

use crate::cmd::*;

pub fn init() {}
pub fn args() {}
pub fn tick() {}

use core::panic::PanicInfo;
use core::sync::atomic;
use core::sync::atomic::Ordering;

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

pub type EFI_HANDLE = *const ();
