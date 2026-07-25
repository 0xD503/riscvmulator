## 1. BUILD stage   ------------------------------------------------------------

ARG DISTRO_RELEASE_VER=trixie
ARG DEV_CONTAINER_NAME=builder
ARG RELEASE_CONTAINER_NAME=runner

## use fixed `major`/`minor`, but latest `patch` version
FROM rust:1.97-slim-${DISTRO_RELEASE_VER} AS ${DEV_CONTAINER_NAME}

## declare stage vars (doesn't affect runtime)
ARG WD=/app
ARG BUILD_TYPE=release
ARG DEV_UID=1000
ARG DEV_GID=1000
ARG DEV_USERNAME=appdev
ARG DEV_GROUPNAME=appdev


## install RISC-V toolchain, build deps and clean cache to keep the image small
RUN apt-get update && apt-get install --assume-yes \
    --no-install-recommends \
    gcc-riscv64-linux-gnu gcc-riscv64-unknown-elf \
    binutils-riscv64-linux-gnu binutils-riscv64-unknown-elf \
    picolibc-riscv64-unknown-elf newlib-source \
    #gdb-riscv64-linux-gnu \
    clang lld mold make llvm-bolt \
    #perf bloaty \
    git \
    less patch \
    python3 \
    #sshd \
    ## convenience tools
    tmux emacs-nox ripgrep fd-find fzf htop lldb \
    vim gdb \
    ## clean apt cache and install some crates
    && rm -rf /var/lib/apt/lists/* \
    && rustup component add clippy rustfmt rust-src rust-analyzer \
    && cargo install --locked just \
    && cargo install --locked wild-linker \
    && cargo install --locked flamegraph \
    && cargo install --locked cargo-bloat \
    && cargo install --locked cargo-show-asm \
    && cargo install --locked cargo-deny \
    && cargo install --locked cargo-audit \
    && cargo install --locked cargo-fuzz

RUN groupadd -g ${DEV_GID} ${DEV_GROUPNAME} \
    && useradd -m -u ${DEV_UID} -g ${DEV_GID} -s /bin/bash ${DEV_USERNAME}

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

## mark "inside dev container" for the justfile's RUN_IN_CONTAINER detection
ENV IN_DEV_CONTAINER=1

## make cargo registry to be owned by user
RUN chown -R ${DEV_UID}:${DEV_GID} /usr/local/cargo
## switch to dev user
USER ${DEV_USERNAME}


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
RUN apt-get update && apt-get install --assume-yes \
    --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR ${WD}

RUN useradd -m -s ${USER_SHELL} ${USERNAME}

## mark "inside container"
ENV IN_APP_CONTAINER=1

USER ${USERNAME}

COPY --from=${DEV_CONTAINER_NAME} ${WD}/target/${BUILD_TYPE}/${APP_NAME} .

## set runtime environment variables
ENV RVMULATOR_EMULATOR__LOGGING__LEVEL=${RELEASE_LOG_LEVEL}
ENV APP_NAME=${APP_NAME}

## set listening port for container
EXPOSE ${LISTEN_PORT}

### run application and pass default arguments that can be overriden fro CLI
#ENTRYPOINT ["./${APP_NAME_ENV}"]
## fixed ENTRYPOINT — ENV vars don't expand in JSON array form
ENTRYPOINT ["sh", "-c", "./$APP_NAME \"$@\"", "--"]
CMD [""]
