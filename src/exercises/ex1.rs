#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::InterruptHandler;
use embassy_rp::pwm::{ChannelAPin, ChannelBPin, Config as ConfigPwm, Slice};
use embassy_rp::{bind_interrupts, Peripheral};
use embassy_time::{Duration, Timer};
#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};
use rgb::{RgbLed, PMW_TOP};

mod rgb;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    let mut rgb: RgbLed = RgbLed::new(
        peripherals.PWM_SLICE0,
        peripherals.PWM_SLICE1,
        peripherals.PIN_0,
        peripherals.PIN_1,
        peripherals.PIN_2
    );

    let delay = Duration::from_millis(1000);

    loop {
        rgb.set_green(0.2392);
        rgb.set_red(1.0);
        rgb.set_blue(0.8);
        info!("PINK!");
        Timer::after(delay).await;

        rgb.set_red(0.0);
        rgb.set_blue(1.0);
        rgb.set_green(1.0);
        info!("CYAN!");
        Timer::after(delay).await;

        rgb.set_blue(0.0);
        rgb.set_green(1.0);
        rgb.set_red(1.0);
        info!("YELLOW!");
        Timer::after(delay).await;
    }
}

