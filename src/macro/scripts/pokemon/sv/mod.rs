use heapless::Vec;

use crate::r#macro::scripts::r#struct::Script;

mod common;

pub fn get_pokemon_sv_scripts() -> Vec<Script, 2> {
    let mut vec = Vec::<Script, 2>::new();
    vec.push(common::open_game()).unwrap();
    vec.push(common::restart_game()).unwrap();
    vec
}
