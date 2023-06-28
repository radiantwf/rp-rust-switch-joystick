use crate::hid;
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
    _timer: &mut hal::Timer,
    _delay: &mut cortex_m::delay::Delay,
) -> ! {
    _uart.write_full_blocking(b"UART Connected\r\n");

    let mut vec: Vec<u8, 10240> = Vec::new();
    let mut buffer = [0u8; 1024];
    let mut release_ticks: u64 = 0;
    loop {
        if release_ticks > 0 && release_ticks < _timer.get_counter().ticks() {
            hid::pro_controller::set_input_line("");
            release_ticks = 0;
        }
        let ret = _uart.read_raw(&mut buffer);
        match ret {
            Ok(size) => {
                vec.extend_from_slice(&buffer[..size]).unwrap();
                if buffer[size - 1] == b'\n' {
                    let buffer_str = core::str::from_utf8(&vec.as_slice()).unwrap();
                    hid::pro_controller::set_input_line(buffer_str);
                    release_ticks = _timer.get_counter().ticks() + 1_000_000 * 60;
                    vec.clear();
                    _delay.delay_ms(1);
                }
            }
            Err(_) => {
                _delay.delay_ms(1);
            }
        }
    }
}
