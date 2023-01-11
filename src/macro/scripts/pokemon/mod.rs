use heapless::Vec;

use super::r#struct::Script;

mod sv;

pub fn get_pokemon_scripts() -> Vec<Script, 2> {
    let mut vec = Vec::<Script, 2>::new();
    let scripts1 = sv::get_pokemon_sv_scripts();
    for script in scripts1 {
        vec.push(script).unwrap();
    }
    vec
}
