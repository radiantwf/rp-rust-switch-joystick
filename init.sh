#!/bin/bash
git config --global --unset http.proxy
git config --global --unset https.proxy
# cargo install --force --git https://github.com/probe-rs/probe-rs probe-rs-debugger
code --install-extension .devcontainer/probe-rs-debugger-0.4.0.vsix
# export http_proxy=192.168.50.2:10087
# export https_proxy=192.168.50.2:10087