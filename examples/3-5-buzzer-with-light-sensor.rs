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

    let mut gpiob = p.GPIOB.split();
    let mut buzzer = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
    let key = gpiob.pb13.into_pull_up_input(&mut gpiob.crh);

    loop {
        if key.is_low() {
            buzzer.set_high();
            cortex_m::asm::delay(1_000_000);
        } else {
            buzzer.set_low();
            cortex_m::asm::delay(1_000_000);
        }
    }
}
