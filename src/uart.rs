use cortex_m::singleton;
use cortex_m::{delay::Delay, prelude::_embedded_hal_serial_Read};
use hal::{
    dma::Channels,
    gpio::{bank0, Function, Pin, Uart},
    pac::UART1,
    uart::{Enabled, UartPeripheral},
    Timer,
};
use rp2040_hal as hal;
// a20201010000000000a303
use crate::hid;

const STX: [u8; 2] = [0xA2, 0x02];
const ETX: [u8; 2] = [0xA3, 0x03];

pub fn run(
    _uart: UartPeripheral<
        Enabled,
        UART1,
        (
            Pin<bank0::Gpio4, Function<Uart>>,
            Pin<bank0::Gpio5, Function<Uart>>,
        ),
    >,
    _dma: Channels,
    _timer: Timer,
    _delay: Delay,
) -> ! {
    let (_rx, _tx) = _uart.split();
    _tx.write_full_blocking(b"UART Connected\r\n");
    let rx_buf = singleton!(: [u8; 11] = [0; 11]).unwrap();
    let mut rx_transfer = hal::dma::single_buffer::Config::new(_dma.ch0, _rx, rx_buf).start();
    loop {
        let (ch0, mut rx, rx_buf) = rx_transfer.wait();

        let len_rx_buf = rx_buf.len();
        let mut misalignment = false;
        for (i, v) in STX.iter().enumerate() {
            if rx_buf[i] != *v {
                misalignment = true;
                break;
            }
        }
        for (i, v) in ETX.iter().enumerate() {
            if rx_buf[len_rx_buf - 2 + i] != *v {
                misalignment = true;
                break;
            }
        }
        if misalignment {
            let mut _etx_index = 0;
            loop {
                let _result = rx.read();
                match _result {
                    Ok(v) => {
                        if v == ETX[_etx_index] {
                            _etx_index += 1;
                            if _etx_index == ETX.len() {
                                break;
                            }
                        } else {
                            _etx_index = 0;
                        }
                    }
                    Err(_) => {
                        _etx_index = 0;
                    }
                }
            }
        } else {
            let mut buf = [0u8; 7];
            buf.clone_from_slice(&rx_buf[2..9]);
            hid::pro_controller::set_input_uart_buffer(buf);
        }

        rx_transfer = hal::dma::single_buffer::Config::new(ch0, rx, rx_buf).start();
    }
}
