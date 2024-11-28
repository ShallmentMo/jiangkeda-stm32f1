#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    time::ms,
    timer::{Channel, Tim2NoRemap},
};

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = p.AFIO.constrain();

    let mut gpioa = p.GPIOA.split();
    let c1 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let pins = (c1);

    let mut pwm = p
        .TIM2
        .pwm_hz::<Tim2NoRemap, _, _>(pins, &mut afio.mapr, 1.kHz(), &clocks);

    pwm.enable(Channel::C1);

    let max = pwm.get_max_duty();

    loop {
        for i in 1..max {
            pwm.set_duty(Channel::C1, i);
            asm::delay(300);
        }
        for i in (1..max - 1).rev() {
            pwm.set_duty(Channel::C1, i);
            asm::delay(300);
        }
    }
}
