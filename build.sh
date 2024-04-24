#!/bin/bash
rustup update
rustup target install thumbv6m-none-eabi 
cargo install flip-link probe-run cargo-embed probe-rs-debugger
cargo install elf2uf2-rs --locked

cargo build --release
rp-rust-switch-joystick % elf2uf2-rs target/thumbv6m-none-eabi/release/rp2040-project-template rp-rust-switch-joystick.uf2
