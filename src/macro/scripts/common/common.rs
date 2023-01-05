use crate::r#macro::scripts::r#struct::Script;

const WAKEUP_JOYSTICK_TEXT: &str = "
LPress:0.1
1
LPress:0.1
3
";

const RETURN_HOME_TEXT: &str = "
Home:0.1
1.5
";

const RETURN_GAME_TEXT: &str = "
[common.return_home]
A:0.1
1.5
";

const CLOSE_GAME_TEXT: &str = "
[common.return_home]
X:0.1
0.5
A:0.1
5
";
const PRESS_BUTTON_A_TEXT: &str = "
A:0.1
0.2
";
const PRESS_BUTTON_B_TEXT: &str = "
B:0.1
0.2
";

pub fn wakeup_joystick() -> Script {
    Script::new("common.wakeup_joystick", "", WAKEUP_JOYSTICK_TEXT)
}

pub fn return_home() -> Script {
    Script::new("common.return_home", "", RETURN_HOME_TEXT)
}

pub fn return_game() -> Script {
    Script::new("common.return_home", "", RETURN_GAME_TEXT)
}

pub fn close_game() -> Script {
    Script::new("common.close_game", "", CLOSE_GAME_TEXT)
}
pub fn press_button_a() -> Script {
    Script::new("common.press_button_a", "连续点击A", PRESS_BUTTON_A_TEXT)
}
pub fn press_button_b() -> Script {
    Script::new("common.press_button_b", "连续点击B", PRESS_BUTTON_B_TEXT)
}
