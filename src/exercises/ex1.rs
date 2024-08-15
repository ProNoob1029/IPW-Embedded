#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_futures::select::{select4, Either4};
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::InterruptHandler;
use embassy_rp::{bind_interrupts};
use embassy_rp::gpio::{Input, Pull};
use rgb::RgbLed;
#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};

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

    let mut button_a = Input::new(peripherals.PIN_12, Pull::Up);
    let mut button_b = Input::new(peripherals.PIN_13, Pull::Up);
    let mut button_x = Input::new(peripherals.PIN_14, Pull::Up);
    let mut button_y = Input::new(peripherals.PIN_15, Pull::Up);

    loop {
        let select_button = select4(
            button_a.wait_for_falling_edge(),
            button_b.wait_for_falling_edge(),
            button_x.wait_for_falling_edge(),
            button_y.wait_for_falling_edge()
        ).await;

        match select_button {
            Either4::First(_) => {
                rgb.set_green(0.2392);
                rgb.set_red(1.0);
                rgb.set_blue(0.8);
                info!("PINK!");
            }
            Either4::Second(_) => {
                rgb.set_red(0.0);
                rgb.set_blue(1.0);
                rgb.set_green(1.0);
                info!("CYAN!");
            }
            Either4::Third(_) => {
                rgb.set_blue(0.0);
                rgb.set_green(1.0);
                rgb.set_red(1.0);
                info!("YELLOW!");
            }
            Either4::Fourth(_) => {
                rgb.set_blue(1.0);
                rgb.set_green(1.0);
                rgb.set_red(1.0);
                info!("WHITE!");
            }
        }
    }
}

