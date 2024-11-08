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
    let mut led0 = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    let mut led1 = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);
    let mut led2 = gpioa.pa2.into_push_pull_output(&mut gpioa.crl);
    let mut led3 = gpioa.pa3.into_push_pull_output(&mut gpioa.crl);
    let mut led4 = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
    let mut led5 = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
    let mut led6 = gpioa.pa6.into_push_pull_output(&mut gpioa.crl);
    let mut led7 = gpioa.pa7.into_push_pull_output(&mut gpioa.crl);
    let mut leds = [
        led0.erase_number(),
        led1.erase_number(),
        led2.erase_number(),
        led3.erase_number(),
        led4.erase_number(),
        led5.erase_number(),
        led6.erase_number(),
        led7.erase_number(),
    ];
    for led in leds.iter_mut() {
        led.set_high();
    }

    loop {
        for led in leds.iter_mut() {
            led.set_low();
            cortex_m::asm::delay(1_000_000);
            led.set_high();
        }
    }
}
