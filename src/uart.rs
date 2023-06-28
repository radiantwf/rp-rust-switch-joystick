use core::fmt::Write;
use hal::{
    gpio::{bank0, Function, Pin, Uart},
    pac::UART1,
    uart::{Enabled, UartPeripheral},
};
use rp2040_hal as hal;

pub fn run(
    _uart: &mut UartPeripheral<
        Enabled,
        UART1,
        (
            Pin<bank0::Gpio4, Function<Uart>>,
            Pin<bank0::Gpio5, Function<Uart>>,
        ),
    >,
    delay: &mut cortex_m::delay::Delay,
) -> ! {
    _uart.write_full_blocking(b"UART example\r\n");

    let mut value = 0u32;
    loop {
        writeln!(_uart, "value: {value:02}\r").unwrap();
        delay.delay_ms(1000);
        value += 1
        // delay.delay_ms(1);
        // hid::pro_controller::set_input_line("A");
        // delay.delay_ms(1);
        // hid::pro_controller::set_input_line("");
    }
}
