use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_embedded_hal::SetConfig;
use embedded_hal_1::spi::SpiBus;
use embassy_sync::blocking_mutex::Mutex;
use core::cell::RefCell;
use embassy_rp::spi::{Blocking, ClkPin, Config as SpiConfig, MisoPin, MosiPin, Spi};
use embassy_rp::Peripheral;
use embassy_rp::gpio::{Output, Pin};
use st7789::ST7789;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_rp::gpio::Level::{High, Low};
use embassy_time::Delay;
use ipw_embedded::display::SPIDeviceInterface;
use st7789::Orientation::Portrait;
use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use embassy_rp::spi::Phase::CaptureOnSecondTransition;
use embassy_rp::spi::Polarity::IdleHigh;
use embedded_graphics::draw_target::DrawTarget;

pub fn init_spi<'d, T: embassy_rp::spi::Instance>(
    inner: impl Peripheral<P = T> + 'd,
    clk: impl Peripheral<P = impl ClkPin<T> + 'd> + 'd,
    mosi: impl Peripheral<P = impl MosiPin<T> + 'd> + 'd,
    miso: impl Peripheral<P = impl MisoPin<T> + 'd> + 'd,
) -> (Spi<'d, T, Blocking>, SpiConfig) {
    let mut spi_config = SpiConfig::default();

    spi_config.frequency = 64_000_000;
    spi_config.phase = CaptureOnSecondTransition;
    spi_config.polarity = IdleHigh;

    let spi = Spi::new_blocking(
        inner,
        clk,
        mosi,
        miso,
        spi_config.clone()
    );

    (spi, spi_config)
}

// lmao ipw 2024
pub fn init_display<'d, M: RawMutex, BUS: SetConfig<Config = embassy_rp::spi::Config> + SpiBus>(
    bus: &'d Mutex<M, RefCell<BUS>>,
    config: SpiConfig,
    display_cs: impl Peripheral<P = impl Pin> + 'd,
    rst: impl Peripheral<P = impl Pin> + 'd,
    dc: impl Peripheral<P = impl Pin> + 'd
) -> ST7789<SPIDeviceInterface<SpiDeviceWithConfig<'d, M, BUS, Output<'d>>, Output<'d>>, Output<'d>> {
    let display_cs = Output::new(display_cs, High);

    let display_spi = SpiDeviceWithConfig::new(
        bus,
        display_cs,
        config
    );

    let dc = Output::new(dc, Low);
    let rst = Output::new(rst, Low);
    let di = SPIDeviceInterface::new(display_spi, dc);

    let mut display = ST7789::new(di, rst, 240, 240);
    display.init(&mut Delay).unwrap();
    display.set_orientation(Portrait).unwrap();

    display.clear(Rgb565::BLACK).unwrap();

    display
}