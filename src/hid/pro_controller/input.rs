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
    buffer: [u8; 11],
}
impl ProControllerInput {
    pub fn create_by_action(_action_line: &str) -> Self {
        let mut action_line: String<1024> = String::new();
        match String::from_str(_action_line.trim()) {
            Ok(_line) => action_line = _line,
            Err(_) => {}
        }

        let mut _buffer = [0u8; 11];
        _buffer[0] = 0x81;
        _buffer[10] = 0x00;

        let mut lx: u32 = 0x800;
        let mut ly: u32 = 0x800;
        let mut rx: u32 = 0x800;
        let mut ry: u32 = 0x800;

        let mut iter = action_line.split("|");
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
        _buffer[4] = lx as u8 & 0xff;
        _buffer[5] = ((lx >> 8) as u8 & 0x0f) | ((ly as u8 & 0x0f) << 4);
        _buffer[6] = (ly >> 4) as u8 & 0xff;
        _buffer[7] = rx as u8 & 0xff;
        _buffer[8] = ((rx >> 8) as u8 & 0x0f) | ((ry as u8 & 0x0f) << 4);
        _buffer[9] = (ry >> 4) as u8 & 0xff;
        ProControllerInput { buffer: _buffer }
    }

    pub fn create_by_uart_buffer(uart_buffer: [u8; 7]) -> Self {
        let mut _buffer = [0u8; 11];
        _buffer[0] = 0x81;
        _buffer[10] = 0x00;

        _buffer[1] = uart_buffer[0];
        _buffer[2] = uart_buffer[1];
        _buffer[3] = uart_buffer[2];

        let _coord =
            ProControllerInput::_coordinate_bytes_convert([uart_buffer[3], uart_buffer[4]]);
        let lx = _coord.0;
        let ly = _coord.1;
        let _coord =
            ProControllerInput::_coordinate_bytes_convert([uart_buffer[5], uart_buffer[6]]);
        let rx = _coord.0;
        let ry = _coord.1;
        _buffer[4] = lx as u8 & 0xff;
        _buffer[5] = ((lx >> 8) as u8 & 0x0f) | ((ly as u8 & 0x0f) << 4);
        _buffer[6] = (ly >> 4) as u8 & 0xff;
        _buffer[7] = rx as u8 & 0xff;
        _buffer[8] = ((rx >> 8) as u8 & 0x0f) | ((ry as u8 & 0x0f) << 4);
        _buffer[9] = (ry >> 4) as u8 & 0xff;
        ProControllerInput { buffer: _buffer }
    }

    pub fn buffer(&self) -> Vec<u8, 11> {
        Vec::from_slice(&self.buffer[..]).unwrap()
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
        let _x = bytes[0] as i32 - 0x80;
        let _y = bytes[1] as i32 - 0x80;
        let x = ((_x + 128) * 16) as u32;
        let y = ((_y * (-1) + 128) * 16) as u32;
        return (x, y);
    }
}
