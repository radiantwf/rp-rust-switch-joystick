#![no_std]
#![no_main]

#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

mod hid;
mod r#macro;

use defmt::*;
use defmt_rtt as _;
use hal::multicore::{Multicore, Stack};
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

    let mut mc = Multicore::new(&mut pac.PSM, &mut pac.PPB, &mut sio.fifo);
    let cores = mc.cores();
    let core1 = &mut cores[1];
    let _test = core1.spawn(unsafe { &mut CORE1_STACK.mem }, move || core1_task());

    r#macro::run("test", &mut _delay);

    loop {
        _delay.delay_ms(1);
        // hid::pro_controller::set_input_line("A");
        // _delay.delay_ms(1);
        // hid::pro_controller::set_input_line("");
    }
}
