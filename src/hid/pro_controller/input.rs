use core::str::FromStr;

use heapless::{String, Vec};
const _INPUT0_Y: u8 = 0b1;
const _INPUT0_X: u8 = 0b10;
const _INPUT0_B: u8 = 0b100;
const _INPUT0_A: u8 = 0b1000;

const _INPUT0_JCL_SR: u8 = 0b10000;
const _INPUT0_JCL_SL: u8 = 0b100000;
const _INPUT0_R: u8 = 0b1000000;
const _INPUT0_ZR: u8 = 0b10000000;

const _INPUT1_MINUS: u8 = 0b1;
const _INPUT1_PLUS: u8 = 0b10;
const _INPUT1_RPRESS: u8 = 0b100;
const _INPUT1_LPRESS: u8 = 0b1000;
const _INPUT1_HOME: u8 = 0b10000;
const _INPUT1_CAPTURE: u8 = 0b100000;

const _INPUT2_BOTTOM: u8 = 0b1;
const _INPUT2_TOP: u8 = 0b10;
const _INPUT2_RIGHT: u8 = 0b100;
const _INPUT2_LEFT: u8 = 0b1000;
const _INPUT2_JCR_SR: u8 = 0b10000;
const _INPUT2_JCR_SL: u8 = 0b100000;
const _INPUT2_L: u8 = 0b1000000;
const _INPUT2_ZL: u8 = 0b10000000;

#[derive(Debug)]
pub struct ProControllerInput {
    action_line: Option<String<1024>>,
    uart_action_buffer: Option<[u8; 7]>,
}
impl ProControllerInput {
    pub fn create_by_action(_action_line: &str) -> Self {
        match String::from_str(_action_line.trim()) {
            Ok(_line) => ProControllerInput {
                action_line: Some(_line),
                uart_action_buffer: None,
            },
            Err(_) => ProControllerInput {
                action_line: Some(String::new()),
                uart_action_buffer: None,
            },
        }
    }

    pub fn create_by_uart_buffer(uart_buffer: [u8; 7]) -> Self {
        ProControllerInput {
            action_line: None,
            uart_action_buffer: Some(uart_buffer),
        }
    }

    pub fn buffer(&self) -> Vec<u8, 11> {
        let _buffer = &mut [0u8; 11];
        _buffer[0] = 0x81;
        _buffer[10] = 0x00;

        let mut lx: u32 = 0x800;
        let mut ly: u32 = 0x800;
        let mut rx: u32 = 0x800;
        let mut ry: u32 = 0x800;

        match &self.action_line {
            Some(line) => {
                let mut iter = line.split("|");
                loop {
                    match iter.next() {
                        None => break,
                        Some(_action) => {
                            let mut upper_action: String<100> = String::from(_action);
                            upper_action.make_ascii_uppercase();

                            match &upper_action[..] {
                                "Y" => _buffer[1] |= _INPUT0_Y,
                                "X" => _buffer[1] |= _INPUT0_X,
                                "B" => _buffer[1] |= _INPUT0_B,
                                "A" => _buffer[1] |= _INPUT0_A,
                                "JCL_SR" => _buffer[1] |= _INPUT0_JCL_SR,
                                "JCL_SL" => _buffer[1] |= _INPUT0_JCL_SL,
                                "R" => _buffer[1] |= _INPUT0_R,
                                "ZR" => _buffer[1] |= _INPUT0_ZR,

                                "MINUS" => _buffer[2] |= _INPUT1_MINUS,
                                "PLUS" => _buffer[2] |= _INPUT1_PLUS,
                                "LPRESS" => _buffer[2] |= _INPUT1_RPRESS,
                                "RPRESS" => _buffer[2] |= _INPUT1_LPRESS,
                                "HOME" => _buffer[2] |= _INPUT1_HOME,
                                "CAPTURE" => _buffer[2] |= _INPUT1_CAPTURE,

                                "BOTTOM" => _buffer[3] |= _INPUT2_BOTTOM,
                                "TOP" => _buffer[3] |= _INPUT2_TOP,
                                "RIGHT" => _buffer[3] |= _INPUT2_RIGHT,
                                "LEFT" => _buffer[3] |= _INPUT2_LEFT,
                                "JCR_SR" => _buffer[3] |= _INPUT2_JCR_SR,
                                "JCR_SL" => _buffer[3] |= _INPUT2_JCR_SL,
                                "L" => _buffer[3] |= _INPUT2_L,
                                "ZL" => _buffer[3] |= _INPUT2_ZL,

                                _action => {
                                    if _action.starts_with("LSTICK@") {
                                        let _coord = ProControllerInput::_coordinate_str_convert(
                                            &_action["LSTICK@".len()..],
                                        );
                                        lx = _coord.0;
                                        ly = _coord.1;
                                    } else if _action.starts_with("RSTICK@") {
                                        let _coord = ProControllerInput::_coordinate_str_convert(
                                            &_action["RSTICK@".len()..],
                                        );
                                        rx = _coord.0;
                                        ry = _coord.1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            None => {}
        }
        match self.uart_action_buffer {
            Some(uart_buffer) => {
                _buffer[1] = uart_buffer[0];
                _buffer[2] = uart_buffer[1];
                _buffer[3] = uart_buffer[2];

                let _coord =
                    ProControllerInput::_coordinate_bytes_convert([uart_buffer[3], uart_buffer[4]]);
                lx = _coord.0;
                ly = _coord.1;
                let _coord =
                    ProControllerInput::_coordinate_bytes_convert([uart_buffer[5], uart_buffer[6]]);
                rx = _coord.0;
                ry = _coord.1;
            }
            None => {}
        }
        _buffer[4] = lx as u8 & 0xff;
        _buffer[5] = ((lx >> 8) as u8 & 0x0f) | ((ly as u8 & 0x0f) << 4);
        _buffer[6] = (ly >> 4) as u8 & 0xff;
        _buffer[7] = rx as u8 & 0xff;
        _buffer[8] = ((rx >> 8) as u8 & 0x0f) | ((ry as u8 & 0x0f) << 4);
        _buffer[9] = (ry >> 4) as u8 & 0xff;
        Vec::from_slice(&_buffer[..]).unwrap()
    }

    fn _coordinate_str_convert(_coord: &str) -> (u32, u32) {
        let mut x = 0x800;
        let mut y = 0x800;
        let _coord: Vec<&str, 100> = _coord.split(",").collect();
        if _coord.len() != 2 {
            return (x, y);
        }
        let mut _x = 0;
        let mut _y = 0;
        match _coord[0].parse::<i32>() {
            Ok(_value) => {
                if _value > 127 {
                    _x = 127;
                } else if _value < -128 {
                    _x = -128;
                } else {
                    _x = _value
                }
            }
            Err(_) => {}
        }
        match _coord[1].parse::<i32>() {
            Ok(_value) => {
                if _value > 127 {
                    _y = 127;
                } else if _value < -128 {
                    _y = -128;
                } else {
                    _y = _value
                }
            }
            Err(_) => {}
        }

        x = ((_x + 128) * 16) as u32;
        y = ((_y * (-1) + 128) * 16) as u32;
        return (x, y);
    }

    fn _coordinate_bytes_convert(bytes: [u8; 2]) -> (u32, u32) {
        let mut _x = bytes[0] as i32 - 0x80;
        let mut _y = bytes[1] as i32 - 0x80;
        (_x, _y) = ProControllerInput::_get_point_in_circle(_x, _y);
        let x = ((_x + 0x80) * 16) as u32;
        let y = ((_y * (-1) + 0x80) * 16) as u32;
        return (x, y);
    }
    fn _get_point_in_circle(x: i32, y: i32) -> (i32, i32) {
        let (intersect_x, intersect_y) = ProControllerInput::_get_intersection(x, y);

        let theta = libm::atan2f(intersect_y as f32, intersect_x as f32);
        let x_cartesian = libm::cosf(theta) * 127.0;
        let y_cartesian = libm::sinf(theta) * 127.0;

        let x = libm::roundf(intersect_x.abs() as f32 * x_cartesian / 127.0) as i32;
        let y = libm::roundf(intersect_y.abs() as f32 * y_cartesian / 127.0) as i32;
        return (x, y);
    }

    fn _get_intersection(x: i32, y: i32) -> (i32, i32) {
        if x >= -127 && x <= 127 && y >= -127 && y <= 127 {
            return (x, y);
        }
        let slope = y as f32 / x as f32;
        let mut x_intersect = 0;
        let mut y_intersect = 0;
        if x < -127 {
            x_intersect = -127;
            y_intersect = (slope * ((x_intersect - x) as f32) + y as f32) as i32;
        } else if x > 127 {
            x_intersect = 127;
            y_intersect = (slope * ((x_intersect - x) as f32) + y as f32) as i32;
        }
        if y < -127 {
            y_intersect = -127;
            x_intersect = (((y_intersect - y) as f32) / slope + x as f32) as i32;
        } else if y > 127 {
            y_intersect = 127;
            x_intersect = (((y_intersect - y) as f32) / slope + x as f32) as i32;
        }
        (x_intersect, y_intersect)
    }
}
