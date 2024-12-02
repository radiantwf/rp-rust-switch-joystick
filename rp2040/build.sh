#!/bin/bash
rustup update
rustup target install thumbv6m-none-eabi
cargo install flip-link
cargo install probe-run
# cargo install cargo-embed
# cargo install probe-rs-debugger
cargo install elf2uf2-rs --locked
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh

cargo build --release
elf2uf2-rs target/thumbv6m-none-eabi/release/rp2040-project-template rp-rust-switch-joystick.uf2
