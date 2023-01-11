use heapless::Vec;

use self::{common::get_common_scripts, pokemon::get_pokemon_scripts, r#struct::Script};

mod common;
mod pokemon;
mod r#struct;

pub fn get_scripts() -> Vec<Script, 12> {
    let mut vec = Vec::<Script, 12>::new();
    let scripts = get_common_scripts();
    for script in scripts {
        vec.push(script).unwrap();
    }
    let scripts = get_pokemon_scripts();
    for script in scripts {
        vec.push(script).unwrap();
    }
    vec
}
