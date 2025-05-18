#[cfg(feature = "cortexM")]
use cortex_m_semihosting::dbg;

pub fn nop() {
    dbg!("\tnop\n");
}

pub fn halt() {
    dbg!("\thalt\n");
    #[cfg(feature = "linux")]
    std::process::exit(0);
    #[cfg(feature = "cortexM")]
    loop {}
}
