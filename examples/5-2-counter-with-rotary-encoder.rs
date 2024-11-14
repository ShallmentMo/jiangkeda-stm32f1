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

static mut A_PIN: MaybeUninit<stm32f1xx_hal::gpio::gpiob::PB0<Input<Floating>>> =
    MaybeUninit::uninit();
static mut B_PIN: MaybeUninit<stm32f1xx_hal::gpio::gpiob::PB1<Input<Floating>>> =
    MaybeUninit::uninit();
static mut COUNTER: i8 = 0;

#[interrupt]
fn EXTI0() {
    let a_pin = unsafe { &mut *A_PIN.as_mut_ptr() };
    let b_pin = unsafe { &mut *B_PIN.as_mut_ptr() };

    if a_pin.check_interrupt() {
        rprintln!("A pin triggered");
        if b_pin.is_low() {
            unsafe {
                COUNTER += 1;
            }
        }
        // if we don't clear this bit, the ISR would trigger indefinitely
        a_pin.clear_interrupt_pending_bit();
    }
}

#[interrupt]
fn EXTI1() {
    let a_pin = unsafe { &mut *A_PIN.as_mut_ptr() };
    let b_pin = unsafe { &mut *B_PIN.as_mut_ptr() };

    if b_pin.check_interrupt() {
        rprintln!("B pin triggered");
        if a_pin.is_low() {
            unsafe {
                COUNTER -= 1;
            }
        }
        // if we don't clear this bit, the ISR would trigger indefinitely
        b_pin.clear_interrupt_pending_bit();
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

        let a_pin = unsafe { &mut *A_PIN.as_mut_ptr() };
        *a_pin = gpiob.pb0.into_floating_input(&mut gpiob.crl);
        a_pin.make_interrupt_source(&mut afio);
        a_pin.trigger_on_edge(&mut p.EXTI, Edge::Falling);
        a_pin.enable_interrupt(&mut p.EXTI);

        let b_pin = unsafe { &mut *B_PIN.as_mut_ptr() };
        *b_pin = gpiob.pb1.into_floating_input(&mut gpiob.crl);
        b_pin.make_interrupt_source(&mut afio);
        b_pin.trigger_on_edge(&mut p.EXTI, Edge::Falling);
        b_pin.enable_interrupt(&mut p.EXTI);
    } // initialization ends here

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::EXTI0);
        pac::NVIC::unmask(pac::Interrupt::EXTI1);
    }

    loop {
        rprintln!(
            "Counter: {}, A: {}, B: {}",
            unsafe { COUNTER },
            unsafe { &*A_PIN.as_ptr() }.is_low(),
            unsafe { &*B_PIN.as_ptr() }.is_low()
        );
        cortex_m::asm::delay(1_000_000);
    }
}
