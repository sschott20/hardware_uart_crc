use core::fmt::Write;
// use stm32f4xx_hal::serial::Serial;

use stm32f4xx_hal::{
    hal::delay::DelayNs,
    pac::USART2,
    prelude::{_embedded_hal_serial_nb_Read, _embedded_hal_serial_nb_Write},
    serial::CommonPins,
    timer::SysDelay,
    uart::Serial,
};
pub const MESSAGE_SIZE: usize = 60;
pub const CHECKSUM_SIZE: usize = 4;
pub const PACKET_SIZE: usize = MESSAGE_SIZE + CHECKSUM_SIZE;
pub const FILE_SIZE: usize = 1000;
// ONLY WORKS FOR USART2!!
// Couldn't figure out how to make it generic, need a rust guru to help

pub struct UartCrc<'a> {
    serial: Serial<USART2>,
    delay: &'a mut SysDelay,
}

impl<'a> UartCrc<'a> {
    pub fn new(serial: Serial<USART2>, delay: &'a mut SysDelay) -> Self {
        Self { serial, delay }
    }
    pub fn send_binary(&mut self, binary: [u8; FILE_SIZE]) -> () {
        let mut current_read_size = 0;
        while current_read_size < FILE_SIZE {
            // hprintln!("current read size: {}", current_read_size);
            let mut data: [u8; PACKET_SIZE] = [0; PACKET_SIZE];
            for i in 0..MESSAGE_SIZE {
                if current_read_size + i >= FILE_SIZE {
                    break;
                }
                data[i] = binary[current_read_size + i];
            }
            let checksum = crc32fast::hash(&data[0..MESSAGE_SIZE]);
            for i in MESSAGE_SIZE..PACKET_SIZE {
                data[i] = checksum.to_le_bytes()[i - MESSAGE_SIZE];
            }

            for i in 0..PACKET_SIZE {
                self.serial.write(data[i] as u8).unwrap();
                self.delay.delay_ns(1_u32);
            }
            current_read_size += MESSAGE_SIZE;
        }
    }
}

#[derive()]
pub struct Packet {
    message: [u8; MESSAGE_SIZE as usize],
    checksum: u32,
}

impl Packet {
    pub fn new(message: [u8; MESSAGE_SIZE]) -> Self {
        let checksum = crc32fast::hash(&message);
        Self { message, checksum }
    }
    pub fn verify(&self) -> bool {
        self.checksum == crc32fast::hash(&self.message)
    }
    pub fn send(&self, serial: &mut Serial<USART2>, delay: &mut SysDelay) -> () {
        let mut data: [u8; MESSAGE_SIZE + CHECKSUM_SIZE as usize] =
            [0; MESSAGE_SIZE + CHECKSUM_SIZE as usize];
        data[..MESSAGE_SIZE].copy_from_slice(&self.message);
        data[MESSAGE_SIZE..].copy_from_slice(&self.checksum.to_le_bytes());

        for i in 0..MESSAGE_SIZE + CHECKSUM_SIZE as usize {
            serial.write(data[i]).unwrap();
            delay.delay_ns(1_u32);
        }
    }
}
