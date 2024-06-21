#![no_std]
#![no_main]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod led;
use core::{cell::RefCell, convert::TryInto, fmt::Write};

use crate::hal::prelude::*;
use led::Color::*;
use led::*;
use stm32f4xx_hal::{
    self as hal, block, gpio,
    pac::{self, sdio::dlen},
    timer::SysDelay,
    uart::{Config, Serial},
};
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
use core::cell::UnsafeCell;
use cortex_m::{
    interrupt::{self, Mutex},
    peripheral::{syst, Peripherals},
};
use cortex_m_rt::{entry, exception, interrupt};
use cortex_m_semihosting::hprintln;
use heapless::String;

static G_LED: Mutex<RefCell<Option<Leds>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let p = Peripherals::take().unwrap();

    // Configure the clocks on the board.
    let clocks: stm32f4xx_hal::rcc::Clocks = dp.RCC.constrain().cfgr.freeze();

    let gpiod = dp.GPIOD.split();
    let mut leds = Leds::new(gpiod);
    interrupt::free(|cs| {
        G_LED.borrow(cs).replace(Some(leds));
    });

    let mut syst = p.SYST;
    syst.set_clock_source(syst::SystClkSource::Core);
    syst.enable_interrupt();
    syst.set_reload(10000);
    syst.clear_current();
    syst.enable_counter();

    let mut i = 0;
    loop {
        hprintln!("{}", i);
        i += 1;
    }
}

#[cortex_m_rt::exception]
fn SysTick() {
    interrupt::free(|cs| {
        if let Some(ref mut leds) = G_LED.borrow(cs).borrow_mut().as_mut() {
            leds.toggle(Color::Red);
        }
    });
}
