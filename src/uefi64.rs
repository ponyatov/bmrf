#![cfg(feature = "uefi")]

use uefi::prelude::*;

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

pub struct EFI_TABLE_HEADER {
    Signature: u64,
    Revision: u32,
    HeaderSize: u32,
    CRC32: u32,
    Reserved: u32,
}

pub type EFI_TEXT_RESET = *const ();

pub type EFI_TEXT_STRING = extern "C" fn(*const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, *const u16);

pub struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    Reset: EFI_TEXT_RESET,
    OutputString: EFI_TEXT_STRING,
    // ... and more stuff that we're ignoring.
}

pub struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL;

pub struct EFI_SYSTEM_TABLE {
    Hdr: EFI_TABLE_HEADER,
    FirmwareVendor: *const u16,
    FirmwareRevision: u32,
    ConsoleInHandle: EFI_HANDLE,
    ConIn: *const EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    ConsoleOutHandle: EFI_HANDLE,
    ConOut: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    // ... other stuff that we're ignoring for now.
}

#[unsafe(no_mangle)]
extern "C" fn efi_main(_ImageHandle: EFI_HANDLE, _SystemTable: *const EFI_SYSTEM_TABLE) -> i32 {
    loop {}
}
