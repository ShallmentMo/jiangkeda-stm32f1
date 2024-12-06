#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{adc, pac, prelude::*};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    // Acquire peripherals
    let p = pac::Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();
    let rcc = p.RCC.constrain();

    // Configure ADC clocks
    // Default value is the slowest possible ADC clock: PCLK2 / 8. Meanwhile ADC
    // clock is configurable. So its frequency may be tweaked to meet certain
    // practical needs. User specified value is be approximated using supported
    // prescaler values 2/4/6/8.
    let clocks = rcc.cfgr.adcclk(2.MHz()).freeze(&mut flash.acr);

    // Setup ADC
    let mut adc1 = adc::Adc::adc1(p.ADC1, &clocks);

    // Setup GPIOA
    let mut gpioa = p.GPIOA.split();

    // Configure pa0 as an analog input
    let mut ch0 = gpioa.pa0.into_analog(&mut gpioa.crl);
    let mut ch1 = gpioa.pa1.into_analog(&mut gpioa.crl);

    loop {
        let data: u16 = adc1.read(&mut ch0).unwrap();
        rprintln!("adc1 ch0: {}", data);
        cortex_m::asm::delay(10_000_000);

        let data1: u16 = adc1.read(&mut ch1).unwrap();
        rprintln!("adc1 ch1: {}", data1);
        cortex_m::asm::delay(10_000_000);
    }
}
