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
    let c3 = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let pins = (c3);
    let mut p4 = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
    let mut p5 = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);

    let mut pwm = p
        .TIM2
        .pwm_hz::<Tim2NoRemap, _, _>(pins, &mut afio.mapr, 20.kHz(), &clocks);

    pwm.enable(Channel::C3);

    let max = pwm.get_max_duty();
    rprintln!("max: {}", max);
    p4.set_high();
    p5.set_low();
    pwm.set_duty(Channel::C3, 0);

    loop {
        for i in (1..=max).step_by(10) {
            pwm.set_duty(Channel::C3, i);
            cortex_m::asm::delay(3000);
        }
        for i in (max - 1..=1).step_by(10) {
            pwm.set_duty(Channel::C3, i);
            cortex_m::asm::delay(3000);
        }
    }
}
