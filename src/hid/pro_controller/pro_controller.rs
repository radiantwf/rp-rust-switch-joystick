use crate::hid::pro_controller::input::ProControllerInput;
use crate::hid::pro_controller::response::get_response;
use crate::hid::pro_controller::response::Error;

use super::descriptor::SWITCH_PRO_DESCRIPTOR;
use cortex_m::delay::Delay;
use defmt::*;
use hal::pac;
use hal::pac::interrupt;
use hal::usb::UsbBus;
use hal::Timer;
use heapless::Vec;
use usb_device::{class_prelude::UsbBusAllocator, prelude::*};
// use usb_device::UsbError;
// use hal::pac::interrupt;
use rp_pico::hal;

use usbd_hid::hid_class::HIDClass;

static mut USB_DEV: Option<UsbDevice<UsbBus>> = None;
static mut USB_HID: Option<HIDClass<UsbBus>> = None;
static mut TIMER: Option<Timer> = None;
static mut DELAY: Option<Delay> = None;
static mut CONTROLLER_INPUT: Option<ProControllerInput> = None;
static mut CONNECTED: bool = false;
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

pub fn init(usb_bus: UsbBus, delay: Delay) {
    static mut ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
    unsafe {
        ALLOCATOR = Some(UsbBusAllocator::new(usb_bus));
    }
    let allocator = unsafe { ALLOCATOR.as_ref().unwrap() };

    let hid = HIDClass::new(allocator, SWITCH_PRO_DESCRIPTOR, 1);
    let dev = UsbDeviceBuilder::new(allocator, UsbVidPid(0x057E, 0x2009))
        .device_release(0x0200)
        .manufacturer("Nintendo Co., Ltd.")
        .product("Pro Controller")
        // .serial_number("000000000001")
        .serial_number("0740D13F26CA")
        .max_packet_size_0(64)
        .device_class(0x00)
        .device_sub_class(0x00)
        .device_protocol(0x00)
        .supports_remote_wakeup(true)
        // .self_powered(true)
        // // .max_power(0xFA)
        .build();

    let mut pac = unsafe { pac::Peripherals::steal() };
    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
    unsafe {
        USB_HID = Some(hid);
        USB_DEV = Some(dev);
        DELAY = Some(delay);
        TIMER = Some(timer);
        CONTROLLER_INPUT = Some(ProControllerInput::create_by_action(""));
    }
}

pub fn set_input(input_line: &str) {
    unsafe {
        CONTROLLER_INPUT = Some(ProControllerInput::create_by_action(input_line));
    }
}

pub fn set_input_uart_buffer(uart_buffer: [u8; 7]) {
    unsafe {
        CONTROLLER_INPUT = Some(ProControllerInput::create_by_uart_buffer(uart_buffer));
    }
}

pub fn send(buffer: &[u8]) -> Result<usize, UsbError> {
    let hid = unsafe { USB_HID.as_mut().unwrap() };
    hid.push_raw_input(buffer)
}

pub fn start() -> ! {
    unsafe {
        // Enable the USB interrupt
        pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
    }
    let _delay = unsafe { DELAY.as_mut().unwrap() };
    let timer = unsafe { TIMER.as_mut().unwrap() };
    info!("{}\tstart", timer.get_counter().ticks());
    loop {
        _delay.delay_us(50);
        if unsafe { CONNECTED } {
            let mut vec: Vec<u8, 64> = Vec::new();
            let input = unsafe { CONTROLLER_INPUT.as_mut().unwrap() };
            vec.push(0x30).unwrap();
            vec.push(
                ((timer.get_counter().ticks() / 1000) % 255)
                    .try_into()
                    .unwrap(),
            )
            .unwrap();
            vec.extend_from_slice(&input.buffer()).unwrap();
            let mut response: [u8; 64] = [0u8; 64];
            response[0..vec.len()].copy_from_slice(&vec[..]);
            match send(&response) {
                Ok(_count) => {
                    // info!("{}\tSend {}", timer.get_counter(), &response[.._count]);
                }
                Err(_err) => {
                    // info!("{}\tSend Data Error", timer.get_counter());
                }
            }
        }
    }
}

#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    let timer = TIMER.as_mut().unwrap();
    let dev = USB_DEV.as_mut().unwrap();
    let hid = USB_HID.as_mut().unwrap();
    if !dev.poll(&mut [hid]) {
        return;
    }

    let mut buffer: [u8; 64] = [0u8; 64];
    match hid.pull_raw_output(&mut buffer) {
        Ok(_count) => {
            // info!("{}\tRecv {}", timer.get_counter(), &buffer);
            let mut _status = 0;
            let mut _sent_len = 0;
            let mut response: [u8; 64] = [0u8; 64];
            let input = CONTROLLER_INPUT.as_mut().unwrap();

            match get_response(
                &buffer,
                &input.buffer(),
                ((timer.get_counter().ticks() / 1000) % 255)
                    .try_into()
                    .unwrap(),
                &mut response,
            ) {
                Ok((status, count)) => {
                    _sent_len = count;
                    _status = status;
                }
                Err(Error::InvalidDataError) => {
                    // info!("{}\tProcess Recv Data Error", timer.get_counter());
                }
                Err(_err) => {
                    info!("{}\tProcess Recv Data Error", timer.get_counter().ticks());
                }
            }
            match _status {
                1 => {
                    CONNECTED = true;
                    info!("Connected!!!");
                }
                2 => {
                    CONNECTED = true;
                    info!("Disconnected!!!");
                }
                _ => {}
            }

            if _sent_len <= 0 {
                return;
            }
            if _status == 0 {
                loop {
                    match send(&response) {
                        Ok(_count) => {
                            // info!("{}\tSend {}", timer.get_counter(), &response[.._count]);
                            break;
                        }
                        Err(_err) => {
                            // info!("{}\tSend Data Error", timer.get_counter());
                            continue;
                        }
                    }
                }
            }
        }
        Err(UsbError::WouldBlock) => {
            return;
        }
        Err(_err) => {
            error!("{}\tUSB ERROR", timer.get_counter().ticks());
            return;
        }
    }
}
