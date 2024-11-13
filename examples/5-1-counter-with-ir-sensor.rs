#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;

use core::mem::MaybeUninit;
use cortex_m_rt::entry;
use pac::interrupt;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::gpio::*;
use stm32f1xx_hal::{pac, prelude::*};

static mut INT_PIN: MaybeUninit<stm32f1xx_hal::gpio::gpiob::PB14<Input<Floating>>> =
    MaybeUninit::uninit();

#[interrupt]
fn EXTI15_10() {
    let int_pin = unsafe { &mut *INT_PIN.as_mut_ptr() };

    if int_pin.check_interrupt() {
        rprintln!("Interrupt triggered");
        // if we don't clear this bit, the ISR would trigger indefinitely
        int_pin.clear_interrupt_pending_bit();
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    // initialization phase
    let mut p = pac::Peripherals::take().unwrap();
    let _cp = cortex_m::peripheral::Peripherals::take().unwrap();
    {
        // the scope ensures that the int_pin reference is dropped before the first ISR can be executed.

        let mut gpiob = p.GPIOB.split();
        let mut afio = p.AFIO.constrain();

        let int_pin = unsafe { &mut *INT_PIN.as_mut_ptr() };
        *int_pin = gpiob.pb14.into_floating_input(&mut gpiob.crh);
        int_pin.make_interrupt_source(&mut afio);
        int_pin.trigger_on_edge(&mut p.EXTI, Edge::Falling);
        int_pin.enable_interrupt(&mut p.EXTI);
    } // initialization ends here

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::EXTI15_10);
    }

    loop {}
}
