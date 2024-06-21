#![no_std]
#![no_main]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod uart;
mod led;
use core::{convert::TryInto, fmt::Write};

use crate::hal::prelude::*;
use uart::*;
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
use cortex_m::peripheral::{syst, Peripherals};
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use heapless::String;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let p = Peripherals::take().unwrap();

    // Configure the clocks on the board.
    let clocks: stm32f4xx_hal::rcc::Clocks = dp.RCC.constrain().cfgr.freeze();

    let gpiod = dp.GPIOD.split();
    let mut delay = p.SYST.delay(&clocks);

    let mut leds = Leds::new(gpiod);
    leds.on(Red);

    let gpioa = dp.GPIOA.split();
    let usart2_pins = (
        gpioa.pa2.into_alternate::<7>(),
        gpioa.pa3.into_alternate::<7>(),
    );

    // Initialize USART2. The baudrate is not meaningful on QEMU, but is
    // important on physical hardware.
    let mut usart2: Serial<_, u8> = dp
        .USART2
        .serial(
            usart2_pins,
            Config::default().baudrate(115200.bps()),
            // Config::default().baudrate(2400.bps()),
            &clocks,
        )
        .unwrap();

    let mut uart_crc = UartCrc::new(usart2, &mut delay);
    leds.on(Blue);

    let mut binary = [0; FILE_SIZE];
    for i in 0..FILE_SIZE {
        binary[i] = i as u8;
    }

    uart_crc.send_binary(binary);

    

    loop {
        circle(&mut leds, &mut delay, 100);
    }
}
