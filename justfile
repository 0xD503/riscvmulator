## load `.env` file
set dotenv-load

### set timestamps for each command
#set timestamp := true


## detect if we're already inside a container
## if so, run commands directly; if not, run them via {{CONTAINER_ENGINE}}-compose
## Podman automatically sets `$container` variable to `"podman"`
DEV_SERVICE_NAME := env_var_or_default("DEV_SERVICE_NAME", "dev")
IS_IN_CONTAINER := env_var_or_default("container", "")
CONTAINER_ENGINE := env_var_or_default("CONTAINER_ENGINE", "podman")
COMPOSE := CONTAINER_ENGINE + "-compose run --rm"
RUN_IN_CONTAINER := if IS_IN_CONTAINER != "" { "" } else { COMPOSE + " " + DEV_SERVICE_NAME + " " }
RUN_IN_CONTAINER_DEBUG := if IS_IN_CONTAINER != "" { "RUST_LOG=debug" } else { COMPOSE + " -e RUST_LOG=debug " + DEV_SERVICE_NAME + " " }
RUN_IN_CONTAINER_TRACE := if IS_IN_CONTAINER != "" { "RUST_LOG=trace" } else { COMPOSE + " -e RUST_LOG=trace " + DEV_SERVICE_NAME + " " }


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
  {{RUN_IN_CONTAINER_DEBUG}}cargo run --release

run-dev-trace:
  {{RUN_IN_CONTAINER_TRACE}}cargo run --release

build-deb:
  {{RUN_IN_CONTAINER}}cargo build

run-deb:
  {{RUN_IN_CONTAINER_DEBUG}}cargo run

run-deb-trace:
  {{RUN_IN_CONTAINER_TRACE}}cargo run


## release

build:
  {{RUN_IN_CONTAINER}}cargo build --release

run:
  {{RUN_IN_CONTAINER}}cargo run --release

run-release-container:
  #!/bin/env sh
  if [ -n "$container" ]; then
    echo "Already inside a container, do nothing..."
  else
    {{CONTAINER_ENGINE}}-compose run --rm --entrypoint ${USER_SHELL} emulator
  fi


## quality

test:
  {{RUN_IN_CONTAINER}}cargo test --release

bench:
  {{RUN_IN_CONTAINER}}cargo bench

lint:
  {{RUN_IN_CONTAINER}}cargo clippy -- -D warnings

fmt:
  {{RUN_IN_CONTAINER}}cargo fmt

doc:
  {{RUN_IN_CONTAINER}}cargo doc --release --no-deps --open

check:
  {{RUN_IN_CONTAINER}}cargo check --release


## clean

clean:
  {{RUN_IN_CONTAINER}}cargo clean


## container management

## build container images
container-build:
  {{CONTAINER_ENGINE}}-compose build

## remove containers
container-clean:
  {{CONTAINER_ENGINE}} kill --all


## optimization

## post-link optimization (PLOT)
bolt:
  {{RUN_IN_CONTAINER}}llvm-bolt target/release/riscvmulator -o target/release/riscvmulator-bolt


## miscelaneous

##
