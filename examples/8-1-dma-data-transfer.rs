#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;

use core::sync::atomic::{self, Ordering};
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{pac, prelude::*};

static A: [u32; 4] = [1, 2, 3, 4];
static B: [u32; 4] = [0, 0, 0, 0];

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let p = pac::Peripherals::take().unwrap();

    rprintln!("A: {:?}", A);
    rprintln!("A: {:?}", &A as *const _ as u32);

    let mut dma_ch1 = p.DMA1.split().1;
    dma_ch1.set_peripheral_address(&A as *const _ as u32, true);
    dma_ch1.set_memory_address(&B as *const _ as u32, true);
    dma_ch1.set_transfer_length(4);
    dma_ch1.start();

    while dma_ch1.in_progress() {}

    dma_ch1.stop();
    rprintln!("A: {:?}", A);
    rprintln!("B: {:?}", B);

    loop {}
}
