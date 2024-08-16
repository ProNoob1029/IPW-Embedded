use embassy_rp::i2c::{Async, I2c};
use embassy_rp::peripherals::I2C0;

const AT24C256C_ADDR: u16 = 0x50;

pub trait Eeprom {
    async fn read_eeprom(&mut self, addr: u16, bytes: &mut [u8]);

    async fn write_eeprom(&mut self, addr: u16, bytes: impl IntoIterator<Item = u8>);
}

impl<'d> Eeprom for I2c<'d, I2C0, Async> {
    async fn read_eeprom(&mut self, addr: u16, bytes: &mut [u8]) {
        self.write_read_async(AT24C256C_ADDR, addr.to_be_bytes(), bytes).await.unwrap();
    }

    async fn write_eeprom(&mut self, addr: u16, bytes: impl IntoIterator<Item = u8>) {
        let addr_iter = addr.to_be_bytes().into_iter();
        let bytes_iter = bytes.into_iter();
        self.write_async(AT24C256C_ADDR, addr_iter.chain(bytes_iter)).await.unwrap();
    }
}