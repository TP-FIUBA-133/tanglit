init:
	npm install

build-native:
	cd native && cargo build --release

build: build-native
	npx tsc

watch:
	npx tsc -watch

check:
	npx tsc --noEmit
	cd native && cargo clippy -- -D warnings
	cd native && cargo fmt -- --check

format:
	cd native && cargo fmt

package: build
	npx vsce package

clean:
	rm -rf out
	cd native && cargo clean
