## load `.env` file
set dotenv-load

### set timestamps for each command
#set timestamp := true


## detect if we're already inside a container
## if so, run commands directly; if not, run them via {{COMPOSE_CMD}}
## Podman automatically sets `$container` variable to `"podman"`
## In development container we set `IN_DEV_CONTAINER` var to `1`
DEV_SERVICE_NAME := env_var_or_default("DEV_SERVICE_NAME", "dev")
IS_IN_CONTAINER := env_var_or_default("IN_DEV_CONTAINER", "")
CONTAINER_ENGINE := env_var_or_default("CONTAINER_ENGINE", "podman")
## exported so every recipe hands it to compose (compose.yml reads
## `${DEV_USERNS_MODE:-keep-id}`)
export DEV_USERNS_MODE := if CONTAINER_ENGINE == "podman" { "keep-id" } else { "host" }
COMPOSE_SUBCMD := if CONTAINER_ENGINE == "podman" { "-compose" } else { " compose" }
COMPOSE_CMD := CONTAINER_ENGINE + COMPOSE_SUBCMD
COMPOSE := COMPOSE_CMD + " run --rm"
RUN_IN_CONTAINER := if IS_IN_CONTAINER != "" { "" } else { COMPOSE + " " + DEV_SERVICE_NAME + " " }
RUN_IN_CONTAINER_DEBUG := if IS_IN_CONTAINER != "" { "RVMULATOR_EMULATOR__LOGGING__LEVEL=debug" + " " } else { COMPOSE + " -e RVMULATOR_EMULATOR__LOGGING__LEVEL=debug -e RUST_BACKTRACE=1 " + DEV_SERVICE_NAME + " " }
RUN_IN_CONTAINER_TRACE := if IS_IN_CONTAINER != "" { "RVMULATOR_EMULATOR__LOGGING__LEVEL=trace" + " " } else { COMPOSE + " -e RVMULATOR_EMULATOR__LOGGING__LEVEL=trace -e RUST_BACKTRACE=1 " + DEV_SERVICE_NAME + " " }


## development

## drop into a development container shell (no-op if already inside)
run-dev-container:
  #!/usr/bin/env bash
  if [ -n "$container" ]; then
    echo "Already inside a container, do nothing..."
  else
    {{COMPOSE_CMD}} run --rm dev
  fi

build-dev:
  {{RUN_IN_CONTAINER}}cargo build --profile=release-checked

run-dev:
  {{RUN_IN_CONTAINER_DEBUG}}cargo run --profile=release-checked

run-dev-trace:
  {{RUN_IN_CONTAINER_TRACE}}cargo run --profile=release-checked

asm:
  {{RUN_IN_CONTAINER_DEBUG}}cargo asm --profile=release

build-deb:
  {{RUN_IN_CONTAINER}}cargo build

run-deb:
  {{RUN_IN_CONTAINER_DEBUG}}cargo run

run-deb-trace:
  {{RUN_IN_CONTAINER_TRACE}}cargo run


## release

build:
  {{RUN_IN_CONTAINER}}cargo build --profile=release

run:
  {{RUN_IN_CONTAINER}}cargo run --profile=release

run-release-container:
  #!/bin/env sh
  if [ -n "$container" ]; then
    echo "Already inside a container, do nothing..."
  else
    {{COMPOSE_CMD}} run --rm --entrypoint ${USER_SHELL} emulator
  fi


## quality

test:
  {{RUN_IN_CONTAINER_DEBUG}}cargo test --profile=release-checked

bench:
  {{RUN_IN_CONTAINER_DEBUG}}cargo bench

lint:
  {{RUN_IN_CONTAINER}}cargo clippy --profile=release-checked -- -D warnings

fmt:
  {{RUN_IN_CONTAINER}}cargo fmt

doc:
  {{RUN_IN_CONTAINER}}cargo doc --profile=release --no-deps --open

check:
  {{RUN_IN_CONTAINER}}cargo check --profile=release-checked

audit:
  {{RUN_IN_CONTAINER}}cargo audit


## legal

legal:
  {{RUN_IN_CONTAINER}}cargo deny check


## clean

clean:
  {{RUN_IN_CONTAINER}}cargo clean


## container management

## build container images
container-build:
  {{COMPOSE_CMD}} build

## stop and remove this project's compose services
container-clean:
  {{COMPOSE_CMD}} down


## optimization

## post-link optimization (PLOT)
bolt:
  {{RUN_IN_CONTAINER}}llvm-bolt target/release/riscvmulator -o target/release/riscvmulator-bolt


## miscelaneous

##
