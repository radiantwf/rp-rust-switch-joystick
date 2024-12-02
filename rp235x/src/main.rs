#![no_std]
#![no_main]

mod hid;
mod r#uart;

use defmt::*;
use defmt_rtt as _;
use fugit::RateExtU32;
use rp235x_hal as hal;
use hal::dma::DMAExt;
use hal::multicore::{Multicore, Stack};
use hal::uart::{DataBits, StopBits, UartConfig};
use hal::Clock;
use hal::{
    // clocks::Clock,
    // uart::common_configs,
    clocks::{init_clocks_and_plls, ClockSource},
    entry,
    pac,
    usb::UsbBus,
    watchdog::Watchdog,
};
use panic_probe as _;

#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

static mut CORE1_STACK: Stack<4096> = Stack::new();

fn core1_task() -> ! {
    let mut pac = unsafe { pac::Peripherals::steal() };
    let core = unsafe { cortex_m::Peripherals::steal() };

    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    // Leaving this here so that clocks is used.
    let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.get_freq().to_Hz());

    let usb_bus = UsbBus::new(
        pac.USB,
        pac.USB_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    );

    hid::pro_controller::init(usb_bus, delay);
    hid::pro_controller::start();
}

#[entry]
fn main() -> ! {
    info!("Program start!");
    let mut pac = pac::Peripherals::take().unwrap();
    let _core = cortex_m::Peripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut sio = hal::sio::Sio::new(pac.SIO);

    let mut _delay =
        cortex_m::delay::Delay::new(_core.SYST, clocks.system_clock.get_freq().to_Hz());

    let mut mc = Multicore::new(&mut pac.PSM, &mut pac.PPB, &mut sio.fifo);
    let cores = mc.cores();
    let core1 = &mut cores[1];
    let _test = core1.spawn(unsafe { &mut CORE1_STACK.mem }, move || core1_task());

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let uart_pins = (
        // UART TX (characters sent from RP2040) on pin 1 (GPIO0)
        pins.gpio0.into_function::<hal::gpio::FunctionUart>(),
        // UART RX (characters received by RP2040) on pin 2 (GPIO1)
        pins.gpio1.into_function::<hal::gpio::FunctionUart>(),
    );

    let mut _uart = hal::uart::UartPeripheral::new(pac.UART0, uart_pins, &mut pac.RESETS)
        .enable(
            UartConfig::new(115200.Hz(), DataBits::Eight, None, StopBits::One),
            clocks.peripheral_clock.freq(),
        )
        .unwrap();
    let _dma = pac.DMA.split(&mut pac.RESETS);
    // let (_rx, _tx) = _uart.split();

    r#uart::run(_uart, _dma, _delay);
}
