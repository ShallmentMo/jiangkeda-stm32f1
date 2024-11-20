#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;

use stm32f1xx_hal as hal;

use crate::hal::{
    gpio::{gpioc, Output, PinState, PushPull},
    pac::{interrupt, Interrupt, Peripherals, TIM2},
    prelude::*,
    timer::{CounterMs, Event},
};

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};

// Make timer interrupt registers globally available
static G_TIM: Mutex<RefCell<Option<CounterMs<TIM2>>>> = Mutex::new(RefCell::new(None));

// Define an interrupt handler, i.e. function to call when interrupt occurs.
// This specific interrupt will "trip" when the timer TIM2 times out
#[interrupt]
fn TIM2() {
    static mut TIM: Option<CounterMs<TIM2>> = None;

    let tim = TIM.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| G_TIM.borrow(cs).replace(None).unwrap())
    });

    rprintln!("TIM2 interrupt");
    let _ = tim.wait();
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc
        .cfgr
        .sysclk(8.MHz())
        .pclk1(8.MHz())
        .freeze(&mut flash.acr);

    // Set up a timer expiring after 1s
    let mut timer = dp.TIM2.counter_ms(&clocks);
    timer.start(1.secs()).unwrap();

    // Generate an interrupt when the timer expires
    timer.listen(Event::Update);

    // Move the timer into our global storage
    cortex_m::interrupt::free(|cs| *G_TIM.borrow(cs).borrow_mut() = Some(timer));

    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupt::TIM2);
    }

    loop {}
}
