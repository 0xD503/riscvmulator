## 1. DEV stage   --------------------------------------------------------------

ARG DISTRO_RELEASE_VER=trixie

## use fixed `major`/`minor`, but latest `patch` version
FROM docker.io/library/rust:1.99-slim-${DISTRO_RELEASE_VER} AS builder

## declare stage vars (doesn't affect runtime)
ARG WD=/app
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
    clang lld mold make llvm llvm-bolt \
    #perf bloaty gperf \
    git \
    less patch \
    python3 \
    #sshd \
    ## convenience tools \
    tmux emacs-nox ripgrep fd-find fzf htop lldb \
    vim gdb \
    curl openssh-server \
    unzip \
    ## RISC-V Arch test deps
    #autoconf automake autotools-dev python3-pip  \
    #python3-tomli libmpc-dev libmpfr-dev libgmp-dev gawk build-essential bison   \
    #flex texinfo libtool patchutils bc zlib1g-dev libexpat-dev ninja-build \
    #cmake libglib2.0-dev libslirp-dev libncurses-dev
    ## clean apt cache and install some crates \
    && rm -rf /var/lib/apt/lists/* \
    && rustup component add clippy rustfmt rust-src rust-analyzer \
    && rustup target add riscv64gc-unknown-none-elf \
    && cargo install --locked just \
    && cargo install --locked wild-linker \
    && cargo install --locked flamegraph \
    && cargo install --locked tokei \
    && cargo install --locked cargo-bloat \
    && cargo install --locked cargo-show-asm \
    && cargo install --locked cargo-deny \
    && cargo install --locked --features cli cargo-about \
    && cargo install --locked cargo-audit \
    && cargo install --locked cargo-fuzz

ARG CODE_LLDB_VERSION=1.12.3
RUN curl --fail --silent --show-error --location -o /tmp/codelldb.vsix \
    https://github.com/vadimcn/codelldb/releases/download/v${CODE_LLDB_VERSION}/codelldb-linux-x64.vsix \
    && unzip -q /tmp/codelldb.vsix 'extension/*' -d /tmp/codelldb \
    && mv /tmp/codelldb/extension /opt/codelldb \
    && rm -rf /tmp/codelldb /tmp/codelldb.vsix

## add deps for RISC-V arch tests
ARG SAIL_RISCV_VERSION=0.15
ARG MISE_VERSION=v2026.10.8
RUN mkdir -p /opt/sail_riscv \
    && curl --fail --silent --show-error --location https://github.com/riscv/sail-riscv/releases/download/${SAIL_RISCV_VERSION}/sail-riscv-$(uname)-$(arch).tar.gz | tar -xz --directory=/usr/local --strip-components=1 \
    && sail_riscv_sim --version \
    ## mise tools \
    && curl https://mise.jdx.dev/install.sh | MISE_INSTALL_PATH=/usr/local/bin/mise MISE_VERSION=${MISE_VERSION} sh \
    && mise --version \
    ## install tools with mise
    && cd 3rd_party/riscv-arch-test \
    && mise trust .mise.toml \
    && mise install #\
    #&& make tests \
    #&& cd -

RUN groupadd -g ${DEV_GID} ${DEV_GROUPNAME} \
    && useradd -m -u ${DEV_UID} -g ${DEV_GID} -s /bin/bash ${DEV_USERNAME} \
    ## switch ASLR for debuggers \
    && echo "settings set target.disable-aslr false" \
        > /home/${DEV_USERNAME}/.lldbinit \
    && echo "set disable-randomization off" \
        > /home/${DEV_USERNAME}/.gdbinit \
    && chown ${DEV_UID}:${DEV_GID} \
        /home/${DEV_USERNAME}/.lldbinit \
        /home/${DEV_USERNAME}/.gdbinit

## set WORKDIR
WORKDIR ${WD}

## mark "inside dev container" for the justfile's RUN_IN_CONTAINER detection
ENV IN_DEV_CONTAINER=1

RUN echo 'export PATH="$CARGO_HOME/bin:$PATH"' > /etc/profile.d/cargo.sh

## make cargo registry to be owned by user
RUN chown -R ${DEV_UID}:${DEV_GID} /usr/local/cargo
## switch to dev user
USER ${DEV_USERNAME}


## 2. BUILD stage   ------------------------------------------------------------

FROM builder AS release-build

ARG BUILD_TYPE=release

## build as root user
USER root

## copy dependencies to WORKDIR (to cache them)
## putting this before copying all source lets Podman cache dependency
## resolution so rebuilds are faster when only source changes
## create a fake src/ directory and a minimal main.rs; this lets
## `cargo build --${BUILD_TYPE}` succeed even though the full source is not yet in the
## context
COPY Cargo.toml Cargo.lock .
COPY crates/riscvmulator-soc/Cargo.toml crates/riscvmulator-soc/
COPY crates/riscvmulator-emulator/Cargo.toml crates/riscvmulator-emulator/
COPY crates_guest/riscvmulator-simple_firmware/Cargo.toml \
     crates_guest/riscvmulator-simple_firmware/
COPY .cargo .cargo
RUN mkdir -p src \
             crates/riscvmulator-soc/src \
             crates/riscvmulator-emulator/src \
             crates_guest/riscvmulator-simple_firmware/src \
    && echo "fn main() {}" > src/main.rs \
    && echo "fn main() {}" > crates_guest/riscvmulator-simple_firmware/src/main.rs \
    && touch src/lib.rs \
             crates/riscvmulator-soc/src/lib.rs \
             crates/riscvmulator-emulator/src/lib.rs \
             crates_guest/riscvmulator-simple_firmware/src/lib.rs
RUN cargo build --${BUILD_TYPE}
RUN rm -rf src crates crates_guest

## copy the actual app sources to WORKDIR and build the app
## we `touch` file becuse the timestamp on src/main.rs might not change after
## removal if content is identical. `touch` guarantees forced build rerun
COPY . .
RUN find src crates crates_guest -name "*.rs" -exec touch {} + \
    && cargo build --${BUILD_TYPE}


## 2. RUN stage     ------------------------------------------------------------

FROM docker.io/library/debian:${DISTRO_RELEASE_VER}-slim AS runner

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

COPY --from=release-build ${WD}/target/${BUILD_TYPE}/${APP_NAME} .

## set runtime environment variables
ENV RVMULATOR_EMULATOR__LOGGING__LEVEL=${RELEASE_LOG_LEVEL}
ENV APP_NAME=${APP_NAME}

## set listening port for container
EXPOSE ${LISTEN_PORT}

### run application and pass default arguments that can be overriden fro CLI
#ENTRYPOINT ["./${APP_NAME_ENV}"]
## fixed ENTRYPOINT - ENV vars don't expand in JSON array form
ENTRYPOINT ["sh", "-c", "./$APP_NAME \"$@\"", "--"]
CMD [""]
