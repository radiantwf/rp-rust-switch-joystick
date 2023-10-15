#!/bin/bash
cargo build --release
rp-rust-switch-joystick % elf2uf2-rs target/thumbv6m-none-eabi/release/rp2040-project-template rp-rust-switch-joystick.uf2
