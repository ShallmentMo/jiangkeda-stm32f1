#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use stm32f1xx_hal as _;

use cortex_m_rt::entry;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    loop {}
}
