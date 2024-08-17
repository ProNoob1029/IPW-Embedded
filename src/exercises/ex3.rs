#![no_std]
#![no_main]

use core::cell::RefCell;
use defmt::*;
use embassy_embedded_hal::SetConfig;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_executor::Spawner;
use embassy_rp::gpio::Level::{High, Low};
use embassy_rp::gpio::{Output, Pin};
use embassy_rp::Peripheral;
use embassy_rp::spi::Phase::CaptureOnSecondTransition;
use embassy_rp::spi::Polarity::IdleHigh;
use embassy_rp::spi::{Blocking, ClkPin, Config as SpiConfig, MisoPin, MosiPin, Spi};
use embassy_sync::blocking_mutex::raw::{NoopRawMutex, RawMutex};
use embassy_sync::blocking_mutex::Mutex;
use embassy_time::{Delay, Duration, Timer};
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use embedded_graphics::prelude::Point;
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;
use embedded_hal_1::spi::SpiBus;
use ipw_embedded::display::SPIDeviceInterface;
use st7789::Orientation::Portrait;
use st7789::ST7789;
#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};

mod display_init;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    let (spi, spi_config) = display_init::init_spi(
        peripherals.SPI0,
        peripherals.PIN_18,
        peripherals.PIN_19,
        peripherals.PIN_4
    );

    let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));

    let mut display = display_init::init_display(
        &spi_bus,
        spi_config,
        peripherals.PIN_17,
        peripherals.PIN_0,
        peripherals.PIN_16
    );

    let color = Rgb565::new(255, 255, 0);

    let style = MonoTextStyle::new(&FONT_10X20, color);

    let text = Text::new("CLOOOOOOJ", Point::new(0, 60), style);

    let delay = Duration::from_millis(500);

    loop {
        text.draw(&mut display).unwrap();
        info!("CLOOOOJ");
        Timer::after(delay).await;

        display.clear(Rgb565::BLACK).unwrap();
        Timer::after(delay).await;
    }
}
