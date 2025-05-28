.PHONY: lint test format run run-release

BACKEND_SRC_FILES := $(shell find ./src -type f)

lint:
	cargo clippy -- -D warnings

test:
	cargo test

format:
	cargo fmt

run: ./target/debug/backend
	./target/debug/backend

./target/debug/backend: $(BACKEND_SRC_FILES)
	cargo build

run-release: ./target/release/backend
	./target/release/backend

./target/release/backend: $(BACKEND_SRC_FILES)
	cargo build --release
