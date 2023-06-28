use crate::hid;
use core::fmt::Write;
use hal::{
    gpio::{bank0, Function, Pin, Uart},
    pac::UART1,
    uart::{Enabled, UartPeripheral},
};
use heapless::Vec;
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
    _uart.write_full_blocking(b"UART Connected\r\n");

    let mut vec: Vec<u8, 10240> = Vec::new();
    let mut buffer = [0u8; 1024];
    loop {
        let ret = _uart.read_raw(&mut buffer);
        match ret {
            Ok(size) => {
                vec.extend_from_slice(&buffer[..size]).unwrap();
                if buffer[size - 1] == b'\n' {
                    let buffer_str = core::str::from_utf8(&vec.as_slice()).unwrap();
                    hid::pro_controller::set_input_line(buffer_str);
                    vec.clear();
                    delay.delay_ms(1);
                }
            }
            Err(_) => {
                delay.delay_ms(1);
            }
        }
    }
}
