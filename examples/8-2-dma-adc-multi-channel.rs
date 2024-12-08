#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m::singleton;

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{adc, dma::Half, pac, prelude::*};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    // Acquire peripherals
    let p = pac::Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();
    let rcc = p.RCC.constrain();

    // Configure ADC clocks
    let clocks = rcc.cfgr.adcclk(2.MHz()).freeze(&mut flash.acr);

    let dma_ch1 = p.DMA1.split().1;

    // Setup ADC
    let adc1 = adc::Adc::adc1(p.ADC1, &clocks);

    // Setup GPIOA
    let mut gpioa = p.GPIOA.split();

    let adc_ch0 = gpioa.pa0.into_analog(&mut gpioa.crl);

    let adc_dma = adc1.with_dma(adc_ch0, dma_ch1);
    let buf = singleton!(: [[u16; 8]; 2] = [[0; 8]; 2]).unwrap();
    let mut circ_buffer = adc_dma.circ_read(buf);

    loop {
        while circ_buffer.readable_half().unwrap() != Half::First {}

        let first_half = circ_buffer.peek(|half, _| *half).unwrap();

        while circ_buffer.readable_half().unwrap() != Half::Second {}

        let second_half = circ_buffer.peek(|half, _| *half).unwrap();
        rprintln!("first_half: {:?}", first_half);
        rprintln!("second_half: {:?}", second_half);
    }
}
