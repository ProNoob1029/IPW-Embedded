#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::InterruptHandler;
use embassy_rp::pwm::{ChannelAPin, ChannelBPin, Config as ConfigPwm, Pwm, Slice};
use embassy_rp::{bind_interrupts, Peripheral};
use embassy_time::{Duration, Timer};
#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

const PMW_TOP: u16 = 0x8000;

struct RgbLed<'d> {
    pwm_blue_green: Pwm<'d>,
    pwm_red: Pwm<'d>,
    config_blue_green: ConfigPwm,
    config_red: ConfigPwm
}

impl<'d> RgbLed<'d> {
    fn new<T: Slice, U: Slice>(
        slice_blue_green: impl Peripheral<P = T> + 'd,
        slice_red: impl Peripheral<P = U> + 'd,
        pin_blue: impl Peripheral<P = impl ChannelAPin<T>> + 'd,
        pin_green: impl Peripheral<P = impl ChannelBPin<T>> + 'd,
        pin_red: impl Peripheral<P = impl ChannelAPin<U>> + 'd,
    ) -> Self {
        let mut config: ConfigPwm = Default::default();
        config.top = PMW_TOP;
        config.compare_a = PMW_TOP / 2;
        config.compare_b = PMW_TOP / 2;

        RgbLed {
            pwm_blue_green: Pwm::new_output_ab(
                slice_blue_green,
                pin_blue,
                pin_green,
                config.clone()
            ),
            pwm_red: Pwm::new_output_a(
                slice_red,
                pin_red,
                config.clone()
            ),
            config_blue_green: config.clone(),
            config_red: config.clone()
        }
    }

    fn set_blue(&mut self, power: f32) {
        self.config_blue_green.compare_a = power_to_pmw(power);
        self.pwm_blue_green.set_config(&self.config_blue_green);
    }

    fn set_green(&mut self, power: f32) {
        self.config_blue_green.compare_b = power_to_pmw(power);
        self.pwm_blue_green.set_config(&self.config_blue_green);
    }

    fn set_red(&mut self, power: f32) {
        self.config_red.compare_a = power_to_pmw(power);
        self.pwm_red.set_config(&self.config_red);
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    let mut config: ConfigPwm = Default::default();
    config.top = PMW_TOP;
    config.compare_a = PMW_TOP / 2;
    config.compare_b = PMW_TOP / 2;


    let mut rgb: RgbLed = RgbLed::new(
        peripherals.PWM_SLICE0,
        peripherals.PWM_SLICE1,
        peripherals.PIN_0,
        peripherals.PIN_1,
        peripherals.PIN_2
    );

    let delay = Duration::from_millis(1000);

    loop {
        rgb.set_green(0.0);
        rgb.set_red(1.0);
        rgb.set_blue(1.0);
        info!("MAGENTA!");
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

fn power_to_pmw(power: f32) -> u16 {
    ((PMW_TOP as f32 * (1.0 - power)) as u16).clamp(0, PMW_TOP)
}