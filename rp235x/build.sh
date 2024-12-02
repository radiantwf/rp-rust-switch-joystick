#!/bin/bash
rustup update
rustup target add thumbv8m.main-none-eabihf
rustup target add riscv32imac-unknown-none-elf
cargo install flip-link
cargo install probe-run
# cargo install cargo-embed
# cargo install probe-rs-debugger
cargo install elf2uf2-rs --locked
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh

cargo build --target thumbv8m.main-none-eabihf --release
elf2uf2-rs target/thumbv8m.main-none-eabihf/release/rp235x-project-template rp235x-switch-joystick.uf2
probe-rs download --chip RP2350 target/thumbv8m.main-none-eabihf/release/rp235x-project-template