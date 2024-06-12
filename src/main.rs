#![no_std]
#![no_main]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod crc;
mod led;
use core::{convert::TryInto, fmt::Write};

use crate::hal::prelude::*;
use crc::{Packet, MESSAGE_LENGTH};
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

    // Take the pins used for USART1.
    // PA9 is for TX, PA10 is for RX.
    // They should be set to mode alternative function 7.
    // See STM32F405 datasheet for details.
    // https://www.st.com/resource/en/datasheet/stm32f405rg.pdf

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

    leds.on(Blue);

    let mut message: [u8; 60] = [0; 60];
    for i in 0..MESSAGE_LENGTH {
        message[i] = i as u8;
    }
    let data = Packet::new(message);
    // usart2.read().unwrap();
    data.send(&mut usart2, &mut delay);

    let mut received: [u8; MESSAGE_LENGTH] = [0; MESSAGE_LENGTH];

    for i in 0..MESSAGE_LENGTH {
        received[i] = block!(usart2.read()).unwrap();
    }
    for i in 0..MESSAGE_LENGTH {
        hprintln!("Received: {}", received[i]).unwrap();
    }
    loop {
        circle(&mut leds, &mut delay, 100);
    }
}
