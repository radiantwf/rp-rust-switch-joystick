ARG VARIANT=stable-slim
FROM debian:${VARIANT}
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8

# Arguments
ARG CONTAINER_USER=rp2040
ARG CONTAINER_GROUP=rp2040
# ARG TOOLCHAIN_VERSION=1.64.0.0
# ARG ESP_IDF_VERSION=release/v4.4
# ARG ESP_BOARD=esp32
# ARG INSTALL_RUST_TOOLCHAIN=install-rust-toolchain.sh

# Install dependencies
RUN apt-get update \
    && apt-get install -y git curl gcc clang ninja-build libudev-dev unzip xz-utils ca-certificates \
    python3 python3-pip python3-venv libusb-1.0-0 libssl-dev pkg-config libtinfo5 libpython2.7 \
    libusb-1.0-0-dev libftdi1-dev \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/* /tmp/library-scripts

# Set users
RUN adduser --disabled-password --gecos "" ${CONTAINER_USER}
USER ${CONTAINER_USER}
WORKDIR /home/${CONTAINER_USER}

# Install Rust toolchain, extra crates and esp-idf
ENV PATH=${PATH}:/home/${CONTAINER_USER}/.cargo/bin:/home/${CONTAINER_USER}/opt/bin

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN echo '[source.crates-io]\nreplace-with = "tuna"\n\n[source.tuna]\nregistry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"' | tee /home/${CONTAINER_USER}/.cargo/config
RUN rustup target install thumbv6m-none-eabi
RUN cargo install flip-link probe-run cargo-embed probe-rs-debugger
RUN cargo install elf2uf2-rs --locked

# ADD --chown=${CONTAINER_USER}:${CONTAINER_GROUP} \
#     https://github.com/esp-rs/rust-build/releases/download/v${TOOLCHAIN_VERSION}/${INSTALL_RUST_TOOLCHAIN} \
#     /home/${CONTAINER_USER}/${INSTALL_RUST_TOOLCHAIN}

# RUN chmod a+x ${INSTALL_RUST_TOOLCHAIN} \
#     && ./${INSTALL_RUST_TOOLCHAIN} \
#     --extra-crates "ldproxy cargo-espflash wokwi-server web-flash" \
#     --export-file /home/${CONTAINER_USER}/export-esp.sh \
#     --esp-idf-version "${ESP_IDF_VERSION}" \
#     --minified-esp-idf "YES" \
#     --build-target "${ESP_BOARD}" \
#     && rustup component add clippy rustfmt

# # Activate ESP environment
# RUN echo "source /home/${CONTAINER_USER}/export-esp.sh" >> ~/.bashrc

CMD [ "/bin/bash" ]
