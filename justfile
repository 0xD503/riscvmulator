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
export DEV_USERNS_MODE := if CONTAINER_ENGINE == "podman" { "keep-id:uid=1000,gid=1000" } else { "host" }
COMPOSE_SUBCMD := if CONTAINER_ENGINE == "podman" { "-compose" } else { " compose" }
COMPOSE_CMD := CONTAINER_ENGINE + COMPOSE_SUBCMD
COMPOSE := COMPOSE_CMD + " run --rm"
ENSURE_DEV_IMAGE_EXISTS := just_executable() + " --justfile " + quote(justfile()) + " " + "ensure-dev-image-exists" + " && "
COMMON_ENV := "JUST_TIME=true"
RUN_UNDER_NICE := "nice"
RUN_IN_CONTAINER := if IS_IN_CONTAINER != "" { "" } else { ENSURE_DEV_IMAGE_EXISTS + COMPOSE + " " + DEV_SERVICE_NAME + " " + RUN_UNDER_NICE + " " }
DEBUG_ENV := "RVMULATOR_EMULATOR__LOGGING__LEVEL=debug RUST_BACKTRACE=full"
TRACE_ENV := "RVMULATOR_EMULATOR__LOGGING__LEVEL=trace RUST_BACKTRACE=full"
RUN_IN_CONTAINER_DEBUG := if IS_IN_CONTAINER != "" { DEBUG_ENV + " " } else { ENSURE_DEV_IMAGE_EXISTS + COMPOSE + " -e " + replace(DEBUG_ENV, " ", " -e ") + " " + DEV_SERVICE_NAME + " " + RUN_UNDER_NICE + " " }
RUN_IN_CONTAINER_TRACE := if IS_IN_CONTAINER != "" { TRACE_ENV + " " } else { ENSURE_DEV_IMAGE_EXISTS + COMPOSE + " -e " + replace(TRACE_ENV, " ", " -e ") + " " + DEV_SERVICE_NAME + " " + RUN_UNDER_NICE + " " }
DEV_IMAGE := env_var("DEV_SERVICE_STARTING_IMAGE_NAME") + ":local"


## development

## drop into a development container shell (no-op if already inside)
run-dev-container: ensure-dev-image-exists
  #!/usr/bin/env sh
  if [ -n "$IN_DEV_CONTAINER" ]; then
    echo "Already inside a container, do nothing..."
  else
    {{COMPOSE_CMD}} run --rm {{DEV_SERVICE_NAME}}
  fi

build-dev:
  {{RUN_IN_CONTAINER}} cargo build --workspace --profile=release-checked

run-dev:
  {{RUN_IN_CONTAINER_DEBUG}} cargo run --profile=release-checked

ci-dev: test run-dev

run-dev-trace:
  {{RUN_IN_CONTAINER_TRACE}} cargo run --profile=release-checked

run-linux-dev: build-dev test
  {{RUN_IN_CONTAINER_DEBUG}} cargo run --profile=release-checked

asm *args:
  {{RUN_IN_CONTAINER_DEBUG}} cargo asm --profile=release {{args}}

build-deb:
  {{RUN_IN_CONTAINER}} cargo build --workspace

run-deb:
  {{RUN_IN_CONTAINER_DEBUG}} cargo run

run-deb-trace:
  {{RUN_IN_CONTAINER_TRACE}} cargo run


## release

build:
  {{RUN_IN_CONTAINER}} cargo build --workspace --profile=release

run:
  {{RUN_IN_CONTAINER}} cargo run --profile=release

run-release-container:
  #!/usr/bin/env sh
  if [ -n "$IN_DEV_CONTAINER" ]; then
    echo "Already inside a container, do nothing..."
  else
    {{COMPOSE_CMD}} run --rm --entrypoint ${USER_SHELL} emulator
  fi


## quality

test:
  {{RUN_IN_CONTAINER_DEBUG}} scripts/riscv_tests.temp.sh
  {{RUN_IN_CONTAINER_DEBUG}} cargo test --workspace --profile=release-checked

test-debug:
  {{RUN_IN_CONTAINER_DEBUG}} scripts/riscv_tests.temp.sh
  {{RUN_IN_CONTAINER_DEBUG}} cargo test --workspace --profile=dev

bench:
  {{RUN_IN_CONTAINER_DEBUG}} cargo bench --workspace  --profile=release-checked

lint:
  {{RUN_IN_CONTAINER}} cargo clippy --workspace --profile=release-checked -- -D warnings

fmt:
  {{RUN_IN_CONTAINER}} cargo fmt --all

doc:
  {{RUN_IN_CONTAINER}} cargo doc --workspace --profile=release --no-deps

check:
  {{RUN_IN_CONTAINER}} cargo check --workspace --all-targets --profile=release-checked

audit:
  {{RUN_IN_CONTAINER}} cargo audit

count-loc *ARGS:
  {{RUN_IN_CONTAINER}} tokei {{ARGS}}


## legal

legal:
  {{RUN_IN_CONTAINER}} cargo deny --workspace check

generate-about: legal
  {{RUN_IN_CONTAINER}} cargo about generate --workspace about.hbs -o THIRD_PARTY_LICENSES.html


## clean

clean:
  {{RUN_IN_CONTAINER}} cargo clean


## container management

## build container images
container-build: && check-devcontainer-image-version
  {{COMPOSE_CMD}} build

## stop and remove this project's compose services
container-clean:
  {{COMPOSE_CMD}} down

## build the dev image if it's missing
[private]
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
  {{COMPOSE_CMD}} build {{DEV_SERVICE_NAME}}
  ## ensure that image was built
  if ! {{CONTAINER_ENGINE}} image inspect {{DEV_IMAGE}} > /dev/null 2>&1; then
    echo 'ERROR: failed to build "{{DEV_IMAGE}}" image' >&2
    exit 1
  fi

## check current container version
[private]
check-devcontainer-image-version:
  #!/usr/bin/env sh
  set -eu
  CURRENT_IMG_VER=$({{CONTAINER_ENGINE}} image inspect {{DEV_IMAGE}} --format '{{{{.Id}}') || exit 0
  for container in $({{CONTAINER_ENGINE}} ps -aq --filter "label=devcontainer.local_folder={{justfile_directory()}}"); do
    if [ "$({{CONTAINER_ENGINE}} inspect "$container" --format '{{{{.Image}}')" != "$CURRENT_IMG_VER" ]; then
      name=$({{CONTAINER_ENGINE}} inspect "$container" --format '{{{{.Name}}')
      echo "WARNING: dev container $name runs an outdated image" >&2
    fi
  done


## optimization

## post-link optimization (PLOT)
bolt:
  {{RUN_IN_CONTAINER}} llvm-bolt target/release/riscvmulator -o target/release/riscvmulator-bolt


## miscelaneous

##
