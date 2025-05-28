init:
	npm install

run-dev: init
	npm run tauri dev

clean: init
	cd src-tauri
	cargo clean

build-debug: init
	npm run tauri build -- --debug

build-release: init
	npm run tauri build --release
