use core::fmt::Write;
use cortex_m::singleton;
use cortex_m::{delay::Delay, prelude::_embedded_hal_serial_Read};
use hal::{
    dma::Channels,
    gpio::{bank0, Function, Pin, Uart},
    pac::UART0,
    uart::{Enabled, UartPeripheral},
    Timer,
};
use rp2040_hal as hal;
// a201010000000000a3
// a200000080808080a3
use crate::hid;

const STX: &[u8] = b"\xA2";
const ETX: &[u8] = b"\xA3";
const BUFFER_LENGTH: usize = 7;

pub fn run(
    _uart: UartPeripheral<
        Enabled,
        UART0,
        (
            Pin<bank0::Gpio0, Function<Uart>>,
            Pin<bank0::Gpio1, Function<Uart>>,
        ),
    >,
    _dma: Channels,
    _timer: Timer,
    _delay: Delay,
) -> ! {
    let (_rx, mut _tx) = _uart.split();
    _tx.write_full_blocking(b"UART Connected\r\n");
    let rx_buf =
        singleton!(: [u8; BUFFER_LENGTH + STX.len() + ETX.len()] = [0; BUFFER_LENGTH + STX.len() + ETX.len()]).unwrap();
    let mut rx_transfer = hal::dma::single_buffer::Config::new(_dma.ch0, _rx, rx_buf).start();
    loop {
        let (ch0, mut rx, rx_buf) = rx_transfer.wait();

        let len_rx_buf = rx_buf.len();
        writeln!(_tx, "received {} bytes", len_rx_buf).unwrap();

        let mut misalignment = false;
        for (i, v) in STX.iter().enumerate() {
            if rx_buf[i] != *v {
                misalignment = true;
                break;
            }
        }
        for (i, v) in ETX.iter().enumerate() {
            if rx_buf[len_rx_buf - ETX.len() + i] != *v {
                misalignment = true;
                break;
            }
        }
        if misalignment {
            _tx.write_full_blocking(b"data misalignment\r\n");
            let mut _etx_index = 0;
            loop {
                let _result = rx.read();
                match _result {
                    Ok(v) => {
                        if v == ETX[_etx_index] {
                            _etx_index += 1;
                        } else {
                            _etx_index = 0;
                        }
                    }
                    Err(_) => {
                        _etx_index = 0;
                    }
                }
                if _etx_index == ETX.len() {
                    _tx.write_full_blocking(b"ok\r\n");
                    break;
                }
            }
        } else {
            let mut buf = [0u8; BUFFER_LENGTH];
            buf.clone_from_slice(&rx_buf[STX.len()..len_rx_buf - ETX.len()]);
            hid::pro_controller::set_input_uart_buffer(buf);
        }
        rx_transfer = hal::dma::single_buffer::Config::new(ch0, rx, rx_buf).start();
    }
}
