#![no_std]
#![no_main]

use defmt::*;
use eeprom::Eeprom;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::i2c::{Config as I2cConfig, I2c, InterruptHandler as I2CInterruptHandler};
use embassy_rp::peripherals::I2C0;
#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};

mod eeprom;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    let sda = peripherals.PIN_20;
    let scl = peripherals.PIN_21;

    let mut i2c = I2c::new_async(peripherals.I2C0, scl, sda, Irqs, I2cConfig::default());

    let mut read_buf = [0x00; 3];

    //i2c.write_eeprom(1, [68]).await;

    i2c.read_eeprom(0, &mut read_buf).await;

    info!("Read value {}", read_buf[1]);

    loop {

    }
}

/*async fn scan_i2c<'d>(i2c: &mut I2c<'d, I2C0, Async>) {
    let mut rx_buf = [0x00u8; 2];

    info!("Scanning addresses");

    for addr in 0..127_u16 {
        match i2c.read_async(addr, &mut rx_buf).await {
            Ok(_) => {
                info!("Successful read on address 0x{:02x}", addr)
            }
            Err(_) => {}
        }
    }
}*/

bind_interrupts!(struct Irqs {
    I2C0_IRQ => I2CInterruptHandler<I2C0>;
});