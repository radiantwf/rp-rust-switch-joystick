use crate::r#macro::scripts::r#struct::Script;

const OPENGAME_TEXT: &str = "
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

pub fn open_gmae() -> Script {
    Script::new("pokemon.sv.opengame", OPENGAME_TEXT)
}
