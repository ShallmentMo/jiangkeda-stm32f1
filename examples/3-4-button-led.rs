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
    let key = gpioa.pa1.into_pull_up_input(&mut gpioa.crl);
    let mut led = gpioa.pa2.into_push_pull_output(&mut gpioa.crl);

    loop {
        if key.is_low() {
            led.set_low();
        } else {
            led.set_high();
        }
    }
}
