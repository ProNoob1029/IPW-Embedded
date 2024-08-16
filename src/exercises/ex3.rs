#![no_std]
#![no_main]

use defmt::*;
use eeprom::Eeprom;
use embassy_executor::Spawner;
#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};

mod eeprom;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    loop {

    }
}
