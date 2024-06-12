use core::fmt::Write;

use stm32f4xx_hal::{
    hal::delay::DelayNs, pac::USART2, prelude::_embedded_hal_serial_nb_Write, timer::SysDelay,
    uart::Serial,
};
pub const MESSAGE_LENGTH: usize = 60;
pub const CHECKSUM_LENGTH: usize = 4;
pub const PACKET_LENGTH: usize = MESSAGE_LENGTH + CHECKSUM_LENGTH;

#[derive()]
pub struct Packet {
    message: [u8; MESSAGE_LENGTH as usize],
    checksum: u32,
}

impl Packet {
    pub fn new(message: [u8; MESSAGE_LENGTH]) -> Self {
        let checksum = crc32fast::hash(&message);
        Self { message, checksum }
    }
    pub fn verify(&self) -> bool {
        self.checksum == crc32fast::hash(&self.message)
    }
    pub fn send(&self, serial: &mut Serial<USART2>, delay: &mut SysDelay) -> () {
        let mut data: [u8; MESSAGE_LENGTH + CHECKSUM_LENGTH as usize] =
            [0; MESSAGE_LENGTH + CHECKSUM_LENGTH as usize];
        data[..MESSAGE_LENGTH].copy_from_slice(&self.message);
        data[MESSAGE_LENGTH..].copy_from_slice(&self.checksum.to_le_bytes());

        for i in 0..MESSAGE_LENGTH + CHECKSUM_LENGTH as usize {
            serial.write(data[i]).unwrap();
            // delay.delay_ms(1_u32);
            delay.delay_ns(1_u32);
        }
    }
}
