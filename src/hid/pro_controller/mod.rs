use cortex_m::delay::Delay;
use rp2040_hal::usb::UsbBus;
mod descriptor;
mod input;
mod pro_controller;
mod response;

pub fn init(usb_bus: UsbBus, delay: Delay) {
    pro_controller::init(usb_bus, delay);
}

pub fn start() -> ! {
    pro_controller::start();
}

pub fn set_input_line(input_line: &str) {
    pro_controller::set_input(input_line);
}

pub fn set_input_uart_buffer(uart_buffer: [u8; 7]) {
    pro_controller::set_input_uart_buffer(uart_buffer);
}
