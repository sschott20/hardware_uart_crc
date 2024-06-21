#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use crate::hal::gpio::gpiod::PDn;
use crate::hal::gpio::gpiod::Parts;
use crate::hal::gpio::Output;
use crate::hal::gpio::Pin;
use crate::hal::gpio::PushPull;
use crate::hal::prelude::*;
use stm32f4xx_hal as hal;
use stm32f4xx_hal::gpio;
use stm32f4xx_hal::gpio::gpiod;
use stm32f4xx_hal::timer::SysDelay;
use Color::*;
pub enum Color {
    Red,
    Orange,
    Green,
    Blue,
}

pub struct Leds {
    pub leds: [Led; 4],
}
pub struct Led {
    pin: PDn<Output<PushPull>>,
}

pub fn circle(leds: &mut Leds, delay: &mut SysDelay, interval: u32) -> () {
    leds.on(Red);
    delay.delay_ms(interval);
    leds.on(Orange);
    delay.delay_ms(interval);
    leds.on(Green);
    delay.delay_ms(interval);
    leds.on(Blue);
    delay.delay_ms(interval);

    leds.off(Red);
    delay.delay_ms(interval);
    leds.off(Orange);
    delay.delay_ms(interval);
    leds.off(Green);
    delay.delay_ms(interval);
    leds.off(Blue);
    delay.delay_ms(interval);
}

impl Leds {
    pub fn new(gpiod: Parts) -> Self {
        let red = gpiod.pd13.into_push_pull_output();
        let orange = gpiod.pd14.into_push_pull_output();
        let green = gpiod.pd15.into_push_pull_output();
        let blue = gpiod.pd12.into_push_pull_output();
        Self {
            leds: [
                Led {
                    pin: orange.erase_number(),
                },
                Led {
                    pin: red.erase_number(),
                },
                Led {
                    pin: blue.erase_number(),
                },
                Led {
                    pin: green.erase_number(),
                },
            ],
        }
    }
    pub fn on(&mut self, color: Color) {
        match color {
            Color::Orange => self.leds[0].on(),
            Color::Red => self.leds[1].on(),
            Color::Blue => self.leds[2].on(),
            Color::Green => self.leds[3].on(),
        }
    }
    pub fn off(&mut self, color: Color) {
        match color {
            Color::Orange => self.leds[0].off(),
            Color::Red => self.leds[1].off(),
            Color::Blue => self.leds[2].off(),
            Color::Green => self.leds[3].off(),
        }
    }
}
impl Led {
    pub fn off(&mut self) {
        self.pin.set_low();
    }
    pub fn on(&mut self) {
        self.pin.set_high();
    }
    pub fn toggle(&mut self) {
        self.pin.toggle();
    }
}
