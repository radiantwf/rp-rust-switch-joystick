use core::fmt::Write;
use hal::{
    gpio::{bank0, Function, Pin, Uart},
    pac::UART1,
    uart::{Enabled, UartPeripheral},
};
use rp2040_hal as hal;

use crate::hid;

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
    _uart.write_full_blocking(b"UART Connected\r\n");

    loop {
        let mut buffer = [0u8; 1024];
        let ret = _uart.read_raw(&mut buffer);
        match ret {
            Ok(_) => {
                let buffer_str = core::str::from_utf8(&buffer).unwrap();
                hid::pro_controller::set_input_line(buffer_str);
                writeln!(_uart, "recv: {buffer_str}\r").unwrap();
                delay.delay_ms(10);
            }
            Err(_) => {
                delay.delay_ms(1);
            }
        }
    }
}
