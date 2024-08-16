#![no_std]
#![no_main]

use core::cell::RefCell;
use defmt::*;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_executor::Spawner;
use embassy_rp::gpio::Level::{High, Low};
use embassy_rp::gpio::Output;
use embassy_rp::spi::{Config as SpiConfig, Spi};
use embassy_rp::spi::Phase::CaptureOnSecondTransition;
use embassy_rp::spi::Polarity::IdleHigh;
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_time::Delay;
use embedded_graphics::Drawable;
use embedded_graphics::mono_font::ascii::{FONT_10X20, FONT_7X13_BOLD};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use embedded_graphics::prelude::Point;
use embedded_graphics::text::Text;
use st7789::Orientation::Portrait;
use st7789::ST7789;
#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};
use ipw_embedded::display::SPIDeviceInterface;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    let miso = peripherals.PIN_4;
    let display_cs = peripherals.PIN_17;
    let mosi = peripherals.PIN_19;
    let clk = peripherals.PIN_18;
    let reset = peripherals.PIN_0;
    let dc = peripherals.PIN_16;

    let mut display_config = SpiConfig::default();

    display_config.frequency = 64_000_000;
    display_config.phase = CaptureOnSecondTransition;
    display_config.polarity = IdleHigh;

    let spi = Spi::new_blocking(
        peripherals.SPI0,
        clk,
        mosi,
        miso,
        display_config.clone()
    );

    let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));

    let display_spi = SpiDeviceWithConfig::new(
        &spi_bus,
        Output::new(display_cs, High),
        display_config
    );

    let dc = Output::new(dc, Low);
    let reset = Output::new(reset, Low);
    let di = SPIDeviceInterface::new(display_spi, dc);

    let mut display = ST7789::new(di, reset, 240, 240);
    display.init(&mut Delay).unwrap();
    display.set_orientation(Portrait).unwrap();

    use embedded_graphics::draw_target::DrawTarget;

    display.clear(Rgb565::BLACK).unwrap();

    let color = Rgb565::new(255, 255, 0);

    let style = MonoTextStyle::new(&FONT_10X20, color);

    Text::new("CLOOOOOOJ", Point::new(0, 60), style).draw(&mut display).unwrap();

    info!("wow");

    loop {

    }
}
