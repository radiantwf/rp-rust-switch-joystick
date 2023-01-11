use heapless::Vec;

use super::r#struct::Script;

mod common;

pub fn get_common_scripts() -> Vec<Script, 10> {
    let mut vec = Vec::<Script, 10>::new();
    vec.push(common::wakeup_joystick()).unwrap();
    vec.push(common::return_home()).unwrap();
    vec.push(common::return_game()).unwrap();
    vec.push(common::close_game()).unwrap();
    vec.push(common::press_button_a()).unwrap();
    vec.push(common::press_button_b()).unwrap();
    vec
}
