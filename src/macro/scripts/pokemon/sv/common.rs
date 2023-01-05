use crate::r#macro::scripts::r#struct::Script;

const OPEN_GAME_TEXT: &str = "
A:0.1
1.5
A:0.1
19
A:0.1
1
A:0.1
1
A:0.1
21
";
const RESTART_GAME_TEXT: &str = "
[common.close_game]
[pokemon.sv.open_game]
";

pub fn open_game() -> Script {
    Script::new("pokemon.sv.open_game", "", OPEN_GAME_TEXT)
}

pub fn restart_game() -> Script {
    Script::new("pokemon.sv.restart_game", "", RESTART_GAME_TEXT)
}
