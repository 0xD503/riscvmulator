build-dev:
  cargo build

run-dev:
  RUST_LOG=debug cargo run --release

run-dev-deb:
  RUST_LOG=debug cargo run

run-trace:
  RUST_LOG=trace cargo run --release

run-trace-deb:
  RUST_LOG=trace cargo run

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

bench:
  cargo bench
