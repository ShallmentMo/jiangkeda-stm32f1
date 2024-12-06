#![allow(clippy::empty_loop)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let mut gpioa = p.GPIOA.split();
    let mut led = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);

    loop {
        led.set_high();
        cortex_m::asm::delay(1_000_000);
        led.set_low();
        cortex_m::asm::delay(1_000_000);
    }
}
