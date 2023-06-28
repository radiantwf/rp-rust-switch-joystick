#![no_std]
#![no_main]

#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

mod hid;
mod r#uart;

use defmt::*;
use defmt_rtt as _;
use fugit::RateExtU32;
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
use rp_pico::hal;
pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

static mut CORE1_STACK: Stack<4096> = Stack::new();

fn core1_task() -> ! {
    let mut pac = unsafe { pac::Peripherals::steal() };
    let core = unsafe { pac::CorePeripherals::steal() };

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
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
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
    let _core = pac::CorePeripherals::take().unwrap();

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
    let mut _timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);

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
        pins.gpio4.into_mode::<hal::gpio::FunctionUart>(),
        // UART RX (characters received by RP2040) on pin 2 (GPIO1)
        pins.gpio5.into_mode::<hal::gpio::FunctionUart>(),
    );
    let mut _uart = hal::uart::UartPeripheral::new(pac.UART1, uart_pins, &mut pac.RESETS)
        .enable(
            UartConfig::new(9600.Hz(), DataBits::Eight, None, StopBits::One),
            clocks.peripheral_clock.freq(),
        )
        .unwrap();

    r#uart::run(&mut _uart, &mut _timer, &mut _delay);
}
