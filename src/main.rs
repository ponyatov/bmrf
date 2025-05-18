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

#[allow(dead_code)]
#[cfg_attr(feature = "cortexM", entry)]
fn main() -> ! {
    init();
    args();
    loop {
        tick();
    }
}

// type EFI_HANDLE = *const ();

struct EFI_TABLE_HEADER {
    Signature: u64,
    Revision: u32,
    HeaderSize: u32,
    CRC32: u32,
    Reserved: u32,
}

type EFI_TEXT_RESET = *const ();

type EFI_TEXT_STRING = extern "C" fn(*const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, *const u16);

struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    Reset: EFI_TEXT_RESET,
    OutputString: EFI_TEXT_STRING,
    // ... and more stuff that we're ignoring.
}

struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL;

struct EFI_SYSTEM_TABLE {
    Hdr: EFI_TABLE_HEADER,
    FirmwareVendor: *const u16,
    FirmwareRevision: u32,
    ConsoleInHandle: EFI_HANDLE,
    ConIn: *const EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    ConsoleOutHandle: EFI_HANDLE,
    ConOut: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    // ... other stuff that we're ignoring for now.
}

#[cfg(feature = "uefi")]
#[unsafe(no_mangle)]
extern "C" fn efi_main(_ImageHandle: EFI_HANDLE, _SystemTable: *const EFI_SYSTEM_TABLE) -> i32 {
    loop {}
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
