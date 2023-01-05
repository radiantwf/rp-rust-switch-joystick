use super::r#struct::Script;

pub fn restart() -> Script {
    let action_text = "";
    Script::new("pokemon.sv.restart", action_text)
}
