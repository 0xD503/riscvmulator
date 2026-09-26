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
DEV_IMAGE := env_var("DEV_SERVICE_STARTING_IMAGE_NAME") + ":local"


## development

## drop into a development container shell (no-op if already inside)
run-dev-container: ensure-dev-image-exists
  #!/usr/bin/env sh
  if [ -n "$container" ]; then
    echo "Already inside a container, do nothing..."
  else
    {{COMPOSE_CMD}} run --rm dev
  fi

build-dev: ensure-dev-image-exists
  {{RUN_IN_CONTAINER}}cargo build --workspace --profile=release-checked

run-dev: ensure-dev-image-exists
  {{RUN_IN_CONTAINER_DEBUG}}cargo run --profile=release-checked

ci-dev: test run-dev

run-dev-trace: ensure-dev-image-exists
  {{RUN_IN_CONTAINER_TRACE}}cargo run --profile=release-checked

run-linux-dev: ensure-dev-image-exists build-dev test
  {{RUN_IN_CONTAINER_DEBUG}}cargo run --profile=release-checked

asm *args: ensure-dev-image-exists
  {{RUN_IN_CONTAINER_DEBUG}}cargo asm --profile=release {{args}}

build-deb: ensure-dev-image-exists
  {{RUN_IN_CONTAINER}}cargo build --workspace

run-deb: ensure-dev-image-exists
  {{RUN_IN_CONTAINER_DEBUG}}cargo run

run-deb-trace: ensure-dev-image-exists
  {{RUN_IN_CONTAINER_TRACE}}cargo run


## release

build: ensure-dev-image-exists
  {{RUN_IN_CONTAINER}}cargo build --workspace --profile=release

run: ensure-dev-image-exists
  {{RUN_IN_CONTAINER}}cargo run --profile=release

run-release-container:
  #!/usr/bin/env sh
  if [ -n "$container" ]; then
    echo "Already inside a container, do nothing..."
  else
    {{COMPOSE_CMD}} run --rm --entrypoint ${USER_SHELL} emulator
  fi


## quality

test: ensure-dev-image-exists
  {{RUN_IN_CONTAINER_DEBUG}}scripts/riscv_tests.temp.sh
  {{RUN_IN_CONTAINER_DEBUG}}cargo test --workspace --profile=release-checked

bench: ensure-dev-image-exists
  {{RUN_IN_CONTAINER_DEBUG}}cargo bench --workspace  --profile=release-checked

lint: ensure-dev-image-exists
  {{RUN_IN_CONTAINER}}cargo clippy --workspace --profile=release-checked -- -D warnings

fmt: ensure-dev-image-exists
  {{RUN_IN_CONTAINER}}cargo fmt --all

doc: ensure-dev-image-exists
  {{RUN_IN_CONTAINER}}cargo doc --workspace --profile=release --no-deps

check: ensure-dev-image-exists
  {{RUN_IN_CONTAINER}}cargo check --workspace --all-targets --profile=release-checked

audit: ensure-dev-image-exists
  {{RUN_IN_CONTAINER}}cargo audit

count *ARGS: ensure-dev-image-exists
  {{RUN_IN_CONTAINER}}tokei {{ARGS}}


## legal

legal: ensure-dev-image-exists
  {{RUN_IN_CONTAINER}}cargo deny --workspace check

generate-about: ensure-dev-image-exists legal
  {{RUN_IN_CONTAINER}}cargo about generate --workspace about.hbs -o THIRD_PARTY_LICENSES.html


## clean

clean: ensure-dev-image-exists
  {{RUN_IN_CONTAINER}}cargo clean


## container management

## build container images
container-build:
  {{COMPOSE_CMD}} build

## stop and remove this project's compose services
container-clean:
  {{COMPOSE_CMD}} down

## build the dev image if it's missing
ensure-dev-image-exists:
  #!/usr/bin/env sh
  ## exit script on error or unexisting var
  set -eu
  ## exit if we're already within a container
  if [ -n "{{IS_IN_CONTAINER}}" ]; then
    exit 0
  fi
  ## ensure that image name within `.devcontainer.json` is the same as
  ## `{{DEV_IMAGE}}`
  ## check if image with name {{DEV_IMAGE}} is present in devcontainer.json
  if ! grep -qF '"image": "{{DEV_IMAGE}}"' .devcontainer/devcontainer.json; then
    echo 'ERROR: image "{{DEV_IMAGE}}" is not set in .devcontainer/devcontainer.json' >&2
    exit 1
  fi
  ## check that development image exists
  if {{CONTAINER_ENGINE}} image inspect {{DEV_IMAGE}} > /dev/null 2>&1; then
    ## development image exists, continue
    exit 0
  fi
  ## development image doesn't exists, build it
  echo 'NOTICE: Development image "{{DEV_IMAGE}}" is missing, building it...' >&2
  {{COMPOSE_CMD}} build {{DEV_IMAGE}}
  ## ensure that image was built
  if ! {{CONTAINER_ENGINE}} image inspect {{DEV_IMAGE}} > /dev/null 2>&1; then
    echo 'ERROR: failed to build "{{DEV_IMAGE}}" image' >&2
    exit 1
  fi


## optimization

## post-link optimization (PLOT)
bolt: ensure-dev-image-exists
  {{RUN_IN_CONTAINER}}llvm-bolt target/release/riscvmulator -o target/release/riscvmulator-bolt


## miscelaneous

##
