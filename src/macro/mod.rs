use crate::r#macro::scripts::get_scripts;
use defmt::*;
use defmt_rtt as _;

mod scripts;

pub fn run(script: &str, delay: &mut cortex_m::delay::Delay) {
    info!("script: {}", script);
    let vec = get_scripts();
    for v in vec {
        info!("{}", v.name());
    }
    loop {
        delay.delay_ms(1);
    }
}
