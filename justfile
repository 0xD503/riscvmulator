build-dev:
  cargo build

run-dev:
  RUST_LOG=debug cargo run --release

build:
  cargo build --release

run:
  cargo run --release

clean:
  cargo clean

fmt:
  cargo fmt

lint:
  cargo clippy -- -D warnings
