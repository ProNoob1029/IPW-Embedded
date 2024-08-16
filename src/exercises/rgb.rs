use embassy_rp::pwm::{ChannelAPin, ChannelBPin, Config as ConfigPwm, Pwm, Slice};
use embassy_rp::Peripheral;

pub const PMW_TOP: u16 = 0x8000;

pub struct RgbLed<'d> {
    pwm_blue_green: Pwm<'d>,
    pwm_red: Pwm<'d>,
    config_blue_green: ConfigPwm,
    config_red: ConfigPwm
}

impl<'d> RgbLed<'d> {
    pub(crate) fn new<T: Slice, U: Slice>(
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

    pub fn set_blue(&mut self, power: f32) {
        self.config_blue_green.compare_a = power_to_pmw(power);
        self.pwm_blue_green.set_config(&self.config_blue_green);
    }

    pub fn set_green(&mut self, power: f32) {
        self.config_blue_green.compare_b = power_to_pmw(power);
        self.pwm_blue_green.set_config(&self.config_blue_green);
    }

    pub fn set_red(&mut self, power: f32) {
        self.config_red.compare_a = power_to_pmw(power);
        self.pwm_red.set_config(&self.config_red);
    }
}

fn power_to_pmw(power: f32) -> u16 {
    ((PMW_TOP as f32 * (1.0 - power)) as u16).clamp(0, PMW_TOP)
}