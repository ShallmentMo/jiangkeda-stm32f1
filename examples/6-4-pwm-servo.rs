#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m::asm;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{
    pac,
    prelude::*,
    timer::{Channel, Tim2NoRemap},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let p = pac::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = p.AFIO.constrain();

    let mut gpioa = p.GPIOA.split();
    let c2 = gpioa.pa1.into_alternate_push_pull(&mut gpioa.crl);
    let pins = (c2);

    let mut pwm = p
        .TIM2
        .pwm_hz::<Tim2NoRemap, _, _>(pins, &mut afio.mapr, 50.Hz(), &clocks);

    pwm.enable(Channel::C2);

    let max = pwm.get_max_duty();
    let step = max / 40;
    rprintln!("max: {}", max);
    rprintln!("step: {}", step);

    loop {
        for i in 1..=5 {
            pwm.set_duty(Channel::C2, step * i);
            asm::delay(5000);
        }
    }
}
