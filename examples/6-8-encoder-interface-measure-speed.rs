#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{pac, prelude::*, timer::pwm_input::*};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let p = pac::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = p.AFIO.constrain();
    let mut dbg = p.DBGMCU;

    let gpioa = p.GPIOA.split();
    let pa6 = gpioa.pa6;
    let pa7 = gpioa.pa7;

    let pwm_input = p.TIM3.pwm_input(
        (pa6, pa7),
        &mut dbg,
        Configuration::Frequency(10.kHz()),
        &clocks,
    );

    loop {
        let freq = pwm_input
            .read_frequency(ReadMode::Instant, &clocks)
            .unwrap();
        let duty_cycle = pwm_input.read_duty(ReadMode::Instant).unwrap();
        rprintln!(
            "freq: {}, duty_cycle: ({}, {})",
            freq,
            duty_cycle.0,
            duty_cycle.1
        );
    }
}
