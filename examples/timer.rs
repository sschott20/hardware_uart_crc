#![no_std]
#![no_main]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod led;
use core::{convert::TryInto, fmt::Write};

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
use cortex_m::{
    interrupt,
    peripheral::{syst, Peripherals},
};
use cortex_m_rt::{entry, exception, interrupt};
use cortex_m_semihosting::hprintln;
use heapless::String;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let p = Peripherals::take().unwrap();

    // Configure the clocks on the board.
    let clocks: stm32f4xx_hal::rcc::Clocks = dp.RCC.constrain().cfgr.freeze();

    let gpiod = dp.GPIOD.split();
    // let mut delay = p.SYST.delay(&clocks);
    let mut syst = p.SYST;
    let mut delay = syst.delay(&clocks);
    syst.set_clock_source(syst::SystClkSource::Core);
    syst.enable_interrupt();
    syst.set_reload(1000); // 1s
    syst.clear_current();
    syst.enable_counter();

    let mut leds = Leds::new(gpiod);
    let mut i = 0;
    loop {
        hprintln!("{}", i);
        i += 1;
        // circle(&mut leds, &mut delay, 100);
    }
}

#[cortex_m_rt::exception]
fn SysTick() {
    hprintln!("SysTick Interrupt").unwrap();
}
