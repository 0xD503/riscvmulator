## 0. prepare container BUILD stage environment (doesn't affect runtime)
ARG USERNAME=appuser
ARG SHELL=/bin/bash
ARG WD=/app
ARG APP_NAME=riscvmulator
ARG LOG_LEVEL=info
ARG LISTEN_PORT=53732
ARG BUILD_TYPE=release


## 1. BUILD stage   ------------------------------------------------------------

FROM rust:slim AS builder

## install RISC-V toolchain, build deps and clean cache to keep the image small
RUN apt update && apt upgrade --assume-yes && apt install --assume-yes \
    --no-install-recommends \
    gcc-riscv64-linux-gnu gcc-riscv64-unknown-elf \
    binutils-riscv64-linux-gnu binutils-riscv64-unknown-elf \
    #gdb-riscv64-linux-gnu \
    make \
    git \
    less patch \
    && rm -rf /var/lib/apt/lists/*    \

## set WORKDIR
WORKDIR ${WD}

## copy dependencies to WORKDIR (to cache them)
## putting this before copying all source lets Podman cache dependency
## resolution so rebuilds are faster when only source changes
## create a fake src/ directory and a minimal main.rs; this lets
## `cargo build --${BUILD_TYPE}` succeed even though the full source is not yet in the
## context
## this is a trick to pre‑build dependencies without the full app code
COPY Cargo.toml Cargo.lock .
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --${BUILD_TYPE}
RUN rm -rf src

## copy the actual app sources to WORKDIR and build the app
## we `touch` file becuse the timestamp on src/main.rs might not change after
## removal if content is identical. `touch` guarantees forced build rerun
COPY . .
RUN touch src/main.rs && cargo build --${BUILD_TYPE}


## 2. RUN stage     ------------------------------------------------------------

FROM debian:stable-slim as runner

## install deps and clean cache to keep the image small
RUN apt update && apt upgrade --assume-yes && apt install --assume-yes \
    --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -s /bin/bash ${USERNAME}
USER ${USERNAME}

WORKDIR ${WD}

COPY --from=builder ${WD}/target/${BUILD_TYPE}/${APP_NAME} .

## set runtime environment variables
ENV RUST_LOG=${LOG_LEVEL}

## set listening port for container
EXPOSE ${LISTEN_PORT}


## run application and pass default arguments that can be overriden fro CLI
ENTRYPOINT = ["./${APP_NAME}"]
CMD = [""]
