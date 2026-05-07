## load `.env` file
set dotenv-load

### set timestamps for each command
#set timestamp := true


## detect if we're already inside a container
## if so, run commands directly; if not, run them via {{CONTAINER_ENGINE}}-compose
## Podman automatically sets `$container` variable to `"podman"`
IS_IN_CONTAINER := env_var_or_default("container", "")
CONTAINER_ENGINE := env_var_or_default("CONTAINER_ENGINE", "podman")
COMPOSE := CONTAINER_ENGINE + "-compose run --rm dev"
RUN_IN_CONTAINER := if IS_IN_CONTAINER != "" { "" } else { COMPOSE + " " }


## development

## drop into a development container shell (no-op if already inside)
run-dev-container:
  #!/usr/bin/env bash
  if [ -n "$container" ]; then
    echo "Already inside a container, do nothing..."
  else
    {{CONTAINER_ENGINE}}-compose run --rm dev
  fi

build-dev:
  {{RUN_IN_CONTAINER}}cargo build --release

run-dev:
  RUST_LOG=debug {{RUN_IN_CONTAINER}}cargo run --release

run-dev-trace:
  RUST_LOG=trace {{RUN_IN_CONTAINER}}cargo run --release

build-deb:
  {{RUN_IN_CONTAINER}}cargo build

run-deb:
  RUST_LOG=debug {{RUN_IN_CONTAINER}}cargo run

run-deb-trace:
  RUST_LOG=trace {{RUN_IN_CONTAINER}}cargo run


## release

build:
  {{RUN_IN_CONTAINER}}cargo build --release

run:
  {{RUN_IN_CONTAINER}}cargo run --release

run-release-container:
  #!${USER_SHELL}
  if [ -n "$container" ]; then
    echo "Already inside a container, do nothing..."
  else
    {{CONTAINER_ENGINE}}-compose run --rm --entrypoint ${USER_SHELL} emulator
  fi


## quality

test:
  {{RUN_IN_CONTAINER}}cargo test

bench:
  {{RUN_IN_CONTAINER}}cargo bench

lint:
  {{RUN_IN_CONTAINER}}cargo clippy -- -D warnings

fmt:
  {{RUN_IN_CONTAINER}}cargo fmt

doc:
  {{RUN_IN_CONTAINER}}cargo doc --no-deps --open

check:
  {{RUN_IN_CONTAINER}}cargo check


## clean

clean:
  {{RUN_IN_CONTAINER}}cargo clean


## container management

## build container images
container-build:
  {{CONTAINER_ENGINE}}-compose build

## remove container images
container-clean:
  {{CONTAINER_ENGINE}} kill --all


## optimization

## post-link optimization (PLOT)
bolt:
  {{RUN_IN_CONTAINER}}llvm-bolt target/release/riscvmulator -o target/release/riscvmulator-bolt


## miscelaneous

##
