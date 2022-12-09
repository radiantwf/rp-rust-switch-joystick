// use defmt::*;
use heapless::Vec;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidDataError,
    // BufferOverflow,
    UnknownError,
}

impl Default for Error {
    fn default() -> Self {
        Self::UnknownError
    }
}
pub type Result<T> = core::result::Result<T, Error>;

// Recv 0x01
// Recv 0x01 0x10 ......
// spi flash notes
// https://github.com/dekuNukem/Nintendo_Switch_Reverse_Engineering/blob/master/spi_flash_notes.md

// Section Range	Subsection Range	Exits	Remarks
// x6000-x600F		-------------		+		Serial number in non-extended ASCII. If first byte is >= x80, no S/N. If a byte is 00 NUL, skip. Max 15 chars, if 16 chars last one is skipped.
// x6012			-------------		-		Device type. JC (L): x01, JC (R): x02, Pro: x03. Only the 3 LSB are accounted for. Used internally and for x02 subcmd.
// x6013			-------------		-		Unknown, seems to always be xA0
// x601B			-------------		-		Color info exists if x01. If 0, default colors used are ARGB #55555555, #FFFFFFFF. Used for x02 subcmd.
// x6020-x6037		-------------		+		Factory configuration & calibration 1
// 					x6020 - x6037		+		6-Axis motion sensor Factory calibration
// x603D-x6055		-------------		+		Factory configuration & calibration 2
// 					x603D - x6045		+		Left analog stick calibration
// 					x6046 - x604E		+		Right analog stick calibration
// 					x6050 - x6052		+		Body #RGB color, 24-bit
// 					x6053 - x6055		+		Buttons #RGB color, 24-bit
// 					x6056 - x6058		+		Left Grip #RGB color, 24-bit (Added in 5.0.0 for Pro)
// 					x6059 - x605B		+		Right Grip #RGB color, 24-bit (Added in 5.0.0 for Pro)
// x6080-x6097		-------------		+		Factory Sensor and Stick device parameters
// 					x6080-x6085			+		6-Axis Horizontal Offsets. (JC sideways)
// 					x6086-x6097			+		Stick device parameters 1
// x6098-x60A9		-------------		+		Factory Stick device parameters 2
// 					x6098-x60A9			+		Stick device parameters 2. Normally the same with 1, even in Pro Contr.
// x6E00-x6EFF		-------------		-		Unknown data values.. Exists only in Joy-Con
const SWITCH_PRO_SPI_MAP_0X60: &[u8] = &[
    // 00 ~ 0F
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
    // 10 ~ 1F
    0xff, 0xff, 0x03, 0xa0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    // 20 ~ 2F
    0x09, 0x01, 0x18, 0xFF, 0xED, 0xFF, 0x00, 0x40, 0x00, 0x40, 0x00, 0x40, 0xFA, 0xFF, 0xD0, 0xFF,
    // 30 ~ 3F
    0xC7, 0xFF, 0x3B, 0x34, 0x3B, 0x34, 0x3B, 0x34, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    // 40 ~ 4F
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    // 50 ~ 5F
    0xbc, 0x11, 0x42, 0x75, 0xa9, 0x28, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    // 60 ~ 6F
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    // 70 ~ 7F
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    // 80 ~ 8F
    0x50, 0xFD, 0x00, 0x00, 0xC6, 0x0F, 0x00, 0x40, 0x00, 0x40, 0x00, 0x40, 0xFA, 0xFF, 0xD0, 0xFF,
    // 90 ~ 9F
    0xC7, 0xFF, 0x3B, 0x34, 0x3B, 0x34, 0x3B, 0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    // A0 ~ AF
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];

// Section Range	Subsection Range	Exits	Remarks
// x8010-x8025		-------------		-		User Analog sticks calibration
// 					x8010, x8011		-		Magic xB2 xA1 for user available calibration
// 					x8012 - x801A		-		Actual User Left Stick Calibration data
// 					x801B, x801C		-		Magic xB2 xA1 for user available calibration
// 					x801D - x8025		-		Actual user Right Stick Calibration data
// x8026-x803F		-------------		-		User 6-Axis Motion Sensor calibration
// 					x8026, x8027		-		Magic xB2 xA1 for user available calibration
// 					x8028-x803F			-		Actual 6-Axis Motion Sensor Calibration data
const SWITCH_PRO_SPI_MAP_0X80: &[u8] = &[
    // 00 ~ 0F
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    // 10 ~ 1F
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    // 20 ~ 2F
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    // 30 ~ 3F
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];

// Recv 0x01 ......
// bluetooth hid subcommands notes
// https://github.com/dekuNukem/Nintendo_Switch_Reverse_Engineering/blob/master/bluetooth_hid_subcommands_notes.md
const SWITCH_PRO_SUBCOMMAND_0X00: &[u8] = &[0x81, 0x00, 0x03];
const SWITCH_PRO_SUBCOMMAND_0X01: &[u8] = &[0x81, 0x01, 0x03];
const SWITCH_PRO_SUBCOMMAND_0X02: &[u8] = &[
    0x82, 0x02, //
    0x04, 0x21, // 0-1 Firmware version (Eg: 4.33)
    0x03, // 2 Controller ID (1 = Joy-Con (L), 2 = Joy-Con (R), 3 = Pro Controller)
    0x02, // 3 Unknown, always 02 (maybe?)
    0x00, 0x00, 0x5e, 0x00, 0x53, 0x5e, // 4-9 Controller Bluetooth MAC address
    0x01, // 10 Unknown, always 01 (maybe?)
    0x01, // 11 If 01, colors in SPI used for Controller color
];
const SWITCH_PRO_SUBCOMMAND_0X03: &[u8] = &[0x80, 0x03];
const SWITCH_PRO_SUBCOMMAND_0X04: &[u8] = &[0x83, 0x04];
const SWITCH_PRO_SUBCOMMAND_0X21: &[u8] =
    &[0xa0, 0x21, 0x01, 0x00, 0xff, 0x00, 0x08, 0x00, 0x1B, 0x01];
const SWITCH_PRO_SUBCOMMAND_0X22: &[u8] = &[0x80, 0x22];

pub fn get_response(
    _recv: &[u8],
    _input_buffer: &[u8],
    _counter: u8,
    response: &mut [u8],
) -> Result<(u8, usize)> {
    let mut vec: Vec<u8, 64> = Vec::new();
    match _recv[0] {
        0x80 => match _recv[1] {
            0x01 => {
                vec.extend_from_slice(&[0x81, _recv[1]]).unwrap();
                vec.extend_from_slice(&[0x00, 0x03]).unwrap();
                vec.extend_from_slice(&SWITCH_PRO_SUBCOMMAND_0X02[6..12])
                    .unwrap();
            }
            0x02 | 0x03 => {
                vec.extend_from_slice(&[0x81, _recv[1]]).unwrap();
            }
            0x04 => {
                return Ok((1, 0));
            }
            0x05 => {
                return Ok((2, 0));
            }
            _ => return Err(Error::InvalidDataError),
        },
        0x01 => {
            vec.push(0x21).unwrap();
            vec.push(_counter).unwrap();
            vec.extend_from_slice(&_input_buffer).unwrap();
            match _recv[10] {
                0x00 => {
                    vec.extend_from_slice(&SWITCH_PRO_SUBCOMMAND_0X00).unwrap();
                }
                0x01 => {
                    vec.extend_from_slice(&SWITCH_PRO_SUBCOMMAND_0X01).unwrap();
                }
                0x02 => {
                    vec.extend_from_slice(&SWITCH_PRO_SUBCOMMAND_0X02).unwrap();
                }
                0x03 | 0x33 | 0x08 | 0x30 | 0x38 | 0x40 | 0x41 | 0x48 => {
                    // vec.extend_from_slice(&SWITCH_PRO_SUBCOMMAND_0X03[0..1])
                    //     .unwrap();
                    vec.push(SWITCH_PRO_SUBCOMMAND_0X03[0]).unwrap();
                    vec.push(_recv[10]).unwrap();
                }
                0x04 => {
                    vec.extend_from_slice(&SWITCH_PRO_SUBCOMMAND_0X04).unwrap();
                }
                0x21 => {
                    vec.extend_from_slice(&SWITCH_PRO_SUBCOMMAND_0X21).unwrap();
                }
                0x22 => {
                    vec.extend_from_slice(&SWITCH_PRO_SUBCOMMAND_0X22).unwrap();
                }
                0x10 => {
                    vec.extend_from_slice(&[0x90, 0x10]).unwrap();
                    vec.extend_from_slice(&_recv[11..13]).unwrap();
                    vec.extend_from_slice(&[0x00, 0x00]).unwrap();
                    match _recv[12] {
                        0x80 => match _recv[11] {
                            0x10 => {
                                const LEN: u8 = 24;
                                vec.push(LEN).unwrap();
                                vec.extend_from_slice(
                                    &SWITCH_PRO_SPI_MAP_0X80
                                        [_recv[10] as usize.._recv[11] as usize + LEN as usize],
                                )
                                .unwrap();
                            }
                            _ => return Err(Error::InvalidDataError),
                        },
                        0x60 => match _recv[11] {
                            0x00 => {
                                const LEN: u8 = 16;
                                vec.push(LEN).unwrap();
                                vec.extend_from_slice(
                                    &SWITCH_PRO_SPI_MAP_0X60
                                        [_recv[11] as usize.._recv[11] as usize + LEN as usize],
                                )
                                .unwrap();
                            }
                            0x20 => {
                                const LEN: u8 = 24;
                                vec.push(LEN).unwrap();
                                vec.extend_from_slice(
                                    &SWITCH_PRO_SPI_MAP_0X60
                                        [_recv[11] as usize.._recv[11] as usize + LEN as usize],
                                )
                                .unwrap();
                            }
                            0x3d => {
                                const LEN: u8 = 18;
                                vec.push(LEN).unwrap();
                                vec.extend_from_slice(
                                    &SWITCH_PRO_SPI_MAP_0X60
                                        [_recv[11] as usize.._recv[11] as usize + LEN as usize],
                                )
                                .unwrap();
                            }
                            0x50 => {
                                const LEN: u8 = 12;
                                vec.push(LEN).unwrap();
                                vec.extend_from_slice(
                                    &SWITCH_PRO_SPI_MAP_0X60
                                        [_recv[11] as usize.._recv[11] as usize + LEN as usize],
                                )
                                .unwrap();
                            }
                            0x80 => {
                                const LEN: u8 = 6;
                                vec.push(LEN).unwrap();
                                vec.extend_from_slice(
                                    &SWITCH_PRO_SPI_MAP_0X60
                                        [_recv[11] as usize.._recv[11] as usize + LEN as usize],
                                )
                                .unwrap();
                            }
                            0x86 => {
                                const LEN: u8 = 18;
                                vec.push(LEN).unwrap();
                                vec.extend_from_slice(
                                    &SWITCH_PRO_SPI_MAP_0X60
                                        [_recv[11] as usize.._recv[11] as usize + LEN as usize],
                                )
                                .unwrap();
                            }
                            0x98 => {
                                const LEN: u8 = 18;
                                vec.push(LEN).unwrap();
                                vec.extend_from_slice(
                                    &SWITCH_PRO_SPI_MAP_0X60
                                        [_recv[11] as usize.._recv[11] as usize + LEN as usize],
                                )
                                .unwrap();
                            }
                            _ => return Err(Error::InvalidDataError),
                        },
                        _ => return Err(Error::InvalidDataError),
                    }
                }
                _ => return Err(Error::InvalidDataError),
            }
        }
        _ => return Err(Error::InvalidDataError),
    }
    if vec.len() > 0 {
        response[0..vec.len()].copy_from_slice(&vec[..]);
    }
    return Ok((0, vec.len()));
}
