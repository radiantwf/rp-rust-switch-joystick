use hal::{
    pac,
    uart::{DataBits, StopBits, UartConfig},
    Clock,
};
use rp2040_hal as hal;

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

pub fn run(delay: &mut cortex_m::delay::Delay) -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
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

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // let uart_pins = (
    //     // UART TX
    //     pins.gpio4.into_function(),
    //     // UART RX
    //     pins.gpio5.into_function(),
    // );
    // let mut uart = hal::uart::UartPeripheral::new(pac.UART1, uart_pins, &mut pac.RESETS)
    //     .enable(
    //         UartConfig::new(9600.Hz(), DataBits::Eight, None, StopBits::One),
    //         clocks.peripheral_clock.freq(),
    //     )
    //     .unwrap();

    // uart.write_full_blocking(b"UART example\r\n");

    // let mut value = 0u32;
    loop {
        // writeln!(uart, "value: {value:02}\r").unwrap();
        // delay.delay_ms(1000);
        // value += 1
        delay.delay_ms(1000);
    }
}
