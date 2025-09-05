init:
	npm install

run-dev: init
	npm run tauri dev

clean: init
	cd src-tauri && cargo clean

build: init
	npm run tauri build

check:
	npm ci && npm run lint && npx prettier --check src
	cd src-tauri && cargo check && cargo clippy -- -D warnings && cargo fmt -- --check

format:
	npm ci && npx prettier --write src
