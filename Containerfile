## 1. BUILD stage   ------------------------------------------------------------

ARG DISTRO_RELEASE_VER=trixie
ARG DEV_CONTAINER_NAME=builder
ARG RELEASE_CONTAINER_NAME=runner

## use fixed `major`/`minor`, but latest `patch` version
FROM rust:1.95-slim-${DISTRO_RELEASE_VER} AS ${DEV_CONTAINER_NAME}

## declare stage vars (doesn't affect runtime)
ARG WD=/app
ARG BUILD_TYPE=release


## install RISC-V toolchain, build deps and clean cache to keep the image small
RUN apt update && apt upgrade --assume-yes && apt install --assume-yes \
    --no-install-recommends \
    gcc-riscv64-linux-gnu gcc-riscv64-unknown-elf \
    binutils-riscv64-linux-gnu binutils-riscv64-unknown-elf \
    picolibc-riscv64-unknown-elf newlib-source \
    #gdb-riscv64-linux-gnu \
    mold make \
    git \
    less patch \
    ## convenience tools
    tmux emacs-nox ripgrep fd-find fzf htop lldb \
    ## some options for perverts
    vim gdb \
    && rm -rf /var/lib/apt/lists/* \
    && rustup component add clippy rustfmt \
    && cargo install just

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

FROM debian:${DISTRO_RELEASE_VER}-slim AS ${RELEASE_CONTAINER_NAME}

## declare stage vars (doesn't affect runtime)
ARG WD=/app
ARG USER_SHELL=/bin/bash
ARG USERNAME=appuser
ARG DEV_CONTAINER_NAME=builder
ARG BUILD_TYPE=release
ARG APP_NAME=riscvmulator
ARG RELEASE_LOG_LEVEL=info
ARG LISTEN_PORT=53732

## install deps and clean cache to keep the image small
RUN apt update && apt upgrade --assume-yes && apt install --assume-yes \
    --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR ${WD}

RUN useradd -m -s ${USER_SHELL} ${USERNAME}
USER ${USERNAME}

COPY --from=${DEV_CONTAINER_NAME} ${WD}/target/${BUILD_TYPE}/${APP_NAME} .

## set runtime environment variables
ENV RUST_LOG=${RELEASE_LOG_LEVEL}
ENV APP_NAME=${APP_NAME}

## set listening port for container
EXPOSE ${LISTEN_PORT}

### run application and pass default arguments that can be overriden fro CLI
#ENTRYPOINT ["./${APP_NAME_ENV}"]
## fixed ENTRYPOINT — ENV vars don't expand in JSON array form
ENTRYPOINT ["sh", "-c", "./$APP_NAME \"$@\"", "--"]
CMD [""]
